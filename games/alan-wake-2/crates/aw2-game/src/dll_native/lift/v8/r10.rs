#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_700083eb74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700083eb74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700083eb78); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700083eb7e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700083eb82); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x700083eb86); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700083eb8a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700083eb8e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x700083eb92); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083eb9b); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083eba4); };
            c.w64(8, t3);
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083ebab); };
            c.logic(t4 & 0x1000000u64, 32);
            if c.cc_ne() { blk = 306; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083ebb9); };
            c.cmp(t5, c.r32(3), 32);
            if c.cc_e() { blk = 369; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(7)));
            c.w32(12, c.r32(8));
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700083ebcd); };
            c.w32(15, t6);
            let r7 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r7);
            let r8 = c.r32(12) | 0x2u64;
            c.w32(12, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_ne() { blk = 307; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700083ebe2); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            blk = 7; continue;
        }
        7 => {
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 211; continue; }
            blk = 8; continue;
        }
        8 => {
            let r11 = c.r32(9) & 0x3u64;
            c.w32(9, r11);
            c.logic(r11, 32);
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 133; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083ec05); };
            c.w32(9, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x700083ec09); };
            c.cmp(t13, c.r32(9), 32);
            if c.cc_e() { blk = 129; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083ec16); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            c.w32(15, 0);
            c.logic(0, 32);
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083ec25); };
            c.cmp(t16, c.r32(15), 32);
            if c.cc_ne() { blk = 306; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t17) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083ec30); };
            c.w32(12, t17);
            let r18 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r18);
            c.w32(15, c.r32(12));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700083ec3a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x700083ec3e); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xf00)) else { return RegfileExit::fault(0x700083ec4c); };
            c.cmp(t19, c.r32(9), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083ec55); };
            c.w32(8, t20);
            let r21 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r21);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 306; continue; }
            blk = 15; continue;
        }
        15 => {
            let r22 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r22);
            c.logic(r22, 64);
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083ec6a); };
            c.logic(t23 & 0x1000000u64, 32);
            if c.cc_ne() { blk = 306; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t24) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083ec78); };
            c.w64(5, t24);
            let r25 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r25);
            let Some(t26) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x700083ec80); };
            if !c.push64(t26) { return RegfileExit::fault(0x700083ec80); }
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(3)) { return RegfileExit::fault(0x700083ec83); }
            c.w64(11, c.r64(8));
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083ec8b); };
            c.logic(t27 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            let r28 = c.add(c.r64(12), 0x3fu64, 64);
            c.w64(12, r28);
            blk = 19; continue;
        }
        18 => {
            let Some(t29) = c.ld32(c.r64(12).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700083ec9c); };
            c.w32(12, t29);
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x700083eca1); };
            c.w64(12, t30);
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::jump(c.r64(12));
        }
        20 => {
            let Some(t31) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083ecac); };
            c.w64(5, t31);
            let r32 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r32);
            let Some(t33) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083ecb4); };
            c.logic(t33 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let r34 = c.add(c.r64(12), 0x3fu64, 64);
            c.w64(12, r34);
            blk = 23; continue;
        }
        22 => {
            let Some(t35) = c.ld32(c.r64(12).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700083ecc5); };
            c.w32(12, t35);
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x700083ecca); };
            c.w64(12, t36);
            blk = 23; continue;
        }
        23 => {
            return RegfileExit::jump(c.r64(12));
        }
        24 => {
            c.w64(9, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.logic(c.r8(9) & 0x30u64, 8);
            if c.cc_e() { blk = 46; continue; }
            blk = 25; continue;
        }
        25 => {
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 38; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t37) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083ecef); };
            c.w32(9, t37);
            let r38 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r38);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700083ecfc); };
            c.w64(9, t39);
            blk = 28; continue;
        }
        28 => {
            let Some(t40) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083ed03); };
            c.w32(11, t40);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r41 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r41);
            let Some(t42) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083ed11); };
            c.w32(3, t42);
            let r43 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r43);
            let r44 = c.r64(3) & c.r64(11);
            c.w64(3, r44);
            c.logic(r44, 64);
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 30; continue;
        }
        29 => {
            let r45 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r45);
            c.w64(12, c.r64(7).wrapping_add(c.r64(3)));
            c.w64(15, c.r64(11));
            let r46 = c.r64(15) & c.r64(12);
            c.w64(15, r46);
            c.logic(r46, 64);
            c.w64(3, c.r64(15));
            blk = 30; continue;
        }
        30 => {
            c.w64(12, c.r64(3).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(15, c.r64(12).wrapping_mul(4));
            let Some(t47) = c.ld32(c.r64(9).wrapping_add(c.r64(15)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083ed3c); };
            c.w32(2, t47);
            let Some(t48) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083ed41); };
            c.cmp(t48, c.r32(2), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(2), c.r32(1), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t49) = c.ld32(c.r64(9).wrapping_add(c.r64(15)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083ed4e); };
            c.w32(8, t49);
            c.logic(c.r8(8) & 0x12u64, 8);
            if c.cc_ne() { blk = 322; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t50) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083ed5d); };
            c.w32(8, t50);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(15, c.r64(12).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_ae() { blk = 321; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(8, c.r64(12).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(9).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700083ed7d); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r51 = c.r64(12) & c.r64(9);
            c.w64(12, r51);
            c.logic(r51, 64);
            let Some(t52) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083ed8f); };
            c.logic(t52 & 0x4u64, 8);
            if c.cc_ne() { blk = 416; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083ed9e); }
            return c.ret(0x0, 0x700083ed9f);
        }
        37 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083eda0); };
            c.w64(7, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083eda4); };
            c.w64(2, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083eda8); };
            c.w64(15, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700083edac); };
            c.w64(12, t56);
            blk = 46; continue;
        }
        38 => {
            let Some(t57) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083edb5); };
            c.w32(9, t57);
            let r58 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r58);
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 306; continue; }
            blk = 39; continue;
        }
        39 => {
            let r59 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r59);
            c.logic(r59, 64);
            let Some(t60) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083edca); };
            c.w32(11, t60);
            let r61 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r61);
            let Some(t62) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083edd1); };
            c.w32(11, t62);
            let r63 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r63);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t64) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083eddd); };
            c.w32(3, t64);
            let r65 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r65);
            let Some(t66) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083ede3); };
            c.cmp(t66, 0xbeu64, 16);
            if c.cc_ne() { blk = 323; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t67) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083edef); };
            c.w32(3, t67);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0xa9u64, 64);
            if c.cc_a() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            return RegfileExit::fault(0x700083ee02);
        }
        43 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083ee03); };
            c.w32(3, t68);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t69) = c.ld32(c.r64(9).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x700083ee0a); };
            c.w32(9, t69);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0xa9u64, 64);
            if c.cc_a() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            return RegfileExit::fault(0x700083ee1d);
        }
        45 => {
            let Some(t70) = c.ld32(c.r64(11).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x700083ee1e); };
            c.cmp(t70, c.r32(9), 32);
            if c.cc_ne() { blk = 324; continue; }
            blk = 46; continue;
        }
        46 => {
            let r71 = c.r32(15) & 0x1eu64;
            c.w32(15, r71);
            c.logic(r71, 32);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(15), 0x12u64, 32);
            if c.cc_e() { blk = 98; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t72) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083ee43); };
            c.w32(9, t72);
            let r73 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r73);
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 333; continue; }
            blk = 49; continue;
        }
        49 => {
            let r74 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r74);
            c.logic(r74, 64);
            c.cmp(c.r32(15), 0xcu64, 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(15), 0x4u64, 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(15), 0x6u64, 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(15), 0x8u64, 32);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r32(15), 0xau64, 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 54; continue;
        }
        54 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 327; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t75) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083ee9c); };
            c.w32(8, t75);
            let r76 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r76);
            let Some(t77) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083eea3); };
            c.w32(8, t77);
            c.cmp(c.r16(8), 0x42u64, 16);
            if c.cc_ne() { blk = 59; continue; }
            blk = 56; continue;
        }
        56 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700083eeaf); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(12) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 59; continue; }
            blk = 57; continue;
        }
        57 => {
            if c.cc_ne() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(15, 0x1fffffffffffffu64);
            let r78 = c.add(c.r64(15), c.r64(12), 64);
            c.w64(15, r78);
            c.w64(12, 0x3ffffffffffffeu64);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 327; continue; }
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_e() { blk = 68; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 325; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t79) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083eefc); };
            c.w32(12, t79);
            c.logic(c.r32(12) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 327; continue; }
            blk = 62; continue;
        }
        62 => {
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 327; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r16(8), 0x25u64, 16);
            if c.cc_e() { blk = 67; continue; }
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r16(8), 0x2du64, 16);
            if c.cc_e() { blk = 67; continue; }
            blk = 65; continue;
        }
        65 => {
            c.logic(c.r8(8) & 0x20u64, 8);
            if c.cc_ne() { blk = 327; continue; }
            blk = 66; continue;
        }
        66 => {
            blk = 68; continue;
        }
        67 => {
            let Some(t80) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083ef31); };
            c.w32(1, t80);
            let r81 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r81);
            blk = 68; continue;
        }
        68 => {
            c.w64(3, c.r64(1));
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(9));
            return c.call(0x70008be194, 0x700083ef45, 0x700083ef40);
        }
        69 => {
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083ef45); };
            c.w64(0, t82);
            blk = 36; continue;
        }
        70 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t83) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083ef54); };
            c.w32(8, t83);
            let r84 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r84);
            blk = 73; continue;
        }
        72 => {
            let Some(t85) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083ef5d); };
            c.w32(8, t85);
            let r86 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r86);
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            let r87 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r87);
            c.logic(r87, 64);
            blk = 76; continue;
        }
        75 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 76; continue;
        }
        76 => {
            let Some(t88) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083ef73); };
            c.w32(12, t88);
            let r89 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r89);
            let Some(t90) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083ef7a); };
            c.w32(9, t90);
            let r91 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r91);
            let Some(t92) = c.ld64(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083ef81); };
            c.w64(12, t92);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t93) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083ef8b); };
            c.w32(15, t93);
            let r94 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r94);
            let Some(t95) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083ef92); };
            c.w32(15, t95);
            let r96 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r96);
            blk = 79; continue;
        }
        78 => {
            c.w64(15, c.r64(2));
            blk = 79; continue;
        }
        79 => {
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700083ef9e); };
            c.w64(3, t97);
            let Some(t98) = c.ld8(c.r64(3)) else { return RegfileExit::fault(0x700083efa5); };
            c.cmp(t98, 0x20u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 80; continue;
        }
        80 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083efae); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083efaf); }
            c.w32(1, 0x1u64);
            c.w64(2, c.r64(12));
            c.w64(3, c.r64(9));
            c.w64(7, c.r64(15));
            c.w64(6, c.r64(8));
            return c.call(0x70007fe8d4, 0x700083efc6, 0x700083efc1);
        }
        81 => {
            blk = 36; continue;
        }
        82 => {
            let r99 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r99);
            if let Some(sr) = c.shr(c.r64(12), 0x6u64, 64) { c.w64(12, sr); }
            let r100 = c.r64(12) & 0x3ffu64;
            c.w64(12, r100);
            c.logic(r100, 64);
            let r101 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(8, r101);
            let Some(t102) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083efdd); };
            c.w32(12, t102);
            let r103 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r103);
            let Some(t104) = c.ld32(c.r64(12).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083efe4); };
            c.w32(12, t104);
            let r105 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r105);
            let Some(t106) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083efec); };
            c.w32(8, t106);
            let r107 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r107);
            let Some(t108) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083eff4); };
            c.w64(5, t108);
            let r109 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r109);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700083effc); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083effe); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083f000); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083f001); }
            let Some(t110) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083f002); };
            if !c.push64(t110) { return RegfileExit::fault(0x700083f002); }
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2078)) else { return RegfileExit::fault(0x700083f006); };
            c.w64(3, t111);
            c.w32(0, 0x5u64);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(2)) { return RegfileExit::fault(0x700083f012); }
            return RegfileExit::jump(0x700086ed74);
        }
        83 => {
            let r112 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r112);
            if let Some(sr) = c.shr(c.r64(12), 0x6u64, 64) { c.w64(12, sr); }
            let r113 = c.r64(12) & 0x3ffu64;
            c.w64(12, r113);
            c.logic(r113, 64);
            let r114 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(8, r114);
            let Some(t115) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083f02e); };
            c.w32(9, t115);
            let r116 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r116);
            let Some(t117) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083f035); };
            c.w32(9, t117);
            let r118 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r118);
            let Some(t119) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083f03c); };
            c.w32(8, t119);
            let r120 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r120);
            let Some(t121) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083f044); };
            c.w32(7, t121);
            let r122 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r122);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083f04b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083f04c); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2274, 0x700083f057, 0x700083f052);
        }
        84 => {
            blk = 36; continue;
        }
        85 => {
            let Some(t123) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083f05c); };
            c.w32(8, t123);
            let Some(t124) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083f060); };
            c.w32(12, t124);
            let r125 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r125);
            c.w32(15, c.r32(8));
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 332; continue; }
            blk = 86; continue;
        }
        86 => {
            let r126 = c.r32(15) & 0x180u64;
            c.w32(15, r126);
            c.logic(r126, 32);
            c.cmp(c.r32(15), 0x100u64, 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 88; continue;
        }
        88 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 92; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 329; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t127) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083f0a4); };
            c.w32(8, t127);
            let Some(t128) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083f0a8); };
            c.cmp(t128, c.r32(8), 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 91; continue;
        }
        91 => {
            blk = 328; continue;
        }
        92 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 330; continue; }
            blk = 93; continue;
        }
        93 => {
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700083f0be); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r129 = c.r64(8) & c.r64(9);
            c.w64(8, r129);
            c.logic(r129, 64);
            let Some(t130) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083f0d0); };
            c.logic(t130 & 0x4u64, 8);
            if c.cc_ne() { blk = 413; continue; }
            blk = 95; continue;
        }
        95 => {
            blk = 36; continue;
        }
        96 => {
            c.cmp(c.r32(12), c.r32(0), 32);
            if c.cc_ne() { blk = 331; continue; }
            blk = 97; continue;
        }
        97 => {
            blk = 36; continue;
        }
        98 => {
            let Some(t131) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083f0ee); };
            c.w64(5, t131);
            let r132 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r132);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083f0f6); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083f0f7); }
            let Some(t133) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700083f0f8); };
            if !c.push64(t133) { return RegfileExit::fault(0x700083f0f8); }
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0x2030)) else { return RegfileExit::fault(0x700083f0fc); };
            c.w64(3, t134);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(0)) { return RegfileExit::fault(0x700083f103); }
            c.w32(0, 0x3u64);
            return RegfileExit::jump(0x700086ed74);
        }
        99 => {
            let Some(t135) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083f112); };
            c.w32(8, t135);
            let r136 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r136);
            let Some(t137) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083f119); };
            c.logic(t137 & 0x100000u64, 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t138) = c.ld32(c.r64(8).wrapping_add(0x23)) else { return RegfileExit::fault(0x700083f123); };
            c.w32(9, t138);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 103; continue; }
            blk = 101; continue;
        }
        101 => {
            c.w64(10, c.r64(4));
            let r139 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r139);
            let r140 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r140);
            c.logic(r140, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700083f138); }
            c.w64(1, c.r64(8));
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x1390)) else { return RegfileExit::fault(0x700083f140); };
            c.w64(0, t141);
            c.w64(10, 0x700083f14eu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700083f14e); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700083f152); }
            return c.call(c.r64(0), 0x700083f158, 0x700083f156);
        }
        102 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700083f158); }
            let Some(t142) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083f160); };
            c.w64(4, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083f165); };
            c.w64(0, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083f169); };
            c.w64(2, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083f16d); };
            c.w64(1, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083f171); };
            c.w64(6, t146);
            blk = 103; continue;
        }
        103 => {
            let Some(t147) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083f175); };
            c.w32(8, t147);
            let r148 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r148);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700083f182); };
            c.w64(8, t149);
            blk = 105; continue;
        }
        105 => {
            let Some(t150) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083f189); };
            c.w32(9, t150);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t151) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083f190); };
            c.w32(12, t151);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 334; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t152) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083f1a1); };
            c.w32(15, t152);
            let r153 = c.add(c.r32(15), 0x2u64, 32);
            c.w32(15, r153);
            c.w64(3, c.r64(15));
            let r154 = c.shr(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r154);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let r155 = c.r64(3) & 0xfffffffffffffffeu64;
            c.w64(3, r155);
            c.logic(r155, 64);
            let r156 = c.add(c.r32(3), c.r32(15), 32);
            c.w32(3, r156);
            c.w32(7, c.r32(12));
            c.cmp(c.r32(12), c.r32(3), 32);
            if c.cc_b() { blk = 128; continue; }
            blk = 107; continue;
        }
        107 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 335; continue; }
            blk = 108; continue;
        }
        108 => {
            let r157 = c.sub(c.r32(7), c.r32(15), 32);
            c.w32(7, r157);
            let r158 = c.shr(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r158);
            c.w64(12, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t159) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083f1d6); };
            c.w32(3, t159);
            let r160 = c.r64(12) & 0xfffffffffffffffeu64;
            c.w64(12, r160);
            c.logic(r160, 64);
            c.cmp(c.r32(12), c.r32(3), 32);
            if c.cc_b() { blk = 128; continue; }
            blk = 109; continue;
        }
        109 => {
            c.cmp(c.r64(9), 0x3u64, 64);
            if c.cc_be() { blk = 336; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t161) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083f1f1); };
            c.w32(9, t161);
            let r162 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r162);
            c.w32(12, c.r64(9).wrapping_add(0x2));
            c.cmp(c.r32(12), 0xfffffeu64, 32);
            if c.cc_a() { blk = 128; continue; }
            blk = 111; continue;
        }
        111 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(0x13), c.r32(12)) { return RegfileExit::fault(0x700083f20c); }
            let Some(t163) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083f210); };
            c.w32(12, t163);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 337; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w64(12, (c.r32(15) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700083f227); }
            let Some(t164) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083f22b); };
            c.w32(12, t164);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r165 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r165);
            let Some(t166) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083f239); };
            c.w32(15, t166);
            let r167 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r167);
            let r168 = c.r64(15) & c.r64(12);
            c.w64(15, r168);
            c.logic(r168, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 114; continue;
        }
        113 => {
            let r169 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r169);
            let r170 = c.add(c.r64(15), c.r64(3), 64);
            c.w64(15, r170);
            c.w64(2, c.r64(12));
            let r171 = c.r64(2) & c.r64(15);
            c.w64(2, r171);
            c.logic(r171, 64);
            c.w64(15, c.r64(2));
            blk = 114; continue;
        }
        114 => {
            c.w64(2, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(6, c.r64(2).wrapping_mul(4));
            c.w64(7, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            let Some(t172) = c.ld32(c.r64(8).wrapping_add(c.r64(6)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083f26d); };
            c.w32(11, t172);
            let Some(t173) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083f272); };
            c.cmp(t173, c.r32(11), 32);
            if c.cc_e() { blk = 116; continue; }
            blk = 115; continue;
        }
        115 => {
            let Some(t174) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083f27b); };
            c.cmp(t174, c.r32(11), 32);
            if c.cc_ne() { blk = 113; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t175) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083f284); };
            c.w32(12, t175);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(2), c.r64(12), 64);
            if c.cc_ae() { blk = 338; continue; }
            blk = 117; continue;
        }
        117 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(1)) { return RegfileExit::fault(0x700083f297); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r176 = c.r64(12) & c.r64(8);
            c.w64(12, r176);
            c.logic(r176, 64);
            let Some(t177) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083f2aa); };
            c.logic(t177 & 0x4u64, 8);
            if c.cc_ne() { blk = 410; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t178) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083f2b6); };
            c.w32(12, t178);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_ae() { blk = 339; continue; }
            blk = 120; continue;
        }
        120 => {
            if let Some(sr) = c.shl(c.r64(9), 0x8u64, 64) { c.w64(9, sr); }
            c.w32(12, c.r32(9));
            c.w64(15, c.r64(2).wrapping_mul(4).wrapping_add(0xb));
            c.w64(3, c.r64(0));
            if !c.st32(c.r64(8).wrapping_add(c.r64(15)), c.r32(3)) { return RegfileExit::fault(0x700083f2df); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 122; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r179 = c.r64(0) & c.r64(8);
            c.w64(0, r179);
            c.logic(r179, 64);
            let Some(t180) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083f2f2); };
            c.logic(t180 & 0x4u64, 8);
            if c.cc_ne() { blk = 407; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t181) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083f2fc); };
            c.w32(15, t181);
            let Some(t182) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700083f300); };
            c.cmp(t182, c.r32(15), 32);
            if c.cc_ne() { blk = 125; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t183) = c.ld8(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083f309); };
            c.logic(t183 & 0x1u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 124; continue;
        }
        124 => {
            let r184 = c.r32(12) | 0x20u64;
            c.w32(12, r184);
            c.logic(r184, 32);
            c.w64(9, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 126; continue;
        }
        125 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 126; continue;
        }
        126 => {
            let Some(t185) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083f31b); };
            c.w32(12, t185);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(15, c.r64(2).wrapping_add(0x2));
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_ae() { blk = 340; continue; }
            blk = 127; continue;
        }
        127 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(6)).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x700083f332); }
            c.w64(0, c.r64(3));
            blk = 36; continue;
        }
        128 => {
            let Some(t186) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083f33f); };
            c.w64(5, t186);
            let r187 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r187);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083f347); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083f348); }
            let Some(t188) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700083f349); };
            if !c.push64(t188) { return RegfileExit::fault(0x700083f349); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x700083f34d); }
            let Some(t189) = c.ld64(c.r64(13).wrapping_add(0x2300)) else { return RegfileExit::fault(0x700083f352); };
            c.w64(3, t189);
            c.w32(0, 0x3u64);
            return RegfileExit::jump(0x700086ed74);
        }
        129 => {
            let Some(t190) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083f363); };
            c.w64(5, t190);
            let r191 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r191);
            let Some(t192) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083f36b); };
            c.logic(t192 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            let r193 = c.add(c.r64(8), 0x3fu64, 64);
            c.w64(8, r193);
            blk = 132; continue;
        }
        131 => {
            let Some(t194) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700083f37b); };
            c.w32(8, t194);
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x700083f37f); };
            c.w64(8, t195);
            blk = 132; continue;
        }
        132 => {
            return RegfileExit::jump(c.r64(8));
        }
        133 => {
            c.w64(12, c.r64(0));
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 361; continue; }
            blk = 134; continue;
        }
        134 => {
            let r196 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r196);
            c.logic(r196, 64);
            let Some(t197) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083f39b); };
            c.w32(9, t197);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700083f39f); }
            let Some(t198) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083f3a3); };
            c.cmp(t198, c.r32(9), 32);
            if c.cc_e() { blk = 148; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t199) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083f3b0); };
            c.w32(9, t199);
            let Some(t200) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083f3b4); };
            c.w32(15, t200);
            let r201 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r201);
            c.w32(0, c.r32(9));
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 345; continue; }
            blk = 136; continue;
        }
        136 => {
            let r202 = c.r32(0) & 0x180u64;
            c.w32(0, r202);
            c.logic(r202, 32);
            c.cmp(c.r32(0), 0x100u64, 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 137; continue;
        }
        137 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 143; continue; }
            blk = 138; continue;
        }
        138 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 142; continue; }
            blk = 139; continue;
        }
        139 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 342; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t203) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083f3f5); };
            c.w32(9, t203);
            let Some(t204) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083f3fa); };
            c.cmp(t204, c.r32(9), 32);
            if c.cc_e() { blk = 143; continue; }
            blk = 141; continue;
        }
        141 => {
            blk = 341; continue;
        }
        142 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 343; continue; }
            blk = 143; continue;
        }
        143 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x700083f40f); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r205 = c.r64(9) & c.r64(8);
            c.w64(9, r205);
            c.logic(r205, 64);
            let Some(t206) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083f423); };
            c.logic(t206 & 0x4u64, 8);
            if c.cc_ne() { blk = 404; continue; }
            blk = 145; continue;
        }
        145 => {
            c.w64(0, c.r64(12));
            blk = 36; continue;
        }
        146 => {
            c.cmp(c.r32(15), c.r32(12), 32);
            if c.cc_ne() { blk = 344; continue; }
            blk = 147; continue;
        }
        147 => {
            c.w64(0, c.r64(12));
            blk = 36; continue;
        }
        148 => {
            let Some(t207) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700083f447); };
            c.w32(9, t207);
            let r208 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r208);
            c.w32(15, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), c.r32(15), 32);
            if c.cc_e() { blk = 150; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t209) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083f456); };
            c.cmp(t209, c.r32(15), 32);
            if c.cc_ne() { blk = 346; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t210) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083f460); };
            c.w32(9, t210);
            c.logic(c.r32(9) & 0x1000000u64, 32);
            if c.cc_ne() { blk = 360; continue; }
            blk = 151; continue;
        }
        151 => {
            if let Some(sr) = c.shr(c.r64(9), 0xau64, 64) { c.w64(9, sr); }
            let r211 = c.r64(9) & 0x3ffu64;
            c.w64(9, r211);
            c.logic(r211, 64);
            c.w64(9, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_sub(0x3));
            if let Some(sr) = c.shl(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let Some(t212) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083f485); };
            c.w32(15, t212);
            let r213 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r213);
            let Some(t214) = c.ld32(c.r64(15).wrapping_add(c.r64(9)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083f48c); };
            c.w32(0, t214);
            c.w32(11, c.r32(0));
            c.logic(c.r8(0) & 0x4u64, 8);
            if c.cc_e() { blk = 155; continue; }
            blk = 152; continue;
        }
        152 => {
            let Some(t215) = c.ld32(c.r64(15).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083f498); };
            c.cmp(t215, c.r32(12), 32);
            if c.cc_ne() { blk = 347; continue; }
            blk = 153; continue;
        }
        153 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083f4a3); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r216 = c.r64(9) & c.r64(2);
            c.w64(9, r216);
            c.logic(r216, 64);
            let Some(t217) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083f4b1); };
            c.logic(t217 & 0x4u64, 8);
            if c.cc_ne() { blk = 401; continue; }
            blk = 154; continue;
        }
        154 => {
            blk = 210; continue;
        }
        155 => {
            let r218 = c.r32(11) & 0x380u64;
            c.w32(11, r218);
            c.logic(r218, 32);
            c.cmp(c.r32(11), 0x80u64, 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 156; continue;
        }
        156 => {
            c.cmp(c.r32(11), 0x100u64, 32);
            if c.cc_e() { blk = 166; continue; }
            blk = 157; continue;
        }
        157 => {
            c.cmp(c.r32(11), 0x180u64, 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 158; continue;
        }
        158 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 348; continue; }
            blk = 159; continue;
        }
        159 => {
            blk = 170; continue;
        }
        160 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 352; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t219) = c.ld32(c.r64(15).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083f4fd); };
            c.w32(9, t219);
            let r220 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r220);
            c.w32(15, 0x4u64);
            c.cmp(c.r32(15), c.r32(9), 32);
            if c.cc_e() { blk = 351; continue; }
            blk = 162; continue;
        }
        162 => {
            c.w32(15, 0x2u64);
            c.cmp(c.r32(15), c.r32(9), 32);
            if c.cc_e() { blk = 170; continue; }
            blk = 163; continue;
        }
        163 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 350; continue; }
            blk = 164; continue;
        }
        164 => {
            let r221 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r221);
            c.logic(r221, 64);
            let Some(t222) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083f52d); };
            c.cmp(t222, c.r32(9), 32);
            if c.cc_e() { blk = 170; continue; }
            blk = 165; continue;
        }
        165 => {
            blk = 349; continue;
        }
        166 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 170; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t223) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083f53f); };
            c.w32(9, t223);
            let Some(t224) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083f544); };
            c.cmp(t224, c.r32(9), 32);
            if c.cc_e() { blk = 170; continue; }
            blk = 168; continue;
        }
        168 => {
            c.w64(0, c.r64(12));
            blk = 306; continue;
        }
        169 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 353; continue; }
            blk = 170; continue;
        }
        170 => {
            let r225 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r225);
            if let Some(sr) = c.shr(c.r64(0), 0x13u64, 64) { c.w64(0, sr); }
            let Some(t226) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x700083f565); };
            c.w32(9, t226);
            let Some(t227) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083f56a); };
            c.w32(15, t227);
            let r228 = c.r64(0) & 0x3ffu64;
            c.w64(0, r228);
            c.logic(r228, 64);
            c.cmp(c.r32(11), 0x100u64, 32);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            let r229 = c.add(c.r64(9), c.r64(0), 64);
            c.w64(9, r229);
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_b() { blk = 197; continue; }
            blk = 171; continue;
        }
        171 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 172; continue;
        }
        172 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 174; continue; }
            blk = 173; continue;
        }
        173 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700083f599); }
            blk = 175; continue;
        }
        174 => {
            c.w64(0, c.r64(12));
            let r230 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r230);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 175; continue;
        }
        175 => {
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083f5af); };
            c.w64(0, t231);
            let Some(t232) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700083f5b6); };
            c.w64(3, t232);
            c.w64(1, c.r64(3).wrapping_add(0xc));
            let Some(t233) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083f5bd); };
            c.w64(6, t233);
            let Some(t234) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700083f5c4); };
            c.cmp(t234, c.r64(1), 64);
            if c.cc_be() { blk = 354; continue; }
            blk = 176; continue;
        }
        176 => {
            if !c.st64(c.r64(0), c.r64(1)) { return RegfileExit::fault(0x700083f5cd); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 177; continue;
        }
        177 => {
            let Some(t235) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083f5d4); };
            c.w64(3, t235);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700083f5db); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083f5de); }
            blk = 179; continue;
        }
        178 => {
            c.w64(0, c.r64(12));
            blk = 179; continue;
        }
        179 => {
            let Some(t236) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083f5e8); };
            c.w32(3, t236);
            let r237 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r237);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700083f5ee); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 181; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t238) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083f5f7); };
            c.w32(1, t238);
            let r239 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r239);
            c.w64(6, c.r64(1));
            let r240 = c.r32(6) & 0x3ffu64;
            c.w32(6, r240);
            c.logic(r240, 32);
            let r241 = c.r32(1) & 0x3ffffc00u64;
            c.w32(1, r241);
            c.logic(r241, 32);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(1));
            c.w64(1, c.r64(10));
            blk = 182; continue;
        }
        181 => {
            let r242 = c.shl(c.r32(3), 0x9u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r242);
            let Some(t243) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083f61c); };
            c.w64(1, t243);
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 182; continue;
        }
        182 => {
            let r244 = c.sub(c.r64(9), c.r64(15), 64);
            c.w64(9, r244);
            c.cmp(c.r64(9), c.r64(6), 64);
            if c.cc_b() { blk = 193; continue; }
            blk = 183; continue;
        }
        183 => {
            c.w64(15, c.r64(6).wrapping_add(0x3));
            let Some(t245) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083f635); };
            c.w64(7, t245);
            let Some(t246) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700083f63c); };
            c.w64(11, t246);
            c.w64(12, c.r64(15).wrapping_mul(4).wrapping_add(0x8));
            c.w64(8, c.r64(12).wrapping_add(c.r64(11)));
            let Some(t247) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083f64b); };
            c.w64(0, t247);
            let Some(t248) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700083f652); };
            c.cmp(t248, c.r64(8), 64);
            if c.cc_be() { blk = 356; continue; }
            blk = 184; continue;
        }
        184 => {
            if !c.st64(c.r64(7), c.r64(8)) { return RegfileExit::fault(0x700083f65b); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 185; continue;
        }
        185 => {
            c.w32(12, c.r32(15));
            c.w32(0, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t249) = c.ld64(c.r64(13).wrapping_add(0x250)) else { return RegfileExit::fault(0x700083f669); };
            c.w64(7, t249);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700083f670); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700083f674); }
            c.w64(0, c.r64(6).wrapping_mul(4).wrapping_add(0x7));
            c.w64(15, c.r64(15).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(15), c.r64(0), 64);
            if c.cc_e() { blk = 188; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t250) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083f68d); };
            c.w64(6, t250);
            blk = 187; continue;
        }
        187 => {
            let r251 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r251);
            if !c.st32(c.r64(15).wrapping_add(c.r64(8)), c.r32(6)) { return RegfileExit::fault(0x700083f698); }
            c.cmp(c.r64(15), c.r64(0), 64);
            if c.cc_ne() { blk = 187; continue; }
            blk = 188; continue;
        }
        188 => {
            c.cmp(c.r64(0), 0x7u64, 64);
            if c.cc_e() { blk = 190; continue; }
            blk = 189; continue;
        }
        189 => {
            let r252 = c.sub(c.r64(0), 0x4u64, 64);
            c.w64(0, r252);
            let Some(t253) = c.ld32(c.r64(0).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x700083f6ab); };
            c.w32(15, t253);
            if !c.st32(c.r64(0).wrapping_add(c.r64(8)), c.r32(15)) { return RegfileExit::fault(0x700083f6af); }
            c.cmp(c.r64(0), 0x7u64, 64);
            if c.cc_ne() { blk = 189; continue; }
            blk = 190; continue;
        }
        190 => {
            let r254 = c.r32(12) | c.r32(3);
            c.w32(12, r254);
            c.logic(r254, 32);
            let r255 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r255);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700083f6bf); }
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700083f6c3); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 192; continue; }
            blk = 191; continue;
        }
        191 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r256 = c.r64(12) & c.r64(2);
            c.w64(12, r256);
            c.logic(r256, 64);
            let Some(t257) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083f6d7); };
            c.logic(t257 & 0x4u64, 8);
            if c.cc_ne() { blk = 398; continue; }
            blk = 192; continue;
        }
        192 => {
            c.w64(1, c.r64(8));
            blk = 193; continue;
        }
        193 => {
            c.w64(8, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083f6ee); };
            c.w64(0, t258);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700083f6f2); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 195; continue; }
            blk = 194; continue;
        }
        194 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r259 = c.r64(9) & c.r64(1);
            c.w64(9, r259);
            c.logic(r259, 64);
            let Some(t260) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083f704); };
            c.logic(t260 & 0x4u64, 8);
            if c.cc_ne() { blk = 395; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083f70f); };
            c.w64(8, t261);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083f713); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r262 = c.r64(9) & c.r64(2);
            c.w64(9, r262);
            c.logic(r262, 64);
            let Some(t263) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083f721); };
            c.logic(t263 & 0x4u64, 8);
            if c.cc_ne() { blk = 392; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083f72c); };
            c.w64(12, t264);
            blk = 210; continue;
        }
        197 => {
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 202; continue; }
            blk = 198; continue;
        }
        198 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083f742); }
            c.w64(15, 0xfffffffffffc0000u64);
            let r265 = c.r64(15) & c.r64(2);
            c.w64(15, r265);
            c.logic(r265, 64);
            let Some(t266) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083f750); };
            c.logic(t266 & 0x4u64, 8);
            if c.cc_ne() { blk = 389; continue; }
            blk = 199; continue;
        }
        199 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(12)) { return RegfileExit::fault(0x700083f75b); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 201; continue; }
            blk = 200; continue;
        }
        200 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r267 = c.r64(8) & c.r64(2);
            c.w64(8, r267);
            c.logic(r267, 64);
            let Some(t268) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083f76f); };
            c.logic(t268 & 0x4u64, 8);
            if c.cc_ne() { blk = 386; continue; }
            blk = 201; continue;
        }
        201 => {
            blk = 210; continue;
        }
        202 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 204; continue; }
            blk = 203; continue;
        }
        203 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700083f785); }
            blk = 205; continue;
        }
        204 => {
            c.w64(15, c.r64(12));
            let r269 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r269);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            blk = 205; continue;
        }
        205 => {
            let Some(t270) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083f79d); };
            c.w64(15, t270);
            let Some(t271) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700083f7a4); };
            c.w64(0, t271);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t272) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083f7ab); };
            c.w64(1, t272);
            let Some(t273) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700083f7b2); };
            c.cmp(t273, c.r64(3), 64);
            if c.cc_be() { blk = 358; continue; }
            blk = 206; continue;
        }
        206 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x700083f7bb); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            blk = 207; continue;
        }
        207 => {
            let Some(t274) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083f7c2); };
            c.w64(0, t274);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700083f7c9); }
            if !c.movsd_mr(c.r64(15).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083f7cd); }
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083f7d3); }
            c.w64(0, 0xfffffffffffc0000u64);
            let r275 = c.r64(0) & c.r64(2);
            c.w64(0, r275);
            c.logic(r275, 64);
            let Some(t276) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083f7e1); };
            c.logic(t276 & 0x4u64, 8);
            if c.cc_ne() { blk = 383; continue; }
            blk = 208; continue;
        }
        208 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(15)) { return RegfileExit::fault(0x700083f7eb); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 210; continue; }
            blk = 209; continue;
        }
        209 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r277 = c.r64(8) & c.r64(2);
            c.w64(8, r277);
            c.logic(r277, 64);
            let Some(t278) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083f7ff); };
            c.logic(t278 & 0x4u64, 8);
            if c.cc_ne() { blk = 380; continue; }
            blk = 210; continue;
        }
        210 => {
            c.w64(0, c.r64(12));
            blk = 36; continue;
        }
        211 => {
            c.w64(12, c.r64(8));
            let r279 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r279);
            let r280 = c.r32(9) & 0x1eu64;
            c.w32(9, r280);
            c.logic(r280, 32);
            c.w64(15, c.r64(12));
            let r281 = c.r32(15) & 0xfu64;
            c.w32(15, r281);
            c.logic(r281, 32);
            c.cmp(c.r32(15), 0x6u64, 32);
            if c.cc_l() { blk = 244; continue; }
            blk = 212; continue;
        }
        212 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 228; continue; }
            blk = 213; continue;
        }
        213 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_e() { blk = 227; continue; }
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 226; continue; }
            blk = 215; continue;
        }
        215 => {
            let Some(t282) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083f84b); };
            c.w32(8, t282);
            let r283 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r283);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 217; continue; }
            blk = 216; continue;
        }
        216 => {
            let Some(t284) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700083f858); };
            c.w64(8, t284);
            blk = 217; continue;
        }
        217 => {
            let Some(t285) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083f85f); };
            c.w32(9, t285);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r286 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r286);
            let Some(t287) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083f86d); };
            c.w32(12, t287);
            let r288 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r288);
            let r289 = c.r64(12) & c.r64(9);
            c.w64(12, r289);
            c.logic(r289, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 219; continue;
        }
        218 => {
            let r290 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r290);
            let r291 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r291);
            c.w64(3, c.r64(9));
            let r292 = c.r64(3) & c.r64(12);
            c.w64(3, r292);
            c.logic(r292, 64);
            c.w64(12, c.r64(3));
            blk = 219; continue;
        }
        219 => {
            c.w64(11, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(3, c.r64(11).wrapping_mul(4));
            let Some(t293) = c.ld32(c.r64(8).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083f89a); };
            c.w32(6, t293);
            let Some(t294) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083f89f); };
            c.cmp(t294, c.r32(6), 32);
            if c.cc_e() { blk = 364; continue; }
            blk = 220; continue;
        }
        220 => {
            c.cmp(c.r32(6), c.r32(1), 32);
            if c.cc_ne() { blk = 218; continue; }
            blk = 221; continue;
        }
        221 => {
            let Some(t295) = c.ld32(c.r64(8).wrapping_add(c.r64(3)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083f8b0); };
            c.w32(9, t295);
            c.logic(c.r8(9) & 0x12u64, 8);
            if c.cc_ne() { blk = 363; continue; }
            blk = 222; continue;
        }
        222 => {
            let Some(t296) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083f8bf); };
            c.w32(9, t296);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(12, c.r64(11).wrapping_add(0x1));
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_ae() { blk = 362; continue; }
            blk = 223; continue;
        }
        223 => {
            c.w64(9, c.r64(11).wrapping_mul(4).wrapping_add(0xb));
            c.w64(12, c.r64(0));
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)), c.r32(12)) { return RegfileExit::fault(0x700083f8e1); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 225; continue; }
            blk = 224; continue;
        }
        224 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r297 = c.r64(15) & c.r64(8);
            c.w64(15, r297);
            c.logic(r297, 64);
            let Some(t298) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083f8f5); };
            c.logic(t298 & 0x4u64, 8);
            if c.cc_ne() { blk = 377; continue; }
            blk = 225; continue;
        }
        225 => {
            c.w64(0, c.r64(12));
            blk = 36; continue;
        }
        226 => {
            let Some(t299) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083f908); };
            c.w64(5, t299);
            let r300 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r300);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083f910); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083f911); }
            let Some(t301) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700083f912); };
            if !c.push64(t301) { return RegfileExit::fault(0x700083f912); }
            let Some(t302) = c.ld64(c.r64(13).wrapping_add(0x2030)) else { return RegfileExit::fault(0x700083f916); };
            c.w64(3, t302);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(0)) { return RegfileExit::fault(0x700083f91d); }
            c.w32(0, 0x3u64);
            return RegfileExit::jump(0x700086ed74);
        }
        227 => {
            let Some(t303) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083f92c); };
            c.w64(5, t303);
            let r304 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r304);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083f934); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083f935); }
            let Some(t305) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700083f936); };
            if !c.push64(t305) { return RegfileExit::fault(0x700083f936); }
            let Some(t306) = c.ld64(c.r64(13).wrapping_add(0x20a8)) else { return RegfileExit::fault(0x700083f93a); };
            c.w64(3, t306);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(0)) { return RegfileExit::fault(0x700083f941); }
            c.w32(0, 0x3u64);
            return RegfileExit::jump(0x700086ed74);
        }
        228 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 367; continue; }
            blk = 229; continue;
        }
        229 => {
            let Some(t307) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083f959); };
            c.w32(8, t307);
            let r308 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r308);
            let Some(t309) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083f960); };
            c.w32(8, t309);
            c.cmp(c.r16(8), 0x42u64, 16);
            if c.cc_ne() { blk = 233; continue; }
            blk = 230; continue;
        }
        230 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700083f96c); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 233; continue; }
            blk = 231; continue;
        }
        231 => {
            if c.cc_ne() { blk = 233; continue; }
            blk = 232; continue;
        }
        232 => {
            let Some(t310) = c.ld64(0x700083eeccu64) else { return RegfileExit::fault(0x700083f987); };
            c.w64(12, t310);
            let r311 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r311);
            let Some(t312) = c.ld64(0x700083eed9u64) else { return RegfileExit::fault(0x700083f991); };
            c.w64(9, t312);
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 367; continue; }
            blk = 233; continue;
        }
        233 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_e() { blk = 242; continue; }
            blk = 234; continue;
        }
        234 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 365; continue; }
            blk = 235; continue;
        }
        235 => {
            let Some(t313) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083f9b3); };
            c.w32(9, t313);
            c.logic(c.r32(9) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 367; continue; }
            blk = 236; continue;
        }
        236 => {
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_e() { blk = 367; continue; }
            blk = 237; continue;
        }
        237 => {
            c.cmp(c.r16(8), 0x25u64, 16);
            if c.cc_e() { blk = 241; continue; }
            blk = 238; continue;
        }
        238 => {
            c.cmp(c.r16(8), 0x2du64, 16);
            if c.cc_e() { blk = 241; continue; }
            blk = 239; continue;
        }
        239 => {
            c.logic(c.r8(8) & 0x20u64, 8);
            if c.cc_ne() { blk = 367; continue; }
            blk = 240; continue;
        }
        240 => {
            blk = 242; continue;
        }
        241 => {
            let Some(t314) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083f9e8); };
            c.w32(1, t314);
            let r315 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r315);
            blk = 242; continue;
        }
        242 => {
            c.w64(3, c.r64(1));
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(2));
            return c.call(0x70008be194, 0x700083f9fc, 0x700083f9f7);
        }
        243 => {
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083f9fc); };
            c.w64(0, t316);
            blk = 36; continue;
        }
        244 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_e() { blk = 304; continue; }
            blk = 245; continue;
        }
        245 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_e() { blk = 303; continue; }
            blk = 246; continue;
        }
        246 => {
            c.w64(15, c.r64(12));
            let r317 = c.shr(c.r32(15), 0x11u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r317);
            let r318 = c.r32(15) & 0x7u64;
            c.w32(15, r318);
            c.logic(r318, 32);
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_e() { blk = 294; continue; }
            blk = 247; continue;
        }
        247 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 279; continue; }
            blk = 248; continue;
        }
        248 => {
            c.cmp(c.r32(15), 0x4u64, 32);
            if c.cc_e() { blk = 269; continue; }
            blk = 249; continue;
        }
        249 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 252; continue; }
            blk = 250; continue;
        }
        250 => {
            let Some(t319) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083fa46); };
            c.w32(15, t319);
            let Some(t320) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083fa4a); };
            c.cmp(t320, c.r32(15), 32);
            if c.cc_ne() { blk = 306; continue; }
            blk = 251; continue;
        }
        251 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700083fa57); }
            blk = 253; continue;
        }
        252 => {
            c.w64(15, c.r64(0));
            let r321 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r321);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            blk = 253; continue;
        }
        253 => {
            c.w64(15, c.r64(12));
            if let Some(sr) = c.shr(c.r64(15), 0x6u64, 64) { c.w64(15, sr); }
            let r322 = c.r64(15) & 0x3ffu64;
            c.w64(15, r322);
            c.logic(r322, 64);
            let r323 = c.imul2(c.r64(15), 0x3u64, 64);
            c.w64(15, r323);
            let Some(t324) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083fa7f); };
            c.w32(11, t324);
            let r325 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r325);
            let Some(t326) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083fa86); };
            c.w32(11, t326);
            let r327 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r327);
            let Some(t328) = c.ld32(c.r64(11).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083fa8d); };
            c.w32(15, t328);
            c.w32(15, c.r32(15));
            let r329 = c.r32(15) & 0x380u64;
            c.w32(15, r329);
            c.logic(r329, 32);
            c.cmp(c.r32(15), 0x100u64, 32);
            if c.cc_ne() { blk = 306; continue; }
            blk = 254; continue;
        }
        254 => {
            if let Some(sr) = c.shr(c.r64(12), 0x14u64, 64) { c.w64(12, sr); }
            c.cmp(c.r32(9), 0x2u64, 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let r330 = c.r64(12) & 0x7ffu64;
            c.w64(12, r330);
            c.logic(r330, 64);
            c.logic(c.r32(8) & 0x20000u64, 32);
            if c.cc_ne() { blk = 257; continue; }
            blk = 255; continue;
        }
        255 => {
            let Some(t331) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083fac9); };
            c.w32(8, t331);
            let r332 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r332);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 258; continue; }
            blk = 256; continue;
        }
        256 => {
            let Some(t333) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083fad6); };
            c.w64(8, t333);
            blk = 258; continue;
        }
        257 => {
            c.w64(8, c.r64(2));
            blk = 258; continue;
        }
        258 => {
            let Some(t334) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083fae2); };
            c.w32(8, t334);
            let r335 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r335);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 260; continue; }
            blk = 259; continue;
        }
        259 => {
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083faf0); }
            blk = 36; continue;
        }
        260 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700083fafb); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 262; continue; }
            blk = 261; continue;
        }
        261 => {
            if c.cc_e() { blk = 267; continue; }
            blk = 262; continue;
        }
        262 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 264; continue; }
            blk = 263; continue;
        }
        263 => {
            if c.cc_e() { blk = 306; continue; }
            blk = 264; continue;
        }
        264 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 97; continue; }
            blk = 265; continue;
        }
        265 => {
            if c.cc_e() { blk = 306; continue; }
            blk = 266; continue;
        }
        266 => {
            blk = 97; continue;
        }
        267 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 268; continue;
        }
        268 => {
            blk = 306; continue;
        }
        269 => {
            if let Some(sr) = c.shr(c.r64(12), 0x14u64, 64) { c.w64(12, sr); }
            let r336 = c.r64(12) & 0x7ffu64;
            c.w64(12, r336);
            c.logic(r336, 64);
            c.logic(c.r32(8) & 0x20000u64, 32);
            if c.cc_ne() { blk = 272; continue; }
            blk = 270; continue;
        }
        270 => {
            let Some(t337) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083fb5e); };
            c.w32(8, t337);
            let r338 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r338);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 273; continue; }
            blk = 271; continue;
        }
        271 => {
            let Some(t339) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083fb6b); };
            c.w64(8, t339);
            blk = 273; continue;
        }
        272 => {
            c.w64(8, c.r64(2));
            blk = 273; continue;
        }
        273 => {
            c.w64(15, c.r64(12).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 277; continue; }
            blk = 274; continue;
        }
        274 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(15)), c.r32(0)) { return RegfileExit::fault(0x700083fb85); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 276; continue; }
            blk = 275; continue;
        }
        275 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r340 = c.r64(9) & c.r64(8);
            c.w64(9, r340);
            c.logic(r340, 64);
            let Some(t341) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083fb97); };
            c.logic(t341 & 0x4u64, 8);
            if c.cc_ne() { blk = 374; continue; }
            blk = 276; continue;
        }
        276 => {
            blk = 36; continue;
        }
        277 => {
            let Some(t342) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083fba7); };
            c.cmp(t342, c.r32(0), 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 278; continue;
        }
        278 => {
            blk = 306; continue;
        }
        279 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 306; continue; }
            blk = 280; continue;
        }
        280 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 284; continue; }
            blk = 281; continue;
        }
        281 => {
            c.w64(15, c.r64(12));
            if let Some(sr) = c.shr(c.r64(15), 0x6u64, 64) { c.w64(15, sr); }
            let r343 = c.r64(15) & 0x3ffu64;
            c.w64(15, r343);
            c.logic(r343, 64);
            let r344 = c.imul2(c.r64(15), 0x3u64, 64);
            c.w64(15, r344);
            let Some(t345) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083fbe3); };
            c.w32(11, t345);
            let r346 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r346);
            let Some(t347) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083fbea); };
            c.w32(11, t347);
            let r348 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r348);
            let Some(t349) = c.ld32(c.r64(11).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083fbf1); };
            c.w32(15, t349);
            let r350 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r350);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 284; continue; }
            blk = 282; continue;
        }
        282 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 306; continue; }
            blk = 283; continue;
        }
        283 => {
            let r351 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r351);
            c.logic(r351, 64);
            let Some(t352) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083fc0d); };
            c.cmp(t352, c.r32(15), 32);
            if c.cc_ne() { blk = 306; continue; }
            blk = 284; continue;
        }
        284 => {
            c.logic(c.r32(8) & 0x20000u64, 32);
            if c.cc_ne() { blk = 287; continue; }
            blk = 285; continue;
        }
        285 => {
            let Some(t353) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083fc20); };
            c.w32(8, t353);
            let r354 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r354);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 288; continue; }
            blk = 286; continue;
        }
        286 => {
            let Some(t355) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083fc2d); };
            c.w64(8, t355);
            blk = 288; continue;
        }
        287 => {
            c.w64(8, c.r64(2));
            blk = 288; continue;
        }
        288 => {
            if let Some(sr) = c.shr(c.r64(12), 0x14u64, 64) { c.w64(12, sr); }
            let r356 = c.r64(12) & 0x7ffu64;
            c.w64(12, r356);
            c.logic(r356, 64);
            c.w64(15, c.r64(12).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 292; continue; }
            blk = 289; continue;
        }
        289 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(15)), c.r32(0)) { return RegfileExit::fault(0x700083fc52); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 291; continue; }
            blk = 290; continue;
        }
        290 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r357 = c.r64(9) & c.r64(8);
            c.w64(9, r357);
            c.logic(r357, 64);
            let Some(t358) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083fc64); };
            c.logic(t358 & 0x4u64, 8);
            if c.cc_ne() { blk = 371; continue; }
            blk = 291; continue;
        }
        291 => {
            blk = 36; continue;
        }
        292 => {
            let Some(t359) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083fc74); };
            c.cmp(t359, c.r32(0), 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 293; continue;
        }
        293 => {
            blk = 306; continue;
        }
        294 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 306; continue; }
            blk = 295; continue;
        }
        295 => {
            c.logic(c.r32(8) & 0x20000u64, 32);
            if c.cc_ne() { blk = 298; continue; }
            blk = 296; continue;
        }
        296 => {
            let Some(t360) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083fc95); };
            c.w32(8, t360);
            let r361 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r361);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 299; continue; }
            blk = 297; continue;
        }
        297 => {
            let Some(t362) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083fca2); };
            c.w64(8, t362);
            blk = 299; continue;
        }
        298 => {
            c.w64(8, c.r64(2));
            blk = 299; continue;
        }
        299 => {
            if let Some(sr) = c.shr(c.r64(12), 0x14u64, 64) { c.w64(12, sr); }
            let r363 = c.r64(12) & 0x7ffu64;
            c.w64(12, r363);
            c.logic(r363, 64);
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 301; continue; }
            blk = 300; continue;
        }
        300 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700083fcbf); }
            blk = 36; continue;
        }
        301 => {
            let Some(t364) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083fcc9); };
            c.cmp(t364, c.r32(0), 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 302; continue;
        }
        302 => {
            blk = 306; continue;
        }
        303 => {
            if let Some(sr) = c.shr(c.r64(12), 0x6u64, 64) { c.w64(12, sr); }
            let r365 = c.r64(12) & 0x3ffu64;
            c.w64(12, r365);
            c.logic(r365, 64);
            let r366 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(8, r366);
            let Some(t367) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083fce8); };
            c.w32(9, t367);
            let r368 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r368);
            let Some(t369) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083fcef); };
            c.w32(9, t369);
            let r370 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r370);
            let Some(t371) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083fcf6); };
            c.w32(8, t371);
            let r372 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r372);
            let Some(t373) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083fcfe); };
            c.w64(5, t373);
            let r374 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r374);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083fd06); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083fd07); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083fd09); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083fd0a); }
            let Some(t375) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083fd0b); };
            if !c.push64(t375) { return RegfileExit::fault(0x700083fd0b); }
            let Some(t376) = c.ld64(c.r64(13).wrapping_add(0x2078)) else { return RegfileExit::fault(0x700083fd0f); };
            c.w64(3, t376);
            c.w32(0, 0x5u64);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(2)) { return RegfileExit::fault(0x700083fd1b); }
            return RegfileExit::jump(0x700086ed74);
        }
        304 => {
            if let Some(sr) = c.shr(c.r64(12), 0x6u64, 64) { c.w64(12, sr); }
            let r377 = c.r64(12) & 0x3ffu64;
            c.w64(12, r377);
            c.logic(r377, 64);
            let r378 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(8, r378);
            let Some(t379) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083fd34); };
            c.w32(9, t379);
            let r380 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r380);
            let Some(t381) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083fd3b); };
            c.w32(9, t381);
            let r382 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r382);
            let Some(t383) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083fd42); };
            c.w32(8, t383);
            let r384 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r384);
            let Some(t385) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083fd4a); };
            c.w32(7, t385);
            let r386 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r386);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083fd51); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083fd52); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2274, 0x700083fd5d, 0x700083fd58);
        }
        305 => {
            blk = 36; continue;
        }
        306 => {
            let Some(t387) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083fd62); };
            c.w64(5, t387);
            let r388 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r388);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083fd6a); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700083fd6b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083fd6c); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083fd6d); }
            let Some(t389) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083fd6e); };
            if !c.push64(t389) { return RegfileExit::fault(0x700083fd6e); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(0)) { return RegfileExit::fault(0x700083fd72); }
            c.w32(0, 0x5u64);
            let Some(t390) = c.ld64(c.r64(13).wrapping_add(0x2020)) else { return RegfileExit::fault(0x700083fd7c); };
            c.w64(3, t390);
            return RegfileExit::jump(0x700086ed74);
        }
        307 => {
            c.w32(12, c.r32(15));
            let r391 = c.r32(12) & 0x3u64;
            c.w32(12, r391);
            c.logic(r391, 32);
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_ne() { blk = 306; continue; }
            blk = 308; continue;
        }
        308 => {
            let Some(t392) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083fd95); };
            c.w32(12, t392);
            let Some(t393) = c.ld32(c.r64(13).wrapping_add(0x2c8)) else { return RegfileExit::fault(0x700083fd99); };
            c.cmp(t393, c.r32(12), 32);
            if c.cc_e() { blk = 316; continue; }
            blk = 309; continue;
        }
        309 => {
            let Some(t394) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700083fda2); };
            c.cmp(t394, c.r32(15), 32);
            if c.cc_e() { blk = 369; continue; }
            blk = 310; continue;
        }
        310 => {
            c.cmp(c.r32(15), c.r32(1), 32);
            if c.cc_ne() { blk = 306; continue; }
            blk = 311; continue;
        }
        311 => {
            let Some(t395) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700083fdb4); };
            c.w32(9, t395);
            let r396 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r396);
            let Some(t397) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083fdbc); };
            c.w32(12, t397);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r398 = c.sub(c.r64(12), 0x2u64, 64);
            c.w64(12, r398);
            blk = 312; continue;
        }
        312 => {
            c.w64(15, c.r64(12).wrapping_mul(4));
            c.w32(11, c.r32(8));
            let Some(t399) = c.ld32(c.r64(9).wrapping_add(c.r64(15)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083fdd5); };
            c.w32(7, t399);
            let r400 = c.r32(11) | 0x2u64;
            c.w32(11, r400);
            c.logic(r400, 32);
            c.cmp(c.r32(11), c.r32(7), 32);
            if c.cc_e() { blk = 315; continue; }
            blk = 313; continue;
        }
        313 => {
            let r401 = c.sub(c.r64(12), 0x2u64, 64);
            c.w64(12, r401);
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_ge() { blk = 312; continue; }
            blk = 314; continue;
        }
        314 => {
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083fded); };
            c.w64(7, t402);
            blk = 306; continue;
        }
        315 => {
            let Some(t403) = c.ld32(c.r64(9).wrapping_add(c.r64(15)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083fdf6); };
            c.w32(8, t403);
            let r404 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r404);
            let Some(t405) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083fdfe); };
            c.w64(7, t405);
            blk = 7; continue;
        }
        316 => {
            let Some(t406) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083fe07); };
            c.w32(9, t406);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r407 = c.sub(c.r64(9), 0x2u64, 64);
            c.w64(9, r407);
            blk = 317; continue;
        }
        317 => {
            c.w64(12, c.r64(9).wrapping_mul(4));
            c.w32(11, c.r32(8));
            let Some(t408) = c.ld32(c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083fe20); };
            c.w32(7, t408);
            let r409 = c.r32(11) | 0x2u64;
            c.w32(11, r409);
            c.logic(r409, 32);
            c.cmp(c.r32(11), c.r32(7), 32);
            if c.cc_e() { blk = 320; continue; }
            blk = 318; continue;
        }
        318 => {
            let r410 = c.sub(c.r64(9), 0x2u64, 64);
            c.w64(9, r410);
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_ge() { blk = 317; continue; }
            blk = 319; continue;
        }
        319 => {
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083fe38); };
            c.w64(7, t411);
            blk = 306; continue;
        }
        320 => {
            let Some(t412) = c.ld32(c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083fe41); };
            c.w32(8, t412);
            let r413 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r413);
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083fe49); };
            c.w64(7, t414);
            blk = 7; continue;
        }
        321 => {
            return RegfileExit::fault(0x700083fe52);
        }
        322 => {
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083fe53); };
            c.w64(7, t415);
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083fe57); };
            c.w64(3, t416);
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083fe5b); };
            c.w64(2, t417);
            blk = 306; continue;
        }
        323 => {
            let Some(t418) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083fe64); };
            c.w64(3, t418);
            blk = 306; continue;
        }
        324 => {
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083fe6d); };
            c.w64(3, t419);
            blk = 306; continue;
        }
        325 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_ne() { blk = 327; continue; }
            blk = 326; continue;
        }
        326 => {
            let Some(t420) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083fe7d); };
            c.w32(1, t420);
            let r421 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r421);
            blk = 68; continue;
        }
        327 => {
            let Some(t422) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083fe88); };
            c.w64(5, t422);
            let r423 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r423);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083fe90); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083fe91); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083fe92); }
            let Some(t424) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700083fe93); };
            if !c.push64(t424) { return RegfileExit::fault(0x700083fe93); }
            let Some(t425) = c.ld64(c.r64(13).wrapping_add(0x2630)) else { return RegfileExit::fault(0x700083fe97); };
            c.w64(3, t425);
            c.w32(0, 0x4u64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(9)) { return RegfileExit::fault(0x700083fea3); }
            return RegfileExit::jump(0x700086ed74);
        }
        328 => {
            let Some(t426) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083fead); };
            c.w64(3, t426);
            blk = 306; continue;
        }
        329 => {
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083feb6); };
            c.w64(3, t427);
            blk = 306; continue;
        }
        330 => {
            let Some(t428) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083febf); };
            c.w64(3, t428);
            blk = 306; continue;
        }
        331 => {
            let Some(t429) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083fec8); };
            c.w64(3, t429);
            blk = 306; continue;
        }
        332 => {
            let Some(t430) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083fed1); };
            c.w64(3, t430);
            blk = 306; continue;
        }
        333 => {
            let Some(t431) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083feda); };
            c.w64(3, t431);
            blk = 306; continue;
        }
        334 => {
            return RegfileExit::fault(0x700083fee3);
        }
        335 => {
            return RegfileExit::fault(0x700083fee4);
        }
        336 => {
            return RegfileExit::fault(0x700083fee5);
        }
        337 => {
            return RegfileExit::fault(0x700083fee6);
        }
        338 => {
            return RegfileExit::fault(0x700083fee7);
        }
        339 => {
            return RegfileExit::fault(0x700083fee8);
        }
        340 => {
            return RegfileExit::fault(0x700083fee9);
        }
        341 => {
            c.w64(0, c.r64(12));
            blk = 306; continue;
        }
        342 => {
            c.w64(0, c.r64(12));
            blk = 306; continue;
        }
        343 => {
            c.w64(0, c.r64(12));
            blk = 306; continue;
        }
        344 => {
            c.w64(0, c.r64(12));
            blk = 306; continue;
        }
        345 => {
            c.w64(0, c.r64(12));
            blk = 306; continue;
        }
        346 => {
            c.w64(0, c.r64(12));
            blk = 306; continue;
        }
        347 => {
            c.w64(0, c.r64(12));
            blk = 306; continue;
        }
        348 => {
            c.w64(0, c.r64(12));
            blk = 306; continue;
        }
        349 => {
            c.w64(0, c.r64(12));
            blk = 306; continue;
        }
        350 => {
            c.w64(0, c.r64(12));
            blk = 306; continue;
        }
        351 => {
            c.w64(0, c.r64(12));
            blk = 306; continue;
        }
        352 => {
            c.w64(0, c.r64(12));
            blk = 306; continue;
        }
        353 => {
            c.w64(0, c.r64(12));
            blk = 306; continue;
        }
        354 => {
            c.w32(0, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700083ff57); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x700083ff5b); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700083ff5f); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x700083ff6c, 0x700083ff67);
        }
        355 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700083ff6c); }
            let Some(t432) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083ff71); };
            c.w64(12, t432);
            let Some(t433) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083ff75); };
            c.w64(2, t433);
            let Some(t434) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083ff79); };
            c.w64(8, t434);
            let Some(t435) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083ff7d); };
            c.w64(9, t435);
            let Some(t436) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700083ff81); };
            c.w64(15, t436);
            blk = 177; continue;
        }
        356 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700083ff8a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700083ff8e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700083ff92); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x700083ff96); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x700083ff9a); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febf4, 0x700083ffa6, 0x700083ffa1);
        }
        357 => {
            c.w64(8, c.r64(0));
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083ffa9); };
            c.w64(2, t437);
            let Some(t438) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083ffad); };
            c.w64(9, t438);
            let Some(t439) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083ffb1); };
            c.w64(1, t439);
            let Some(t440) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083ffb5); };
            c.w64(3, t440);
            let Some(t441) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700083ffb9); };
            c.w64(15, t441);
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700083ffbd); };
            c.w64(6, t442);
            blk = 185; continue;
        }
        358 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700083ffcb); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700083ffcf); }
            c.w64(15, c.r64(2));
            return c.call(0x70007febf4, 0x700083ffdc, 0x700083ffd7);
        }
        359 => {
            c.w64(15, c.r64(0));
            let Some(t443) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083ffdf); };
            c.w64(12, t443);
            let Some(t444) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083ffe3); };
            c.w64(2, t444);
            let Some(t445) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083ffe7); };
            c.w64(9, t445);
            let Some(t446) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083ffeb); };
            c.w64(8, t446);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700083ffef); }
            blk = 207; continue;
        }
        360 => {
            c.w64(0, c.r64(12));
            blk = 306; continue;
        }
        361 => {
            c.w64(0, c.r64(12));
            blk = 306; continue;
        }
        362 => {
            return RegfileExit::fault(0x7000840009);
        }
        363 => {
            let Some(t447) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084000a); };
            c.w64(3, t447);
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084000e); };
            c.w64(6, t448);
            blk = 306; continue;
        }
        364 => {
            let Some(t449) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000840017); };
            c.w64(3, t449);
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084001b); };
            c.w64(6, t450);
            blk = 306; continue;
        }
        365 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_ne() { blk = 367; continue; }
            blk = 366; continue;
        }
        366 => {
            let Some(t451) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700084002b); };
            c.w32(1, t451);
            let r452 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r452);
            blk = 242; continue;
        }
        367 => {
            let Some(t453) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000840036); };
            c.w64(5, t453);
            let r454 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r454);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700084003e); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084003f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000840040); }
            let Some(t455) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000840041); };
            if !c.push64(t455) { return RegfileExit::fault(0x7000840041); }
            let Some(t456) = c.ld64(c.r64(13).wrapping_add(0x2630)) else { return RegfileExit::fault(0x7000840045); };
            c.w64(3, t456);
            c.w32(0, 0x4u64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(2)) { return RegfileExit::fault(0x7000840051); }
            return RegfileExit::jump(0x700086ed74);
        }
        368 => {
            blk = 306; continue;
        }
        369 => {
            let Some(t457) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000840060); };
            c.w64(5, t457);
            let r458 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r458);
            return RegfileExit::jump(0x7000800df4);
        }
        370 => {
            blk = 306; continue;
        }
        371 => {
            c.w32(0, c.r32(0));
            let r459 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r459);
            c.w64(9, 0xfffffffffffc0000u64);
            let r460 = c.r64(9) & c.r64(0);
            c.w64(9, r460);
            c.logic(r460, 64);
            let Some(t461) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000840081); };
            c.logic(t461 & 0x2u64, 8);
            if c.cc_e() { blk = 291; continue; }
            blk = 372; continue;
        }
        372 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000840090); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000840091); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700084009e, 0x7000840099);
        }
        373 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700084009e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008400a0); }
            blk = 291; continue;
        }
        374 => {
            c.w32(0, c.r32(0));
            let r462 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r462);
            c.w64(9, 0xfffffffffffc0000u64);
            let r463 = c.r64(9) & c.r64(0);
            c.w64(9, r463);
            c.logic(r463, 64);
            let Some(t464) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008400b5); };
            c.logic(t464 & 0x2u64, 8);
            if c.cc_e() { blk = 276; continue; }
            blk = 375; continue;
        }
        375 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008400c4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008400c5); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008400d2, 0x70008400cd);
        }
        376 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008400d2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008400d4); }
            blk = 276; continue;
        }
        377 => {
            c.w32(12, c.r32(12));
            let r465 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r465);
            c.w64(15, 0xfffffffffffc0000u64);
            let r466 = c.r64(15) & c.r64(12);
            c.w64(15, r466);
            c.logic(r466, 64);
            let Some(t467) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008400ea); };
            c.logic(t467 & 0x2u64, 8);
            if c.cc_e() { blk = 225; continue; }
            blk = 378; continue;
        }
        378 => {
            c.w64(0, c.r64(8).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008400f9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008400fa); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x7000840107, 0x7000840102);
        }
        379 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000840107); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000840109); }
            blk = 225; continue;
        }
        380 => {
            c.w32(15, c.r32(15));
            let r468 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r468);
            c.w64(8, 0xfffffffffffc0000u64);
            let r469 = c.r64(8) & c.r64(15);
            c.w64(8, r469);
            c.logic(r469, 64);
            let Some(t470) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700084011f); };
            c.logic(t470 & 0x2u64, 8);
            if c.cc_e() { blk = 210; continue; }
            blk = 381; continue;
        }
        381 => {
            c.w64(0, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084012e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700084012f); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x700084013c, 0x7000840137);
        }
        382 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700084013c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700084013e); }
            blk = 210; continue;
        }
        383 => {
            c.w32(8, c.r32(8));
            let r471 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r471);
            c.w64(0, 0xfffffffffffc0000u64);
            let r472 = c.r64(0) & c.r64(8);
            c.w64(0, r472);
            c.logic(r472, 64);
            let Some(t473) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000840154); };
            c.logic(t473 & 0x2u64, 8);
            if c.cc_e() { blk = 208; continue; }
            blk = 384; continue;
        }
        384 => {
            c.w64(3, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000840162); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000840163); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(3));
            return c.call(0x70007f13b4, 0x7000840170, 0x700084016b);
        }
        385 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000840170); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000840172); }
            blk = 208; continue;
        }
        386 => {
            c.w32(12, c.r32(12));
            let r474 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r474);
            c.w64(8, 0xfffffffffffc0000u64);
            let r475 = c.r64(8) & c.r64(12);
            c.w64(8, r475);
            c.logic(r475, 64);
            let Some(t476) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000840188); };
            c.logic(t476 & 0x2u64, 8);
            if c.cc_e() { blk = 201; continue; }
            blk = 387; continue;
        }
        387 => {
            c.w64(15, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000840197); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000840198); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008401a5, 0x70008401a0);
        }
        388 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008401a5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008401a7); }
            blk = 201; continue;
        }
        389 => {
            c.w32(8, c.r32(8));
            let r477 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r477);
            c.w64(15, 0xfffffffffffc0000u64);
            let r478 = c.r64(15) & c.r64(8);
            c.w64(15, r478);
            c.logic(r478, 64);
            let Some(t479) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008401bd); };
            c.logic(t479 & 0x2u64, 8);
            if c.cc_e() { blk = 199; continue; }
            blk = 390; continue;
        }
        390 => {
            c.w64(0, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008401cc); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008401cd); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(0));
            return c.call(0x70007f13b4, 0x70008401da, 0x70008401d5);
        }
        391 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008401da); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008401dc); }
            blk = 199; continue;
        }
        392 => {
            c.w32(8, c.r32(8));
            let r480 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r480);
            c.w64(9, 0xfffffffffffc0000u64);
            let r481 = c.r64(9) & c.r64(8);
            c.w64(9, r481);
            c.logic(r481, 64);
            let Some(t482) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008401f2); };
            c.logic(t482 & 0x2u64, 8);
            if c.cc_e() { blk = 196; continue; }
            blk = 393; continue;
        }
        393 => {
            c.w64(12, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000840201); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000840202); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f13b4, 0x700084020f, 0x700084020a);
        }
        394 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700084020f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000840211); }
            blk = 196; continue;
        }
        395 => {
            c.w32(0, c.r32(0));
            let r483 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r483);
            c.w64(9, 0xfffffffffffc0000u64);
            let r484 = c.r64(9) & c.r64(0);
            c.w64(9, r484);
            c.logic(r484, 64);
            let Some(t485) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000840226); };
            c.logic(t485 & 0x2u64, 8);
            if c.cc_e() { blk = 195; continue; }
            blk = 396; continue;
        }
        396 => {
            c.w64(12, c.r64(1).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000840235); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000840236); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000840243, 0x700084023e);
        }
        397 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000840243); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000840245); }
            blk = 195; continue;
        }
        398 => {
            c.w32(8, c.r32(8));
            let r486 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r486);
            c.w64(12, 0xfffffffffffc0000u64);
            let r487 = c.r64(12) & c.r64(8);
            c.w64(12, r487);
            c.logic(r487, 64);
            let Some(t488) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700084025b); };
            c.logic(t488 & 0x2u64, 8);
            if c.cc_e() { blk = 192; continue; }
            blk = 399; continue;
        }
        399 => {
            c.w64(15, c.r64(2).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084026b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700084026c); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000840279, 0x7000840274);
        }
        400 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000840279); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700084027b); }
            blk = 192; continue;
        }
        401 => {
            c.w32(8, c.r32(8));
            let r489 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r489);
            c.w64(9, 0xfffffffffffc0000u64);
            let r490 = c.r64(9) & c.r64(8);
            c.w64(9, r490);
            c.logic(r490, 64);
            let Some(t491) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000840291); };
            c.logic(t491 & 0x2u64, 8);
            if c.cc_e() { blk = 154; continue; }
            blk = 402; continue;
        }
        402 => {
            c.w64(15, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008402a0); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008402a1); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x70008402ae, 0x70008402a9);
        }
        403 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008402ae); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008402b0); }
            blk = 154; continue;
        }
        404 => {
            c.w32(12, c.r32(12));
            let r492 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r492);
            c.w64(9, 0xfffffffffffc0000u64);
            let r493 = c.r64(9) & c.r64(12);
            c.w64(9, r493);
            c.logic(r493, 64);
            let Some(t494) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008402c6); };
            c.logic(t494 & 0x2u64, 8);
            if c.cc_e() { blk = 145; continue; }
            blk = 405; continue;
        }
        405 => {
            c.w64(15, c.r64(8).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008402d5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008402d6); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008402e3, 0x70008402de);
        }
        406 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008402e3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008402e5); }
            blk = 145; continue;
        }
        407 => {
            c.w32(3, c.r32(3));
            let r495 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r495);
            c.w64(0, 0xfffffffffffc0000u64);
            let r496 = c.r64(0) & c.r64(3);
            c.w64(0, r496);
            c.logic(r496, 64);
            let Some(t497) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008402fa); };
            c.logic(t497 & 0x2u64, 8);
            if c.cc_e() { blk = 122; continue; }
            blk = 408; continue;
        }
        408 => {
            c.w64(7, c.r64(8).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000840308); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000840309); }
            let (x498, x499) = (c.r64(8), c.r64(7));
            c.w64(8, x499);
            c.w64(7, x498);
            return c.call(0x70007f0e54, 0x7000840313, 0x700084030e);
        }
        409 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000840313); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000840315); }
            blk = 122; continue;
        }
        410 => {
            c.w32(1, c.r32(1));
            let r500 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r500);
            c.w64(12, 0xfffffffffffc0000u64);
            let r501 = c.r64(12) & c.r64(1);
            c.w64(12, r501);
            c.logic(r501, 64);
            let Some(t502) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700084032a); };
            c.logic(t502 & 0x2u64, 8);
            if c.cc_e() { blk = 119; continue; }
            blk = 411; continue;
        }
        411 => {
            c.w64(15, c.r64(8).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084033a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700084033b); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000840348, 0x7000840343);
        }
        412 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000840348); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700084034a); }
            blk = 119; continue;
        }
        413 => {
            c.w32(0, c.r32(0));
            let r503 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r503);
            c.w64(8, 0xfffffffffffc0000u64);
            let r504 = c.r64(8) & c.r64(0);
            c.w64(8, r504);
            c.logic(r504, 64);
            let Some(t505) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700084035f); };
            c.logic(t505 & 0x2u64, 8);
            if c.cc_e() { blk = 95; continue; }
            blk = 414; continue;
        }
        414 => {
            c.w64(12, c.r64(9).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084036e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700084036f); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700084037c, 0x7000840377);
        }
        415 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700084037c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700084037e); }
            blk = 95; continue;
        }
        416 => {
            c.w32(0, c.r32(0));
            let r506 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r506);
            c.w64(12, 0xfffffffffffc0000u64);
            let r507 = c.r64(12) & c.r64(0);
            c.w64(12, r507);
            c.logic(r507, 64);
            let Some(t508) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000840393); };
            c.logic(t508 & 0x2u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 417; continue;
        }
        417 => {
            c.w64(15, c.r64(9).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008403a3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008403a4); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008403b1, 0x70008403ac);
        }
        418 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008403b1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008403b3); }
            blk = 36; continue;
        }
        419 => {
            return RegfileExit::jump(0x70008403bc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008403d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008403d7); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008403db); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008403e2); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008403e9); };
            c.w32(9, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008403ed); };
            c.cmp(t6, c.r32(9), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008403f6); };
            c.w64(8, t7);
            blk = 2; continue;
        }
        2 => {
            c.w64(3, c.r64(8));
            return RegfileExit::jump(0x700083eb74);
        }
        3 => {
            return RegfileExit::jump(0x7000840408);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000840414(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000840417); };
            c.w64(9, t0);
            let Some(t1) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084041b); };
            c.w64(8, t1);
            c.w64(6, c.r64(8));
            c.w64(3, c.r64(9));
            return RegfileExit::jump(0x700083eb74);
        }
        1 => {
            return RegfileExit::jump(0x700084042c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000840434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840439); };
            c.w32(8, t0);
            let r1 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1);
            blk = 3; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000840442); };
            c.w64(8, t2);
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000840449); };
            c.logic(t3 & 0x1000000u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000840457); };
            c.cmp(t4, c.r32(3), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(7)));
            c.w32(12, c.r32(8));
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084046b); };
            c.w32(15, t5);
            let r6 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r6);
            let r7 = c.r32(12) | 0x2u64;
            c.w32(12, r7);
            c.logic(r7, 32);
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000840480); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            blk = 7; continue;
        }
        7 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t10) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840492); };
            c.w32(9, t10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x7000840496); };
            c.cmp(t11, c.r32(9), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700084049f); };
            c.w32(9, t12);
            let r13 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r13);
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 10; continue;
        }
        10 => {
            let r14 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r14);
            c.logic(r14, 64);
            let Some(t15) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008404b4); };
            c.logic(t15 & 0x1000000u64, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008404c2); };
            c.w32(8, t16);
            let r17 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r17);
            let Some(t18) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70008404c9); };
            if !c.push64(t18) { return RegfileExit::fault(0x70008404c9); }
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(3)) { return RegfileExit::fault(0x70008404cc); }
            c.w64(11, c.r64(9));
            let Some(t19) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008404d4); };
            c.logic(t19 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let r20 = c.add(c.r64(8), 0x3fu64, 64);
            c.w64(8, r20);
            blk = 14; continue;
        }
        13 => {
            let Some(t21) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008404e4); };
            c.w32(8, t21);
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008404e8); };
            c.w64(8, t22);
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::jump(c.r64(8));
        }
        15 => {
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008404f3); };
            c.logic(t23 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            let r24 = c.add(c.r64(8), 0x3fu64, 64);
            c.w64(8, r24);
            blk = 18; continue;
        }
        17 => {
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000840503); };
            c.w32(8, t25);
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x7000840507); };
            c.w64(8, t26);
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::jump(c.r64(8));
        }
        19 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000840512); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000840513); }
            let Some(t27) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000840514); };
            if !c.push64(t27) { return RegfileExit::fault(0x7000840514); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x20a0)) else { return RegfileExit::fault(0x7000840518); };
            c.w64(3, t28);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(0)) { return RegfileExit::fault(0x700084051f); }
            c.w32(0, 0x3u64);
            return RegfileExit::jump(0x700086ed74);
        }
        20 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700084052e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000840532); }
            let r29 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r29);
            c.w32(9, c.r32(15));
            let r30 = c.r32(9) & 0x3u64;
            c.w32(9, r30);
            c.logic(r30, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700084053f); }
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t31) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084054d); };
            c.w32(9, t31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x2c8)) else { return RegfileExit::fault(0x7000840551); };
            c.cmp(t32, c.r32(9), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700084055a); };
            c.cmp(t33, c.r32(15), 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t34) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000840567); };
            c.w64(5, t34);
            let r35 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r35);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700084056f); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000840570); }
            let Some(t36) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000840571); };
            if !c.push64(t36) { return RegfileExit::fault(0x7000840571); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x20a0)) else { return RegfileExit::fault(0x7000840575); };
            c.w64(3, t37);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(0)) { return RegfileExit::fault(0x700084057c); }
            c.w32(0, 0x3u64);
            return RegfileExit::jump(0x700086ed74);
        }
        24 => {
            let Some(t38) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084058b); };
            c.w32(9, t38);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r39 = c.sub(c.r64(9), 0x2u64, 64);
            c.w64(9, r39);
            blk = 25; continue;
        }
        25 => {
            c.w64(12, c.r64(9).wrapping_mul(4));
            c.w32(11, c.r32(8));
            let Some(t40) = c.ld32(c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008405a4); };
            c.w32(0, t40);
            let r41 = c.r32(11) | 0x2u64;
            c.w32(11, r41);
            c.logic(r41, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let r42 = c.sub(c.r64(9), 0x2u64, 64);
            c.w64(9, r42);
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_ge() { blk = 25; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008405bc); };
            c.w64(0, t43);
            blk = 33; continue;
        }
        28 => {
            let Some(t44) = c.ld32(c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008405c2); };
            c.w32(8, t44);
            let r45 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008405ca); };
            c.w64(0, t46);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008405d1); }
            blk = 7; continue;
        }
        29 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008405d7); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008405db); }
            let r47 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r47);
            blk = 33; continue;
        }
        30 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008405e3); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008405e7); }
            let r48 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r48);
            blk = 33; continue;
        }
        31 => {
            c.w64(3, c.r64(1));
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(2));
            return RegfileExit::jump(0x7000875c54);
        }
        32 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008405fd); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000840601); }
            let r49 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r49);
            blk = 33; continue;
        }
        33 => {
            let Some(t50) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000840607); };
            c.w64(5, t50);
            let r51 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r51);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084060f); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000840610); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000840611); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000840612); }
            let Some(t52) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000840613); };
            if !c.push64(t52) { return RegfileExit::fault(0x7000840613); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(0)) { return RegfileExit::fault(0x7000840617); }
            c.w32(0, 0x5u64);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2028)) else { return RegfileExit::fault(0x7000840621); };
            c.w64(3, t53);
            return RegfileExit::jump(0x700086ed74);
        }
        34 => {
            return RegfileExit::jump(0x7000840630);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000840634(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000840637); };
            c.w64(9, t0);
            let Some(t1) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084063b); };
            c.w64(8, t1);
            c.w64(6, c.r64(8));
            c.w64(3, c.r64(9));
            return RegfileExit::jump(0x7000840434);
        }
        1 => {
            return RegfileExit::jump(0x700084064c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000840654(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084065a); };
            c.w64(2, t0);
            c.w32(7, c.r32(3));
            c.w64(8, c.r64(2));
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840663); };
            c.w32(9, t1);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000840673); };
            c.w32(11, t2);
            let r3 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r3);
            let Some(t4) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084067a); };
            c.w32(11, t4);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.logic(c.r64(11) & 0x4000000u64, 64);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000840693); };
            c.w32(11, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000840697); };
            c.cmp(t6, c.r32(11), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008406a6); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let r9 = c.add(c.r32(7), 0xffffffffffffffffu64, 32);
            c.w32(7, r9);
            if c.cc_ne() { blk = 1; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 11; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008406b4);
        }
        9 => {
            return RegfileExit::fault(0x70008406b5);
        }
        10 => {
            return RegfileExit::fault(0x70008406b6);
        }
        11 => {
            let Some(t10) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008406b7); };
            c.w32(2, t10);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(1), c.r64(2), 64);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x70008406c3);
        }
        13 => {
            let Some(t11) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008406c4); };
            c.w32(0, t11);
            let r12 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r12);
            return c.ret(0x0, 0x70008406cc);
        }
        14 => {
            let Some(t13) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70008406cd); };
            if !c.push64(t13) { return RegfileExit::fault(0x70008406cd); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x26c8)) else { return RegfileExit::fault(0x70008406d0); };
            c.w64(3, t14);
            c.w64(6, c.r64(2));
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(0)) { return RegfileExit::fault(0x70008406da); }
            c.w32(0, 0x1u64);
            return RegfileExit::jump(0x700086ed74);
        }
        15 => {
            return RegfileExit::jump(0x70008406ec);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008406f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008406fa); };
            c.w64(2, t0);
            c.w32(7, c.r32(3));
            c.w64(8, c.r64(2));
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840703); };
            c.w32(9, t1);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000840713); };
            c.w32(11, t2);
            let r3 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r3);
            let Some(t4) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084071a); };
            c.w32(11, t4);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.logic(c.r64(11) & 0x4000000u64, 64);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000840733); };
            c.w32(11, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000840737); };
            c.cmp(t6, c.r32(11), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000840746); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let r9 = c.add(c.r32(7), 0xffffffffffffffffu64, 32);
            c.w32(7, r9);
            if c.cc_ne() { blk = 1; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 11; continue;
        }
        8 => {
            return RegfileExit::fault(0x7000840754);
        }
        9 => {
            return RegfileExit::fault(0x7000840755);
        }
        10 => {
            return RegfileExit::fault(0x7000840756);
        }
        11 => {
            let Some(t10) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840757); };
            c.w32(2, t10);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(1), c.r64(2), 64);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x7000840763);
        }
        13 => {
            let Some(t11) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000840764); };
            c.w32(0, t11);
            let r12 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r12);
            return c.ret(0x0, 0x700084076c);
        }
        14 => {
            let Some(t13) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x700084076d); };
            if !c.push64(t13) { return RegfileExit::fault(0x700084076d); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x26d0)) else { return RegfileExit::fault(0x7000840770); };
            c.w64(3, t14);
            c.w64(6, c.r64(2));
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(0)) { return RegfileExit::fault(0x700084077a); }
            c.w32(0, 0x1u64);
            return RegfileExit::jump(0x700086ed74);
        }
        15 => {
            return RegfileExit::jump(0x700084078c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000840794(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000840794); };
            c.cmp(t0, c.r32(3), 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t1) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008407a5); };
            c.w32(9, t1);
            let r2 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r2);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let r3 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r3);
            c.logic(r3, 64);
            let Some(t4) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008407c1); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008407c8); };
            c.cmp(t6, c.r32(8), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x70008407d8);
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008407d9); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008407e0); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008407e7); };
            c.w32(11, t11);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xa7u64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70008407fa);
        }
        7 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x70008407fb); };
            c.w32(8, t12);
            let r13 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r13);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840808); };
            c.w32(11, t14);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            let r15 = c.r64(12) & 0xfffu64;
            c.w64(12, r15);
            c.logic(r15, 64);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(15, c.r64(12).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_ae() { blk = 242; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t16) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700084082d); };
            c.w32(8, t16);
            let r17 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r17);
            let Some(t18) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840835); };
            c.w32(11, t18);
            if let Some(sr) = c.shr(c.r64(9), 0xcu64, 64) { c.w64(9, sr); }
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r19 = c.r64(9) & 0x3ffffu64;
            c.w64(9, r19);
            c.logic(r19, 64);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x700084084f);
        }
        10 => {
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000840850); };
            c.w32(0, t20);
            let r21 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r21);
            return c.ret(0x0, 0x7000840858);
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000840859); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084085d); }
            let r22 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r22);
            blk = 241; continue;
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000840868); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084086c); }
            let r23 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r23);
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000840872); };
            c.w32(8, t24);
            let r25 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r25);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x700084087a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700084087e); }
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000840882); };
            c.cmp(t26, c.r32(8), 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t27) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084088f); };
            c.w32(9, t27);
            let r28 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r28);
            let Some(t29) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000840896); };
            c.w32(9, t29);
            let r30 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r30);
            let Some(t31) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084089d); };
            c.w32(11, t31);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x3u64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x70008408ad);
        }
        15 => {
            let Some(t32) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008408ae); };
            c.w32(12, t32);
            let r33 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r33);
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x70008408bb);
        }
        17 => {
            let Some(t34) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008408bc); };
            c.w32(2, t34);
            let r35 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r35);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008408cd); };
            c.w32(9, t36);
            let Some(t37) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x70008408d1); };
            c.cmp(t37, c.r32(9), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008408da); };
            c.w32(9, t38);
            let r39 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r39);
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t40) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008408e9); };
            c.cmp(t40, c.r32(11), 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008408f3); };
            c.w32(9, t41);
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008408f7); };
            c.w32(11, t42);
            let r43 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r43);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000840904); };
            c.cmp(t44, c.r32(11), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 24; continue;
        }
        24 => {
            let r45 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r45);
            c.logic(r45, 64);
            blk = 26; continue;
        }
        25 => {
            c.w64(11, c.r64(2));
            blk = 26; continue;
        }
        26 => {
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(10));
            blk = 34; continue;
        }
        27 => {
            c.w64(0, c.r64(11));
            blk = 28; continue;
        }
        28 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000840931); }
            return c.ret(0x0, 0x7000840932);
        }
        29 => {
            let Some(t46) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000840933); };
            c.w64(5, t46);
            let r47 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r47);
            let Some(t48) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700084093b); };
            c.logic(t48 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            let r49 = c.add(c.r64(8), 0x3fu64, 64);
            c.w64(8, r49);
            blk = 32; continue;
        }
        31 => {
            let Some(t50) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084094b); };
            c.w32(8, t50);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x700084094f); };
            c.w64(8, t51);
            blk = 32; continue;
        }
        32 => {
            return RegfileExit::jump(c.r64(8));
        }
        33 => {
            c.w64(9, c.r64(2));
            c.w64(11, c.r64(8));
            blk = 34; continue;
        }
        34 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(15, c.r64(11));
            let r52 = c.r64(15) & 0xfu64;
            c.w64(15, r52);
            c.logic(r52, 64);
            c.cmp(c.r64(15), 0x4u64, 64);
            if c.cc_e() { blk = 174; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(15), 0x5u64, 64);
            if c.cc_e() { blk = 173; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(15), 0xdu64, 64);
            if c.cc_e() { blk = 172; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r64(15), 0x2u64, 64);
            if c.cc_e() { blk = 112; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r64(15), 0x6u64, 64);
            if c.cc_e() { blk = 110; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_e() { blk = 107; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r64(15), 0x8u64, 64);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 97; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r64(15), 0x9u64, 64);
            if c.cc_e() { blk = 97; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r64(15), 0x3u64, 64);
            if c.cc_e() { blk = 54; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_e() { blk = 109; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r64(15), 0xcu64, 64);
            if c.cc_e() { blk = 52; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r64(15), 0xeu64, 64);
            if c.cc_e() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t53) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008409ec); };
            c.w64(5, t53);
            let r54 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r54);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008409f4); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008409f6); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008409f8); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008409f9); }
            let Some(t55) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008409fa); };
            if !c.push64(t55) { return RegfileExit::fault(0x70008409fa); }
            c.w32(0, 0x5u64);
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x7000840a03); };
            c.w64(3, t56);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(1)) { return RegfileExit::fault(0x7000840a0a); }
            return RegfileExit::jump(0x700086ed74);
        }
        47 => {
            let Some(t57) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000840a14); };
            c.w32(8, t57);
            let r58 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r58);
            let Some(t59) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840a1b); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840a22); };
            c.w32(9, t61);
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r62 = c.r64(11) & 0x1ffffffu64;
            c.w64(11, r62);
            c.logic(r62, 64);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            return RegfileExit::fault(0x7000840a3c);
        }
        49 => {
            let Some(t63) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000840a3d); };
            c.w32(8, t63);
            let r64 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r64);
            let Some(t65) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840a45); };
            c.w32(0, t65);
            let r66 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r66);
            let Some(t67) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000840a4c); };
            c.cmp(t67, c.r32(0), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 28; continue;
        }
        51 => {
            let Some(t68) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000840a5a); };
            c.w64(5, t68);
            let r69 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r69);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000840a62); }
            let Some(t70) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000840a63); };
            if !c.push64(t70) { return RegfileExit::fault(0x7000840a63); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x7000840a67); };
            c.w64(3, t71);
            c.w32(0, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), 0x174u64) { return RegfileExit::fault(0x7000840a73); }
            return RegfileExit::jump(0x700086ed74);
        }
        52 => {
            let Some(t72) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000840a81); };
            c.w64(5, t72);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(9));
            c.w64(3, c.r64(1));
            c.w64(1, c.r64(12));
            let r73 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r73);
            return RegfileExit::jump(0x70008bc174);
        }
        53 => {
            blk = 109; continue;
        }
        54 => {
            let Some(t74) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000840a9e); };
            c.w32(8, t74);
            let r75 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r75);
            let Some(t76) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000840aa5); };
            c.w32(11, t76);
            let Some(t77) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000840aa9); };
            c.cmp(t77, c.r32(8), 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 55; continue;
        }
        55 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t78) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840ac0); };
            c.w32(11, t78);
            let r79 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r79);
            let Some(t80) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000840ac7); };
            c.cmp(t80, c.r32(11), 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t81) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840ad4); };
            c.w32(2, t81);
            let r82 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r82);
            let Some(t83) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840adb); };
            c.w32(11, t83);
            let r84 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r84);
            let Some(t85) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000840ae2); };
            c.logic(t85 & 0x2u64, 8);
            if c.cc_ne() { blk = 77; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t86) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000840aed); };
            c.cmp(t86, c.r32(11), 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000840af6); };
            c.w64(0, t87);
            blk = 28; continue;
        }
        60 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000840b02); };
            c.w64(11, t88);
            let Some(t89) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000840b09); };
            c.cmp(t89, 0x20u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t90) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000840b13); };
            c.w32(11, t90);
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000840b17); };
            c.cmp(t91, c.r32(11), 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t92) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840b24); };
            c.w32(11, t92);
            let r93 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r93);
            let Some(t94) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840b2b); };
            c.w32(15, t94);
            let Some(t95) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000840b2f); };
            c.cmp(t95, c.r32(15), 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t96) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000840b38); };
            c.w32(11, t96);
            let r97 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r97);
            let Some(t98) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840b3f); };
            c.w32(15, t98);
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000840b43); };
            c.cmp(t99, c.r32(15), 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 64; continue;
        }
        64 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 241; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t100) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840b56); };
            c.w32(15, t100);
            let r101 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r101);
            let Some(t102) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000840b5d); };
            c.w32(7, t102);
            c.w32(6, c.r64(7).wrapping_sub(0x80c));
            c.cmp(c.r32(6), 0xeu64, 32);
            if c.cc_be() { blk = 70; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(11, c.r16(7));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t103) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000840b7a); };
            c.cmp(t103, c.r32(15), 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000840b83); };
            c.w64(6, t104);
            blk = 241; continue;
        }
        69 => {
            c.w64(9, c.r64(8));
            blk = 71; continue;
        }
        70 => {
            c.w64(9, c.r64(11));
            blk = 71; continue;
        }
        71 => {
            let Some(t105) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000840b94); };
            c.w32(8, t105);
            let r106 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r106);
            let Some(t107) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840b9b); };
            c.w32(8, t107);
            let r108 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r108);
            let Some(t109) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000840ba2); };
            c.w32(9, t109);
            let r110 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r110);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 74; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t111) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000840bb4); };
            c.w32(6, t111);
            let r112 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r112);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000840bbb); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000840bc4, 0x7000840bbf);
        }
        73 => {
            blk = 28; continue;
        }
        74 => {
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000840bc9); };
            c.w64(6, t113);
            blk = 241; continue;
        }
        75 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000840bd2); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000840bd4); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000840bd5); }
            c.w32(0, 0x3u64);
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000840bdc); };
            c.w64(3, t114);
            c.w64(15, c.r64(0));
            return c.call(0x700086ed74, 0x7000840beb, 0x7000840be6);
        }
        76 => {
            blk = 28; continue;
        }
        77 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000840bf0); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(2));
            return c.call(0x70007f2274, 0x7000840bfc, 0x7000840bf7);
        }
        78 => {
            blk = 28; continue;
        }
        79 => {
            let Some(t115) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840c01); };
            c.w32(12, t115);
            let r116 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r116);
            let Some(t117) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000840c08); };
            c.w32(15, t117);
            c.cmp(c.r16(15), 0x834u64, 16);
            if c.cc_e() { blk = 94; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w32(2, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_be() { blk = 86; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r16(15), 0x411u64, 16);
            if c.cc_ne() { blk = 241; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t118) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840c32); };
            c.w32(9, t118);
            let Some(t119) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000840c36); };
            c.cmp(t119, c.r32(9), 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 83; continue;
        }
        83 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 241; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t120) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000840c4d); };
            c.cmp(t120, 0x40u64, 16);
            if c.cc_ae() { blk = 241; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t121) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000840c59); };
            c.w32(8, t121);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 28; continue;
        }
        86 => {
            let Some(t122) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840c66); };
            c.w32(11, t122);
            let Some(t123) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000840c6a); };
            c.cmp(t123, c.r32(11), 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t124) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000840c77); };
            c.w32(11, t124);
            c.w64(12, c.r64(11));
            let r125 = c.r32(12) & 0xc0u64;
            c.w32(12, r125);
            c.logic(r125, 32);
            c.cmp(c.r32(12), 0xc0u64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t126) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000840c90); };
            c.w32(8, t126);
            let r127 = c.r32(8) & 0x1fu64;
            c.w32(8, r127);
            c.logic(r127, 32);
            let r128 = c.sub(c.r32(8), 0x10u64, 32);
            c.w32(8, r128);
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_a() { blk = 241; continue; }
            blk = 89; continue;
        }
        89 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 241; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t129) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000840cb0); };
            c.w32(8, t129);
            let r130 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r130);
            let Some(t131) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000840cb7); };
            c.cmp(t131, c.r32(8), 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t132) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840cc4); };
            c.w32(9, t132);
            let Some(t133) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000840cc8); };
            c.cmp(t133, c.r32(9), 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w64(0, c.r64(8));
            blk = 28; continue;
        }
        93 => {
            let Some(t134) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000840cd9); };
            c.w32(0, t134);
            let r135 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r135);
            blk = 28; continue;
        }
        94 => {
            let Some(t136) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840ce5); };
            c.w32(9, t136);
            let Some(t137) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000840ce9); };
            c.cmp(t137, c.r32(9), 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(0, c.r64(8));
            blk = 28; continue;
        }
        96 => {
            c.w64(0, c.r64(8));
            blk = 28; continue;
        }
        97 => {
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000840d06); };
            c.w64(15, t138);
            let Some(t139) = c.ld8(c.r64(15)) else { return RegfileExit::fault(0x7000840d0d); };
            c.cmp(t139, 0x20u64, 8);
            if c.cc_e() { blk = 109; continue; }
            blk = 98; continue;
        }
        98 => {
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 100; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t140) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000840d1d); };
            c.w32(8, t140);
            let r141 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r141);
            blk = 101; continue;
        }
        100 => {
            let Some(t142) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000840d26); };
            c.w32(8, t142);
            let r143 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r143);
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_ne() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            return RegfileExit::fault(0x7000840d33);
        }
        103 => {
            let r144 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r144);
            c.logic(r144, 64);
            let Some(t145) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000840d38); };
            c.w32(11, t145);
            let r146 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r146);
            let Some(t147) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000840d3f); };
            c.w32(3, t147);
            let r148 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r148);
            let Some(t149) = c.ld64(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840d46); };
            c.w64(9, t149);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t150) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840d4f); };
            c.w32(11, t150);
            let r151 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r151);
            let Some(t152) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000840d56); };
            c.w32(2, t152);
            let r153 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r153);
            blk = 105; continue;
        }
        105 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000840d5d); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(2));
            c.w64(2, c.r64(9));
            c.w64(6, c.r64(8));
            return c.call(0x70007fe8d4, 0x7000840d6f, 0x7000840d6a);
        }
        106 => {
            blk = 28; continue;
        }
        107 => {
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000840d74); };
            c.w64(8, t154);
            let Some(t155) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x7000840d7b); };
            c.cmp(t155, 0x20u64, 8);
            if c.cc_e() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            let r156 = c.r64(11) & 0x3ffu64;
            c.w64(11, r156);
            c.logic(r156, 64);
            let r157 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(8, r157);
            let Some(t158) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840d90); };
            c.w32(11, t158);
            let r159 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r159);
            let Some(t160) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000840d97); };
            c.w32(11, t160);
            let r161 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r161);
            let Some(t162) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000840d9e); };
            c.w32(3, t162);
            let r163 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r163);
            let Some(t164) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000840da6); };
            c.w64(5, t164);
            c.w64(2, c.r64(12));
            c.w64(1, c.r64(9));
            let r165 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r165);
            return RegfileExit::jump(0x70007fea14);
        }
        109 => {
            let Some(t166) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000840db9); };
            c.w64(5, t166);
            let r167 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r167);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000840dc1); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000840dc2); }
            let Some(t168) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000840dc3); };
            if !c.push64(t168) { return RegfileExit::fault(0x7000840dc3); }
            c.w32(0, 0x3u64);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(1)) { return RegfileExit::fault(0x7000840dcc); }
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0x2048)) else { return RegfileExit::fault(0x7000840dd1); };
            c.w64(3, t169);
            return RegfileExit::jump(0x700086ed74);
        }
        110 => {
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            let r170 = c.r64(11) & 0x3ffu64;
            c.w64(11, r170);
            c.logic(r170, 64);
            let r171 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(8, r171);
            let Some(t172) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840dec); };
            c.w32(9, t172);
            let r173 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r173);
            let Some(t174) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000840df3); };
            c.w32(9, t174);
            let r175 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r175);
            let Some(t176) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000840dfa); };
            c.w32(8, t176);
            let r177 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r177);
            let Some(t178) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840e02); };
            c.w32(7, t178);
            let r179 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r179);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000840e09); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000840e12, 0x7000840e0d);
        }
        111 => {
            blk = 28; continue;
        }
        112 => {
            let Some(t180) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840e17); };
            c.w32(8, t180);
            let r181 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r181);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t182) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000840e24); };
            c.w64(8, t182);
            blk = 114; continue;
        }
        114 => {
            let Some(t183) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000840e2b); };
            c.w32(11, t183);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r184 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r184);
            let Some(t185) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840e39); };
            c.w32(15, t185);
            let r186 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r186);
            let r187 = c.r64(15) & c.r64(11);
            c.w64(15, r187);
            c.logic(r187, 64);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 115; continue;
        }
        115 => {
            c.w64(7, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(6, c.r64(7).wrapping_mul(4));
            let Some(t188) = c.ld32(c.r64(8).wrapping_add(c.r64(6)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000840e53); };
            c.w32(0, t188);
            let Some(t189) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000840e58); };
            c.cmp(t189, c.r32(0), 32);
            if c.cc_e() { blk = 171; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(0), c.r32(1), 32);
            if c.cc_e() { blk = 118; continue; }
            blk = 117; continue;
        }
        117 => {
            let r190 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r190);
            let r191 = c.add(c.r64(15), c.r64(2), 64);
            c.w64(15, r191);
            c.w64(0, c.r64(11));
            let r192 = c.r64(0) & c.r64(15);
            c.w64(0, r192);
            c.logic(r192, 64);
            c.w64(15, c.r64(0));
            blk = 115; continue;
        }
        118 => {
            let Some(t193) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840e7b); };
            c.w32(11, t193);
            let Some(t194) = c.ld32(c.r64(8).wrapping_add(c.r64(6)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000840e7f); };
            c.w32(15, t194);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(0, c.r64(7).wrapping_add(0x1));
            c.cmp(c.r64(0), c.r64(11), 64);
            if c.cc_b() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            return RegfileExit::fault(0x7000840e93);
        }
        120 => {
            let Some(t195) = c.ld32(c.r64(8).wrapping_add(c.r64(6)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000840e94); };
            c.w32(0, t195);
            let r196 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r196);
            c.logic(c.r8(15) & 0x2u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t197) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840ea6); };
            c.w32(8, t197);
            let Some(t198) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000840eaa); };
            c.cmp(t198, c.r32(8), 32);
            if c.cc_ne() { blk = 146; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t199) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000840eb7); };
            c.w32(2, t199);
            let r200 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r200);
            let Some(t201) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840ebd); };
            c.w32(8, t201);
            let r202 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r202);
            let Some(t203) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000840ec4); };
            c.logic(t203 & 0x2u64, 8);
            if c.cc_ne() { blk = 144; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t204) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000840ecf); };
            c.cmp(t204, c.r32(8), 32);
            if c.cc_e() { blk = 125; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000840ed8); };
            c.w64(0, t205);
            blk = 28; continue;
        }
        125 => {
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000840ee4); };
            c.w64(8, t206);
            let Some(t207) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x7000840eeb); };
            c.cmp(t207, 0x20u64, 8);
            if c.cc_e() { blk = 142; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t208) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000840ef5); };
            c.w32(8, t208);
            let Some(t209) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000840ef9); };
            c.cmp(t209, c.r32(8), 32);
            if c.cc_e() { blk = 128; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000840f02); };
            c.w64(6, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000840f06); };
            c.w64(0, t211);
            blk = 241; continue;
        }
        128 => {
            let Some(t212) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840f0f); };
            c.w32(8, t212);
            let r213 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r213);
            let Some(t214) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840f16); };
            c.w32(11, t214);
            let Some(t215) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000840f1a); };
            c.cmp(t215, c.r32(11), 32);
            if c.cc_ne() { blk = 130; continue; }
            blk = 129; continue;
        }
        129 => {
            let Some(t216) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000840f23); };
            c.w32(8, t216);
            let r217 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r217);
            let Some(t218) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840f2a); };
            c.w32(11, t218);
            let Some(t219) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000840f2e); };
            c.cmp(t219, c.r32(11), 32);
            if c.cc_e() { blk = 129; continue; }
            blk = 130; continue;
        }
        130 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 141; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t220) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840f41); };
            c.w32(11, t220);
            let r221 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r221);
            let Some(t222) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000840f48); };
            c.w32(15, t222);
            c.w32(0, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 136; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w32(8, c.r16(15));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t223) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000840f66); };
            c.cmp(t223, c.r32(11), 32);
            if c.cc_ne() { blk = 137; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000840f6f); };
            c.w64(6, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000840f73); };
            c.w64(0, t225);
            blk = 241; continue;
        }
        135 => {
            let Some(t226) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000840f7c); };
            c.w32(11, t226);
            let r227 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r227);
            c.w64(9, c.r64(11));
            blk = 137; continue;
        }
        136 => {
            c.w64(9, c.r64(8));
            blk = 137; continue;
        }
        137 => {
            let Some(t228) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000840f8b); };
            c.w32(8, t228);
            let r229 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r229);
            let Some(t230) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000840f92); };
            c.w32(8, t230);
            let r231 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r231);
            let Some(t232) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000840f99); };
            c.w32(9, t232);
            let r233 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r233);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 140; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t234) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000840fab); };
            c.w32(6, t234);
            let r235 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r235);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000840fb2); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000840fbb, 0x7000840fb6);
        }
        139 => {
            blk = 28; continue;
        }
        140 => {
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000840fc0); };
            c.w64(6, t236);
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000840fc4); };
            c.w64(0, t237);
            blk = 241; continue;
        }
        141 => {
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000840fcd); };
            c.w64(6, t238);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000840fd1); };
            c.w64(0, t239);
            blk = 241; continue;
        }
        142 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000840fda); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000840fdc); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000840fdd); }
            let Some(t240) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000840fdf); };
            c.w64(3, t240);
            c.w32(0, 0x3u64);
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000840feb); };
            c.w64(6, t241);
            return c.call(0x700086ed74, 0x7000840ff4, 0x7000840fef);
        }
        143 => {
            blk = 28; continue;
        }
        144 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000840ff9); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841000); };
            c.w64(6, t242);
            return c.call(0x70007f2274, 0x7000841009, 0x7000841004);
        }
        145 => {
            blk = 28; continue;
        }
        146 => {
            let Some(t243) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084100e); };
            c.w32(8, t243);
            let r244 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r244);
            let Some(t245) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000841015); };
            c.w32(11, t245);
            c.cmp(c.r16(11), 0x834u64, 16);
            if c.cc_e() { blk = 168; continue; }
            blk = 147; continue;
        }
        147 => {
            c.w32(12, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 157; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r16(11), 0x411u64, 16);
            if c.cc_e() { blk = 150; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084103f); };
            c.w64(6, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841043); };
            c.w64(0, t247);
            blk = 241; continue;
        }
        150 => {
            let Some(t248) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084104c); };
            c.w32(8, t248);
            let Some(t249) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000841050); };
            c.cmp(t249, c.r32(8), 32);
            if c.cc_e() { blk = 152; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841059); };
            c.w64(6, t250);
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084105d); };
            c.w64(0, t251);
            blk = 241; continue;
        }
        152 => {
            let Some(t252) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000841066); };
            c.w32(8, t252);
            let r253 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r253);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 154; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841073); };
            c.w64(6, t254);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841077); };
            c.w64(0, t255);
            blk = 241; continue;
        }
        154 => {
            let Some(t256) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841080); };
            c.w32(9, t256);
            let r257 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r257);
            let Some(t258) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000841087); };
            c.cmp(t258, 0x40u64, 16);
            if c.cc_b() { blk = 156; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084108f); };
            c.w64(6, t259);
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841093); };
            c.w64(0, t260);
            blk = 241; continue;
        }
        156 => {
            let Some(t261) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084109c); };
            c.w32(8, t261);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 28; continue;
        }
        157 => {
            let Some(t262) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008410a9); };
            c.w32(11, t262);
            let Some(t263) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008410ad); };
            c.cmp(t263, c.r32(11), 32);
            if c.cc_e() { blk = 159; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008410b6); };
            c.w64(6, t264);
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008410ba); };
            c.w64(0, t265);
            blk = 241; continue;
        }
        159 => {
            let Some(t266) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008410c3); };
            c.w32(8, t266);
            c.w64(11, c.r64(8));
            let r267 = c.r32(11) & 0xc0u64;
            c.w32(11, r267);
            c.logic(r267, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 162; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t268) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008410db); };
            c.w32(11, t268);
            let r269 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r269);
            let Some(t270) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008410e2); };
            c.w32(11, t270);
            let r271 = c.r32(11) & 0x1fu64;
            c.w32(11, r271);
            c.logic(r271, 32);
            let r272 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r272);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008410f4); };
            c.w64(6, t273);
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008410f8); };
            c.w64(0, t274);
            blk = 241; continue;
        }
        162 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 167; continue; }
            blk = 163; continue;
        }
        163 => {
            let Some(t275) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000841107); };
            c.w32(0, t275);
            let r276 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r276);
            let Some(t277) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700084110e); };
            c.cmp(t277, c.r32(0), 32);
            if c.cc_e() { blk = 166; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t278) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841117); };
            c.w32(8, t278);
            let Some(t279) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700084111b); };
            c.cmp(t279, c.r32(8), 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 165; continue;
        }
        165 => {
            let Some(t280) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000841128); };
            c.w32(0, t280);
            let r281 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r281);
            blk = 28; continue;
        }
        166 => {
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841133); };
            c.w64(6, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841137); };
            c.w64(0, t283);
            blk = 241; continue;
        }
        167 => {
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841140); };
            c.w64(6, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841144); };
            c.w64(0, t285);
            blk = 241; continue;
        }
        168 => {
            let Some(t286) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084114d); };
            c.w32(8, t286);
            let Some(t287) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000841151); };
            c.cmp(t287, c.r32(8), 32);
            if c.cc_e() { blk = 170; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084115a); };
            c.w64(6, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084115e); };
            c.w64(0, t289);
            blk = 241; continue;
        }
        170 => {
            let Some(t290) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000841167); };
            c.w32(0, t290);
            let r291 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r291);
            blk = 28; continue;
        }
        171 => {
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841173); };
            c.w64(6, t292);
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841177); };
            c.w64(0, t293);
            blk = 241; continue;
        }
        172 => {
            let Some(t294) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000841180); };
            c.w64(5, t294);
            let r295 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r295);
            let Some(t296) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x7000841188); };
            if !c.push64(t296) { return RegfileExit::fault(0x7000841188); }
            c.w32(0, 0x1u64);
            let Some(t297) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x7000841190); };
            c.w64(3, t297);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(1)) { return RegfileExit::fault(0x7000841197); }
            return RegfileExit::jump(0x700086ed74);
        }
        173 => {
            c.w64(0, c.r64(9));
            blk = 28; continue;
        }
        174 => {
            c.logic(c.r8(11) & 0x40u64, 8);
            if c.cc_ne() { blk = 188; continue; }
            blk = 175; continue;
        }
        175 => {
            c.w64(8, c.r64(11));
            if let Some(sr) = c.shr(c.r64(8), 0x9u64, 64) { c.w64(8, sr); }
            c.logic(c.r16(11) & 0x100u64, 16);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            let r298 = c.r64(8) & 0x7ffu64;
            c.w64(8, r298);
            c.logic(r298, 64);
            c.logic(c.r8(11) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 183; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t299) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008411d5); };
            c.w32(9, t299);
            let r300 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r300);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 178; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t301) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008411e2); };
            c.w64(9, t301);
            blk = 178; continue;
        }
        178 => {
            let Some(t302) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008411e9); };
            c.w32(8, t302);
            let r303 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r303);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 180; continue; }
            blk = 179; continue;
        }
        179 => {
            c.w64(0, c.r64(8));
            blk = 28; continue;
        }
        180 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 241; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t304) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841209); };
            c.w32(9, t304);
            let Some(t305) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084120d); };
            c.cmp(t305, c.r32(9), 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 182; continue;
        }
        182 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700084121a); }
            blk = 233; continue;
        }
        183 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 185; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t306) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084122b); };
            c.w32(0, t306);
            let r307 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r307);
            blk = 28; continue;
        }
        185 => {
            let Some(t308) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841238); };
            c.w32(8, t308);
            let r309 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r309);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 241; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t310) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084124a); };
            c.w32(9, t310);
            let Some(t311) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084124e); };
            c.cmp(t311, c.r32(9), 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 187; continue;
        }
        187 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700084125b); }
            blk = 233; continue;
        }
        188 => {
            c.w64(8, c.r64(11));
            let r312 = c.shr(c.r32(8), 0x7u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r312);
            let r313 = c.r32(8) & 0xfu64;
            c.w32(8, r313);
            c.logic(r313, 32);
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_l() { blk = 195; continue; }
            blk = 189; continue;
        }
        189 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_l() { blk = 193; continue; }
            blk = 190; continue;
        }
        190 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_e() { blk = 239; continue; }
            blk = 191; continue;
        }
        191 => {
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 192; continue;
        }
        192 => {
            c.cmp(c.r32(8), 0x9u64, 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 193; continue;
        }
        193 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_e() { blk = 231; continue; }
            blk = 194; continue;
        }
        194 => {
            c.cmp(c.r32(8), 0x6u64, 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 195; continue;
        }
        195 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_l() { blk = 199; continue; }
            blk = 196; continue;
        }
        196 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 204; continue; }
            blk = 197; continue;
        }
        197 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 198; continue;
        }
        198 => {
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_e() { blk = 218; continue; }
            blk = 199; continue;
        }
        199 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 202; continue; }
            blk = 200; continue;
        }
        200 => {
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 201; continue;
        }
        201 => {
            return RegfileExit::fault(0x70008412db);
        }
        202 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r314 = c.r64(11) & 0xfffffu64;
            c.w64(11, r314);
            c.logic(r314, 64);
            let Some(t315) = c.ld8(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008412e7); };
            c.w32(8, (t315 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 28; continue;
        }
        203 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r316 = c.r64(11) & 0xfffffu64;
            c.w64(11, r316);
            c.logic(r316, 64);
            let Some(t317) = c.ld16(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841301); };
            c.w32(8, (t317 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 28; continue;
        }
        204 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r318 = c.r64(11) & 0xfffffu64;
            c.w64(11, r318);
            c.logic(r318, 64);
            let Some(t319) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084131b); };
            c.w32(8, t319);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000841320); }
            c.w64(9, c.r64(8));
            let r320 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r320);
            if c.cc_o() { blk = 206; continue; }
            blk = 205; continue;
        }
        205 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 28; continue;
        }
        206 => {
            let Some(t321) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000841334); };
            c.w64(9, t321);
            let Some(t322) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700084133b); };
            c.w64(11, t322);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t323) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000841342); };
            c.w64(15, t323);
            let Some(t324) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000841349); };
            c.cmp(t324, c.r64(12), 64);
            if c.cc_a() { blk = 209; continue; }
            blk = 207; continue;
        }
        207 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000841358, 0x7000841353);
        }
        208 => {
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841358); };
            c.w64(8, t325);
            blk = 210; continue;
        }
        209 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700084135e); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 210; continue;
        }
        210 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t326) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000841371); };
            c.w64(8, t326);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000841378); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084137c); }
            blk = 28; continue;
        }
        211 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r327 = c.r64(11) & 0xfffffu64;
            c.w64(11, r327);
            c.logic(r327, 64);
            let Some(t328) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841391); };
            c.w32(8, t328);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000841396); }
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 213; continue; }
            blk = 212; continue;
        }
        212 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 28; continue;
        }
        213 => {
            let Some(t329) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008413ac); };
            c.w64(9, t329);
            let Some(t330) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008413b3); };
            c.w64(11, t330);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t331) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008413ba); };
            c.w64(15, t331);
            let Some(t332) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008413c1); };
            c.cmp(t332, c.r64(12), 64);
            if c.cc_a() { blk = 216; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008413d0, 0x70008413cb);
        }
        215 => {
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008413d0); };
            c.w64(8, t333);
            blk = 217; continue;
        }
        216 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008413d6); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 217; continue;
        }
        217 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t334) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008413ec); };
            c.w64(8, t334);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008413f3); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008413f7); }
            blk = 28; continue;
        }
        218 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r335 = c.r64(11) & 0xfffffu64;
            c.w64(11, r335);
            c.logic(r335, 64);
            let Some(t336) = c.ld64(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084140c); };
            c.w64(8, t336);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000841411); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 226; continue; }
            blk = 219; continue;
        }
        219 => {
            let Some(t337) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084141b); };
            c.w64(9, t337);
            let Some(t338) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000841422); };
            c.w64(11, t338);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t339) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000841429); };
            c.w64(15, t339);
            let Some(t340) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000841430); };
            c.cmp(t340, c.r64(12), 64);
            if c.cc_a() { blk = 222; continue; }
            blk = 220; continue;
        }
        220 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x700084143f, 0x700084143a);
        }
        221 => {
            let Some(t341) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084143f); };
            c.w64(8, t341);
            blk = 223; continue;
        }
        222 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000841445); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 223; continue;
        }
        223 => {
            let Some(t342) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700084144f); };
            c.w64(11, t342);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000841456); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 225; continue; }
            blk = 224; continue;
        }
        224 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000841460); }
            let r343 = c.neg(c.r64(8), 64);
            c.w64(8, r343);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700084146a); }
            blk = 28; continue;
        }
        225 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x7000841473); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700084147a); }
            blk = 28; continue;
        }
        226 => {
            let Some(t344) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000841483); };
            c.w64(8, t344);
            let Some(t345) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700084148a); };
            c.w64(9, t345);
            c.w64(12, c.r64(9).wrapping_add(0x8));
            let Some(t346) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000841491); };
            c.w64(11, t346);
            let Some(t347) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000841498); };
            c.cmp(t347, c.r64(12), 64);
            if c.cc_a() { blk = 229; continue; }
            blk = 227; continue;
        }
        227 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70008414a7, 0x70008414a2);
        }
        228 => {
            blk = 230; continue;
        }
        229 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008414a9); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 230; continue;
        }
        230 => {
            let Some(t348) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008414b3); };
            c.w64(8, t348);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008414ba); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008414be); }
            blk = 28; continue;
        }
        231 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r349 = c.r64(11) & 0xfffffu64;
            c.w64(11, r349);
            c.logic(r349, 64);
            if !c.movss_rm(0, c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008414d5); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 233; continue;
        }
        232 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r350 = c.r64(11) & 0xfffffu64;
            c.w64(11, r350);
            c.logic(r350, 64);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008414ed); }
            blk = 233; continue;
        }
        233 => {
            let Some(t351) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008414f4); };
            c.w64(8, t351);
            let Some(t352) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008414fb); };
            c.w64(9, t352);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x7000841502); }
            let Some(t353) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000841507); };
            c.w64(12, t353);
            let Some(t354) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700084150e); };
            c.cmp(t354, c.r64(11), 64);
            if c.cc_a() { blk = 236; continue; }
            blk = 234; continue;
        }
        234 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700084151e, 0x7000841519);
        }
        235 => {
            blk = 237; continue;
        }
        236 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000841520); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 237; continue;
        }
        237 => {
            let Some(t355) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084152a); };
            c.w64(8, t355);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000841531); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x7000841535); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084153a); }
            blk = 28; continue;
        }
        238 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r356 = c.r64(11) & 0xfffffu64;
            c.w64(11, r356);
            c.logic(r356, 64);
            let Some(t357) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084154f); };
            c.w32(0, t357);
            let r358 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r358);
            blk = 28; continue;
        }
        239 => {
            let Some(t359) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700084155c); };
            c.w64(8, t359);
            let Some(t360) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000841563); };
            c.w32(8, t360);
            let r361 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r361);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700084156a); }
            let Some(t362) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x700084156c); };
            c.w64(3, t362);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700084157f, 0x700084157a);
        }
        240 => {
            return RegfileExit::fault(0x700084157f);
        }
        241 => {
            let Some(t363) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000841580); };
            c.w64(5, t363);
            let r364 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r364);
            let Some(t365) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000841588); };
            if !c.push64(t365) { return RegfileExit::fault(0x7000841588); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(0)) { return RegfileExit::fault(0x700084158c); }
            c.w32(0, 0x4u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(3)) { return RegfileExit::fault(0x7000841596); }
            let Some(t366) = c.ld64(c.r64(13).wrapping_add(0x2040)) else { return RegfileExit::fault(0x700084159b); };
            c.w64(3, t366);
            if !c.st64(c.r64(4).wrapping_add(0x8), 0x2u64) { return RegfileExit::fault(0x70008415a2); }
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(1)) { return RegfileExit::fault(0x70008415ab); }
            return RegfileExit::jump(0x700086ed74);
        }
        242 => {
            return RegfileExit::fault(0x70008415b5);
        }
        243 => {
            c.w32(0, 0xcu64);
            return RegfileExit::jump(0x7000804df4);
        }
        244 => {
            return RegfileExit::jump(0x70008415c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008415d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008415d4); };
            c.cmp(t0, c.r32(3), 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t1) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008415e5); };
            c.w32(9, t1);
            let r2 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r2);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let r3 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r3);
            c.logic(r3, 64);
            let Some(t4) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000841601); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000841608); };
            c.cmp(t6, c.r32(8), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x7000841618);
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841619); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000841620); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841627); };
            c.w32(11, t11);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xa7u64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x700084163a);
        }
        7 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x700084163b); };
            c.w32(8, t12);
            let r13 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r13);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841648); };
            c.w32(11, t14);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            let r15 = c.r64(12) & 0xfffu64;
            c.w64(12, r15);
            c.logic(r15, 64);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(15, c.r64(12).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_ae() { blk = 242; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t16) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700084166d); };
            c.w32(8, t16);
            let r17 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r17);
            let Some(t18) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841675); };
            c.w32(11, t18);
            if let Some(sr) = c.shr(c.r64(9), 0xcu64, 64) { c.w64(9, sr); }
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r19 = c.r64(9) & 0x3ffffu64;
            c.w64(9, r19);
            c.logic(r19, 64);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x700084168f);
        }
        10 => {
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000841690); };
            c.w32(0, t20);
            let r21 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r21);
            return c.ret(0x0, 0x7000841698);
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000841699); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700084169d); }
            let r22 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r22);
            blk = 241; continue;
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008416a8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008416ac); }
            let r23 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r23);
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008416b2); };
            c.w32(8, t24);
            let r25 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r25);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008416ba); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008416be); }
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70008416c2); };
            c.cmp(t26, c.r32(8), 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t27) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008416cf); };
            c.w32(9, t27);
            let r28 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r28);
            let Some(t29) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008416d6); };
            c.w32(9, t29);
            let r30 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r30);
            let Some(t31) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008416dd); };
            c.w32(11, t31);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x3u64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x70008416ed);
        }
        15 => {
            let Some(t32) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008416ee); };
            c.w32(12, t32);
            let r33 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r33);
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x70008416fb);
        }
        17 => {
            let Some(t34) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008416fc); };
            c.w32(2, t34);
            let r35 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r35);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084170d); };
            c.w32(9, t36);
            let Some(t37) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x7000841711); };
            c.cmp(t37, c.r32(9), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084171a); };
            c.w32(9, t38);
            let r39 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r39);
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t40) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841729); };
            c.cmp(t40, c.r32(11), 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841733); };
            c.w32(9, t41);
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000841737); };
            c.w32(11, t42);
            let r43 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r43);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000841744); };
            c.cmp(t44, c.r32(11), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 24; continue;
        }
        24 => {
            let r45 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r45);
            c.logic(r45, 64);
            blk = 26; continue;
        }
        25 => {
            c.w64(11, c.r64(2));
            blk = 26; continue;
        }
        26 => {
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(10));
            blk = 34; continue;
        }
        27 => {
            c.w64(0, c.r64(11));
            blk = 28; continue;
        }
        28 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000841771); }
            return c.ret(0x0, 0x7000841772);
        }
        29 => {
            let Some(t46) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000841773); };
            c.w64(5, t46);
            let r47 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r47);
            let Some(t48) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700084177b); };
            c.logic(t48 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            let r49 = c.add(c.r64(8), 0x3fu64, 64);
            c.w64(8, r49);
            blk = 32; continue;
        }
        31 => {
            let Some(t50) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084178b); };
            c.w32(8, t50);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x700084178f); };
            c.w64(8, t51);
            blk = 32; continue;
        }
        32 => {
            return RegfileExit::jump(c.r64(8));
        }
        33 => {
            c.w64(9, c.r64(2));
            c.w64(11, c.r64(8));
            blk = 34; continue;
        }
        34 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(15, c.r64(11));
            let r52 = c.r64(15) & 0xfu64;
            c.w64(15, r52);
            c.logic(r52, 64);
            c.cmp(c.r64(15), 0x4u64, 64);
            if c.cc_e() { blk = 174; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(15), 0x5u64, 64);
            if c.cc_e() { blk = 173; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(15), 0xdu64, 64);
            if c.cc_e() { blk = 172; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r64(15), 0x2u64, 64);
            if c.cc_e() { blk = 112; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r64(15), 0x6u64, 64);
            if c.cc_e() { blk = 110; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_e() { blk = 107; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r64(15), 0x8u64, 64);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 97; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r64(15), 0x9u64, 64);
            if c.cc_e() { blk = 97; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r64(15), 0x3u64, 64);
            if c.cc_e() { blk = 54; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_e() { blk = 109; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r64(15), 0xcu64, 64);
            if c.cc_e() { blk = 52; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r64(15), 0xeu64, 64);
            if c.cc_e() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t53) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700084182c); };
            c.w64(5, t53);
            let r54 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r54);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000841834); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000841836); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000841838); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000841839); }
            let Some(t55) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700084183a); };
            if !c.push64(t55) { return RegfileExit::fault(0x700084183a); }
            c.w32(0, 0x5u64);
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x7000841843); };
            c.w64(3, t56);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(1)) { return RegfileExit::fault(0x700084184a); }
            return RegfileExit::jump(0x700086ed74);
        }
        47 => {
            let Some(t57) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000841854); };
            c.w32(8, t57);
            let r58 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r58);
            let Some(t59) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084185b); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841862); };
            c.w32(9, t61);
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r62 = c.r64(11) & 0x1ffffffu64;
            c.w64(11, r62);
            c.logic(r62, 64);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            return RegfileExit::fault(0x700084187c);
        }
        49 => {
            let Some(t63) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084187d); };
            c.w32(8, t63);
            let r64 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r64);
            let Some(t65) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841885); };
            c.w32(0, t65);
            let r66 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r66);
            let Some(t67) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700084188c); };
            c.cmp(t67, c.r32(0), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 28; continue;
        }
        51 => {
            let Some(t68) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700084189a); };
            c.w64(5, t68);
            let r69 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r69);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008418a2); }
            let Some(t70) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008418a3); };
            if !c.push64(t70) { return RegfileExit::fault(0x70008418a3); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x70008418a7); };
            c.w64(3, t71);
            c.w32(0, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), 0x174u64) { return RegfileExit::fault(0x70008418b3); }
            return RegfileExit::jump(0x700086ed74);
        }
        52 => {
            let Some(t72) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008418c1); };
            c.w64(5, t72);
            c.w32(2, 0x2u64);
            c.w64(0, c.r64(9));
            c.w64(3, c.r64(1));
            c.w64(1, c.r64(12));
            let r73 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r73);
            return RegfileExit::jump(0x70008bc174);
        }
        53 => {
            blk = 109; continue;
        }
        54 => {
            let Some(t74) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008418e1); };
            c.w32(8, t74);
            let r75 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r75);
            let Some(t76) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008418e8); };
            c.w32(11, t76);
            let Some(t77) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008418ec); };
            c.cmp(t77, c.r32(8), 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 55; continue;
        }
        55 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t78) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841903); };
            c.w32(11, t78);
            let r79 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r79);
            let Some(t80) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700084190a); };
            c.cmp(t80, c.r32(11), 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t81) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841917); };
            c.w32(2, t81);
            let r82 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r82);
            let Some(t83) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084191e); };
            c.w32(11, t83);
            let r84 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r84);
            let Some(t85) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000841925); };
            c.logic(t85 & 0x2u64, 8);
            if c.cc_ne() { blk = 77; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t86) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000841930); };
            c.cmp(t86, c.r32(11), 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000841939); };
            c.w64(0, t87);
            blk = 28; continue;
        }
        60 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000841945); };
            c.w64(11, t88);
            let Some(t89) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x700084194c); };
            c.cmp(t89, 0x20u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t90) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000841956); };
            c.w32(11, t90);
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700084195a); };
            c.cmp(t91, c.r32(11), 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t92) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841967); };
            c.w32(11, t92);
            let r93 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r93);
            let Some(t94) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084196e); };
            c.w32(15, t94);
            let Some(t95) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000841972); };
            c.cmp(t95, c.r32(15), 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t96) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700084197b); };
            c.w32(11, t96);
            let r97 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r97);
            let Some(t98) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841982); };
            c.w32(15, t98);
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000841986); };
            c.cmp(t99, c.r32(15), 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 64; continue;
        }
        64 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 241; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t100) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841999); };
            c.w32(15, t100);
            let r101 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r101);
            let Some(t102) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008419a0); };
            c.w32(7, t102);
            c.w32(6, c.r64(7).wrapping_sub(0x80c));
            c.cmp(c.r32(6), 0xeu64, 32);
            if c.cc_be() { blk = 70; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(11, c.r16(7));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t103) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008419bd); };
            c.cmp(t103, c.r32(15), 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008419c6); };
            c.w64(6, t104);
            blk = 241; continue;
        }
        69 => {
            c.w64(9, c.r64(8));
            blk = 71; continue;
        }
        70 => {
            c.w64(9, c.r64(11));
            blk = 71; continue;
        }
        71 => {
            let Some(t105) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008419d7); };
            c.w32(8, t105);
            let r106 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r106);
            let Some(t107) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008419de); };
            c.w32(8, t107);
            let r108 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r108);
            let Some(t109) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008419e5); };
            c.w32(9, t109);
            let r110 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r110);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 74; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t111) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008419f7); };
            c.w32(6, t111);
            let r112 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r112);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008419fe); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000841a07, 0x7000841a02);
        }
        73 => {
            blk = 28; continue;
        }
        74 => {
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841a0c); };
            c.w64(6, t113);
            blk = 241; continue;
        }
        75 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000841a15); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000841a17); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000841a18); }
            c.w32(0, 0x3u64);
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000841a1f); };
            c.w64(3, t114);
            c.w64(15, c.r64(0));
            return c.call(0x700086ed74, 0x7000841a2e, 0x7000841a29);
        }
        76 => {
            blk = 28; continue;
        }
        77 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000841a33); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(2));
            return c.call(0x70007f2274, 0x7000841a3f, 0x7000841a3a);
        }
        78 => {
            blk = 28; continue;
        }
        79 => {
            let Some(t115) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841a44); };
            c.w32(12, t115);
            let r116 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r116);
            let Some(t117) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000841a4b); };
            c.w32(15, t117);
            c.cmp(c.r16(15), 0x834u64, 16);
            if c.cc_e() { blk = 94; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w32(2, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_be() { blk = 86; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r16(15), 0x411u64, 16);
            if c.cc_ne() { blk = 241; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t118) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841a75); };
            c.w32(9, t118);
            let Some(t119) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000841a79); };
            c.cmp(t119, c.r32(9), 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 83; continue;
        }
        83 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 241; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t120) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000841a90); };
            c.cmp(t120, 0x40u64, 16);
            if c.cc_ae() { blk = 241; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t121) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000841a9c); };
            c.w32(8, t121);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 28; continue;
        }
        86 => {
            let Some(t122) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841aa9); };
            c.w32(11, t122);
            let Some(t123) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000841aad); };
            c.cmp(t123, c.r32(11), 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t124) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000841aba); };
            c.w32(11, t124);
            c.w64(12, c.r64(11));
            let r125 = c.r32(12) & 0xc0u64;
            c.w32(12, r125);
            c.logic(r125, 32);
            c.cmp(c.r32(12), 0xc0u64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t126) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000841ad3); };
            c.w32(8, t126);
            let r127 = c.r32(8) & 0x1fu64;
            c.w32(8, r127);
            c.logic(r127, 32);
            let r128 = c.sub(c.r32(8), 0x10u64, 32);
            c.w32(8, r128);
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_a() { blk = 241; continue; }
            blk = 89; continue;
        }
        89 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 241; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t129) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000841af3); };
            c.w32(8, t129);
            let r130 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r130);
            let Some(t131) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000841afa); };
            c.cmp(t131, c.r32(8), 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t132) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841b07); };
            c.w32(9, t132);
            let Some(t133) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000841b0b); };
            c.cmp(t133, c.r32(9), 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w64(0, c.r64(8));
            blk = 28; continue;
        }
        93 => {
            let Some(t134) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000841b1c); };
            c.w32(0, t134);
            let r135 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r135);
            blk = 28; continue;
        }
        94 => {
            let Some(t136) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841b28); };
            c.w32(9, t136);
            let Some(t137) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000841b2c); };
            c.cmp(t137, c.r32(9), 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(0, c.r64(8));
            blk = 28; continue;
        }
        96 => {
            c.w64(0, c.r64(8));
            blk = 28; continue;
        }
        97 => {
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000841b49); };
            c.w64(15, t138);
            let Some(t139) = c.ld8(c.r64(15)) else { return RegfileExit::fault(0x7000841b50); };
            c.cmp(t139, 0x20u64, 8);
            if c.cc_e() { blk = 109; continue; }
            blk = 98; continue;
        }
        98 => {
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 100; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t140) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000841b60); };
            c.w32(8, t140);
            let r141 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r141);
            blk = 101; continue;
        }
        100 => {
            let Some(t142) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000841b69); };
            c.w32(8, t142);
            let r143 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r143);
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_ne() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            return RegfileExit::fault(0x7000841b76);
        }
        103 => {
            let r144 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r144);
            c.logic(r144, 64);
            let Some(t145) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000841b7b); };
            c.w32(11, t145);
            let r146 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r146);
            let Some(t147) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000841b82); };
            c.w32(3, t147);
            let r148 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r148);
            let Some(t149) = c.ld64(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841b89); };
            c.w64(9, t149);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t150) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841b92); };
            c.w32(11, t150);
            let r151 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r151);
            let Some(t152) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000841b99); };
            c.w32(2, t152);
            let r153 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r153);
            blk = 105; continue;
        }
        105 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000841ba0); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(2));
            c.w64(2, c.r64(9));
            c.w64(6, c.r64(8));
            return c.call(0x70007fe8d4, 0x7000841bb2, 0x7000841bad);
        }
        106 => {
            blk = 28; continue;
        }
        107 => {
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000841bb7); };
            c.w64(8, t154);
            let Some(t155) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x7000841bbe); };
            c.cmp(t155, 0x20u64, 8);
            if c.cc_e() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            let r156 = c.r64(11) & 0x3ffu64;
            c.w64(11, r156);
            c.logic(r156, 64);
            let r157 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(8, r157);
            let Some(t158) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841bd3); };
            c.w32(11, t158);
            let r159 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r159);
            let Some(t160) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000841bda); };
            c.w32(11, t160);
            let r161 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r161);
            let Some(t162) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000841be1); };
            c.w32(3, t162);
            let r163 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r163);
            let Some(t164) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000841be9); };
            c.w64(5, t164);
            c.w64(2, c.r64(12));
            c.w64(1, c.r64(9));
            let r165 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r165);
            return RegfileExit::jump(0x70007fea14);
        }
        109 => {
            let Some(t166) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000841bfc); };
            c.w64(5, t166);
            let r167 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r167);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000841c04); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000841c05); }
            let Some(t168) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000841c06); };
            if !c.push64(t168) { return RegfileExit::fault(0x7000841c06); }
            c.w32(0, 0x3u64);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(1)) { return RegfileExit::fault(0x7000841c0f); }
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0x2048)) else { return RegfileExit::fault(0x7000841c14); };
            c.w64(3, t169);
            return RegfileExit::jump(0x700086ed74);
        }
        110 => {
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            let r170 = c.r64(11) & 0x3ffu64;
            c.w64(11, r170);
            c.logic(r170, 64);
            let r171 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(8, r171);
            let Some(t172) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841c2f); };
            c.w32(9, t172);
            let r173 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r173);
            let Some(t174) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000841c36); };
            c.w32(9, t174);
            let r175 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r175);
            let Some(t176) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000841c3d); };
            c.w32(8, t176);
            let r177 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r177);
            let Some(t178) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841c45); };
            c.w32(7, t178);
            let r179 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r179);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000841c4c); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000841c55, 0x7000841c50);
        }
        111 => {
            blk = 28; continue;
        }
        112 => {
            let Some(t180) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841c5a); };
            c.w32(8, t180);
            let r181 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r181);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t182) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000841c67); };
            c.w64(8, t182);
            blk = 114; continue;
        }
        114 => {
            let Some(t183) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000841c6e); };
            c.w32(11, t183);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r184 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r184);
            let Some(t185) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841c7c); };
            c.w32(15, t185);
            let r186 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r186);
            let r187 = c.r64(15) & c.r64(11);
            c.w64(15, r187);
            c.logic(r187, 64);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 115; continue;
        }
        115 => {
            c.w64(7, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(6, c.r64(7).wrapping_mul(4));
            let Some(t188) = c.ld32(c.r64(8).wrapping_add(c.r64(6)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000841c96); };
            c.w32(0, t188);
            let Some(t189) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000841c9b); };
            c.cmp(t189, c.r32(0), 32);
            if c.cc_e() { blk = 171; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(0), c.r32(1), 32);
            if c.cc_e() { blk = 118; continue; }
            blk = 117; continue;
        }
        117 => {
            let r190 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r190);
            let r191 = c.add(c.r64(15), c.r64(2), 64);
            c.w64(15, r191);
            c.w64(0, c.r64(11));
            let r192 = c.r64(0) & c.r64(15);
            c.w64(0, r192);
            c.logic(r192, 64);
            c.w64(15, c.r64(0));
            blk = 115; continue;
        }
        118 => {
            let Some(t193) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841cbe); };
            c.w32(11, t193);
            let Some(t194) = c.ld32(c.r64(8).wrapping_add(c.r64(6)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000841cc2); };
            c.w32(15, t194);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(0, c.r64(7).wrapping_add(0x1));
            c.cmp(c.r64(0), c.r64(11), 64);
            if c.cc_b() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            return RegfileExit::fault(0x7000841cd6);
        }
        120 => {
            let Some(t195) = c.ld32(c.r64(8).wrapping_add(c.r64(6)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000841cd7); };
            c.w32(0, t195);
            let r196 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r196);
            c.logic(c.r8(15) & 0x2u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t197) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841ce9); };
            c.w32(8, t197);
            let Some(t198) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000841ced); };
            c.cmp(t198, c.r32(8), 32);
            if c.cc_ne() { blk = 146; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t199) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841cfa); };
            c.w32(2, t199);
            let r200 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r200);
            let Some(t201) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841d00); };
            c.w32(8, t201);
            let r202 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r202);
            let Some(t203) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000841d07); };
            c.logic(t203 & 0x2u64, 8);
            if c.cc_ne() { blk = 144; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t204) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000841d12); };
            c.cmp(t204, c.r32(8), 32);
            if c.cc_e() { blk = 125; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000841d1b); };
            c.w64(0, t205);
            blk = 28; continue;
        }
        125 => {
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000841d27); };
            c.w64(8, t206);
            let Some(t207) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x7000841d2e); };
            c.cmp(t207, 0x20u64, 8);
            if c.cc_e() { blk = 142; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t208) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000841d38); };
            c.w32(8, t208);
            let Some(t209) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000841d3c); };
            c.cmp(t209, c.r32(8), 32);
            if c.cc_e() { blk = 128; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841d45); };
            c.w64(6, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841d49); };
            c.w64(0, t211);
            blk = 241; continue;
        }
        128 => {
            let Some(t212) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841d52); };
            c.w32(8, t212);
            let r213 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r213);
            let Some(t214) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841d59); };
            c.w32(11, t214);
            let Some(t215) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000841d5d); };
            c.cmp(t215, c.r32(11), 32);
            if c.cc_ne() { blk = 130; continue; }
            blk = 129; continue;
        }
        129 => {
            let Some(t216) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000841d66); };
            c.w32(8, t216);
            let r217 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r217);
            let Some(t218) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841d6d); };
            c.w32(11, t218);
            let Some(t219) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000841d71); };
            c.cmp(t219, c.r32(11), 32);
            if c.cc_e() { blk = 129; continue; }
            blk = 130; continue;
        }
        130 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 141; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t220) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841d84); };
            c.w32(11, t220);
            let r221 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r221);
            let Some(t222) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000841d8b); };
            c.w32(15, t222);
            c.w32(0, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 136; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w32(8, c.r16(15));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t223) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000841da9); };
            c.cmp(t223, c.r32(11), 32);
            if c.cc_ne() { blk = 137; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841db2); };
            c.w64(6, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841db6); };
            c.w64(0, t225);
            blk = 241; continue;
        }
        135 => {
            let Some(t226) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000841dbf); };
            c.w32(11, t226);
            let r227 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r227);
            c.w64(9, c.r64(11));
            blk = 137; continue;
        }
        136 => {
            c.w64(9, c.r64(8));
            blk = 137; continue;
        }
        137 => {
            let Some(t228) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000841dce); };
            c.w32(8, t228);
            let r229 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r229);
            let Some(t230) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841dd5); };
            c.w32(8, t230);
            let r231 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r231);
            let Some(t232) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000841ddc); };
            c.w32(9, t232);
            let r233 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r233);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 140; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t234) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000841dee); };
            c.w32(6, t234);
            let r235 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r235);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000841df5); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000841dfe, 0x7000841df9);
        }
        139 => {
            blk = 28; continue;
        }
        140 => {
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841e03); };
            c.w64(6, t236);
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841e07); };
            c.w64(0, t237);
            blk = 241; continue;
        }
        141 => {
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841e10); };
            c.w64(6, t238);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841e14); };
            c.w64(0, t239);
            blk = 241; continue;
        }
        142 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000841e1d); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000841e1f); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000841e20); }
            let Some(t240) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000841e22); };
            c.w64(3, t240);
            c.w32(0, 0x3u64);
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841e2e); };
            c.w64(6, t241);
            return c.call(0x700086ed74, 0x7000841e37, 0x7000841e32);
        }
        143 => {
            blk = 28; continue;
        }
        144 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000841e3c); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841e43); };
            c.w64(6, t242);
            return c.call(0x70007f2274, 0x7000841e4c, 0x7000841e47);
        }
        145 => {
            blk = 28; continue;
        }
        146 => {
            let Some(t243) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841e51); };
            c.w32(8, t243);
            let r244 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r244);
            let Some(t245) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000841e58); };
            c.w32(11, t245);
            c.cmp(c.r16(11), 0x834u64, 16);
            if c.cc_e() { blk = 168; continue; }
            blk = 147; continue;
        }
        147 => {
            c.w32(12, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 157; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r16(11), 0x411u64, 16);
            if c.cc_e() { blk = 150; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841e82); };
            c.w64(6, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841e86); };
            c.w64(0, t247);
            blk = 241; continue;
        }
        150 => {
            let Some(t248) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841e8f); };
            c.w32(8, t248);
            let Some(t249) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000841e93); };
            c.cmp(t249, c.r32(8), 32);
            if c.cc_e() { blk = 152; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841e9c); };
            c.w64(6, t250);
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841ea0); };
            c.w64(0, t251);
            blk = 241; continue;
        }
        152 => {
            let Some(t252) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000841ea9); };
            c.w32(8, t252);
            let r253 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r253);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 154; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841eb6); };
            c.w64(6, t254);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841eba); };
            c.w64(0, t255);
            blk = 241; continue;
        }
        154 => {
            let Some(t256) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841ec3); };
            c.w32(9, t256);
            let r257 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r257);
            let Some(t258) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000841eca); };
            c.cmp(t258, 0x40u64, 16);
            if c.cc_b() { blk = 156; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841ed2); };
            c.w64(6, t259);
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841ed6); };
            c.w64(0, t260);
            blk = 241; continue;
        }
        156 => {
            let Some(t261) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000841edf); };
            c.w32(8, t261);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 28; continue;
        }
        157 => {
            let Some(t262) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841eec); };
            c.w32(11, t262);
            let Some(t263) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000841ef0); };
            c.cmp(t263, c.r32(11), 32);
            if c.cc_e() { blk = 159; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841ef9); };
            c.w64(6, t264);
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841efd); };
            c.w64(0, t265);
            blk = 241; continue;
        }
        159 => {
            let Some(t266) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000841f06); };
            c.w32(8, t266);
            c.w64(11, c.r64(8));
            let r267 = c.r32(11) & 0xc0u64;
            c.w32(11, r267);
            c.logic(r267, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 162; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t268) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000841f1e); };
            c.w32(11, t268);
            let r269 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r269);
            let Some(t270) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000841f25); };
            c.w32(11, t270);
            let r271 = c.r32(11) & 0x1fu64;
            c.w32(11, r271);
            c.logic(r271, 32);
            let r272 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r272);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841f37); };
            c.w64(6, t273);
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841f3b); };
            c.w64(0, t274);
            blk = 241; continue;
        }
        162 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 167; continue; }
            blk = 163; continue;
        }
        163 => {
            let Some(t275) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000841f4a); };
            c.w32(0, t275);
            let r276 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r276);
            let Some(t277) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000841f51); };
            c.cmp(t277, c.r32(0), 32);
            if c.cc_e() { blk = 166; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t278) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000841f5a); };
            c.w32(8, t278);
            let Some(t279) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000841f5e); };
            c.cmp(t279, c.r32(8), 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 165; continue;
        }
        165 => {
            let Some(t280) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000841f6b); };
            c.w32(0, t280);
            let r281 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r281);
            blk = 28; continue;
        }
        166 => {
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841f76); };
            c.w64(6, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841f7a); };
            c.w64(0, t283);
            blk = 241; continue;
        }
        167 => {
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841f83); };
            c.w64(6, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841f87); };
            c.w64(0, t285);
            blk = 241; continue;
        }
        168 => {
            let Some(t286) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000841f90); };
            c.w32(8, t286);
            let Some(t287) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000841f94); };
            c.cmp(t287, c.r32(8), 32);
            if c.cc_e() { blk = 170; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841f9d); };
            c.w64(6, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841fa1); };
            c.w64(0, t289);
            blk = 241; continue;
        }
        170 => {
            let Some(t290) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000841faa); };
            c.w32(0, t290);
            let r291 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r291);
            blk = 28; continue;
        }
        171 => {
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000841fb6); };
            c.w64(6, t292);
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000841fba); };
            c.w64(0, t293);
            blk = 241; continue;
        }
        172 => {
            let Some(t294) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000841fc3); };
            c.w64(0, t294);
            blk = 28; continue;
        }
        173 => {
            c.w64(0, c.r64(9));
            blk = 28; continue;
        }
        174 => {
            c.logic(c.r8(11) & 0x40u64, 8);
            if c.cc_ne() { blk = 188; continue; }
            blk = 175; continue;
        }
        175 => {
            c.w64(8, c.r64(11));
            if let Some(sr) = c.shr(c.r64(8), 0x9u64, 64) { c.w64(8, sr); }
            c.logic(c.r16(11) & 0x100u64, 16);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            let r295 = c.r64(8) & 0x7ffu64;
            c.w64(8, r295);
            c.logic(r295, 64);
            c.logic(c.r8(11) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 183; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t296) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000842003); };
            c.w32(9, t296);
            let r297 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r297);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 178; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t298) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000842010); };
            c.w64(9, t298);
            blk = 178; continue;
        }
        178 => {
            let Some(t299) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842017); };
            c.w32(8, t299);
            let r300 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r300);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 180; continue; }
            blk = 179; continue;
        }
        179 => {
            c.w64(0, c.r64(8));
            blk = 28; continue;
        }
        180 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 241; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t301) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842037); };
            c.w32(9, t301);
            let Some(t302) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084203b); };
            c.cmp(t302, c.r32(9), 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 182; continue;
        }
        182 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000842048); }
            blk = 233; continue;
        }
        183 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 185; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t303) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842059); };
            c.w32(0, t303);
            let r304 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r304);
            blk = 28; continue;
        }
        185 => {
            let Some(t305) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842066); };
            c.w32(8, t305);
            let r306 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r306);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 241; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t307) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842078); };
            c.w32(9, t307);
            let Some(t308) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084207c); };
            c.cmp(t308, c.r32(9), 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 187; continue;
        }
        187 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000842089); }
            blk = 233; continue;
        }
        188 => {
            c.w64(8, c.r64(11));
            let r309 = c.shr(c.r32(8), 0x7u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r309);
            let r310 = c.r32(8) & 0xfu64;
            c.w32(8, r310);
            c.logic(r310, 32);
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_l() { blk = 195; continue; }
            blk = 189; continue;
        }
        189 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_l() { blk = 193; continue; }
            blk = 190; continue;
        }
        190 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_e() { blk = 239; continue; }
            blk = 191; continue;
        }
        191 => {
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 192; continue;
        }
        192 => {
            c.cmp(c.r32(8), 0x9u64, 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 193; continue;
        }
        193 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_e() { blk = 231; continue; }
            blk = 194; continue;
        }
        194 => {
            c.cmp(c.r32(8), 0x6u64, 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 195; continue;
        }
        195 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_l() { blk = 199; continue; }
            blk = 196; continue;
        }
        196 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 204; continue; }
            blk = 197; continue;
        }
        197 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 198; continue;
        }
        198 => {
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_e() { blk = 218; continue; }
            blk = 199; continue;
        }
        199 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 202; continue; }
            blk = 200; continue;
        }
        200 => {
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 201; continue;
        }
        201 => {
            return RegfileExit::fault(0x7000842109);
        }
        202 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r311 = c.r64(11) & 0xfffffu64;
            c.w64(11, r311);
            c.logic(r311, 64);
            let Some(t312) = c.ld8(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842115); };
            c.w32(8, (t312 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 28; continue;
        }
        203 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r313 = c.r64(11) & 0xfffffu64;
            c.w64(11, r313);
            c.logic(r313, 64);
            let Some(t314) = c.ld16(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084212f); };
            c.w32(8, (t314 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 28; continue;
        }
        204 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r315 = c.r64(11) & 0xfffffu64;
            c.w64(11, r315);
            c.logic(r315, 64);
            let Some(t316) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842149); };
            c.w32(8, t316);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700084214e); }
            c.w64(9, c.r64(8));
            let r317 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r317);
            if c.cc_o() { blk = 206; continue; }
            blk = 205; continue;
        }
        205 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 28; continue;
        }
        206 => {
            let Some(t318) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000842162); };
            c.w64(9, t318);
            let Some(t319) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000842169); };
            c.w64(11, t319);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t320) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000842170); };
            c.w64(15, t320);
            let Some(t321) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000842177); };
            c.cmp(t321, c.r64(12), 64);
            if c.cc_a() { blk = 209; continue; }
            blk = 207; continue;
        }
        207 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000842186, 0x7000842181);
        }
        208 => {
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000842186); };
            c.w64(8, t322);
            blk = 210; continue;
        }
        209 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700084218c); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 210; continue;
        }
        210 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t323) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084219f); };
            c.w64(8, t323);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008421a6); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008421aa); }
            blk = 28; continue;
        }
        211 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r324 = c.r64(11) & 0xfffffu64;
            c.w64(11, r324);
            c.logic(r324, 64);
            let Some(t325) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008421bf); };
            c.w32(8, t325);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70008421c4); }
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 213; continue; }
            blk = 212; continue;
        }
        212 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 28; continue;
        }
        213 => {
            let Some(t326) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008421da); };
            c.w64(9, t326);
            let Some(t327) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008421e1); };
            c.w64(11, t327);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t328) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008421e8); };
            c.w64(15, t328);
            let Some(t329) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008421ef); };
            c.cmp(t329, c.r64(12), 64);
            if c.cc_a() { blk = 216; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008421fe, 0x70008421f9);
        }
        215 => {
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008421fe); };
            c.w64(8, t330);
            blk = 217; continue;
        }
        216 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000842204); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 217; continue;
        }
        217 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t331) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084221a); };
            c.w64(8, t331);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000842221); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000842225); }
            blk = 28; continue;
        }
        218 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r332 = c.r64(11) & 0xfffffu64;
            c.w64(11, r332);
            c.logic(r332, 64);
            let Some(t333) = c.ld64(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084223a); };
            c.w64(8, t333);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700084223f); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 226; continue; }
            blk = 219; continue;
        }
        219 => {
            let Some(t334) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000842249); };
            c.w64(9, t334);
            let Some(t335) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000842250); };
            c.w64(11, t335);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t336) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000842257); };
            c.w64(15, t336);
            let Some(t337) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700084225e); };
            c.cmp(t337, c.r64(12), 64);
            if c.cc_a() { blk = 222; continue; }
            blk = 220; continue;
        }
        220 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x700084226d, 0x7000842268);
        }
        221 => {
            let Some(t338) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084226d); };
            c.w64(8, t338);
            blk = 223; continue;
        }
        222 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000842273); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 223; continue;
        }
        223 => {
            let Some(t339) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700084227d); };
            c.w64(11, t339);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000842284); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 225; continue; }
            blk = 224; continue;
        }
        224 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700084228e); }
            let r340 = c.neg(c.r64(8), 64);
            c.w64(8, r340);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000842298); }
            blk = 28; continue;
        }
        225 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70008422a1); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x70008422a8); }
            blk = 28; continue;
        }
        226 => {
            let Some(t341) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008422b1); };
            c.w64(8, t341);
            let Some(t342) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008422b8); };
            c.w64(9, t342);
            c.w64(12, c.r64(9).wrapping_add(0x8));
            let Some(t343) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008422bf); };
            c.w64(11, t343);
            let Some(t344) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008422c6); };
            c.cmp(t344, c.r64(12), 64);
            if c.cc_a() { blk = 229; continue; }
            blk = 227; continue;
        }
        227 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70008422d5, 0x70008422d0);
        }
        228 => {
            blk = 230; continue;
        }
        229 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008422d7); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 230; continue;
        }
        230 => {
            let Some(t345) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008422e1); };
            c.w64(8, t345);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008422e8); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008422ec); }
            blk = 28; continue;
        }
        231 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r346 = c.r64(11) & 0xfffffu64;
            c.w64(11, r346);
            c.logic(r346, 64);
            if !c.movss_rm(0, c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000842303); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 233; continue;
        }
        232 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r347 = c.r64(11) & 0xfffffu64;
            c.w64(11, r347);
            c.logic(r347, 64);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) { return RegfileExit::fault(0x700084231b); }
            blk = 233; continue;
        }
        233 => {
            let Some(t348) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000842322); };
            c.w64(8, t348);
            let Some(t349) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000842329); };
            c.w64(9, t349);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x7000842330); }
            let Some(t350) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000842335); };
            c.w64(12, t350);
            let Some(t351) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700084233c); };
            c.cmp(t351, c.r64(11), 64);
            if c.cc_a() { blk = 236; continue; }
            blk = 234; continue;
        }
        234 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700084234c, 0x7000842347);
        }
        235 => {
            blk = 237; continue;
        }
        236 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700084234e); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 237; continue;
        }
        237 => {
            let Some(t352) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000842358); };
            c.w64(8, t352);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700084235f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x7000842363); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000842368); }
            blk = 28; continue;
        }
        238 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r353 = c.r64(11) & 0xfffffu64;
            c.w64(11, r353);
            c.logic(r353, 64);
            let Some(t354) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084237d); };
            c.w32(0, t354);
            let r355 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r355);
            blk = 28; continue;
        }
        239 => {
            let Some(t356) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700084238a); };
            c.w64(8, t356);
            let Some(t357) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000842391); };
            c.w32(8, t357);
            let r358 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r358);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000842398); }
            let Some(t359) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x700084239a); };
            c.w64(3, t359);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008423ad, 0x70008423a8);
        }
        240 => {
            return RegfileExit::fault(0x70008423ad);
        }
        241 => {
            let Some(t360) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008423ae); };
            c.w64(5, t360);
            let r361 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r361);
            let Some(t362) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008423b6); };
            if !c.push64(t362) { return RegfileExit::fault(0x70008423b6); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(0)) { return RegfileExit::fault(0x70008423ba); }
            c.w32(0, 0x4u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(3)) { return RegfileExit::fault(0x70008423c4); }
            let Some(t363) = c.ld64(c.r64(13).wrapping_add(0x2040)) else { return RegfileExit::fault(0x70008423c9); };
            c.w64(3, t363);
            if !c.st64(c.r64(4).wrapping_add(0x8), 0x0u64) { return RegfileExit::fault(0x70008423d0); }
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(1)) { return RegfileExit::fault(0x70008423d9); }
            return RegfileExit::jump(0x700086ed74);
        }
        242 => {
            return RegfileExit::fault(0x70008423e3);
        }
        243 => {
            c.w32(0, 0xeu64);
            return RegfileExit::jump(0x7000804df4);
        }
        244 => {
            return RegfileExit::jump(0x70008423f0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008423f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008423f7); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008423fb); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000842401); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842407); };
            c.w32(7, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700084240a); };
            c.cmp(t6, c.r32(7), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000842413); };
            c.w64(2, t7);
            blk = 2; continue;
        }
        2 => {
            c.w64(3, c.r64(2));
            return RegfileExit::jump(0x7000840794);
        }
        3 => {
            return RegfileExit::jump(0x7000842424);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000842434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000842437); };
            c.w64(7, t0);
            let Some(t1) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084243b); };
            c.w64(2, t1);
            c.w64(6, c.r64(2));
            c.w64(3, c.r64(7));
            return RegfileExit::jump(0x7000840794);
        }
        1 => {
            return RegfileExit::jump(0x700084244c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000842454(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000842457); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700084245b); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000842461); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842467); };
            c.w32(7, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700084246a); };
            c.cmp(t6, c.r32(7), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000842473); };
            c.w64(2, t7);
            blk = 2; continue;
        }
        2 => {
            c.w64(3, c.r64(2));
            return RegfileExit::jump(0x70008415d4);
        }
        3 => {
            return RegfileExit::jump(0x7000842484);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000842494(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000842497); };
            c.w64(7, t0);
            let Some(t1) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084249b); };
            c.w64(2, t1);
            c.w64(6, c.r64(2));
            c.w64(3, c.r64(7));
            return RegfileExit::jump(0x70008415d4);
        }
        1 => {
            return RegfileExit::jump(0x70008424ac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008424b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008424ba); };
            c.w64(7, t0);
            c.w32(8, c.r32(3));
            c.w64(9, c.r64(7));
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008424c4); };
            c.w32(11, t1);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008424d4); };
            c.w32(12, t2);
            let r3 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008424db); };
            c.w32(12, t4);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.logic(c.r64(12) & 0x4000000u64, 64);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008424f5); };
            c.w32(12, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008424f9); };
            c.cmp(t6, c.r32(12), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t7) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000842508); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            let r9 = c.add(c.r32(8), 0xffffffffffffffffu64, 32);
            c.w32(8, r9);
            if c.cc_ne() { blk = 1; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 11; continue;
        }
        8 => {
            return RegfileExit::fault(0x7000842517);
        }
        9 => {
            return RegfileExit::fault(0x7000842518);
        }
        10 => {
            return RegfileExit::fault(0x7000842519);
        }
        11 => {
            let Some(t10) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084251a); };
            c.w64(2, t10);
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(3, c.r64(2));
            c.w64(6, c.r64(7));
            return RegfileExit::jump(0x7000840794);
        }
        12 => {
            let Some(t11) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x7000842532); };
            if !c.push64(t11) { return RegfileExit::fault(0x7000842532); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x26c8)) else { return RegfileExit::fault(0x7000842535); };
            c.w64(3, t12);
            c.w64(6, c.r64(7));
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(0)) { return RegfileExit::fault(0x700084253f); }
            c.w32(0, 0x1u64);
            return RegfileExit::jump(0x700086ed74);
        }
        13 => {
            return RegfileExit::jump(0x7000842550);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000842554(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084255a); };
            c.w64(7, t0);
            c.w32(8, c.r32(3));
            c.w64(9, c.r64(7));
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000842564); };
            c.w32(11, t1);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000842574); };
            c.w32(12, t2);
            let r3 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084257b); };
            c.w32(12, t4);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.logic(c.r64(12) & 0x4000000u64, 64);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000842595); };
            c.w32(12, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000842599); };
            c.cmp(t6, c.r32(12), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t7) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008425a8); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            let r9 = c.add(c.r32(8), 0xffffffffffffffffu64, 32);
            c.w32(8, r9);
            if c.cc_ne() { blk = 1; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 11; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008425b7);
        }
        9 => {
            return RegfileExit::fault(0x70008425b8);
        }
        10 => {
            return RegfileExit::fault(0x70008425b9);
        }
        11 => {
            let Some(t10) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008425ba); };
            c.w64(2, t10);
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(3, c.r64(2));
            c.w64(6, c.r64(7));
            return RegfileExit::jump(0x70008415d4);
        }
        12 => {
            let Some(t11) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70008425d2); };
            if !c.push64(t11) { return RegfileExit::fault(0x70008425d2); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x26d0)) else { return RegfileExit::fault(0x70008425d5); };
            c.w64(3, t12);
            c.w64(6, c.r64(7));
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(0)) { return RegfileExit::fault(0x70008425df); }
            c.w32(0, 0x1u64);
            return RegfileExit::jump(0x700086ed74);
        }
        13 => {
            return RegfileExit::jump(0x70008425f0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008425f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008425f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008425f8); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008425fe); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842606); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084260e); };
            c.w64(7, t3);
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000842615); }
            let Some(t4) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000842619); };
            c.logic(t4 & 0x1000000u64, 32);
            if c.cc_ne() { blk = 87; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000842626); };
            c.cmp(t5, c.r32(2), 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(9, c.r64(1).wrapping_add(c.r64(1)));
            c.w32(11, c.r32(7));
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084263a); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let r8 = c.r32(11) | 0x2u64;
            c.w32(11, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ne() { blk = 77; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700084264f); };
            c.w32(9, t9);
            let r10 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r10);
            blk = 7; continue;
        }
        7 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000842657); };
            c.cmp(t11, c.r32(0), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000842666); };
            c.cmp(t12, c.r32(0), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000842675); }
            let r13 = c.r32(11) | c.r32(12);
            c.w32(11, r13);
            c.logic(r13, 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000842682); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000842689); };
            c.w32(15, t16);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t17) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842691); };
            c.w32(3, t17);
            c.w64(2, c.r64(15));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700084269c); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008426a0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008426a4); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 33; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70008426b2); };
            c.cmp(t18, c.r32(3), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008426bf); };
            c.cmp(t19, c.r32(3), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(2), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 93; continue; }
            blk = 12; continue;
        }
        12 => {
            let r20 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r20);
            c.w64(8, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008426df); };
            c.w64(15, t21);
            let Some(t22) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008426e6); };
            c.w64(15, t22);
            c.w64(1, c.r64(8).wrapping_add(0x8));
            c.w64(6, c.r64(1).wrapping_add(c.r64(15)));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008426f1); };
            c.w64(7, t23);
            let Some(t24) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008426f8); };
            c.cmp(t24, c.r64(6), 64);
            if c.cc_be() { blk = 91; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 91; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084270e); };
            c.w64(7, t25);
            if !c.st64(c.r64(7), c.r64(6)) { return RegfileExit::fault(0x7000842715); }
            c.w64(7, c.r64(15).wrapping_add(0x1));
            blk = 15; continue;
        }
        15 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700084271c); }
            c.w64(15, c.r64(7));
            let r26 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r26);
            c.logic(r26, 64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x7000842729); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x700084272c); }
            let Some(t27) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000842730); };
            c.logic(t27 & 0x4u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(15, c.r64(8).wrapping_add(0x7));
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_e() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(3, 0x7u64);
            blk = 18; continue;
        }
        18 => {
            let Some(t28) = c.ld32(c.r64(12).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000842746); };
            c.w32(2, t28);
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)), c.r32(2)) { return RegfileExit::fault(0x700084274a); }
            let r29 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r29);
            c.cmp(c.r64(15), c.r64(3), 64);
            if c.cc_ne() { blk = 18; continue; }
            blk = 19; continue;
        }
        19 => {
            blk = 22; continue;
        }
        20 => {
            c.w64(2, c.r64(12).wrapping_add(0x7));
            c.w64(1, c.r64(7).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r30 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r30);
            let r31 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r31);
            c.logic(r31, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700084276c); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000842771); };
            c.w64(0, t32);
            c.w64(10, 0x700084277fu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700084277f); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000842783); }
            return c.call(c.r64(0), 0x7000842789, 0x7000842787);
        }
        21 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000842789); }
            let Some(t33) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000842791); };
            c.w64(4, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000842796); };
            c.w64(0, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084279a); };
            c.w64(9, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084279e); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008427a2); };
            c.w64(7, t37);
            blk = 22; continue;
        }
        22 => {
            c.w64(12, c.r64(7));
            blk = 34; continue;
        }
        23 => {
            c.cmp(c.r64(2), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 99; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(8, c.r64(15).wrapping_mul(4));
            c.w64(15, c.r64(8).wrapping_add(0x8));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70008427c7); }
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 95; continue; }
            blk = 25; continue;
        }
        25 => {
            let r38 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r38);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008427da); };
            c.w64(1, t39);
            let Some(t40) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008427e1); };
            c.w64(6, t40);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70008427e4); }
            let Some(t41) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008427e8); };
            c.w64(6, t41);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008427ec); }
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_be() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let r42 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r42);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008427fc); }
            c.w32(15, 0x4u64);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x7000842804); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700084280d); };
            c.w64(3, t43);
            c.w64(15, c.r64(3));
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x700084281f, 0x700084281a);
        }
        27 => {
            blk = 30; continue;
        }
        28 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000842821); };
            c.w64(7, t44);
            c.w64(9, c.r64(15).wrapping_add(c.r64(7)));
            c.cmp(c.r64(6), c.r64(9), 64);
            if c.cc_be() { blk = 97; continue; }
            blk = 29; continue;
        }
        29 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x7000842832); }
            let r45 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r45);
            c.w64(0, c.r64(7));
            blk = 30; continue;
        }
        30 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084283c); };
            c.w64(7, t46);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000842840); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r47 = c.r64(9) & c.r64(0);
            c.w64(9, r47);
            c.logic(r47, 64);
            let Some(t48) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700084284d); };
            c.logic(t48 & 0x4u64, 8);
            if c.cc_ne() { blk = 115; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000842858); };
            c.w64(7, t49);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700084285c); }
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084285f); };
            c.w64(7, t50);
            c.w64(2, c.r64(7).wrapping_add(0x7));
            c.w64(1, c.r64(0).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r51 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r51);
            let r52 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r52);
            c.logic(r52, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000842876); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700084287b); }
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084287f); };
            c.w64(8, t53);
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000842883); };
            c.w64(0, t54);
            c.w64(10, 0x7000842891u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000842891); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000842895); }
            return c.call(c.r64(0), 0x700084289b, 0x7000842899);
        }
        32 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700084289b); }
            let Some(t55) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008428a3); };
            c.w64(4, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008428a8); };
            c.w64(12, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008428ac); };
            c.w64(0, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008428b0); };
            c.w64(9, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008428b4); };
            c.w64(11, t59);
            blk = 34; continue;
        }
        33 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008428ba); };
            c.w64(12, t60);
            blk = 34; continue;
        }
        34 => {
            c.w64(7, c.r64(0));
            let Some(t61) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008428c4); };
            c.w32(15, t61);
            let r62 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r62);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t63) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008428d5); };
            c.cmp(t63, c.r32(15), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t64) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008428e2); };
            c.w32(0, t64);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r65 = c.r64(0) & 0x3ffu64;
            c.w64(0, r65);
            c.logic(r65, 64);
            if c.cc_e() { blk = 54; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008428f8); };
            c.w64(3, t66);
            let Some(t67) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008428ff); };
            c.w64(2, t67);
            c.w64(1, c.r64(0).wrapping_mul(4).wrapping_add(0x8));
            c.w64(6, c.r64(1).wrapping_add(c.r64(2)));
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084290e); };
            c.w64(8, t68);
            let Some(t69) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000842915); };
            c.cmp(t69, c.r64(6), 64);
            if c.cc_be() { blk = 101; continue; }
            blk = 38; continue;
        }
        38 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x700084291e); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 39; continue;
        }
        39 => {
            c.w32(2, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x250)) else { return RegfileExit::fault(0x7000842928); };
            c.w64(1, t70);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700084292f); }
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x7000842932); }
            c.w64(0, c.r64(0).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(0), 0x7u64, 64);
            if c.cc_e() { blk = 43; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(2, c.r64(0));
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000842946); };
            c.w64(1, t71);
            blk = 41; continue;
        }
        41 => {
            let r72 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r72);
            if !c.st32(c.r64(2).wrapping_add(c.r64(3)), c.r32(1)) { return RegfileExit::fault(0x7000842951); }
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_ne() { blk = 41; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 44; continue;
        }
        43 => {
            c.w32(2, 0x1u64);
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 45; continue;
        }
        45 => {
            let r73 = c.sub(c.r64(0), 0x4u64, 64);
            c.w64(0, r73);
            let Some(t74) = c.ld32(c.r64(0).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x7000842970); };
            c.w32(2, t74);
            let r75 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r75);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t76) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084297c); };
            c.w32(1, t76);
            let Some(t77) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084297f); };
            c.cmp(t77, c.r32(1), 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 47; continue;
        }
        47 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000842988); }
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084298d); };
            c.w64(2, t78);
            let Some(t79) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000842994); };
            c.w64(1, t79);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084299b); };
            c.w64(8, t80);
            let Some(t81) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008429a2); };
            c.cmp(t81, c.r64(6), 64);
            if c.cc_be() { blk = 103; continue; }
            blk = 48; continue;
        }
        48 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x70008429ab); }
            c.w64(2, c.r64(1).wrapping_add(0x1));
            blk = 49; continue;
        }
        49 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008429b2); };
            c.w64(1, t82);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008429b9); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008429bc); }
            blk = 50; continue;
        }
        50 => {
            if !c.st32(c.r64(3).wrapping_add(c.r64(0)), c.r32(2)) { return RegfileExit::fault(0x70008429c1); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r83 = c.r64(1) & c.r64(3);
            c.w64(1, r83);
            c.logic(r83, 64);
            let Some(t84) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008429d3); };
            c.logic(t84 & 0x4u64, 8);
            if c.cc_ne() { blk = 112; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r64(0), 0x7u64, 64);
            if c.cc_ne() { blk = 45; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(0, c.r64(3));
            blk = 58; continue;
        }
        54 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008429e8); };
            c.w64(0, t85);
            blk = 58; continue;
        }
        55 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008429f1); };
            c.w64(0, t86);
            blk = 58; continue;
        }
        56 => {
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008429fa); };
            c.w64(0, t87);
            blk = 58; continue;
        }
        57 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000842a03); }
            c.w64(7, c.r64(0));
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000842a0a); };
            c.w64(0, t88);
            c.w64(12, c.r64(0));
            blk = 58; continue;
        }
        58 => {
            let Some(t89) = c.ld8(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000842a14); };
            c.w32(15, t89);
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000842a19); };
            c.w64(3, t90);
            let Some(t91) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000842a20); };
            c.w64(2, t91);
            if let Some(sr) = c.shl(c.r64(15), 0x2u64, 64) { c.w64(15, sr); }
            c.w64(1, c.r64(15).wrapping_add(c.r64(2)));
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000842a2b); };
            c.w64(6, t92);
            let Some(t93) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000842a32); };
            c.cmp(t93, c.r64(1), 64);
            if c.cc_be() { blk = 105; continue; }
            blk = 59; continue;
        }
        59 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x7000842a3b); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 60; continue;
        }
        60 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000842a42); }
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x7000842a46); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x7000842a49); }
            c.w64(12, c.r64(15).wrapping_sub(0x1));
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_e() { blk = 63; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000842a57); };
            c.w64(15, t94);
            blk = 62; continue;
        }
        62 => {
            let r95 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r95);
            if !c.st32(c.r64(12).wrapping_add(c.r64(3)), c.r32(15)) { return RegfileExit::fault(0x7000842a62); }
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_ne() { blk = 62; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 76; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000842a76); };
            c.w64(11, t96);
            let Some(t97) = c.ld8(c.r64(11).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000842a7a); };
            c.w32(12, t97);
            let Some(t98) = c.ld8(c.r64(9).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000842a7f); };
            c.w32(9, t98);
            c.w64(15, c.r64(9));
            let r99 = c.sub(c.r64(15), c.r64(12), 64);
            c.w64(15, r99);
            if let Some(sr) = c.shl(c.r64(15), 0x2u64, 64) { c.w64(15, sr); }
            let Some(t100) = c.ld8(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000842a8e); };
            c.w32(11, t100);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_e() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(0, c.r64(12).wrapping_mul(4));
            c.w64(2, c.r64(15).wrapping_add(c.r64(12).wrapping_mul(4)));
            let Some(t101) = c.ld32(c.r64(7).wrapping_add(c.r64(0)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842aa4); };
            c.w32(0, t101);
            if !c.st32(c.r64(3).wrapping_add(c.r64(2)).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000842aa8); }
            let r102 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r102);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_ne() { blk = 65; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(7, c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)));
            if let Some(sr) = c.shl(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), c.r64(7), 64);
            if c.cc_e() { blk = 74; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t103) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842aca); };
            c.w32(12, t103);
            let r104 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r104);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t105) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842ad8); };
            c.w32(15, t105);
            let Some(t106) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000842add); };
            c.cmp(t106, c.r32(15), 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000842ae6); }
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000842aed); };
            c.w64(12, t107);
            let Some(t108) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000842af4); };
            c.w64(15, t108);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000842afc); };
            c.w64(2, t109);
            let Some(t110) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000842b03); };
            c.cmp(t110, c.r64(0), 64);
            if c.cc_be() { blk = 107; continue; }
            blk = 70; continue;
        }
        70 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x7000842b0c); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 71; continue;
        }
        71 => {
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000842b14); };
            c.w64(15, t111);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000842b1b); }
            if !c.movsd_mr(c.r64(12).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000842b20); }
            if !c.st32(c.r64(3).wrapping_add(c.r64(11)), c.r32(12)) { return RegfileExit::fault(0x7000842b27); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r112 = c.r64(15) & c.r64(3);
            c.w64(15, r112);
            c.logic(r112, 64);
            let Some(t113) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000842b3b); };
            c.logic(t113 & 0x4u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 73; continue;
        }
        73 => {
            let r114 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r114);
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ne() { blk = 67; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w64(0, c.r64(3));
            blk = 75; continue;
        }
        75 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000842b59); }
            return c.ret(0x0, 0x7000842b5a);
        }
        76 => {
            c.w64(0, c.r64(3));
            blk = 75; continue;
        }
        77 => {
            c.w32(9, c.r32(12));
            let r115 = c.r32(9) & 0x3u64;
            c.w32(9, r115);
            c.logic(r115, 32);
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_ne() { blk = 87; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t116) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842b6d); };
            c.w32(9, t116);
            let Some(t117) = c.ld32(c.r64(13).wrapping_add(0x2c8)) else { return RegfileExit::fault(0x7000842b72); };
            c.cmp(t117, c.r32(9), 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t118) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000842b7b); };
            c.cmp(t118, c.r32(12), 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 80; continue;
        }
        80 => {
            blk = 87; continue;
        }
        81 => {
            let Some(t119) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000842b86); };
            c.w32(9, t119);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r120 = c.sub(c.r64(9), 0x2u64, 64);
            c.w64(9, r120);
            blk = 82; continue;
        }
        82 => {
            c.w64(11, c.r64(9).wrapping_mul(4));
            c.w32(15, c.r32(7));
            let Some(t121) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000842ba0); };
            c.w32(8, t121);
            let r122 = c.r32(15) | 0x2u64;
            c.w32(15, r122);
            c.logic(r122, 32);
            c.cmp(c.r32(15), c.r32(8), 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 83; continue;
        }
        83 => {
            let r123 = c.sub(c.r64(9), 0x2u64, 64);
            c.w64(9, r123);
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_ge() { blk = 82; continue; }
            blk = 84; continue;
        }
        84 => {
            blk = 87; continue;
        }
        85 => {
            let Some(t124) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000842bba); };
            c.w32(9, t124);
            let r125 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r125);
            blk = 7; continue;
        }
        86 => {
            let Some(t126) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000842bc7); };
            c.w64(5, t126);
            let r127 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r127);
            return RegfileExit::jump(0x7000842e54);
        }
        87 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000842bd4); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000842bd5); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000842bd6); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000842bd7); }
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x20b0)) else { return RegfileExit::fault(0x7000842bd8); };
            c.w64(3, t128);
            c.w32(0, 0x4u64);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x7000842bec, 0x7000842be7);
        }
        88 => {
            let Some(t129) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842bec); };
            c.w32(7, t129);
            let Some(t130) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000842bef); };
            c.cmp(t130, c.r32(7), 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            blk = 75; continue;
        }
        90 => {
            c.w64(9, c.r64(0));
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000842c00); };
            c.w64(0, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000842c04); };
            c.w64(7, t132);
            blk = 7; continue;
        }
        91 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000842c0d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000842c11); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x7000842c1d, 0x7000842c18);
        }
        92 => {
            c.w64(7, c.r64(0));
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000842c20); };
            c.w64(0, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000842c24); };
            c.w64(9, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000842c28); };
            c.w64(11, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000842c2c); };
            c.w64(12, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000842c30); };
            c.w64(3, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000842c34); };
            c.w64(8, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000842c38); };
            c.w64(2, t139);
            blk = 15; continue;
        }
        93 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000842c45); };
            c.w64(3, t140);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x7000842c54, 0x7000842c4f);
        }
        94 => {
            return RegfileExit::fault(0x7000842c54);
        }
        95 => {
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x7000842c55); };
            c.w64(3, t141);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000842c65, 0x7000842c60);
        }
        96 => {
            return RegfileExit::fault(0x7000842c65);
        }
        97 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000842c6a); }
            c.w32(7, 0x4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000842c70); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000842c78); };
            c.w64(3, t142);
            c.w64(7, c.r64(3));
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x7000842c8a, 0x7000842c85);
        }
        98 => {
            blk = 30; continue;
        }
        99 => {
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000842c8f); };
            c.w64(3, t143);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000842c9f, 0x7000842c9a);
        }
        100 => {
            return RegfileExit::fault(0x7000842c9f);
        }
        101 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x7000842ca0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x7000842ca4); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000842ca8); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x7000842cb4, 0x7000842caf);
        }
        102 => {
            c.w64(3, c.r64(0));
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000842cb7); };
            c.w64(7, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000842cbb); };
            c.w64(9, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000842cbf); };
            c.w64(11, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000842cc3); };
            c.w64(12, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000842cc7); };
            c.w64(15, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000842ccb); };
            c.w64(0, t149);
            blk = 39; continue;
        }
        103 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000842cd9); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x7000842cdd); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000842ce1); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x7000842ce5); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x7000842cea); }
            return c.call(0x70007febf4, 0x7000842cf3, 0x7000842cee);
        }
        104 => {
            c.w64(2, c.r64(0));
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000842cf6); };
            c.w64(7, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000842cfa); };
            c.w64(9, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000842cfe); };
            c.w64(11, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000842d02); };
            c.w64(3, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000842d06); };
            c.w64(12, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000842d0a); };
            c.w64(0, t155);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x7000842d0e); }
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000842d13); };
            c.w64(15, t156);
            blk = 49; continue;
        }
        105 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x7000842d1c); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x7000842d20); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x7000842d24); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x7000842d30, 0x7000842d2b);
        }
        106 => {
            c.w64(3, c.r64(0));
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000842d33); };
            c.w64(7, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000842d37); };
            c.w64(9, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000842d3b); };
            c.w64(11, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000842d3f); };
            c.w64(15, t160);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000842d43); };
            c.w64(12, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000842d47); };
            c.w64(0, t162);
            blk = 60; continue;
        }
        107 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x7000842d55); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000842d59); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000842d5d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000842d61); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x7000842d65); }
            return c.call(0x70007febf4, 0x7000842d6f, 0x7000842d6a);
        }
        108 => {
            c.w64(12, c.r64(0));
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000842d72); };
            c.w64(3, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000842d76); };
            c.w64(7, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000842d7a); };
            c.w64(9, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000842d7e); };
            c.w64(11, t166);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x7000842d82); }
            blk = 71; continue;
        }
        109 => {
            c.w32(12, c.r32(12));
            let r167 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r167);
            c.w64(15, 0xfffffffffffc0000u64);
            let r168 = c.r64(15) & c.r64(12);
            c.w64(15, r168);
            c.logic(r168, 64);
            let Some(t169) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000842d9c); };
            c.logic(t169 & 0x2u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 110; continue;
        }
        110 => {
            c.w64(0, c.r64(3).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000842dab); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000842dac); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x7000842db9, 0x7000842db4);
        }
        111 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000842db9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000842dbb); }
            blk = 73; continue;
        }
        112 => {
            c.w32(2, c.r32(2));
            let r170 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r170);
            c.w64(1, 0xfffffffffffc0000u64);
            let r171 = c.r64(1) & c.r64(2);
            c.w64(1, r171);
            c.logic(r171, 64);
            let Some(t172) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000842dd0); };
            c.logic(t172 & 0x2u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000842dde); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000842ddf); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x7000842dec, 0x7000842de7);
        }
        114 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000842dec); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000842dee); }
            blk = 52; continue;
        }
        115 => {
            c.w32(7, c.r32(7));
            let r173 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r173);
            c.w64(9, 0xfffffffffffc0000u64);
            let r174 = c.r64(9) & c.r64(7);
            c.w64(9, r174);
            c.logic(r174, 64);
            let Some(t175) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000842e03); };
            c.logic(t175 & 0x2u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 116; continue;
        }
        116 => {
            c.w64(11, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000842e12); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000842e13); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000842e20, 0x7000842e1b);
        }
        117 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000842e20); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000842e22); }
            blk = 31; continue;
        }
        118 => {
            return RegfileExit::jump(0x7000842e2c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000842e34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000842e37); };
            c.w64(8, t0);
            let Some(t1) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000842e3b); };
            c.w64(7, t1);
            c.w64(6, c.r64(7));
            c.w64(2, c.r64(8));
            return RegfileExit::jump(0x70008425f4);
        }
        1 => {
            return RegfileExit::jump(0x7000842e4c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000842e54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000842e54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000842e58); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842e5e); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000842e65); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000842e6c); };
            c.w32(9, t5);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x7000842e76); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x7000842e7a); }
            c.cmp(c.r64(9), 0x8cu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x7000842e87);
        }
        2 => {
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x237)) else { return RegfileExit::fault(0x7000842e88); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000842e92); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000842e99); };
            c.w64(9, t10);
            let Some(t11) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000842ea0); };
            c.w64(11, t11);
            let Some(t12) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000842ea3); };
            c.w32(12, t12);
            c.w64(2, c.r64(12).wrapping_mul(4));
            c.w64(12, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000842eb4); };
            c.w64(15, t13);
            let Some(t14) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000842ebb); };
            c.cmp(t14, c.r64(12), 64);
            if c.cc_be() { blk = 120; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000842ec4); }
            let r15 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r15);
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000842ecf); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000842ed3); };
            c.w64(8, t16);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000842eda); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000842ede); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(11)) { return RegfileExit::fault(0x7000842ee2); }
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000842eec); };
            c.w64(2, t17);
            blk = 6; continue;
        }
        6 => {
            let r18 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r18);
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(2)) { return RegfileExit::fault(0x7000842ef7); }
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_ne() { blk = 6; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 9; continue;
        }
        8 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000842f03); };
            c.w64(2, t19);
            blk = 9; continue;
        }
        9 => {
            c.logic(c.r8(3) & 0x20u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000842f0f); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000842f11); };
            c.w64(12, t20);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000842f18); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x23c0)) else { return RegfileExit::fault(0x7000842f1a); };
            c.w64(3, t21);
            c.w32(0, 0x2u64);
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x7000842f2e, 0x7000842f29);
        }
        11 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000842f2e); };
            c.w64(11, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000842f32); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000842f36); };
            c.w64(2, t24);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000842f3d); };
            c.w64(9, t25);
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000842f44); };
            c.w64(15, t26);
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000842f4b); };
            c.w64(8, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000842f52); };
            c.w64(0, t28);
            blk = 12; continue;
        }
        12 => {
            let Some(t29) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000842f56); };
            c.cmp(t29, c.r32(0), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000842f65); };
            c.cmp(t30, c.r32(0), 32);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            let r31 = c.r32(12) | c.r32(3);
            c.w32(12, r31);
            c.logic(r31, 32);
            if c.cc_ne() { blk = 119; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 122; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842f83); };
            c.w32(12, t32);
            let r33 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r33);
            let Some(t34) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000842f8a); };
            c.cmp(t34, 0xbau64, 16);
            if c.cc_b() { blk = 122; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t35) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000842f98); };
            c.w32(12, t35);
            let r36 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r36);
            let Some(t37) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000842f9f); };
            c.w32(3, t37);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x7000842fa5); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000842fa9); }
            c.cmp(c.r16(3), 0xbdu64, 16);
            if c.cc_b() { blk = 129; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t38) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000842fb8); };
            c.w32(1, t38);
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000842fbb); };
            c.cmp(t39, c.r32(1), 32);
            if c.cc_ne() { blk = 129; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(3, c.r16(3));
            c.cmp(c.r32(3), 0x411u64, 32);
            if c.cc_le() { blk = 129; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t40) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000842fd7); };
            c.w32(3, t40);
            c.logic(c.r32(3) & 0x200000u64, 32);
            if c.cc_ne() { blk = 129; continue; }
            blk = 19; continue;
        }
        19 => {
            let r41 = c.shr(c.r32(3), 0xau64, 32).unwrap_or(c.r32(3));
            c.w32(3, r41);
            let Some(t42) = c.ld32(c.r64(12).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000842feb); };
            c.w32(1, t42);
            let r43 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r43);
            let r44 = c.r32(3) & 0x3ffu64;
            c.w32(3, r44);
            c.logic(r44, 32);
            c.w32(3, c.r64(3).wrapping_add(c.r64(3).wrapping_mul(2)));
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x7000842fff); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_e() { blk = 117; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(7, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), 0x1u64) { return RegfileExit::fault(0x700084300f); }
            c.w64(9, c.r64(7));
            blk = 22; continue;
        }
        21 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084301c); };
            c.w64(1, t45);
            blk = 22; continue;
        }
        22 => {
            let Some(t46) = c.ld32(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000843020); };
            c.w32(3, t46);
            let r47 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r47);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000843028); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700084302c); }
            let Some(t48) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000843030); };
            c.cmp(t48, 0x0u64, 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084303a); };
            c.w32(9, t49);
            let r50 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r50);
            let Some(t51) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000843041); };
            c.w32(15, t51);
            c.logic(c.r32(15) & 0x200000u64, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000843052); }
            let Some(t52) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000843056); };
            c.w32(1, t52);
            let r53 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r53);
            c.logic(c.r32(15) & 0xffc00u64, 32);
            if c.cc_e() { blk = 115; continue; }
            blk = 25; continue;
        }
        25 => {
            let r54 = c.shr(c.r32(15), 0xau64, 32).unwrap_or(c.r32(15));
            c.w32(15, r54);
            let r55 = c.r32(15) & 0x3ffu64;
            c.w32(15, r55);
            c.logic(r55, 32);
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_be() { blk = 41; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t56) = c.ld16(c.r64(1).wrapping_add(0x5)) else { return RegfileExit::fault(0x700084307f); };
            c.w32(6, (t56 as u16 as i16 as i64 as u64));
            let Some(t57) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000843083); };
            c.w32(2, t57);
            let r58 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r58);
            let r59 = c.sub(c.r32(6), 0x1u64, 32);
            c.w32(6, r59);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x700084308c); }
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        27 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x7000843095); }
            blk = 28; continue;
        }
        28 => {
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000843099); };
            c.w64(11, t60);
            let r61 = c.sub(c.r32(11), c.r32(12), 32);
            c.w32(11, r61);
            let r62 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r62);
            let r63 = c.add(c.r32(11), c.r32(12), 32);
            c.w32(11, r63);
            c.w32(8, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t64) = c.ld32(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008430ad); };
            c.w32(8, t64);
            let r65 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r65);
            let r66 = c.shr(c.r32(8), 0x9u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r66);
            let r67 = c.r32(8) & 0x3ffu64;
            c.w32(8, r67);
            c.logic(r67, 32);
            c.w32(8, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t68) = c.ld32(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008430c7); };
            c.w32(8, t68);
            let r69 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r69);
            let Some(t70) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008430cf); };
            c.w32(8, t70);
            let r71 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r71);
            c.cmp(c.r32(2), c.r32(8), 32);
            if c.cc_be() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(12, c.r64(11).wrapping_add(0x1));
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008430e0); };
            c.w64(11, t72);
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 33; continue;
        }
        32 => {
            let r73 = c.add(c.r32(12), 0x1u64, 32);
            c.w32(12, r73);
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(6), c.r32(12), 32);
            if c.cc_l() { blk = 40; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(8, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t74) = c.ld32(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008430fb); };
            c.w32(8, t74);
            let r75 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r75);
            let r76 = c.shr(c.r32(8), 0x9u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r76);
            let r77 = c.r32(8) & 0x3ffu64;
            c.w32(8, r77);
            c.logic(r77, 32);
            c.w32(11, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t78) = c.ld32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000843115); };
            c.w32(7, t78);
            let r79 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r79);
            let Some(t80) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084311d); };
            c.w32(0, t80);
            let r81 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r81);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(7), c.r32(3), 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(15), c.r32(8), 32);
            if c.cc_be() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(12, c.r64(9));
            c.w64(9, c.r64(11));
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000843136); };
            c.w64(0, t82);
            blk = 58; continue;
        }
        38 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700084313f); };
            c.w64(0, t83);
            blk = 115; continue;
        }
        39 => {
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000843148); };
            c.w64(0, t84);
            blk = 115; continue;
        }
        40 => {
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000843151); };
            c.w64(0, t85);
            blk = 115; continue;
        }
        41 => {
            c.w64(8, (c.r32(15) as u32 as i32 as i64 as u64));
            let r86 = c.imul2(c.r64(8), 0x3u64, 64);
            c.w64(11, r86);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 115; continue; }
            blk = 42; continue;
        }
        42 => {
            let r87 = c.sub(c.r64(11), 0x3u64, 64);
            c.w64(11, r87);
            let Some(t88) = c.ld32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084316f); };
            c.cmp(t88, c.r32(3), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_ne() { blk = 42; continue; }
            blk = 44; continue;
        }
        44 => {
            blk = 115; continue;
        }
        45 => {
            c.w64(12, c.r64(9));
            c.w64(9, c.r64(11));
            blk = 58; continue;
        }
        46 => {
            let Some(t89) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084318c); };
            c.w32(8, t89);
            let r90 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r90);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000843199); };
            c.w64(8, t91);
            blk = 48; continue;
        }
        48 => {
            let Some(t92) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008431a0); };
            c.w32(9, t92);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r93 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r93);
            let Some(t94) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008431ae); };
            c.w32(11, t94);
            let r95 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r95);
            let r96 = c.r64(11) & c.r64(9);
            c.w64(11, r96);
            c.logic(r96, 64);
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 50; continue;
        }
        49 => {
            let r97 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r97);
            let r98 = c.add(c.r64(11), c.r64(7), 64);
            c.w64(11, r98);
            c.w64(12, c.r64(9));
            let r99 = c.r64(12) & c.r64(11);
            c.w64(12, r99);
            c.logic(r99, 64);
            c.w64(11, c.r64(12));
            blk = 50; continue;
        }
        50 => {
            c.w64(12, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(15, c.r64(12).wrapping_mul(4));
            let Some(t100) = c.ld32(c.r64(8).wrapping_add(c.r64(15)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008431da); };
            c.w32(2, t100);
            let Some(t101) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008431df); };
            c.cmp(t101, c.r32(2), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(2), c.r32(3), 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t102) = c.ld32(c.r64(8).wrapping_add(c.r64(15)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008431ec); };
            c.w32(9, t102);
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t103) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008431f7); };
            c.w32(11, t103);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r104 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r104);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000843205); }
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_ae() { blk = 128; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t105) = c.ld32(c.r64(8).wrapping_add(c.r64(15)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000843212); };
            c.w32(8, t105);
            let r106 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r106);
            let r107 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r107);
            c.w64(15, c.r64(9));
            blk = 73; continue;
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000843225); }
            blk = 115; continue;
        }
        56 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700084322e); }
            blk = 115; continue;
        }
        57 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000843237); }
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084323b); };
            c.w64(12, t108);
            blk = 58; continue;
        }
        58 => {
            c.w64(8, c.r64(9).wrapping_mul(4));
            let Some(t109) = c.ld32(c.r64(1).wrapping_add(c.r64(8)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000843247); };
            c.w32(9, t109);
            c.w32(11, c.r32(9));
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 115; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(15, c.r64(9));
            let r110 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r110);
            c.logic(c.r8(9) & 0x4u64, 8);
            if c.cc_e() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t111) = c.ld32(c.r64(1).wrapping_add(c.r64(8)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000843265); };
            c.w32(8, t111);
            let r112 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r112);
            blk = 72; continue;
        }
        61 => {
            c.w64(8, c.r64(15));
            if let Some(sr) = c.shr(c.r64(8), 0x13u64, 64) { c.w64(8, sr); }
            let Some(t113) = c.ld8(c.r64(12).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000843279); };
            c.w32(9, t113);
            let r114 = c.r64(8) & 0x3ffu64;
            c.w64(8, r114);
            c.logic(r114, 64);
            let r115 = c.r32(11) & 0x380u64;
            c.w32(11, r115);
            c.logic(r115, 32);
            let r116 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r116);
            c.cmp(c.r32(11), 0x100u64, 32);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            let Some(t117) = c.ld8(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084329f); };
            c.w32(11, t117);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_b() { blk = 66; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t118) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008432aa); };
            c.w32(12, t118);
            let r119 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r119);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008432b7); };
            c.w64(12, t120);
            blk = 64; continue;
        }
        64 => {
            let r121 = c.sub(c.r64(8), c.r64(11), 64);
            c.w64(8, r121);
            let Some(t122) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008432c1); };
            c.w32(8, t122);
            let r123 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r123);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008432cf); }
            blk = 68; continue;
        }
        66 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t124) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008432dd); };
            c.w32(8, t124);
            let r125 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r125);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008432e5); }
            blk = 68; continue;
        }
        68 => {
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008432eb); };
            c.w64(9, t126);
            let Some(t127) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008432f2); };
            c.w64(8, t127);
            c.w64(11, c.r64(8).wrapping_add(0xc));
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008432f9); };
            c.w64(12, t128);
            let Some(t129) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000843300); };
            c.cmp(t129, c.r64(11), 64);
            if c.cc_be() { blk = 124; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.st64(c.r64(9), c.r64(11)) { return RegfileExit::fault(0x700084330a); }
            let r130 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r130);
            blk = 70; continue;
        }
        70 => {
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000843311); };
            c.w64(11, t131);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000843318); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700084331c); }
            blk = 72; continue;
        }
        71 => {
            let Some(t132) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000843324); };
            c.w32(8, t132);
            let r133 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r133);
            blk = 72; continue;
        }
        72 => {
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084332c); };
            c.w64(1, t134);
            blk = 73; continue;
        }
        73 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 111; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t135) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084333a); };
            c.w32(9, t135);
            let Some(t136) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700084333e); };
            c.cmp(t136, c.r32(9), 32);
            if c.cc_ne() { blk = 93; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t137) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084334b); };
            c.w32(2, t137);
            let r138 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r138);
            let Some(t139) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000843352); };
            c.w32(8, t139);
            let r140 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r140);
            let Some(t141) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000843359); };
            c.logic(t141 & 0x2u64, 8);
            if c.cc_ne() { blk = 91; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t142) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000843364); };
            c.cmp(t142, c.r32(8), 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700084336d); };
            c.w64(8, t143);
            blk = 111; continue;
        }
        78 => {
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000843379); };
            c.w64(8, t144);
            let Some(t145) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x7000843380); };
            c.cmp(t145, 0x20u64, 8);
            if c.cc_e() { blk = 126; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t146) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x700084338a); };
            c.w32(8, t146);
            let Some(t147) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700084338e); };
            c.cmp(t147, c.r32(8), 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t148) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084339b); };
            c.w32(8, t148);
            let r149 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r149);
            let Some(t150) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008433a2); };
            c.w32(9, t150);
            let Some(t151) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008433a6); };
            c.cmp(t151, c.r32(9), 32);
            if c.cc_ne() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t152) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008433af); };
            c.w32(8, t152);
            let r153 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r153);
            let Some(t154) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008433b6); };
            c.w32(9, t154);
            let Some(t155) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008433ba); };
            c.cmp(t155, c.r32(9), 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 82; continue;
        }
        82 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 108; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t156) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008433cd); };
            c.w32(9, t156);
            let r157 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r157);
            let Some(t158) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008433d4); };
            c.w32(11, t158);
            c.w32(12, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 88; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w32(8, c.r16(11));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t159) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008433f3); };
            c.cmp(t159, c.r32(9), 32);
            if c.cc_e() { blk = 108; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w64(8, c.r64(0));
            blk = 88; continue;
        }
        87 => {
            let Some(t160) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000843405); };
            c.w32(8, t160);
            let r161 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r161);
            blk = 88; continue;
        }
        88 => {
            let Some(t162) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084340c); };
            c.w32(8, t162);
            let r163 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r163);
            let Some(t164) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000843413); };
            c.w32(8, t164);
            let r165 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r165);
            let Some(t166) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084341a); };
            c.w32(9, t166);
            let r167 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r167);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 108; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t168) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000843430); };
            c.w32(6, t168);
            let r169 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r169);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000843437); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x700084343f, 0x700084343a);
        }
        90 => {
            c.w64(8, c.r64(0));
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000843442); };
            c.w64(0, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000843446); };
            c.w64(1, t171);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700084344a); };
            c.w64(3, t172);
            blk = 111; continue;
        }
        91 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000843453); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000843459); };
            c.w64(6, t173);
            return c.call(0x70007f2274, 0x7000843462, 0x700084345d);
        }
        92 => {
            c.w64(8, c.r64(0));
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000843465); };
            c.w64(0, t174);
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000843469); };
            c.w64(1, t175);
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700084346d); };
            c.w64(3, t176);
            blk = 111; continue;
        }
        93 => {
            let Some(t177) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000843476); };
            c.w32(9, t177);
            let r178 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r178);
            let Some(t179) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084347d); };
            c.w32(11, t179);
            c.cmp(c.r16(11), 0x834u64, 16);
            if c.cc_e() { blk = 107; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w32(12, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 100; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r16(11), 0x411u64, 16);
            if c.cc_ne() { blk = 108; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t180) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008434a7); };
            c.w32(8, t180);
            let Some(t181) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008434ab); };
            c.cmp(t181, c.r32(8), 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t182) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008434b8); };
            c.w32(8, t182);
            let r183 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r183);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 108; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t184) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008434c9); };
            c.w32(9, t184);
            let r185 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r185);
            let Some(t186) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008434d0); };
            c.cmp(t186, 0x40u64, 16);
            if c.cc_ae() { blk = 108; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t187) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008434dc); };
            c.w32(8, t187);
            let r188 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r188);
            blk = 111; continue;
        }
        100 => {
            let Some(t189) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008434e8); };
            c.w32(8, t189);
            let Some(t190) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008434ec); };
            c.cmp(t190, c.r32(8), 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t191) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008434f9); };
            c.w32(8, t191);
            c.w64(9, c.r64(8));
            let r192 = c.r32(9) & 0xc0u64;
            c.w32(9, r192);
            c.logic(r192, 32);
            c.cmp(c.r32(9), 0xc0u64, 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t193) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000843511); };
            c.w32(9, t193);
            let r194 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r194);
            let Some(t195) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000843518); };
            c.w32(9, t195);
            let r196 = c.r32(9) & 0x1fu64;
            c.w32(9, r196);
            c.logic(r196, 32);
            let r197 = c.sub(c.r32(9), 0x10u64, 32);
            c.w32(9, r197);
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_a() { blk = 108; continue; }
            blk = 103; continue;
        }
        103 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 108; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t198) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000843530); };
            c.w32(8, t198);
            let r199 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r199);
            let Some(t200) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000843537); };
            c.cmp(t200, c.r32(8), 32);
            if c.cc_e() { blk = 108; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t201) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000843540); };
            c.w32(9, t201);
            let Some(t202) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000843544); };
            c.cmp(t202, c.r32(9), 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t203) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084354d); };
            c.w32(8, t203);
            let r204 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r204);
            blk = 111; continue;
        }
        107 => {
            let Some(t205) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000843556); };
            c.w32(8, t205);
            let Some(t206) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700084355a); };
            c.cmp(t206, c.r32(8), 32);
            if c.cc_e() { blk = 110; continue; }
            blk = 108; continue;
        }
        108 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000843563); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000843564); }
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000843565); };
            c.w64(6, t207);
            c.w32(0, 0x2u64);
            let Some(t208) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700084356e); };
            c.w64(3, t208);
            c.w64(8, c.r64(3));
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x7000843580, 0x700084357b);
        }
        109 => {
            c.w64(8, c.r64(0));
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000843583); };
            c.w64(3, t209);
            blk = 111; continue;
        }
        110 => {
            let Some(t210) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000843589); };
            c.w32(8, t210);
            let r211 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r211);
            blk = 111; continue;
        }
        111 => {
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000843590); };
            c.w64(0, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000843594); };
            c.w64(6, t213);
            c.w64(1, c.r64(8));
            return c.call(0x7000875c54, 0x70008435a0, 0x700084359b);
        }
        112 => {
            let Some(t214) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008435a0); };
            c.cmp(t214, 0x0u64, 32);
            if c.cc_ne() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008435a6); };
            c.w64(0, t215);
            blk = 115; continue;
        }
        114 => {
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008435ac); };
            c.w64(0, t216);
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008435b0); };
            c.w64(8, t217);
            let Some(t218) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008435b4); };
            c.cmp(t218, c.r32(8), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            let Some(t219) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008435c0); };
            c.w32(1, t219);
            let r220 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r220);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008435c7); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008435cb); }
            blk = 115; continue;
        }
        115 => {
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008435cf); };
            c.w64(9, t221);
            let r222 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008435d7); };
            c.w64(8, t223);
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ne() { blk = 21; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008435e4); };
            c.w64(11, t224);
            blk = 117; continue;
        }
        117 => {
            c.w64(0, c.r64(11));
            blk = 118; continue;
        }
        118 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008435ee); }
            return c.ret(0x0, 0x70008435ef);
        }
        119 => {
            c.w64(0, c.r64(11));
            blk = 118; continue;
        }
        120 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008435f5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008435f9); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008435fd); }
            return c.call(0x70007febf4, 0x7000843606, 0x7000843601);
        }
        121 => {
            c.w64(11, c.r64(0));
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000843609); };
            c.w64(6, t225);
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084360d); };
            c.w64(9, t226);
            let Some(t227) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000843614); };
            c.w64(15, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700084361b); };
            c.w64(0, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084361f); };
            c.w64(3, t229);
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000843623); };
            c.w64(8, t230);
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000843627); };
            c.w64(2, t231);
            blk = 4; continue;
        }
        122 => {
            return c.call(0x70008a2054, 0x7000843635, 0x7000843630);
        }
        123 => {
            let Some(t232) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000843635); };
            c.w64(8, t232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084363c); };
            c.w64(11, t233);
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000843640); };
            c.w64(6, t234);
            let Some(t235) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000843644); };
            c.w64(2, t235);
            let Some(t236) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084364b); };
            c.w64(9, t236);
            let Some(t237) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000843652); };
            c.w64(15, t237);
            blk = 15; continue;
        }
        124 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x7000843663); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x7000843667); }
            return c.call(0x70007febf4, 0x7000843671, 0x700084366c);
        }
        125 => {
            c.w64(8, c.r64(0));
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000843674); };
            c.w64(0, t238);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000843678); };
            c.w64(3, t239);
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700084367c); };
            c.w64(15, t240);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x7000843680); }
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000843685); };
            c.w64(9, t241);
            let Some(t242) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084368c); };
            c.w64(12, t242);
            blk = 70; continue;
        }
        126 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000843698); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000843699); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084369a); }
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084369b); };
            c.w64(6, t243);
            let Some(t244) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700084369f); };
            c.w64(3, t244);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70008436b3, 0x70008436ae);
        }
        127 => {
            c.w64(8, c.r64(0));
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008436b6); };
            c.w64(0, t245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008436ba); };
            c.w64(1, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008436be); };
            c.w64(3, t247);
            blk = 111; continue;
        }
        128 => {
            return RegfileExit::fault(0x70008436c7);
        }
        129 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008436c8); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008436ca); }
            let Some(t248) = c.ld64(c.r64(13).wrapping_add(0x2330)) else { return RegfileExit::fault(0x70008436cb); };
            c.w64(3, t248);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008436dc, 0x70008436d7);
        }
        130 => {
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008436dc); };
            c.w64(11, t249);
            blk = 117; continue;
        }
        131 => {
            return RegfileExit::jump(0x70008436e8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008436f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008436f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008436f8); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008436fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000843702); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000843706); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700084370a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700084370e); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000843717); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700084371f); };
            c.w64(7, t3);
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000843726); };
            c.logic(t4 & 0x1000000u64, 32);
            if c.cc_ne() { blk = 219; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000843733); };
            c.cmp(t5, c.r32(3), 32);
            if c.cc_e() { blk = 218; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(8, c.r64(0).wrapping_add(c.r64(0)));
            c.w32(9, c.r32(7));
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000843747); };
            c.w32(11, t6);
            let r7 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r7);
            let r8 = c.r32(9) | 0x2u64;
            c.w32(9, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 129; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700084375c); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            blk = 7; continue;
        }
        7 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 169; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(2));
            blk = 9; continue;
        }
        9 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700084377a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700084377e); }
            c.w64(9, c.r64(8));
            let r11 = c.r64(9) & 0xfu64;
            c.w64(9, r11);
            c.logic(r11, 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_e() { blk = 34; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(9), 0x5u64, 64);
            if c.cc_e() { blk = 34; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(9), 0xdu64, 64);
            if c.cc_e() { blk = 33; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_e() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r64(9), 0x6u64, 64);
            if c.cc_e() { blk = 34; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 34; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(9), 0x8u64, 64);
            if c.cc_e() { blk = 34; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(9), 0x9u64, 64);
            if c.cc_e() { blk = 34; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r64(9), 0xbu64, 64);
            if c.cc_e() { blk = 24; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(9), 0x3u64, 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t12) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008437eb); };
            c.w64(5, t12);
            c.w64(0, c.r64(2));
            c.w64(3, c.r64(1));
            let r13 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r13);
            return RegfileExit::jump(0x7000820354);
        }
        21 => {
            let Some(t14) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008437fe); };
            c.w32(7, t14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000843801); };
            c.cmp(t15, c.r32(7), 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700084380e); };
            c.w64(0, t16);
            blk = 23; continue;
        }
        23 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000843818); }
            return c.ret(0x0, 0x7000843819);
        }
        24 => {
            let Some(t17) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700084381a); };
            c.w64(5, t17);
            let r18 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r18);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000843822); }
            let Some(t19) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000843823); };
            if !c.push64(t19) { return RegfileExit::fault(0x7000843823); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x23b8)) else { return RegfileExit::fault(0x7000843827); };
            c.w64(3, t20);
            c.w32(0, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x7000843833); }
            return RegfileExit::jump(0x700086ed74);
        }
        25 => {
            blk = 34; continue;
        }
        26 => {
            let Some(t21) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084383f); };
            c.w32(7, t21);
            let r22 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r22);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700084384b); };
            c.w64(7, t23);
            blk = 28; continue;
        }
        28 => {
            let Some(t24) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000843852); };
            c.w32(8, t24);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r25 = c.sub(c.r64(8), 0x1u64, 64);
            c.w64(8, r25);
            let Some(t26) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000843860); };
            c.w32(9, t26);
            let r27 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r27);
            let r28 = c.r64(9) & c.r64(8);
            c.w64(9, r28);
            c.logic(r28, 64);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 30; continue;
        }
        29 => {
            let r29 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r29);
            let r30 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r30);
            c.w64(12, c.r64(8));
            let r31 = c.r64(12) & c.r64(9);
            c.w64(12, r31);
            c.logic(r31, 64);
            c.w64(9, c.r64(12));
            blk = 30; continue;
        }
        30 => {
            c.w64(12, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t32) = c.ld32(c.r64(7).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000843885); };
            c.w32(12, t32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700084388a); };
            c.cmp(t33, c.r32(12), 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(12), c.r32(1), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700084389c); };
            c.w64(0, t34);
            blk = 23; continue;
        }
        33 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008438a8); };
            c.w64(0, t35);
            blk = 23; continue;
        }
        34 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008438b4); };
            c.w64(0, t36);
            blk = 23; continue;
        }
        35 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t37) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008438c9); };
            c.w32(9, t37);
            let r38 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r38);
            let Some(t39) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008438d0); };
            c.w32(9, t39);
            c.cmp(c.r16(9), 0x42u64, 16);
            if c.cc_ne() { blk = 40; continue; }
            blk = 37; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008438dc); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            if c.cc_ne() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(12, 0x1fffffffffffffu64);
            let r40 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r40);
            c.w64(15, 0x3ffffffffffffeu64);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 45; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_ae() { blk = 219; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(10, c.r64(4));
            let r41 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r41);
            let r42 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r42);
            c.logic(r42, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000843929); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x1478)) else { return RegfileExit::fault(0x700084392e); };
            c.w64(0, t43);
            c.w64(10, 0x700084393cu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700084393c); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000843940); }
            return c.call(c.r64(0), 0x7000843946, 0x7000843944);
        }
        42 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000843946); }
            let Some(t44) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700084394e); };
            c.w64(4, t44);
            c.cmp(c.r32(0), 0xffffffffffffffffu64, 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700084395f); };
            c.w64(8, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000843963); };
            c.w64(2, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000843967); };
            c.w64(1, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084396b); };
            c.w64(0, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084396f); };
            c.w64(3, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000843973); };
            c.w64(6, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000843977); };
            c.w64(7, t51);
            blk = 45; continue;
        }
        44 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9));
            blk = 45; continue;
        }
        45 => {
            c.w64(9, c.r64(8));
            if let Some(sr) = c.shr(c.r64(9), 0xau64, 64) { c.w64(9, sr); }
            let r52 = c.r64(9) & 0xffu64;
            c.w64(9, r52);
            c.logic(r52, 64);
            c.w32(12, c.r32(9));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000843997); }
            c.cmp(c.r32(9), 0xbu64, 32);
            if c.cc_g() { blk = 75; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t53) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008439a5); };
            c.w32(9, t53);
            let r54 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r54);
            c.logic(c.r16(8) & 0x100u64, 16);
            if c.cc_ne() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t55) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008439b4); };
            c.w32(15, t55);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            blk = 49; continue;
        }
        48 => {
            let Some(t56) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008439c0); };
            c.w32(15, t56);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ae() { blk = 207; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(12), 0x6u64, 32);
            if c.cc_l() { blk = 60; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(12), 0x9u64, 32);
            if c.cc_l() { blk = 56; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(12), 0x9u64, 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r32(12), 0xau64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r32(12), 0xbu64, 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 55; continue;
        }
        55 => {
            blk = 202; continue;
        }
        56 => {
            c.cmp(c.r32(12), 0x6u64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(12), 0x7u64, 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 59; continue;
        }
        59 => {
            blk = 202; continue;
        }
        60 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_l() { blk = 65; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 62; continue;
        }
        62 => {
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 64; continue;
        }
        64 => {
            blk = 202; continue;
        }
        65 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            blk = 202; continue;
        }
        69 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000843a4d); };
            c.w64(0, t57);
            blk = 23; continue;
        }
        70 => {
            let Some(t58) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000843a59); };
            c.w32(7, t58);
            let Some(t59) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000843a5e); };
            c.cmp(t59, c.r32(7), 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000843a6b); };
            c.w64(0, t60);
            blk = 23; continue;
        }
        72 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000843a77); };
            c.w64(0, t61);
            blk = 23; continue;
        }
        73 => {
            c.w64(7, 0xfff7fffffff7ffffu64);
            let Some(t62) = c.ld64(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000843a8d); };
            c.cmp(t62, c.r64(7), 64);
            if c.cc_e() { blk = 93; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000843a98); };
            c.w64(0, t63);
            blk = 23; continue;
        }
        75 => {
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_ge() { blk = 99; continue; }
            blk = 76; continue;
        }
        76 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_ge() { blk = 96; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w32(9, c.r32(8));
            let r64 = c.r32(9) & 0x3fc00u64;
            c.w32(9, r64);
            c.logic(r64, 32);
            c.cmp(c.r32(9), 0x3000u64, 32);
            if c.cc_ne() { blk = 202; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w32(9, 0xfffffffeu64);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 207; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t65) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000843ade); };
            c.w32(7, t65);
            let r66 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r66);
            let Some(t67) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000843ae4); };
            c.w32(9, t67);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r68 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r68);
            c.w32(12, c.r32(11));
            c.w64(10, c.r64(4));
            let r69 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r69);
            let r70 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r70);
            c.logic(r70, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000843b00); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000843b05); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000843b09); }
            c.w64(2, c.r64(12));
            c.w64(1, c.r64(13));
            c.w64(15, c.r64(1));
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x7000843b16); };
            c.w64(0, t71);
            c.w64(10, 0x7000843b24u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000843b24); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000843b28); }
            return c.call(c.r64(0), 0x7000843b2e, 0x7000843b2c);
        }
        80 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000843b2e); }
            let Some(t72) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000843b36); };
            c.w64(4, t72);
            c.w32(7, c.r32(0));
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000843b3d); };
            let r74 = c.r64(7) & t73;
            c.w64(7, r74);
            c.logic(r74, 64);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000843b44); };
            c.w64(9, t75);
            blk = 82; continue;
        }
        81 => {
            let r76 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r76);
            let r77 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000843b51); };
            c.w64(11, t78);
            let r79 = c.r64(11) & c.r64(7);
            c.w64(11, r79);
            c.logic(r79, 64);
            c.w64(7, c.r64(11));
            blk = 82; continue;
        }
        82 => {
            c.w64(11, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x4));
            c.w64(12, c.r64(11).wrapping_mul(4));
            let Some(t80) = c.ld32(c.r64(9).wrapping_add(c.r64(12)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000843b68); };
            c.w32(15, t80);
            let r81 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r81);
            let Some(t82) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000843b70); };
            c.cmp(t82, c.r32(15), 32);
            if c.cc_e() { blk = 92; continue; }
            blk = 83; continue;
        }
        83 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 88; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t83) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000843b7f); };
            c.cmp(t83, c.r32(15), 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 85; continue;
        }
        85 => {
            c.xorpd_rr(0, 0);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000843b8c); };
            c.cvtsi2sd(0, (t84 as i64));
            if !c.movsd_rm(1, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000843b92); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 81; continue; }
            blk = 86; continue;
        }
        86 => {
            if c.cc_e() { blk = 89; continue; }
            blk = 87; continue;
        }
        87 => {
            blk = 81; continue;
        }
        88 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000843ba8); };
            c.cmp(c.r64(15), t85, 64);
            if c.cc_ne() { blk = 81; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t86) = c.ld32(c.r64(9).wrapping_add(c.r64(12)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000843bae); };
            c.w32(7, t86);
            c.logic(c.r8(7) & 0x2u64, 8);
            if c.cc_ne() { blk = 204; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t87) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000843bbd); };
            c.w32(7, t87);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ae() { blk = 205; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000843bd4); };
            c.w64(0, t88);
            blk = 23; continue;
        }
        92 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000843be0); };
            c.w64(2, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000843be4); };
            c.w64(1, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000843be8); };
            c.w64(0, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000843bec); };
            c.w64(3, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000843bf0); };
            c.w64(6, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000843bf4); };
            c.w64(8, t94);
            blk = 93; continue;
        }
        93 => {
            c.logic(c.r16(8) & 0x200u64, 16);
            if c.cc_e() { blk = 219; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000843c04); };
            c.w64(7, t95);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t96) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000843c0e); };
            c.cmp(t96, c.r32(8), 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000843c18); };
            c.w64(0, t97);
            blk = 23; continue;
        }
        96 => {
            let Some(t98) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000843c24); };
            c.w32(9, t98);
            let r99 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r99);
            let Some(t100) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000843c2b); };
            c.logic(t100 & 0x4u64, 8);
            if c.cc_ne() { blk = 219; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t101) = c.ld64(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000843c36); };
            c.cmp(t101, c.r64(11), 64);
            if c.cc_be() { blk = 207; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000843c40); };
            c.w64(0, t102);
            blk = 23; continue;
        }
        99 => {
            let Some(t103) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000843c4c); };
            c.w32(9, t103);
            let r104 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r104);
            let Some(t105) = c.ld32(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000843c53); };
            c.w32(12, t105);
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 125; continue; }
            blk = 100; continue;
        }
        100 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 219; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t106) = c.ld64(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000843c6b); };
            c.w64(9, t106);
            let Some(t107) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000843c6f); };
            c.w64(12, t107);
            let Some(t108) = c.ld8(c.r64(7).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000843c73); };
            c.logic(t108 & 0x1u64, 8);
            if c.cc_ne() { blk = 104; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t109) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000843c79); };
            c.w64(15, t109);
            let r110 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r110);
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_a() { blk = 219; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t111) = c.ld64(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000843c89); };
            c.w64(9, t111);
            blk = 127; continue;
        }
        104 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_a() { blk = 219; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t112) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000843c9b); };
            c.w32(15, t112);
            let r113 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r113);
            let Some(t114) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000843ca2); };
            c.w32(15, t114);
            let r115 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r115);
            let r116 = c.r32(15) & 0x3fu64;
            c.w32(15, r116);
            c.logic(r116, 32);
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_l() { blk = 113; continue; }
            blk = 106; continue;
        }
        106 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_l() { blk = 110; continue; }
            blk = 107; continue;
        }
        107 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_e() { blk = 120; continue; }
            blk = 108; continue;
        }
        108 => {
            c.cmp(c.r32(15), 0x25u64, 32);
            if c.cc_e() { blk = 123; continue; }
            blk = 109; continue;
        }
        109 => {
            c.cmp(c.r32(15), 0x26u64, 32);
            if c.cc_e() { blk = 123; continue; }
            blk = 110; continue;
        }
        110 => {
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_e() { blk = 122; continue; }
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r32(15), 0x22u64, 32);
            if c.cc_e() { blk = 122; continue; }
            blk = 112; continue;
        }
        112 => {
            c.cmp(c.r32(15), 0x23u64, 32);
            if c.cc_e() { blk = 123; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_l() { blk = 117; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(15), 0x1fu64, 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_e() { blk = 122; continue; }
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r32(15), 0x1cu64, 32);
            if c.cc_e() { blk = 120; continue; }
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r32(15), 0x1du64, 32);
            if c.cc_e() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            return RegfileExit::fault(0x7000843d07);
        }
        120 => {
            c.w32(15, 0x1u64);
            blk = 124; continue;
        }
        121 => {
            c.w32(15, 0x2u64);
            blk = 124; continue;
        }
        122 => {
            c.w32(15, 0x4u64);
            blk = 124; continue;
        }
        123 => {
            c.w32(15, 0x8u64);
            blk = 124; continue;
        }
        124 => {
            let r117 = c.sub(c.r64(9), c.r64(12), 64);
            c.w64(9, r117);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(9));
            c.w64(9, c.r64(10));
            c.w64(12, c.r64(2));
            c.cqo();
            c.idiv(c.r64(15), 64);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(2, c.r64(12));
            blk = 127; continue;
        }
        125 => {
            c.w64(10, c.r64(4));
            let r118 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r118);
            let r119 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r119);
            c.logic(r119, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000843d53); }
            c.w64(2, c.r64(7));
            c.w64(1, c.r64(13));
            c.w64(9, c.r64(1));
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x7000843d61); };
            c.w64(0, t120);
            c.w64(10, 0x7000843d6fu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000843d6f); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000843d73); }
            return c.call(c.r64(0), 0x7000843d79, 0x7000843d77);
        }
        126 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000843d79); }
            let Some(t121) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000843d81); };
            c.w64(4, t121);
            c.w64(9, c.r64(0));
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000843d89); };
            c.w64(2, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000843d8d); };
            c.w64(1, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000843d91); };
            c.w64(0, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000843d95); };
            c.w64(3, t125);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000843d99); };
            c.w64(6, t126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000843d9d); };
            c.w64(11, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000843da1); };
            c.w64(7, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000843da5); };
            c.w64(8, t129);
            blk = 127; continue;
        }
        127 => {
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ae() { blk = 207; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t130) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000843db2); };
            c.w64(0, t130);
            blk = 23; continue;
        }
        129 => {
            c.w32(8, c.r32(11));
            let r131 = c.r32(8) & 0x3u64;
            c.w32(8, r131);
            c.logic(r131, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000843dc5); }
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_ne() { blk = 219; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t132) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000843dd3); };
            c.w32(8, t132);
            let Some(t133) = c.ld32(c.r64(13).wrapping_add(0x2c8)) else { return RegfileExit::fault(0x7000843dd7); };
            c.cmp(t133, c.r32(8), 32);
            if c.cc_e() { blk = 164; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t134) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000843de4); };
            c.cmp(t134, c.r32(11), 32);
            if c.cc_e() { blk = 218; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t135) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000843df1); };
            c.cmp(t135, c.r32(11), 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(11), c.r32(1), 32);
            if c.cc_e() { blk = 161; continue; }
            blk = 134; continue;
        }
        134 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 219; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t136) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000843e10); };
            c.w32(7, t136);
            let r137 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r137);
            let Some(t138) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000843e16); };
            c.w32(7, t138);
            c.cmp(c.r16(7), 0x42u64, 16);
            if c.cc_ne() { blk = 139; continue; }
            blk = 136; continue;
        }
        136 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000843e20); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 139; continue; }
            blk = 137; continue;
        }
        137 => {
            if c.cc_ne() { blk = 139; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t139) = c.ld64(0x70008438f9u64) else { return RegfileExit::fault(0x7000843e3b); };
            c.w64(9, t139);
            let r140 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r140);
            let Some(t141) = c.ld64(0x7000843906u64) else { return RegfileExit::fault(0x7000843e45); };
            c.w64(8, t141);
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_be() { blk = 219; continue; }
            blk = 139; continue;
        }
        139 => {
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_e() { blk = 158; continue; }
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_b() { blk = 143; continue; }
            blk = 141; continue;
        }
        141 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_ne() { blk = 219; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t142) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000843e6f); };
            c.w32(7, t142);
            let r143 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r143);
            blk = 159; continue;
        }
        143 => {
            let Some(t144) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000843e7a); };
            c.w32(8, t144);
            c.logic(c.r32(8) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 144; continue;
        }
        144 => {
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_e() { blk = 219; continue; }
            blk = 145; continue;
        }
        145 => {
            c.cmp(c.r16(7), 0x25u64, 16);
            if c.cc_e() { blk = 157; continue; }
            blk = 146; continue;
        }
        146 => {
            c.cmp(c.r16(7), 0x2du64, 16);
            if c.cc_e() { blk = 157; continue; }
            blk = 147; continue;
        }
        147 => {
            c.logic(c.r8(7) & 0x20u64, 8);
            if c.cc_ne() { blk = 149; continue; }
            blk = 148; continue;
        }
        148 => {
            c.w64(7, c.r64(1));
            blk = 159; continue;
        }
        149 => {
            c.w64(10, c.r64(4));
            let r145 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r145);
            let r146 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r146);
            c.logic(r146, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000843ec2); }
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(13));
            c.w64(7, c.r64(1));
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x1480)) else { return RegfileExit::fault(0x7000843ed0); };
            c.w64(0, t147);
            c.w64(10, 0x7000843edeu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000843ede); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000843ee2); }
            return c.call(c.r64(0), 0x7000843ee8, 0x7000843ee6);
        }
        150 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000843ee8); }
            let Some(t148) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000843ef0); };
            c.w64(4, t148);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 156; continue; }
            blk = 151; continue;
        }
        151 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 155; continue; }
            blk = 152; continue;
        }
        152 => {
            c.cmp(c.r64(7), 0xfffffffffffffffeu64, 64);
            if c.cc_e() { blk = 154; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000843f0b); };
            c.w64(2, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000843f0f); };
            c.w64(1, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000843f13); };
            c.w64(0, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000843f17); };
            c.w64(3, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000843f1b); };
            c.w64(6, t153);
            blk = 219; continue;
        }
        154 => {
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000843f24); };
            c.w64(2, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000843f28); };
            c.w64(1, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000843f2c); };
            c.w64(0, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000843f30); };
            c.w64(3, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000843f34); };
            c.w64(6, t158);
            blk = 219; continue;
        }
        155 => {
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000843f3d); };
            c.w64(2, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000843f41); };
            c.w64(1, t160);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000843f45); };
            c.w64(0, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000843f49); };
            c.w64(3, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000843f4d); };
            c.w64(6, t163);
            blk = 219; continue;
        }
        156 => {
            c.w64(7, c.r64(0));
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000843f59); };
            c.w64(2, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000843f5d); };
            c.w64(1, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000843f61); };
            c.w64(0, t166);
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000843f65); };
            c.w64(3, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000843f69); };
            c.w64(6, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000843f6d); };
            c.w64(11, t169);
            blk = 159; continue;
        }
        157 => {
            let Some(t170) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000843f73); };
            c.w32(7, t170);
            let r171 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r171);
            blk = 159; continue;
        }
        158 => {
            c.w64(7, c.r64(1));
            blk = 159; continue;
        }
        159 => {
            c.cmp(c.r32(11), c.r32(7), 32);
            if c.cc_e() { blk = 162; continue; }
            blk = 160; continue;
        }
        160 => {
            blk = 219; continue;
        }
        161 => {
            c.w64(7, c.r64(1));
            blk = 162; continue;
        }
        162 => {
            let Some(t172) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000843f8b); };
            c.w64(5, t172);
            c.w64(1, c.r64(7));
            let r173 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r173);
            return RegfileExit::jump(0x700081ebf4);
        }
        163 => {
            blk = 219; continue;
        }
        164 => {
            let Some(t174) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000843fa0); };
            c.w32(8, t174);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r175 = c.sub(c.r64(8), 0x2u64, 64);
            c.w64(8, r175);
            blk = 165; continue;
        }
        165 => {
            c.w64(9, c.r64(8).wrapping_mul(4));
            c.w32(12, c.r32(7));
            let Some(t176) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000843fb9); };
            c.w32(15, t176);
            let r177 = c.r32(12) | 0x2u64;
            c.w32(12, r177);
            c.logic(r177, 32);
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_e() { blk = 168; continue; }
            blk = 166; continue;
        }
        166 => {
            let r178 = c.sub(c.r64(8), 0x2u64, 64);
            c.w64(8, r178);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_ge() { blk = 165; continue; }
            blk = 167; continue;
        }
        167 => {
            blk = 219; continue;
        }
        168 => {
            let Some(t179) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000843fd6); };
            c.w32(7, t179);
            let r180 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r180);
            blk = 7; continue;
        }
        169 => {
            let Some(t181) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000843fe3); };
            c.w32(8, t181);
            let Some(t182) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x7000843fe7); };
            c.cmp(t182, c.r32(8), 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t183) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000843ff4); };
            c.w32(8, t183);
            let r184 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r184);
            c.w32(9, 0);
            c.logic(0, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t185) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000844003); };
            c.cmp(t185, c.r32(9), 32);
            if c.cc_ne() { blk = 219; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t186) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084400d); };
            c.w32(8, t186);
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.logic(c.r8(11) & 0x30u64, 8);
            if c.cc_e() { blk = 190; continue; }
            blk = 173; continue;
        }
        173 => {
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 182; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t187) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000844027); };
            c.w32(11, t187);
            let r188 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r188);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 176; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t189) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000844034); };
            c.w64(11, t189);
            blk = 176; continue;
        }
        176 => {
            let Some(t190) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084403b); };
            c.w32(12, t190);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r191 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r191);
            let Some(t192) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000844049); };
            c.w32(15, t192);
            let r193 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r193);
            let r194 = c.r64(15) & c.r64(12);
            c.w64(15, r194);
            c.logic(r194, 64);
            blk = 177; continue;
        }
        177 => {
            c.w64(0, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t195) = c.ld32(c.r64(11).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000844059); };
            c.w32(0, t195);
            let Some(t196) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700084405e); };
            c.cmp(t196, c.r32(0), 32);
            if c.cc_e() { blk = 181; continue; }
            blk = 178; continue;
        }
        178 => {
            c.cmp(c.r32(0), c.r32(1), 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 179; continue;
        }
        179 => {
            let r197 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r197);
            let r198 = c.add(c.r64(15), c.r64(9), 64);
            c.w64(15, r198);
            c.w64(0, c.r64(12));
            let r199 = c.r64(0) & c.r64(15);
            c.w64(0, r199);
            c.logic(r199, 64);
            c.w64(15, c.r64(0));
            blk = 177; continue;
        }
        180 => {
            let Some(t200) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700084407d); };
            c.w64(0, t200);
            blk = 23; continue;
        }
        181 => {
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000844089); };
            c.w64(0, t201);
            blk = 190; continue;
        }
        182 => {
            let Some(t202) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000844092); };
            c.w32(9, t202);
            let r203 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r203);
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 183; continue;
        }
        183 => {
            let r204 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r204);
            c.logic(r204, 64);
            let Some(t205) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008440a7); };
            c.w32(11, t205);
            let r206 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r206);
            let Some(t207) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008440ae); };
            c.w32(11, t207);
            let r208 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r208);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 190; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t209) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008440ba); };
            c.w32(12, t209);
            let r210 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r210);
            let Some(t211) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008440c1); };
            c.cmp(t211, 0xbeu64, 16);
            if c.cc_ne() { blk = 219; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t212) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008440cf); };
            c.w32(12, t212);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xa9u64, 64);
            if c.cc_a() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            return RegfileExit::fault(0x70008440e2);
        }
        187 => {
            let Some(t213) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008440e3); };
            c.w32(12, t213);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t214) = c.ld32(c.r64(9).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x70008440ea); };
            c.w32(9, t214);
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xa9u64, 64);
            if c.cc_a() { blk = 189; continue; }
            blk = 188; continue;
        }
        188 => {
            return RegfileExit::fault(0x70008440fd);
        }
        189 => {
            let Some(t215) = c.ld32(c.r64(11).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x70008440fe); };
            c.cmp(t215, c.r32(9), 32);
            if c.cc_ne() { blk = 219; continue; }
            blk = 190; continue;
        }
        190 => {
            let Some(t216) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700084410b); };
            c.w32(7, t216);
            let r217 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r217);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 195; continue; }
            blk = 191; continue;
        }
        191 => {
            let Some(t218) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000844117); };
            c.cmp(t218, c.r32(7), 32);
            if c.cc_e() { blk = 194; continue; }
            blk = 192; continue;
        }
        192 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 193; continue;
        }
        193 => {
            let r219 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r219);
            c.logic(r219, 64);
            blk = 9; continue;
        }
        194 => {
            c.w64(7, c.r64(2));
            blk = 9; continue;
        }
        195 => {
            let Some(t220) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700084413a); };
            c.w64(0, t220);
            blk = 23; continue;
        }
        196 => {
            let Some(t221) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000844146); };
            c.w64(5, t221);
            let r222 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r222);
            let Some(t223) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700084414e); };
            c.logic(t223 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 198; continue; }
            blk = 197; continue;
        }
        197 => {
            let r224 = c.add(c.r64(7), 0x3fu64, 64);
            c.w64(7, r224);
            blk = 199; continue;
        }
        198 => {
            let Some(t225) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700084415d); };
            c.w32(7, t225);
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x7000844160); };
            c.w64(7, t226);
            blk = 199; continue;
        }
        199 => {
            return RegfileExit::jump(c.r64(7));
        }
        200 => {
            blk = 219; continue;
        }
        201 => {
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084416f); };
            c.w64(2, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000844173); };
            c.w64(1, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000844177); };
            c.w64(0, t229);
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084417b); };
            c.w64(3, t230);
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084417f); };
            c.w64(6, t231);
            blk = 219; continue;
        }
        202 => {
            return RegfileExit::fault(0x7000844188);
        }
        203 => {
            blk = 219; continue;
        }
        204 => {
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084418e); };
            c.w64(2, t232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000844192); };
            c.w64(1, t233);
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000844196); };
            c.w64(0, t234);
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084419a); };
            c.w64(3, t235);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084419e); };
            c.w64(6, t236);
            blk = 219; continue;
        }
        205 => {
            return RegfileExit::fault(0x70008441a7);
        }
        206 => {
            blk = 219; continue;
        }
        207 => {
            c.logic(c.r8(8) & 0x40u64, 8);
            if c.cc_e() { blk = 219; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t237) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008441b7); };
            c.w32(7, t237);
            let r238 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r238);
            let Some(t239) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008441bd); };
            c.cmp(t239, 0x82eu64, 16);
            if c.cc_e() { blk = 217; continue; }
            blk = 209; continue;
        }
        209 => {
            c.w32(8, 0xfffffffeu64);
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_a() { blk = 219; continue; }
            blk = 210; continue;
        }
        210 => {
            let Some(t240) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008441d4); };
            c.w32(7, t240);
            let r241 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r241);
            let Some(t242) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008441da); };
            c.cmp(t242, c.r32(7), 32);
            if c.cc_e() { blk = 217; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t243) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008441e3); };
            c.w32(8, t243);
            let r244 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r244);
            let Some(t245) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008441ea); };
            c.w32(9, t245);
            c.cmp(c.r16(9), 0x411u64, 16);
            if c.cc_a() { blk = 214; continue; }
            blk = 212; continue;
        }
        212 => {
            c.cmp(c.r16(9), 0x411u64, 16);
            if c.cc_ne() { blk = 219; continue; }
            blk = 213; continue;
        }
        213 => {
            let Some(t246) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008441ff); };
            c.w32(9, t246);
            let Some(t247) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000844203); };
            c.cmp(t247, c.r32(9), 32);
            if c.cc_ne() { blk = 219; continue; }
            blk = 214; continue;
        }
        214 => {
            let Some(t248) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084420c); };
            c.w32(7, t248);
            let Some(t249) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700084420f); };
            c.cmp(t249, c.r32(7), 32);
            if c.cc_e() { blk = 216; continue; }
            blk = 215; continue;
        }
        215 => {
            let Some(t250) = c.ld32(c.r64(13).wrapping_add(0x3f8)) else { return RegfileExit::fault(0x7000844218); };
            c.cmp(t250, c.r32(7), 32);
            if c.cc_ne() { blk = 219; continue; }
            blk = 216; continue;
        }
        216 => {
            c.w64(7, c.r64(8));
            blk = 210; continue;
        }
        217 => {
            let Some(t251) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000844226); };
            c.w64(0, t251);
            blk = 23; continue;
        }
        218 => {
            let Some(t252) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000844232); };
            c.w64(5, t252);
            let r253 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r253);
            return RegfileExit::jump(0x7000844294);
        }
        219 => {
            let Some(t254) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700084423f); };
            c.w64(5, t254);
            let r255 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r255);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000844247); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000844248); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000844249); }
            let Some(t256) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700084424a); };
            if !c.push64(t256) { return RegfileExit::fault(0x700084424a); }
            c.w32(0, 0x4u64);
            let Some(t257) = c.ld64(c.r64(13).wrapping_add(0x20b8)) else { return RegfileExit::fault(0x7000844253); };
            c.w64(3, t257);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(2)) { return RegfileExit::fault(0x700084425a); }
            return RegfileExit::jump(0x700086ed74);
        }
        220 => {
            return RegfileExit::jump(0x7000844268);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000844274(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000844277); };
            c.w64(8, t0);
            let Some(t1) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700084427b); };
            c.w64(7, t1);
            c.w64(6, c.r64(7));
            c.w64(3, c.r64(8));
            return RegfileExit::jump(0x70008436f4);
        }
        1 => {
            return RegfileExit::jump(0x700084428c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000844294(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000844294); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000844298); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700084429e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008442a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008442a6); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 246; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008442b3); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008442b9); };
            c.w32(8, t3);
            c.cmp(c.r16(8), 0xbau64, 16);
            if c.cc_b() { blk = 246; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r16(8), 0xbcu64, 16);
            if c.cc_e() { blk = 241; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008442df); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008442e6); };
            c.w32(9, t6);
            c.cmp(c.r16(9), 0x42u64, 16);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008442f2); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(12, 0x1fffffffffffffu64);
            let r7 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r7);
            c.w64(15, 0x3ffffffffffffeu64);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 97; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_ae() { blk = 231; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084433f); };
            c.w32(11, t8);
            c.logic(c.r32(11) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 246; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(9), 0x25u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(9), 0x2du64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 246; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(9, c.r64(1));
            blk = 19; continue;
        }
        16 => {
            let Some(t9) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000844373); };
            c.w32(9, t9);
            let r10 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r10);
            blk = 19; continue;
        }
        17 => {
            if let Some(sr) = c.shr(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let r11 = c.r64(11) & 0xffffffu64;
            c.w64(11, r11);
            c.logic(r11, 64);
            blk = 97; continue;
        }
        18 => {
            c.w64(9, c.r64(1));
            blk = 19; continue;
        }
        19 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700084438f); };
            c.w64(15, t12);
            c.w64(11, c.r64(2));
            blk = 21; continue;
        }
        20 => {
            let Some(t13) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084439b); };
            c.w32(7, t13);
            let r14 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r14);
            let Some(t15) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008443a2); };
            c.w32(8, t15);
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(8), 0x410u64, 32);
            if c.cc_le() { blk = 50; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t16) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008443b4); };
            c.w32(3, t16);
            c.logic(c.r32(3) & 0x200000u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t17) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008443c3); };
            c.w32(11, t17);
            let r18 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r18);
            c.logic(c.r32(3) & 0xffc00u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 24; continue;
        }
        24 => {
            let r19 = c.shr(c.r32(3), 0xau64, 32).unwrap_or(c.r32(3));
            c.w32(3, r19);
            let r20 = c.r32(3) & 0x3ffu64;
            c.w32(3, r20);
            c.logic(r20, 32);
            c.cmp(c.r32(3), 0x20u64, 32);
            if c.cc_be() { blk = 39; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t21) = c.ld16(c.r64(11).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008443e8); };
            c.w32(2, (t21 as u16 as i16 as i64 as u64));
            let Some(t22) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008443ed); };
            c.w32(6, t22);
            let r23 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r23);
            let r24 = c.sub(c.r32(2), 0x1u64, 32);
            c.w32(2, r24);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008443f7); }
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 27; continue;
        }
        26 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008443ff); }
            blk = 27; continue;
        }
        27 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000844403); };
            c.w64(0, t25);
            let r26 = c.sub(c.r32(0), c.r32(1), 32);
            c.w32(0, r26);
            let r27 = c.shr(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r27);
            let r28 = c.add(c.r32(0), c.r32(1), 32);
            c.w32(0, r28);
            c.w32(12, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000844414); };
            c.w32(12, t29);
            let r30 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r30);
            let r31 = c.shr(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r31);
            let r32 = c.r32(12) & 0x3ffu64;
            c.w32(12, r32);
            c.logic(r32, 32);
            c.w32(12, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t33) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084442e); };
            c.w32(12, t33);
            let r34 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r34);
            let Some(t35) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000844436); };
            c.w32(12, t35);
            let r36 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r36);
            c.cmp(c.r32(6), c.r32(12), 32);
            if c.cc_be() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(1, c.r64(0).wrapping_add(0x1));
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000844447); };
            c.w64(0, t37);
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700084444f); }
            blk = 32; continue;
        }
        31 => {
            let r38 = c.add(c.r32(1), 0x1u64, 32);
            c.w32(1, r38);
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(2), c.r32(1), 32);
            if c.cc_l() { blk = 38; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(12, c.r64(1).wrapping_add(c.r64(1).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t39) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000844463); };
            c.w32(12, t39);
            let r40 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r40);
            let r41 = c.shr(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r41);
            let r42 = c.r32(12) & 0x3ffu64;
            c.w32(12, r42);
            c.logic(r42, 32);
            c.w32(0, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t43) = c.ld32(c.r64(11).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084447d); };
            c.w32(0, t43);
            let r44 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r44);
            let Some(t45) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000844485); };
            c.w32(7, t45);
            let r46 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r46);
            c.cmp(c.r32(6), c.r32(7), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(0), c.r32(9), 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(3), c.r32(12), 32);
            if c.cc_a() { blk = 227; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084449d); };
            c.w64(7, t47);
            blk = 61; continue;
        }
        37 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008444a6); };
            c.w64(7, t48);
            blk = 61; continue;
        }
        38 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008444af); };
            c.w64(7, t49);
            blk = 61; continue;
        }
        39 => {
            c.w64(12, (c.r32(3) as u32 as i32 as i64 as u64));
            let r50 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(12, r50);
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 61; continue; }
            blk = 40; continue;
        }
        40 => {
            let r51 = c.sub(c.r64(12), 0x3u64, 64);
            c.w64(12, r51);
            let Some(t52) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008444cd); };
            c.cmp(t52, c.r32(9), 32);
            if c.cc_e() { blk = 227; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_ne() { blk = 40; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 61; continue;
        }
        43 => {
            let Some(t53) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008444e3); };
            c.w32(11, t53);
            let r54 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r54);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(11, c.r64(15));
            blk = 45; continue;
        }
        45 => {
            let Some(t55) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008444f3); };
            c.w32(12, t55);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r56 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r56);
            let Some(t57) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000844501); };
            c.w32(0, t57);
            let r58 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r58);
            let r59 = c.r64(0) & c.r64(12);
            c.w64(0, r59);
            c.logic(r59, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 47; continue;
        }
        46 => {
            let r60 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r60);
            let r61 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r61);
            c.w64(2, c.r64(12));
            let r62 = c.r64(2) & c.r64(0);
            c.w64(2, r62);
            c.logic(r62, 64);
            c.w64(0, c.r64(2));
            blk = 47; continue;
        }
        47 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t63) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000844524); };
            c.w32(2, t63);
            let Some(t64) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000844529); };
            c.cmp(t64, c.r32(2), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(2), c.r32(9), 32);
            if c.cc_e() { blk = 227; continue; }
            blk = 49; continue;
        }
        49 => {
            blk = 46; continue;
        }
        50 => {
            c.cmp(c.r32(8), 0xbdu64, 32);
            if c.cc_ne() { blk = 246; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t65) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x700084454e); };
            c.logic(t65 & 0x24u64, 8);
            if c.cc_ne() { blk = 246; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t66) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000844558); };
            c.w32(11, t66);
            let r67 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r67);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(11, c.r64(15));
            blk = 54; continue;
        }
        54 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000844568); };
            c.w32(12, t68);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r69 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r69);
            let Some(t70) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000844576); };
            c.w32(0, t70);
            let r71 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r71);
            let r72 = c.r64(0) & c.r64(12);
            c.w64(0, r72);
            c.logic(r72, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        55 => {
            let r73 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r73);
            let r74 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r74);
            c.w64(2, c.r64(12));
            let r75 = c.r64(2) & c.r64(0);
            c.w64(2, r75);
            c.logic(r75, 64);
            c.w64(0, c.r64(2));
            blk = 56; continue;
        }
        56 => {
            c.w64(2, c.r64(0).wrapping_add(0x5));
            let Some(t76) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000844598); };
            c.w32(1, t76);
            let r77 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r77);
            let Some(t78) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008445a0); };
            c.cmp(t78, c.r32(1), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t79) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008445a9); };
            c.cmp(t79, c.r32(1), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t80) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008445b2); };
            c.cmp(t80, c.r32(9), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t81) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008445b8); };
            c.w32(11, t81);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_ae() { blk = 239; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t82) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008445cb); };
            c.w32(11, t82);
            let Some(t83) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008445cf); };
            c.cmp(t83, c.r32(11), 32);
            if c.cc_ne() { blk = 227; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(8), 0x82eu64, 32);
            if c.cc_ne() { blk = 92; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t84) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008445e9); };
            c.w32(8, t84);
            let r85 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r85);
            let Some(t86) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008445f0); };
            c.w32(8, t86);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 92; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t87) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000844600); };
            c.w32(11, t87);
            let r88 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r88);
            c.w32(12, c.r32(11));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 92; continue; }
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r32(11), 0x30u64, 32);
            if c.cc_g() { blk = 92; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w32(11, c.r16(8));
            c.logic(c.r8(8) & 0x7u64, 8);
            if c.cc_e() { blk = 81; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(9));
            c.w64(8, c.r64(6));
            blk = 67; continue;
        }
        67 => {
            c.w64(0, c.r64(11));
            let r89 = c.r32(0) & 0x7u64;
            c.w32(0, r89);
            c.logic(r89, 32);
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_l() { blk = 72; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 70; continue;
        }
        70 => {
            c.cmp(c.r32(0), 0x5u64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 71; continue;
        }
        71 => {
            blk = 233; continue;
        }
        72 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(0), 0x1u64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            blk = 233; continue;
        }
        75 => {
            let Some(t90) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000844666); };
            c.w32(11, t90);
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700084466b); };
            c.cmp(t91, c.r32(11), 32);
            if c.cc_ne() { blk = 234; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t92) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000844678); };
            c.w32(12, t92);
            let r93 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r93);
            let Some(t94) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000844680); };
            c.w32(11, t94);
            let r95 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r95);
            let Some(t96) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000844688); };
            c.w32(11, t96);
            blk = 67; continue;
        }
        77 => {
            let Some(t97) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084468f); };
            c.w32(11, t97);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r98 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r98);
            let Some(t99) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700084469d); };
            c.w32(12, t99);
            let r100 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r100);
            let Some(t101) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008446a5); };
            c.w32(11, t101);
            let r102 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r102);
            let Some(t103) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008446ad); };
            c.w32(11, t103);
            blk = 67; continue;
        }
        78 => {
            let Some(t104) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008446b4); };
            c.w32(12, t104);
            let r105 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r105);
            let Some(t106) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008446bc); };
            c.w32(11, t106);
            let r107 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r107);
            let Some(t108) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008446c4); };
            c.w32(11, t108);
            blk = 67; continue;
        }
        79 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 82; continue;
        }
        80 => {
            c.w32(0, 0x1u64);
            blk = 82; continue;
        }
        81 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(9));
            c.w64(8, c.r64(6));
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 235; continue; }
            blk = 83; continue;
        }
        83 => {
            let r109 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r109);
            blk = 84; continue;
        }
        84 => {
            let r110 = c.r32(11) & 0x8u64;
            c.w32(11, r110);
            c.logic(r110, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t111) = c.ld16(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x70008446fa); };
            c.w32(8, t111);
            blk = 87; continue;
        }
        86 => {
            let Some(t112) = c.ld8(c.r64(12).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000844701); };
            c.w32(8, t112);
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(8), 0x2du64, 32);
            if c.cc_e() { blk = 246; continue; }
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r32(8), 0x49u64, 32);
            if c.cc_e() { blk = 246; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(8), 0x4eu64, 32);
            if c.cc_e() { blk = 246; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(8), 0x30u64, 32);
            if c.cc_b() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r32(8), 0x39u64, 32);
            if c.cc_be() { blk = 246; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000844734); };
            c.w64(12, t113);
            let Some(t114) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000844738); };
            c.w32(8, t114);
            let Some(t115) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700084473d); };
            c.cmp(t115, c.r32(8), 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t116) = c.ld8(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000844746); };
            c.logic(t116 & 0x1u64, 8);
            if c.cc_ne() { blk = 229; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t117) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000844752); };
            c.w32(11, t117);
            let r118 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r118);
            let Some(t119) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000844759); };
            c.cmp(t119, c.r32(11), 32);
            if c.cc_e() { blk = 229; continue; }
            blk = 95; continue;
        }
        95 => {
            blk = 20; continue;
        }
        96 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9));
            blk = 97; continue;
        }
        97 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000844774); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(2));
            blk = 99; continue;
        }
        98 => {
            let Some(t120) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000844783); };
            c.w32(8, t120);
            let r121 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r121);
            let Some(t122) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084478a); };
            c.w32(9, t122);
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(9), 0x410u64, 32);
            if c.cc_le() { blk = 246; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t123) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700084479c); };
            c.w32(9, t123);
            let r124 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r124);
            let r125 = c.r32(9) & 0x3fu64;
            c.w32(9, r125);
            c.logic(r125, 32);
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_l() { blk = 135; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_l() { blk = 119; continue; }
            blk = 102; continue;
        }
        102 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_l() { blk = 111; continue; }
            blk = 103; continue;
        }
        103 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_l() { blk = 108; continue; }
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 105; continue;
        }
        105 => {
            c.cmp(c.r32(9), 0x26u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 106; continue;
        }
        106 => {
            c.cmp(c.r32(9), 0x28u64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 107; continue;
        }
        107 => {
            blk = 246; continue;
        }
        108 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 109; continue;
        }
        109 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 110; continue;
        }
        110 => {
            blk = 246; continue;
        }
        111 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_l() { blk = 116; continue; }
            blk = 112; continue;
        }
        112 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r32(9), 0x22u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 115; continue;
        }
        115 => {
            blk = 246; continue;
        }
        116 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 118; continue;
        }
        118 => {
            blk = 246; continue;
        }
        119 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_l() { blk = 128; continue; }
            blk = 120; continue;
        }
        120 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_l() { blk = 125; continue; }
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 123; continue;
        }
        123 => {
            c.cmp(c.r32(9), 0x1du64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 124; continue;
        }
        124 => {
            blk = 246; continue;
        }
        125 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 126; continue;
        }
        126 => {
            c.cmp(c.r32(9), 0x1au64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 127; continue;
        }
        127 => {
            blk = 246; continue;
        }
        128 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_l() { blk = 132; continue; }
            blk = 129; continue;
        }
        129 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r32(9), 0x18u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 131; continue;
        }
        131 => {
            blk = 246; continue;
        }
        132 => {
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 134; continue;
        }
        134 => {
            blk = 246; continue;
        }
        135 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_l() { blk = 153; continue; }
            blk = 136; continue;
        }
        136 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_l() { blk = 145; continue; }
            blk = 137; continue;
        }
        137 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_l() { blk = 142; continue; }
            blk = 138; continue;
        }
        138 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 139; continue;
        }
        139 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 141; continue;
        }
        141 => {
            blk = 246; continue;
        }
        142 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 143; continue;
        }
        143 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 144; continue;
        }
        144 => {
            blk = 246; continue;
        }
        145 => {
            c.cmp(c.r32(9), 0xbu64, 32);
            if c.cc_l() { blk = 150; continue; }
            blk = 146; continue;
        }
        146 => {
            c.cmp(c.r32(9), 0xbu64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 147; continue;
        }
        147 => {
            c.cmp(c.r32(9), 0xcu64, 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r32(9), 0xfu64, 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 149; continue;
        }
        149 => {
            blk = 246; continue;
        }
        150 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 151; continue;
        }
        151 => {
            c.cmp(c.r32(9), 0xau64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 152; continue;
        }
        152 => {
            blk = 246; continue;
        }
        153 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_l() { blk = 162; continue; }
            blk = 154; continue;
        }
        154 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_l() { blk = 159; continue; }
            blk = 155; continue;
        }
        155 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 156; continue;
        }
        156 => {
            c.cmp(c.r32(9), 0x7u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 157; continue;
        }
        157 => {
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 158; continue;
        }
        158 => {
            blk = 246; continue;
        }
        159 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_e() { blk = 170; continue; }
            blk = 160; continue;
        }
        160 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 170; continue; }
            blk = 161; continue;
        }
        161 => {
            blk = 246; continue;
        }
        162 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 166; continue; }
            blk = 163; continue;
        }
        163 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 164; continue;
        }
        164 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 165; continue;
        }
        165 => {
            blk = 246; continue;
        }
        166 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 168; continue;
        }
        168 => {
            blk = 246; continue;
        }
        169 => {
            blk = 174; continue;
        }
        170 => {
            let Some(t126) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008449c5); };
            c.w32(7, t126);
            let r127 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r127);
            let Some(t128) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008449cb); };
            c.w32(9, t128);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ae() { blk = 175; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w64(9, 0xfff7fffffff7ffffu64);
            let Some(t129) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008449e4); };
            c.cmp(t129, c.r64(9), 64);
            if c.cc_e() { blk = 193; continue; }
            blk = 172; continue;
        }
        172 => {
            blk = 227; continue;
        }
        173 => {
            let Some(t130) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008449f4); };
            c.w32(9, t130);
            let r131 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r131);
            let Some(t132) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008449fb); };
            c.w32(9, t132);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 227; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t133) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000844a08); };
            c.w32(7, t133);
            let r134 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r134);
            let Some(t135) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000844a0e); };
            c.w32(9, t135);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 177; continue; }
            blk = 175; continue;
        }
        175 => {
            c.w32(7, 0xfffffffeu64);
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_b() { blk = 246; continue; }
            blk = 176; continue;
        }
        176 => {
            blk = 193; continue;
        }
        177 => {
            let Some(t136) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000844a30); };
            c.w32(7, t136);
            let Some(t137) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000844a35); };
            c.cmp(t137, c.r32(7), 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 178; continue;
        }
        178 => {
            blk = 227; continue;
        }
        179 => {
            let Some(t138) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000844a47); };
            c.w32(9, t138);
            let r139 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r139);
            let Some(t140) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000844a4e); };
            c.w32(9, t140);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 227; continue; }
            blk = 180; continue;
        }
        180 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000844a5b); }
            c.w32(9, 0xfffffffeu64);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 246; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t141) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000844a6e); };
            c.w32(7, t141);
            let r142 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r142);
            let Some(t143) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000844a74); };
            c.w32(9, t143);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r144 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r144);
            c.w32(12, c.r32(11));
            c.w64(10, c.r64(4));
            let r145 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r145);
            let r146 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r146);
            c.logic(r146, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000844a90); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x7000844a95); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000844a99); }
            c.w64(2, c.r64(12));
            c.w64(1, c.r64(13));
            c.w64(15, c.r64(1));
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x7000844aa6); };
            c.w64(0, t147);
            c.w64(10, 0x7000844ab4u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000844ab4); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000844ab8); }
            return c.call(c.r64(0), 0x7000844abe, 0x7000844abc);
        }
        182 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000844abe); }
            let Some(t148) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000844ac6); };
            c.w64(4, t148);
            c.w32(7, c.r32(0));
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000844acd); };
            let r150 = c.r64(7) & t149;
            c.w64(7, r150);
            c.logic(r150, 64);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000844ad4); };
            c.w64(9, t151);
            blk = 184; continue;
        }
        183 => {
            let r152 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r152);
            let r153 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000844ae1); };
            c.w64(11, t154);
            let r155 = c.r64(11) & c.r64(7);
            c.w64(11, r155);
            c.logic(r155, 64);
            c.w64(7, c.r64(11));
            blk = 184; continue;
        }
        184 => {
            c.w64(11, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x4));
            let Some(t156) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000844af0); };
            c.w32(11, t156);
            let r157 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r157);
            let Some(t158) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000844af8); };
            c.cmp(t158, c.r32(11), 32);
            if c.cc_e() { blk = 192; continue; }
            blk = 185; continue;
        }
        185 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 190; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t159) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000844b07); };
            c.cmp(t159, c.r32(11), 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 187; continue;
        }
        187 => {
            c.xorpd_rr(0, 0);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000844b14); };
            c.cvtsi2sd(0, (t160 as i64));
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000844b1a); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 183; continue; }
            blk = 188; continue;
        }
        188 => {
            if c.cc_e() { blk = 227; continue; }
            blk = 189; continue;
        }
        189 => {
            blk = 183; continue;
        }
        190 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000844b34); };
            c.cmp(c.r64(11), t161, 64);
            if c.cc_e() { blk = 227; continue; }
            blk = 191; continue;
        }
        191 => {
            blk = 183; continue;
        }
        192 => {
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000844b40); };
            c.w64(8, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000844b44); };
            c.w64(11, t163);
            blk = 193; continue;
        }
        193 => {
            c.w64(7, c.r64(8));
            let Some(t164) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000844b4b); };
            c.w32(7, t164);
            let r165 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000844b51); };
            c.w64(2, t166);
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000844b55); };
            c.w64(1, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000844b59); };
            c.w64(6, t168);
            let Some(t169) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000844b5d); };
            c.cmp(t169, c.r32(7), 32);
            if c.cc_e() { blk = 229; continue; }
            blk = 194; continue;
        }
        194 => {
            blk = 98; continue;
        }
        195 => {
            let Some(t170) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000844b6f); };
            c.w32(8, t170);
            let r171 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r171);
            let Some(t172) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000844b76); };
            c.logic(t172 & 0x4u64, 8);
            if c.cc_ne() { blk = 229; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t173) = c.ld64(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000844b81); };
            c.cmp(t173, c.r64(11), 64);
            if c.cc_a() { blk = 227; continue; }
            blk = 197; continue;
        }
        197 => {
            blk = 229; continue;
        }
        198 => {
            let Some(t174) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000844b90); };
            c.w32(8, t174);
            let r175 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r175);
            let Some(t176) = c.ld32(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000844b97); };
            c.w32(9, t176);
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 224; continue; }
            blk = 199; continue;
        }
        199 => {
            c.logic(c.r8(9) & 0x4u64, 8);
            if c.cc_ne() { blk = 229; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t177) = c.ld64(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000844baf); };
            c.w64(8, t177);
            let Some(t178) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000844bb3); };
            c.w64(9, t178);
            let Some(t179) = c.ld8(c.r64(7).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000844bb7); };
            c.logic(t179 & 0x1u64, 8);
            if c.cc_ne() { blk = 203; continue; }
            blk = 201; continue;
        }
        201 => {
            let Some(t180) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000844bbd); };
            c.w64(12, t180);
            let r181 = c.add(c.r64(9), c.r64(12), 64);
            c.w64(9, r181);
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_a() { blk = 229; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t182) = c.ld64(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000844bcd); };
            c.w64(7, t182);
            c.w64(0, c.r64(7));
            blk = 226; continue;
        }
        203 => {
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_a() { blk = 229; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t183) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000844be2); };
            c.w32(7, t183);
            let r184 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r184);
            let Some(t185) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000844be8); };
            c.w32(7, t185);
            let r186 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r186);
            let r187 = c.r32(7) & 0x3fu64;
            c.w32(7, r187);
            c.logic(r187, 32);
            c.cmp(c.r32(7), 0x21u64, 32);
            if c.cc_l() { blk = 212; continue; }
            blk = 205; continue;
        }
        205 => {
            c.cmp(c.r32(7), 0x24u64, 32);
            if c.cc_l() { blk = 209; continue; }
            blk = 206; continue;
        }
        206 => {
            c.cmp(c.r32(7), 0x24u64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r32(7), 0x25u64, 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 208; continue;
        }
        208 => {
            c.cmp(c.r32(7), 0x26u64, 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 209; continue;
        }
        209 => {
            c.cmp(c.r32(7), 0x21u64, 32);
            if c.cc_e() { blk = 221; continue; }
            blk = 210; continue;
        }
        210 => {
            c.cmp(c.r32(7), 0x22u64, 32);
            if c.cc_e() { blk = 221; continue; }
            blk = 211; continue;
        }
        211 => {
            c.cmp(c.r32(7), 0x23u64, 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 212; continue;
        }
        212 => {
            c.cmp(c.r32(7), 0x1eu64, 32);
            if c.cc_l() { blk = 216; continue; }
            blk = 213; continue;
        }
        213 => {
            c.cmp(c.r32(7), 0x1eu64, 32);
            if c.cc_e() { blk = 220; continue; }
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r32(7), 0x1fu64, 32);
            if c.cc_e() { blk = 220; continue; }
            blk = 215; continue;
        }
        215 => {
            c.cmp(c.r32(7), 0x20u64, 32);
            if c.cc_e() { blk = 221; continue; }
            blk = 216; continue;
        }
        216 => {
            c.cmp(c.r32(7), 0x1cu64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r32(7), 0x1du64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 218; continue;
        }
        218 => {
            return RegfileExit::fault(0x7000844c38);
        }
        219 => {
            c.w32(7, 0x1u64);
            blk = 223; continue;
        }
        220 => {
            c.w32(7, 0x2u64);
            blk = 223; continue;
        }
        221 => {
            c.w32(7, 0x4u64);
            blk = 223; continue;
        }
        222 => {
            c.w32(7, 0x8u64);
            blk = 223; continue;
        }
        223 => {
            let r188 = c.sub(c.r64(8), c.r64(9), 64);
            c.w64(8, r188);
            c.w64(0, c.r64(8));
            c.cqo();
            c.idiv(c.r64(7), 64);
            blk = 226; continue;
        }
        224 => {
            c.w64(10, c.r64(4));
            let r189 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r189);
            let r190 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r190);
            c.logic(r190, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000844c6b); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x7000844c76); };
            c.w64(0, t191);
            c.w64(10, 0x7000844c84u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000844c84); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000844c88); }
            return c.call(c.r64(0), 0x7000844c8e, 0x7000844c8c);
        }
        225 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000844c8e); }
            let Some(t192) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000844c96); };
            c.w64(4, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000844c9b); };
            c.w64(11, t193);
            blk = 226; continue;
        }
        226 => {
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_ae() { blk = 229; continue; }
            blk = 227; continue;
        }
        227 => {
            let Some(t194) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000844ca4); };
            c.w64(0, t194);
            blk = 228; continue;
        }
        228 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000844cae); }
            return c.ret(0x0, 0x7000844caf);
        }
        229 => {
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000844cb0); };
            c.w64(0, t195);
            blk = 228; continue;
        }
        230 => {
            blk = 246; continue;
        }
        231 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 246; continue; }
            blk = 232; continue;
        }
        232 => {
            let Some(t196) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000844cc9); };
            c.w32(9, t196);
            let r197 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r197);
            blk = 19; continue;
        }
        233 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000844cd5); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000844cd9); }
            blk = 237; continue;
        }
        234 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000844cdf); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000844ce3); }
            blk = 237; continue;
        }
        235 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000844ce9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000844ced); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 237; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t198) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000844cf7); };
            c.w64(12, t198);
            blk = 84; continue;
        }
        237 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000844d01); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000844d03); }
            let Some(t199) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x7000844d04); };
            c.w64(3, t199);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000844d15, 0x7000844d10);
        }
        238 => {
            let r200 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r200);
            c.w64(8, c.r64(0));
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000844d1a); };
            c.w64(7, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000844d1e); };
            c.w64(9, t202);
            let Some(t203) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000844d22); };
            c.w64(15, t203);
            blk = 87; continue;
        }
        239 => {
            return RegfileExit::fault(0x7000844d2e);
        }
        240 => {
            blk = 246; continue;
        }
        241 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1af4, 0x7000844d39, 0x7000844d34);
        }
        242 => {
            let Some(t204) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000844d39); };
            c.w32(7, t204);
            let Some(t205) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000844d3c); };
            c.cmp(t205, c.r32(7), 32);
            if c.cc_ne() { blk = 244; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t206) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000844d45); };
            c.logic(t206 & 0x1u64, 8);
            if c.cc_ne() { blk = 229; continue; }
            blk = 244; continue;
        }
        244 => {
            c.w64(3, c.r64(0));
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000844d52); };
            c.w64(0, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000844d56); };
            c.w64(6, t208);
            return c.call(0x70008bced4, 0x7000844d5f, 0x7000844d5a);
        }
        245 => {
            blk = 228; continue;
        }
        246 => {
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000844d64); };
            if !c.push64(t209) { return RegfileExit::fault(0x7000844d64); }
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000844d67); };
            if !c.push64(t210) { return RegfileExit::fault(0x7000844d67); }
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x23b8)) else { return RegfileExit::fault(0x7000844d6a); };
            c.w64(3, t211);
            c.w32(0, 0x2u64);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000844d76); };
            c.w64(6, t212);
            return c.call(0x700086ed74, 0x7000844d7f, 0x7000844d7a);
        }
        247 => {
            blk = 228; continue;
        }
        248 => {
            return RegfileExit::jump(0x7000844d88);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000844d94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000844d94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000844d98); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x7000844d9e); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 86; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000844dab); };
            c.w32(9, t1);
            let r2 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000844db2); };
            c.logic(t3 & 0x2u64, 8);
            if c.cc_e() { blk = 86; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000844dbd); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(3));
            return c.call(0x70007f2274, 0x7000844dc8, 0x7000844dc3);
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000844dc8); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 84; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000844dd4); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000844ddb); };
            c.cmp(t6, 0xbau64, 16);
            if c.cc_b() { blk = 84; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000844de8); };
            c.w64(3, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000844def); };
            c.w64(6, t8);
            return c.call(0x7000870914, 0x7000844df8, 0x7000844df3);
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000844df8); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000844dfe); };
            c.w64(9, t9);
            c.w64(11, c.r64(6));
            c.w64(15, c.r64(6));
            c.w64(12, c.r64(9));
            blk = 10; continue;
        }
        7 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000844e10); };
            c.w64(11, t10);
            c.w64(9, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let r11 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r11);
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x7000844e20); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r12 = c.r64(3) & c.r64(12);
            c.w64(3, r12);
            c.logic(r12, 64);
            let Some(t13) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000844e32); };
            c.logic(t13 & 0x4u64, 8);
            if c.cc_ne() { blk = 88; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000844e3c); };
            c.w64(0, t14);
            blk = 10; continue;
        }
        10 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000844e40); };
            if !c.push64(t15) { return RegfileExit::fault(0x7000844e40); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x7000844e43); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000844e47); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x7000844e4b); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000844e54); };
            c.w64(6, t16);
            return c.call(0x70007f21d4, 0x7000844e5d, 0x7000844e58);
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x7000844e5d); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 82; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t17) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000844e69); };
            c.w32(9, t17);
            let r18 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r18);
            let Some(t19) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000844e70); };
            c.cmp(t19, 0xbau64, 16);
            if c.cc_b() { blk = 82; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x7000844e7d); };
            c.w64(3, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000844e84); };
            c.w64(6, t21);
            return c.call(0x7000870914, 0x7000844e8d, 0x7000844e88);
        }
        14 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000844e8d); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w32(9, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000844e9d); };
            c.cmp(t23, c.r32(0), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t24) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000844ea6); };
            c.w32(9, t24);
            let r25 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r25);
            let Some(t26) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000844ead); };
            c.logic(t26 & 0x10u64, 8);
            if c.cc_ne() { blk = 22; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000844eb4); };
            c.cmp(t27, c.r32(9), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t28) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000844ec1); };
            c.cmp(t28, 0x41u64, 16);
            if c.cc_e() { blk = 68; continue; }
            blk = 20; continue;
        }
        20 => {
            blk = 44; continue;
        }
        21 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x7000844edc); };
            c.w64(3, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000844ee3); };
            c.w64(0, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000844ee7); };
            c.w64(6, t31);
            return c.call(0x7000870914, 0x7000844ef0, 0x7000844eeb);
        }
        23 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000844ef0); };
            c.w64(15, t32);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x7000844ef4); }
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000844ef8); };
            c.cmp(c.r64(15), t33, 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000844efe); };
            c.w64(12, t34);
            blk = 7; continue;
        }
        25 => {
            c.w64(9, c.r64(15));
            if let Some(sr) = c.shr(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r35 = c.add(c.r64(9), c.r64(15), 64);
            c.w64(9, r35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000844f10); };
            c.w64(12, t36);
            let Some(t37) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000844f14); };
            c.w32(11, t37);
            let r38 = c.add(c.r64(9), 0x10u64, 64);
            c.w64(9, r38);
            if c.cc_e() { blk = 42; continue; }
            blk = 26; continue;
        }
        26 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000844f23); }
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000844f27); };
            c.cmp(t39, c.r32(11), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000844f30); };
            c.w64(11, t40);
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r64(9), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 80; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000844f44); };
            c.w64(15, t41);
            c.w64(3, c.r64(15).wrapping_mul(4).wrapping_add(0x7));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000844f50); };
            c.w64(2, t42);
            let Some(t43) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000844f57); };
            c.w64(2, t43);
            c.w64(1, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.w64(6, c.r64(1).wrapping_add(c.r64(2)));
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000844f66); };
            c.w64(7, t44);
            let Some(t45) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000844f6d); };
            c.cmp(t45, c.r64(6), 64);
            if c.cc_be() { blk = 72; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 72; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000844f83); };
            c.w64(1, t46);
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x7000844f8a); }
            let r47 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r47);
            blk = 32; continue;
        }
        32 => {
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000844f95); }
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000844f99); }
            c.w64(11, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000844fa4); }
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000844fad); };
            c.w64(1, t48);
            blk = 34; continue;
        }
        34 => {
            let r49 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r49);
            if !c.st32(c.r64(11).wrapping_add(c.r64(2)), c.r32(1)) { return RegfileExit::fault(0x7000844fb8); }
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_ne() { blk = 34; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(11, c.r64(2));
            let r50 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r50);
            c.logic(r50, 64);
            let Some(t51) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000844fcb); };
            c.logic(t51 & 0x4u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_e() { blk = 43; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w32(11, 0x7u64);
            blk = 38; continue;
        }
        38 => {
            let Some(t52) = c.ld32(c.r64(12).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x7000844fe2); };
            c.w32(1, t52);
            if !c.st32(c.r64(2).wrapping_add(c.r64(11)), c.r32(1)) { return RegfileExit::fault(0x7000844fe6); }
            let r53 = c.add(c.r64(11), 0x4u64, 64);
            c.w64(11, r53);
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_ne() { blk = 38; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 43; continue;
        }
        40 => {
            c.w64(8, c.r64(15).wrapping_mul(4));
            c.w64(11, c.r64(12).wrapping_add(0x7));
            c.w64(1, c.r64(2).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r54 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r54);
            let r55 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r55);
            c.logic(r55, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000845011); }
            c.w64(2, c.r64(11));
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000845019); };
            c.w64(0, t56);
            c.w64(10, 0x7000845027u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000845027); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700084502b); }
            return c.call(c.r64(0), 0x7000845031, 0x700084502f);
        }
        41 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000845031); }
            let Some(t57) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000845039); };
            c.w64(4, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084503e); };
            c.w64(15, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000845042); };
            c.w64(0, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000845046); };
            c.w64(9, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084504a); };
            c.w64(2, t61);
            blk = 43; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000845050); }
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000845054); };
            c.w64(2, t62);
            blk = 43; continue;
        }
        43 => {
            c.w64(12, c.r64(2));
            c.w64(15, c.r64(9));
            blk = 7; continue;
        }
        44 => {
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000845066); };
            c.w64(6, t63);
            let Some(t64) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084506a); };
            c.w32(9, t64);
            let r65 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r65);
            let Some(t66) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000845071); };
            c.w32(9, t66);
            let r67 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r67);
            let Some(t68) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000845078); };
            c.w32(11, t68);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            return RegfileExit::fault(0x7000845088);
        }
        46 => {
            let Some(t69) = c.ld32(c.r64(9).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000845089); };
            c.w32(9, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000845090); };
            c.w64(15, t70);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x7000845094); }
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000845098); };
            c.cmp(c.r64(15), t71, 64);
            if c.cc_e() { blk = 63; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008450a2); };
            c.w64(12, t72);
            let Some(t73) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008450a6); };
            c.w32(11, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008450ab); };
            c.cmp(t74, 0x0u64, 64);
            if c.cc_e() { blk = 61; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t75) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008450b6); };
            c.cmp(t75, c.r32(11), 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008450bf); };
            c.w64(11, t76);
            blk = 50; continue;
        }
        50 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008450c6); };
            c.cmp(t77, 0x7fffffdu64, 64);
            if c.cc_g() { blk = 76; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008450d4); };
            c.w64(15, t78);
            c.w32(0, c.r64(15).wrapping_add(c.r64(15)));
            c.w64(8, c.r64(15).wrapping_mul(4));
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008450e4); };
            c.w64(3, t79);
            let Some(t80) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008450eb); };
            c.w64(3, t80);
            c.w64(2, c.r64(8).wrapping_add(0x8));
            c.w64(1, c.r64(2).wrapping_add(c.r64(3)));
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008450f6); };
            c.w64(6, t81);
            let Some(t82) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008450fd); };
            c.cmp(t82, c.r64(1), 64);
            if c.cc_be() { blk = 74; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 74; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000845113); };
            c.w64(2, t83);
            if !c.st64(c.r64(2), c.r64(1)) { return RegfileExit::fault(0x700084511a); }
            let r84 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r84);
            blk = 54; continue;
        }
        54 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000845121); }
            c.w64(11, c.r64(3));
            let r85 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r85);
            c.logic(r85, 64);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700084512f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000845132); }
            let Some(t86) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000845136); };
            c.logic(t86 & 0x4u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(11, c.r64(8).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 62; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w32(0, 0x7u64);
            blk = 57; continue;
        }
        57 => {
            let Some(t87) = c.ld32(c.r64(12).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x700084514c); };
            c.w32(2, t87);
            if !c.st32(c.r64(3).wrapping_add(c.r64(0)), c.r32(2)) { return RegfileExit::fault(0x7000845150); }
            let r88 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r88);
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_ne() { blk = 57; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 62; continue;
        }
        59 => {
            c.w64(2, c.r64(12).wrapping_add(0x7));
            c.w64(1, c.r64(3).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r89 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r89);
            let r90 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r90);
            c.logic(r90, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000845172); }
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000845177); };
            c.w64(0, t91);
            c.w64(10, 0x7000845185u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000845185); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000845189); }
            return c.call(c.r64(0), 0x700084518f, 0x700084518d);
        }
        60 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700084518f); }
            let Some(t92) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000845197); };
            c.w64(4, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084519c); };
            c.w64(9, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008451a0); };
            c.w64(15, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008451a4); };
            c.w64(3, t95);
            blk = 62; continue;
        }
        61 => {
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008451aa); };
            c.w64(3, t96);
            blk = 62; continue;
        }
        62 => {
            c.w64(12, c.r64(3));
            blk = 64; continue;
        }
        63 => {
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008451b6); };
            c.w64(12, t97);
            blk = 64; continue;
        }
        64 => {
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008451ba); };
            c.w64(11, t98);
            let r99 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r99);
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008451c1); };
            c.w64(15, t100);
            let Some(t101) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008451c8); };
            c.w64(0, t101);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008451cf); };
            c.w64(2, t102);
            let Some(t103) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008451d6); };
            c.cmp(t103, c.r64(3), 64);
            if c.cc_be() { blk = 78; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x70008451df); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            blk = 66; continue;
        }
        66 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008451e6); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008451ea); }
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008451ee); };
            c.w64(9, t104);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008451f5); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008451f9); }
            c.w64(0, c.r64(15));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000845203); }
            return c.ret(0x0, 0x7000845204);
        }
        67 => {
            blk = 86; continue;
        }
        68 => {
            let Some(t105) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084520a); };
            c.logic(t105 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 69; continue;
        }
        69 => {
            blk = 44; continue;
        }
        70 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084521c); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 44; continue; }
            blk = 71; continue;
        }
        71 => {
            blk = 22; continue;
        }
        72 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000845234); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000845238); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x7000845244, 0x700084523f);
        }
        73 => {
            c.w64(2, c.r64(0));
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000845247); };
            c.w64(15, t106);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084524b); };
            c.w64(12, t107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084524f); };
            c.w64(0, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000845253); };
            c.w64(9, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000845257); };
            c.w64(3, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700084525b); };
            c.w64(11, t111);
            blk = 32; continue;
        }
        74 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000845264); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000845268); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700084526c); }
            return c.call(0x70007febb4, 0x7000845275, 0x7000845270);
        }
        75 => {
            c.w64(3, c.r64(0));
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000845278); };
            c.w64(9, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084527c); };
            c.w64(15, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000845280); };
            c.w64(12, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000845284); };
            c.w64(8, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000845288); };
            c.w64(11, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084528c); };
            c.w64(0, t117);
            blk = 54; continue;
        }
        76 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000845295); };
            c.w64(3, t118);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008452a5, 0x70008452a0);
        }
        77 => {
            return RegfileExit::fault(0x70008452a5);
        }
        78 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008452ab); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008452af); }
            return c.call(0x70007febf4, 0x70008452b8, 0x70008452b3);
        }
        79 => {
            c.w64(15, c.r64(0));
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008452bb); };
            c.w64(12, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008452bf); };
            c.w64(9, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008452c3); };
            c.w64(11, t121);
            blk = 66; continue;
        }
        80 => {
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008452cc); };
            c.w64(3, t122);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008452dc, 0x70008452d7);
        }
        81 => {
            return RegfileExit::fault(0x70008452dc);
        }
        82 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008452dd); }
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008452de); };
            c.w64(6, t123);
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0x2200)) else { return RegfileExit::fault(0x70008452e2); };
            c.w64(3, t124);
            c.w32(0, 0x1u64);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x70008452f6, 0x70008452f1);
        }
        83 => {
            return RegfileExit::fault(0x70008452f6);
        }
        84 => {
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x70008452f7); };
            c.w64(3, t125);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000845300); };
            c.w64(6, t126);
            return c.call(0x700086ed74, 0x7000845309, 0x7000845304);
        }
        85 => {
            return RegfileExit::fault(0x7000845309);
        }
        86 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084530a); }
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x21f0)) else { return RegfileExit::fault(0x700084530b); };
            c.w64(3, t127);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700084531c, 0x7000845317);
        }
        87 => {
            return RegfileExit::fault(0x700084531c);
        }
        88 => {
            c.w32(0, c.r32(0));
            let r128 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r128);
            c.w64(3, 0xfffffffffffc0000u64);
            let r129 = c.r64(3) & c.r64(0);
            c.w64(3, r129);
            c.logic(r129, 64);
            let Some(t130) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700084532c); };
            c.logic(t130 & 0x2u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w64(2, c.r64(12).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084533a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700084533b); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x7000845348, 0x7000845343);
        }
        90 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000845348); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700084534a); }
            blk = 9; continue;
        }
        91 => {
            return RegfileExit::jump(0x7000845354);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000845374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000845374); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000845378); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x700084537e); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 78; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084538b); };
            c.w32(9, t1);
            let r2 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000845392); };
            c.logic(t3 & 0x2u64, 8);
            if c.cc_e() { blk = 78; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084539d); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(3));
            return c.call(0x70007f2274, 0x70008453a8, 0x70008453a3);
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008453a8); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 76; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008453b4); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008453bb); };
            c.cmp(t6, 0xbau64, 16);
            if c.cc_b() { blk = 76; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x70008453c8); };
            c.w64(3, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008453cf); };
            c.w64(6, t8);
            return c.call(0x7000870914, 0x70008453d8, 0x70008453d3);
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008453d8); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008453de); };
            c.w64(9, t9);
            c.w64(12, c.r64(6));
            c.w64(15, c.r64(6));
            c.w64(11, c.r64(9));
            blk = 10; continue;
        }
        7 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008453f0); };
            c.w64(12, t10);
            c.w64(9, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            let r11 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r11);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x7000845400); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r12 = c.r64(3) & c.r64(11);
            c.w64(3, r12);
            c.logic(r12, 64);
            let Some(t13) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000845412); };
            c.logic(t13 & 0x4u64, 8);
            if c.cc_ne() { blk = 80; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700084541c); };
            c.w64(0, t14);
            blk = 10; continue;
        }
        10 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000845420); };
            if !c.push64(t15) { return RegfileExit::fault(0x7000845420); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x7000845423); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x7000845427); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x700084542b); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000845434); };
            c.w64(6, t16);
            return c.call(0x70007f21d4, 0x700084543d, 0x7000845438);
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700084543d); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t17) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845449); };
            c.w32(9, t17);
            let r18 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r18);
            let Some(t19) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000845450); };
            c.cmp(t19, 0xbau64, 16);
            if c.cc_b() { blk = 74; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x700084545d); };
            c.w64(3, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000845464); };
            c.w64(6, t21);
            return c.call(0x7000870914, 0x700084546d, 0x7000845468);
        }
        14 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700084546d); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w32(9, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700084547d); };
            c.cmp(t23, c.r32(0), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t24) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845486); };
            c.w32(9, t24);
            let r25 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r25);
            let Some(t26) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700084548d); };
            c.logic(t26 & 0x10u64, 8);
            if c.cc_ne() { blk = 22; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000845494); };
            c.cmp(t27, c.r32(9), 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t28) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008454a1); };
            c.cmp(t28, 0x41u64, 16);
            if c.cc_e() { blk = 62; continue; }
            blk = 20; continue;
        }
        20 => {
            blk = 44; continue;
        }
        21 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x70008454bc); };
            c.w64(3, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008454c3); };
            c.w64(0, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008454c7); };
            c.w64(6, t31);
            return c.call(0x7000870914, 0x70008454d0, 0x70008454cb);
        }
        23 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008454d0); };
            c.w64(15, t32);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008454d4); }
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008454d8); };
            c.cmp(c.r64(15), t33, 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008454de); };
            c.w64(11, t34);
            blk = 7; continue;
        }
        25 => {
            c.w64(9, c.r64(15));
            if let Some(sr) = c.shr(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r35 = c.add(c.r64(9), c.r64(15), 64);
            c.w64(9, r35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008454f0); };
            c.w64(11, t36);
            let Some(t37) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008454f4); };
            c.w32(12, t37);
            let r38 = c.add(c.r64(9), 0x10u64, 64);
            c.w64(9, r38);
            if c.cc_e() { blk = 42; continue; }
            blk = 26; continue;
        }
        26 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000845502); }
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000845506); };
            c.cmp(t39, c.r32(12), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700084550f); };
            c.w64(12, t40);
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r64(9), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 72; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000845523); };
            c.w64(15, t41);
            c.w64(3, c.r64(15).wrapping_mul(4).wrapping_add(0x7));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084552f); };
            c.w64(2, t42);
            let Some(t43) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000845536); };
            c.w64(2, t43);
            c.w64(1, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.w64(6, c.r64(1).wrapping_add(c.r64(2)));
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000845545); };
            c.w64(7, t44);
            let Some(t45) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700084554c); };
            c.cmp(t45, c.r64(6), 64);
            if c.cc_be() { blk = 66; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 66; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000845562); };
            c.w64(1, t46);
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x7000845569); }
            let r47 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r47);
            blk = 32; continue;
        }
        32 => {
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000845574); }
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000845578); }
            c.w64(12, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000845583); }
            c.cmp(c.r64(12), c.r64(3), 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700084558c); };
            c.w64(1, t48);
            blk = 34; continue;
        }
        34 => {
            let r49 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r49);
            if !c.st32(c.r64(12).wrapping_add(c.r64(2)), c.r32(1)) { return RegfileExit::fault(0x7000845597); }
            c.cmp(c.r64(12), c.r64(3), 64);
            if c.cc_ne() { blk = 34; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(12, c.r64(2));
            let r50 = c.r64(12) & 0xfffffffffffc0000u64;
            c.w64(12, r50);
            c.logic(r50, 64);
            let Some(t51) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008455aa); };
            c.logic(t51 & 0x4u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_e() { blk = 43; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w32(12, 0x7u64);
            blk = 38; continue;
        }
        38 => {
            let Some(t52) = c.ld32(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008455c2); };
            c.w32(1, t52);
            if !c.st32(c.r64(2).wrapping_add(c.r64(12)), c.r32(1)) { return RegfileExit::fault(0x70008455c6); }
            let r53 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r53);
            c.cmp(c.r64(3), c.r64(12), 64);
            if c.cc_ne() { blk = 38; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 43; continue;
        }
        40 => {
            c.w64(8, c.r64(15).wrapping_mul(4));
            let r54 = c.add(c.r64(11), 0x7u64, 64);
            c.w64(11, r54);
            c.w64(1, c.r64(2).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r55 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r55);
            let r56 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r56);
            c.logic(r56, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008455f0); }
            c.w64(2, c.r64(11));
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x70008455f8); };
            c.w64(0, t57);
            c.w64(10, 0x7000845606u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000845606); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700084560a); }
            return c.call(c.r64(0), 0x7000845610, 0x700084560e);
        }
        41 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000845610); }
            let Some(t58) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000845618); };
            c.w64(4, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084561d); };
            c.w64(15, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000845621); };
            c.w64(0, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000845625); };
            c.w64(9, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000845629); };
            c.w64(2, t62);
            blk = 43; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700084562f); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000845633); };
            c.w64(2, t63);
            blk = 43; continue;
        }
        43 => {
            c.w64(11, c.r64(2));
            c.w64(15, c.r64(9));
            blk = 7; continue;
        }
        44 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000845645); };
            c.w64(11, t64);
            let Some(t65) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845649); };
            c.w32(9, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084564d); };
            c.cmp(t66, 0x0u64, 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t67) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000845658); };
            c.cmp(t67, c.r32(9), 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000845661); };
            c.w64(9, t68);
            blk = 47; continue;
        }
        47 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000845668); };
            c.cmp(t69, 0x7fffffdu64, 64);
            if c.cc_g() { blk = 70; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000845676); };
            c.w64(12, t70);
            c.w64(8, c.r64(12).wrapping_mul(4));
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000845682); };
            c.w64(15, t71);
            let Some(t72) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000845689); };
            c.w64(15, t72);
            c.w64(2, c.r64(8).wrapping_add(0x8));
            c.w64(0, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000845694); };
            c.w64(3, t73);
            let Some(t74) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700084569b); };
            c.cmp(t74, c.r64(0), 64);
            if c.cc_be() { blk = 68; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 68; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008456b1); };
            c.w64(3, t75);
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x70008456b8); }
            let r76 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r76);
            blk = 51; continue;
        }
        51 => {
            let r77 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r77);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008456c2); }
            c.w64(9, c.r64(15));
            let r78 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r78);
            c.logic(r78, 64);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008456d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008456d4); }
            let Some(t79) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008456d8); };
            c.logic(t79 & 0x4u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(9, c.r64(8).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 58; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w32(3, 0x7u64);
            blk = 54; continue;
        }
        54 => {
            let Some(t80) = c.ld32(c.r64(11).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x70008456ee); };
            c.w32(12, t80);
            if !c.st32(c.r64(15).wrapping_add(c.r64(3)), c.r32(12)) { return RegfileExit::fault(0x70008456f2); }
            let r81 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r81);
            c.cmp(c.r64(9), c.r64(3), 64);
            if c.cc_ne() { blk = 54; continue; }
            blk = 55; continue;
        }
        55 => {
            blk = 58; continue;
        }
        56 => {
            c.w64(2, c.r64(11).wrapping_add(0x7));
            c.w64(1, c.r64(15).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r82 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r82);
            let r83 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r83);
            c.logic(r83, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000845714); }
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000845719); };
            c.w64(0, t84);
            c.w64(10, 0x7000845727u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000845727); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700084572b); }
            return c.call(c.r64(0), 0x7000845731, 0x700084572f);
        }
        57 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000845731); }
            let Some(t85) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000845739); };
            c.w64(4, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084573e); };
            c.w64(15, t86);
            blk = 58; continue;
        }
        58 => {
            c.w64(0, c.r64(15));
            blk = 59; continue;
        }
        59 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000845748); }
            return c.ret(0x0, 0x7000845749);
        }
        60 => {
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700084574a); };
            c.w64(0, t87);
            blk = 59; continue;
        }
        61 => {
            blk = 78; continue;
        }
        62 => {
            let Some(t88) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000845758); };
            c.logic(t88 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 63; continue;
        }
        63 => {
            blk = 44; continue;
        }
        64 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084576a); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 44; continue; }
            blk = 65; continue;
        }
        65 => {
            blk = 22; continue;
        }
        66 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000845782); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000845786); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x7000845792, 0x700084578d);
        }
        67 => {
            c.w64(2, c.r64(0));
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000845795); };
            c.w64(11, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000845799); };
            c.w64(15, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084579d); };
            c.w64(0, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008457a1); };
            c.w64(9, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008457a5); };
            c.w64(3, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008457a9); };
            c.w64(12, t94);
            blk = 32; continue;
        }
        68 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x70008457b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008457b6); }
            return c.call(0x70007febb4, 0x70008457bf, 0x70008457ba);
        }
        69 => {
            c.w64(15, c.r64(0));
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008457c2); };
            c.w64(8, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008457c6); };
            c.w64(11, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008457ca); };
            c.w64(9, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008457ce); };
            c.w64(12, t98);
            blk = 51; continue;
        }
        70 => {
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008457d7); };
            c.w64(3, t99);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008457e7, 0x70008457e2);
        }
        71 => {
            return RegfileExit::fault(0x70008457e7);
        }
        72 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008457e8); };
            c.w64(3, t100);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008457f8, 0x70008457f3);
        }
        73 => {
            return RegfileExit::fault(0x70008457f8);
        }
        74 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008457f9); }
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008457fa); };
            c.w64(6, t101);
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x2200)) else { return RegfileExit::fault(0x70008457fe); };
            c.w64(3, t102);
            c.w32(0, 0x1u64);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x7000845812, 0x700084580d);
        }
        75 => {
            return RegfileExit::fault(0x7000845812);
        }
        76 => {
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x7000845813); };
            c.w64(3, t103);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084581c); };
            c.w64(6, t104);
            return c.call(0x700086ed74, 0x7000845825, 0x7000845820);
        }
        77 => {
            return RegfileExit::fault(0x7000845825);
        }
        78 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000845826); }
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x21f0)) else { return RegfileExit::fault(0x7000845827); };
            c.w64(3, t105);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000845838, 0x7000845833);
        }
        79 => {
            return RegfileExit::fault(0x7000845838);
        }
        80 => {
            c.w32(0, c.r32(0));
            let r106 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r106);
            c.w64(3, 0xfffffffffffc0000u64);
            let r107 = c.r64(3) & c.r64(0);
            c.w64(3, r107);
            c.logic(r107, 64);
            let Some(t108) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000845848); };
            c.logic(t108 & 0x2u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w64(2, c.r64(11).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000845856); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000845857); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x7000845864, 0x700084585f);
        }
        82 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000845864); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000845866); }
            blk = 9; continue;
        }
        83 => {
            return RegfileExit::jump(0x7000845870);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000845874(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000845874); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000845878); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000845886); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x700084588a); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x7000845897); };
            c.w64(1, t1);
            c.w32(7, 0);
            c.logic(0, 32);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008458a0); };
            c.cmp(t2, c.r32(7), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008458a9); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008458af); };
            c.cmp(t5, 0x834u64, 16);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(8, 0x1u64);
            blk = 5; continue;
        }
        4 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 5; continue;
        }
        5 => {
            c.w32(7, 0x1u64);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld8(c.r64(1).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008458cd); };
            c.w32(8, t6);
            let r7 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r7);
            let r8 = c.r32(8) & 0x3fu64;
            c.w32(8, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(8), 0xbu64, 32);
            if c.cc_g() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008458e0); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008458e7); };
            c.w32(8, t11);
            let r12 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r12);
            let Some(t13) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008458ee); };
            c.w32(9, t13);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4cu64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008458fe);
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x137)) else { return RegfileExit::fault(0x70008458ff); };
            c.w32(8, t14);
            let Some(t15) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000845906); };
            c.cmp(t15, c.r32(8), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700084590c); };
            c.w64(1, t16);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t17) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000845916); };
            c.cmp(t17, c.r32(8), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(1, c.r64(7));
            blk = 15; continue;
        }
        12 => {
            c.w32(7, 0x1u64);
            blk = 14; continue;
        }
        13 => {
            c.w32(7, 0x1u64);
            blk = 14; continue;
        }
        14 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x7000845938); };
            c.w64(8, t18);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t19) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000845942); };
            c.cmp(t19, c.r32(9), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t20) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084594c); };
            c.w32(8, t20);
            let r21 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r21);
            let Some(t22) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000845953); };
            c.cmp(t22, 0x834u64, 16);
            if c.cc_ne() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(9, c.r64(7));
            blk = 20; continue;
        }
        19 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t23) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700084596a); };
            c.w32(9, t23);
            let r24 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r24);
            let r25 = c.r32(9) & 0x3fu64;
            c.w32(9, r25);
            c.logic(r25, 32);
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_a() { blk = 27; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t26) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084597d); };
            c.w32(9, t26);
            let r27 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r27);
            let Some(t28) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000845984); };
            c.w32(9, t28);
            let r29 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r29);
            let Some(t30) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084598b); };
            c.w32(11, t30);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4cu64, 64);
            if c.cc_a() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            return RegfileExit::fault(0x700084599b);
        }
        24 => {
            let Some(t31) = c.ld32(c.r64(9).wrapping_add(0x137)) else { return RegfileExit::fault(0x700084599c); };
            c.w32(9, t31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008459a3); };
            c.cmp(t32, c.r32(9), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70008459a9); };
            c.w64(8, t33);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008459b3); };
            c.cmp(t34, c.r32(9), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(8, c.r64(7));
            blk = 28; continue;
        }
        27 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        28 => {
            let r35 = c.r32(1) | c.r32(8);
            c.w32(1, r35);
            c.logic(r35, 32);
            if c.cc_ne() { blk = 69; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t36) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008459cf); };
            c.w32(1, t36);
            let r37 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r37);
            let Some(t38) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008459d5); };
            c.cmp(t38, 0x40u64, 16);
            if c.cc_b() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 37; continue;
        }
        32 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x1020)) else { return RegfileExit::fault(0x70008459e0); };
            c.w64(1, t39);
            c.w32(7, 0x2u64);
            let Some(t40) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008459ec); };
            c.cmp(t40, c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 37; continue;
        }
        34 => {
            let Some(t41) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008459f5); };
            c.w32(2, t41);
            c.cmp(c.r64(2), 0x2000000u64, 64);
            if c.cc_g() { blk = 67; continue; }
            blk = 35; continue;
        }
        35 => {
            return c.call(0x700089ceb4, 0x7000845a0a, 0x7000845a05);
        }
        36 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000845a0d); }
            return c.ret(0x0, 0x7000845a0e);
        }
        37 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t42) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845a18); };
            c.w32(1, t42);
            let r43 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r43);
            let Some(t44) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000845a1e); };
            c.w32(7, t44);
            c.cmp(c.r16(7), 0x825u64, 16);
            if c.cc_e() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r16(7), 0x827u64, 16);
            if c.cc_ne() { blk = 50; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(7, 0);
            c.logic(0, 32);
            let Some(t45) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000845a32); };
            c.cmp(t45, c.r32(7), 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x1008)) else { return RegfileExit::fault(0x7000845a37); };
            c.w64(7, t46);
            c.w32(8, 0x2u64);
            let Some(t47) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000845a44); };
            c.cmp(t47, c.r32(8), 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t48) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845a4a); };
            c.w32(7, t48);
            let r49 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r49);
            let Some(t50) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000845a50); };
            c.w32(7, t50);
            let r51 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r51);
            let Some(t52) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000845a56); };
            c.w32(8, t52);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x52u64, 64);
            if c.cc_a() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            return RegfileExit::fault(0x7000845a66);
        }
        44 => {
            let Some(t53) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000845a67); };
            c.w32(1, t53);
            let r54 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r54);
            let Some(t55) = c.ld32(c.r64(7).wrapping_add(0x14f)) else { return RegfileExit::fault(0x7000845a6d); };
            c.cmp(t55, c.r32(1), 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r64(8), 0x51u64, 64);
            if c.cc_a() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            return RegfileExit::fault(0x7000845a7b);
        }
        47 => {
            let Some(t56) = c.ld32(c.r64(7).wrapping_add(0x14b)) else { return RegfileExit::fault(0x7000845a7c); };
            c.w32(7, t56);
            let Some(t57) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845a82); };
            c.w32(1, t57);
            let r58 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r58);
            let Some(t59) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000845a88); };
            c.cmp(t59, c.r32(7), 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 48; continue;
        }
        48 => {
            return c.call(0x70008497f4, 0x7000845a92, 0x7000845a8d);
        }
        49 => {
            blk = 36; continue;
        }
        50 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 67; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t60) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845aa0); };
            c.w32(2, t60);
            let r61 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r61);
            let Some(t62) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000845aa6); };
            c.w32(1, t62);
            c.cmp(c.r16(1), 0x830u64, 16);
            if c.cc_e() { blk = 61; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r16(1), 0x829u64, 16);
            if c.cc_ne() { blk = 67; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t63) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000845abe); };
            c.cmp(t63, c.r32(1), 32);
            if c.cc_ne() { blk = 67; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t64) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845ac7); };
            c.w32(1, t64);
            let r65 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r65);
            let Some(t66) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000845acd); };
            c.w32(1, t66);
            let r67 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r67);
            let Some(t68) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000845ad3); };
            c.w32(7, t68);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x55u64, 64);
            if c.cc_a() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            return RegfileExit::fault(0x7000845ae2);
        }
        56 => {
            let Some(t69) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000845ae3); };
            c.w32(2, t69);
            let r70 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r70);
            let Some(t71) = c.ld32(c.r64(1).wrapping_add(0x15b)) else { return RegfileExit::fault(0x7000845ae9); };
            c.cmp(t71, c.r32(2), 32);
            if c.cc_ne() { blk = 67; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r64(7), 0x51u64, 64);
            if c.cc_a() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            return RegfileExit::fault(0x7000845af7);
        }
        59 => {
            let Some(t72) = c.ld32(c.r64(1).wrapping_add(0x14b)) else { return RegfileExit::fault(0x7000845af8); };
            c.w32(1, t72);
            let Some(t73) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845afe); };
            c.w32(2, t73);
            let r74 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r74);
            let Some(t75) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000845b04); };
            c.cmp(t75, c.r32(1), 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 60; continue;
        }
        60 => {
            blk = 67; continue;
        }
        61 => {
            let Some(t76) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845b0b); };
            c.w32(1, t76);
            let r77 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r77);
            let Some(t78) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000845b11); };
            c.w32(1, t78);
            let r79 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r79);
            let Some(t80) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000845b17); };
            c.w32(7, t80);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x56u64, 64);
            if c.cc_a() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            return RegfileExit::fault(0x7000845b26);
        }
        63 => {
            let Some(t81) = c.ld32(c.r64(1).wrapping_add(0x15f)) else { return RegfileExit::fault(0x7000845b27); };
            c.w32(1, t81);
            let Some(t82) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000845b2d); };
            c.cmp(t82, c.r32(1), 32);
            if c.cc_ne() { blk = 67; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x1018)) else { return RegfileExit::fault(0x7000845b32); };
            c.w64(2, t83);
            c.w32(1, 0x2u64);
            let Some(t84) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000845b3e); };
            c.cmp(t84, c.r32(1), 32);
            if c.cc_ne() { blk = 67; continue; }
            blk = 65; continue;
        }
        65 => {
            return c.call(0x700085c694, 0x7000845b48, 0x7000845b43);
        }
        66 => {
            blk = 36; continue;
        }
        67 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000845b4d); };
            c.w64(3, t85);
            return c.call(0x7000870914, 0x7000845b59, 0x7000845b54);
        }
        68 => {
            let Some(t86) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000845b59); };
            c.w64(5, t86);
            c.w64(3, c.r64(0));
            let Some(t87) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000845b60); };
            c.w64(0, t87);
            let Some(t88) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x7000845b65); };
            c.w64(6, t88);
            let r89 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r89);
            return RegfileExit::jump(0x7000844d94);
        }
        69 => {
            return c.call(0x7000827a94, 0x7000845b77, 0x7000845b72);
        }
        70 => {
            blk = 36; continue;
        }
        71 => {
            return RegfileExit::jump(0x7000845b80);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000845b94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000845b94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000845b98); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000845b9e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x7000845ba2); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000845ba6); };
            c.w64(3, t1);
            return c.call(0x7000870914, 0x7000845bb2, 0x7000845bad);
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000845bb2); };
            c.w64(5, t2);
            c.w64(3, c.r64(0));
            let Some(t3) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000845bb9); };
            c.w64(0, t3);
            let Some(t4) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x7000845bbe); };
            c.w64(6, t4);
            let r5 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r5);
            return RegfileExit::jump(0x7000845374);
        }
        2 => {
            return RegfileExit::jump(0x7000845bcc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000845bd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x7000845bdc); };
            c.w64(2, t0);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000845be5); };
            c.cmp(t1, c.r32(1), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845bea); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000845bf0); };
            c.cmp(t4, 0x834u64, 16);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(7, 0x1u64);
            blk = 5; continue;
        }
        4 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t5) = c.ld8(c.r64(2).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000845c06); };
            c.w32(7, t5);
            let r6 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r6);
            let r7 = c.r32(7) & 0x3fu64;
            c.w32(7, r7);
            c.logic(r7, 32);
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_a() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t8) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845c15); };
            c.w32(7, t8);
            let r9 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000845c1b); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000845c21); };
            c.w32(8, t12);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x4cu64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x7000845c31);
        }
        9 => {
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000845c32); };
            c.w32(7, t13);
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000845c38); };
            c.cmp(t14, c.r32(7), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000845c3d); };
            c.w64(2, t15);
            let Some(t16) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000845c44); };
            c.cmp(t16, c.r32(1), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::jump(0x70008276b4);
        }
        12 => {
            return RegfileExit::jump(0x7000844d94);
        }
        13 => {
            return RegfileExit::jump(0x7000845c54);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000845c74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000845c74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000845c78); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x7000845c7e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000845c82); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000845c86); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000845c8a); };
            c.w64(3, t1);
            c.w64(9, c.r64(3));
            return c.call(0x7000870914, 0x7000845c99, 0x7000845c94);
        }
        1 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845ca1); };
            c.w32(9, t2);
            let r3 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r3);
            let Some(t4) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000845ca8); };
            c.logic(t4 & 0x2u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000845cb3); };
            if !c.push64(t5) { return RegfileExit::fault(0x7000845cb3); }
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000845cb6); };
            c.w64(6, t6);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2274, 0x7000845cc7, 0x7000845cc2);
        }
        4 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000845cc7); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845cd3); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000845cda); };
            c.cmp(t9, 0xbau64, 16);
            if c.cc_b() { blk = 60; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000845ce7); };
            c.w64(3, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000845cee); };
            c.w64(6, t11);
            return c.call(0x7000870914, 0x7000845cf7, 0x7000845cf2);
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000845cf7); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000845cfd); };
            c.w64(9, t12);
            c.w64(12, c.r64(6));
            c.w64(15, c.r64(6));
            c.w64(11, c.r64(9));
            blk = 11; continue;
        }
        8 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000845d0f); };
            c.w64(12, t13);
            c.w64(9, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            let r14 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r14);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x7000845d1f); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r15 = c.r64(3) & c.r64(11);
            c.w64(3, r15);
            c.logic(r15, 64);
            let Some(t16) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000845d31); };
            c.logic(t16 & 0x4u64, 8);
            if c.cc_ne() { blk = 64; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000845d3b); };
            c.w64(0, t17);
            blk = 11; continue;
        }
        11 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000845d3f); };
            if !c.push64(t18) { return RegfileExit::fault(0x7000845d3f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x7000845d42); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000845d46); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x7000845d4a); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000845d53); };
            c.w64(6, t19);
            return c.call(0x70007f21d4, 0x7000845d5c, 0x7000845d57);
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000845d5c); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 58; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t20) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845d68); };
            c.w32(9, t20);
            let r21 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r21);
            let Some(t22) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000845d6f); };
            c.cmp(t22, 0xbau64, 16);
            if c.cc_b() { blk = 58; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x7000845d7c); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000845d83); };
            c.w64(6, t24);
            return c.call(0x7000870914, 0x7000845d8c, 0x7000845d87);
        }
        15 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000845d8c); };
            c.cmp(t25, c.r32(0), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w32(9, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000845d9c); };
            c.cmp(t26, c.r32(0), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t27) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845da5); };
            c.w32(9, t27);
            let r28 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r28);
            let Some(t29) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000845dac); };
            c.logic(t29 & 0x10u64, 8);
            if c.cc_ne() { blk = 23; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000845db3); };
            c.cmp(t30, c.r32(9), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t31) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000845dc0); };
            c.cmp(t31, 0x41u64, 16);
            if c.cc_e() { blk = 50; continue; }
            blk = 21; continue;
        }
        21 => {
            blk = 45; continue;
        }
        22 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x7000845ddb); };
            c.w64(3, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000845de2); };
            c.w64(0, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000845de6); };
            c.w64(6, t34);
            return c.call(0x7000870914, 0x7000845def, 0x7000845dea);
        }
        24 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000845def); };
            c.w64(15, t35);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000845df3); }
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000845df7); };
            c.cmp(c.r64(15), t36, 64);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000845dfd); };
            c.w64(11, t37);
            blk = 8; continue;
        }
        26 => {
            c.w64(9, c.r64(15));
            if let Some(sr) = c.shr(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r38 = c.add(c.r64(9), c.r64(15), 64);
            c.w64(9, r38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000845e0f); };
            c.w64(11, t39);
            let Some(t40) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000845e13); };
            c.w32(12, t40);
            let r41 = c.add(c.r64(9), 0x10u64, 64);
            c.w64(9, r41);
            if c.cc_e() { blk = 43; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000845e21); }
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000845e25); };
            c.cmp(t42, c.r32(12), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000845e2e); };
            c.w64(12, t43);
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r64(9), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 56; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000845e42); };
            c.w64(15, t44);
            let Some(t45) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000845e49); };
            c.w64(3, t45);
            c.w64(2, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.w64(1, c.r64(2).wrapping_add(c.r64(3)));
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000845e58); };
            c.w64(6, t46);
            let Some(t47) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000845e5f); };
            c.cmp(t47, c.r64(1), 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 54; continue; }
            blk = 32; continue;
        }
        32 => {
            if !c.st64(c.r64(15), c.r64(1)) { return RegfileExit::fault(0x7000845e75); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            blk = 33; continue;
        }
        33 => {
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000845e80); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000845e84); }
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000845e88); };
            c.w64(12, t48);
            c.w64(3, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            c.w64(2, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x7000845e9c); }
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_e() { blk = 36; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000845ea5); };
            c.w64(1, t49);
            blk = 35; continue;
        }
        35 => {
            let r50 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r50);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(1)) { return RegfileExit::fault(0x7000845eb0); }
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_ne() { blk = 35; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(2, c.r64(15));
            let r51 = c.r64(2) & 0xfffffffffffc0000u64;
            c.w64(2, r51);
            c.logic(r51, 64);
            let Some(t52) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000845ec3); };
            c.logic(t52 & 0x4u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_e() { blk = 44; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(2, 0x7u64);
            blk = 39; continue;
        }
        39 => {
            let Some(t53) = c.ld32(c.r64(11).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000845ed8); };
            c.w32(1, t53);
            if !c.st32(c.r64(15).wrapping_add(c.r64(2)), c.r32(1)) { return RegfileExit::fault(0x7000845edc); }
            let r54 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r54);
            c.cmp(c.r64(3), c.r64(2), 64);
            if c.cc_ne() { blk = 39; continue; }
            blk = 40; continue;
        }
        40 => {
            blk = 44; continue;
        }
        41 => {
            c.w64(8, c.r64(12).wrapping_mul(4));
            c.w64(2, c.r64(11).wrapping_add(0x7));
            c.w64(1, c.r64(15).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r55 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r55);
            let r56 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r56);
            c.logic(r56, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000845f06); }
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000845f0b); };
            c.w64(0, t57);
            c.w64(10, 0x7000845f19u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000845f19); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000845f1d); }
            return c.call(c.r64(0), 0x7000845f23, 0x7000845f21);
        }
        42 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000845f23); }
            let Some(t58) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000845f2b); };
            c.w64(4, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000845f30); };
            c.w64(12, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000845f34); };
            c.w64(0, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000845f38); };
            c.w64(9, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000845f3c); };
            c.w64(15, t62);
            blk = 44; continue;
        }
        43 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000845f42); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000845f46); };
            c.w64(15, t63);
            blk = 44; continue;
        }
        44 => {
            c.w64(11, c.r64(15));
            c.w64(15, c.r64(9));
            blk = 8; continue;
        }
        45 => {
            let Some(t64) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000845f58); };
            c.w64(9, (t64 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000845f5f); };
            c.w64(12, t65);
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(9, 0x2e2u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000845f6e); }
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000845f70); };
            c.w64(6, t66);
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000845f74); };
            c.w64(3, t67);
            c.w32(0, 0x1u64);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x7000845f88, 0x7000845f83);
        }
        47 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000845f8b); }
            return c.ret(0x0, 0x7000845f8c);
        }
        48 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000845f8d); };
            c.w64(0, t68);
            blk = 47; continue;
        }
        49 => {
            blk = 62; continue;
        }
        50 => {
            let Some(t69) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000845f95); };
            c.logic(t69 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 45; continue;
        }
        52 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000845fa4); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 45; continue; }
            blk = 53; continue;
        }
        53 => {
            blk = 23; continue;
        }
        54 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000845fb8); }
            return c.call(0x70007febb4, 0x7000845fc1, 0x7000845fbc);
        }
        55 => {
            c.w64(15, c.r64(0));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000845fc4); };
            c.w64(11, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000845fc8); };
            c.w64(0, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000845fcc); };
            c.w64(9, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000845fd0); };
            c.w64(12, t73);
            blk = 33; continue;
        }
        56 => {
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000845fd9); };
            c.w64(3, t74);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000845fe9, 0x7000845fe4);
        }
        57 => {
            return RegfileExit::fault(0x7000845fe9);
        }
        58 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000845fea); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2200)) else { return RegfileExit::fault(0x7000845feb); };
            c.w64(3, t75);
            c.w32(0, 0x1u64);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000845ff7); };
            c.w64(6, t76);
            return c.call(0x700086ed74, 0x7000846000, 0x7000845ffb);
        }
        59 => {
            return RegfileExit::fault(0x7000846000);
        }
        60 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x7000846001); };
            c.w64(3, t77);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084600a); };
            c.w64(6, t78);
            return c.call(0x700086ed74, 0x7000846013, 0x700084600e);
        }
        61 => {
            return RegfileExit::fault(0x7000846013);
        }
        62 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000846014); };
            if !c.push64(t79) { return RegfileExit::fault(0x7000846014); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x21f0)) else { return RegfileExit::fault(0x7000846017); };
            c.w64(3, t80);
            c.w32(0, 0x1u64);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000846023); };
            c.w64(6, t81);
            return c.call(0x700086ed74, 0x700084602c, 0x7000846027);
        }
        63 => {
            return RegfileExit::fault(0x700084602c);
        }
        64 => {
            c.w32(0, c.r32(0));
            let r82 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r82);
            c.w64(3, 0xfffffffffffc0000u64);
            let r83 = c.r64(3) & c.r64(0);
            c.w64(3, r83);
            c.logic(r83, 64);
            let Some(t84) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700084603c); };
            c.logic(t84 & 0x2u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(2, c.r64(11).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700084604a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700084604b); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x7000846058, 0x7000846053);
        }
        66 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000846058); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700084605a); }
            blk = 10; continue;
        }
        67 => {
            return RegfileExit::jump(0x7000846064);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000846074(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000846074); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000846078); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x700084607e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x7000846082); }
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000846086); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000846093); };
            c.w64(3, t2);
            return c.call(0x7000870914, 0x700084609f, 0x700084609a);
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 94; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008460a7); };
            c.w32(9, t3);
            let r4 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r4);
            let Some(t5) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008460ae); };
            c.logic(t5 & 0x2u64, 8);
            if c.cc_e() { blk = 94; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008460b9); };
            if !c.push64(t6) { return RegfileExit::fault(0x70008460b9); }
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008460bc); };
            c.w64(6, t7);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2274, 0x70008460cd, 0x70008460c8);
        }
        5 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008460cd); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 92; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008460d9); };
            c.w32(9, t8);
            let r9 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008460e0); };
            c.cmp(t10, 0xbau64, 16);
            if c.cc_b() { blk = 92; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x70008460ed); };
            c.w64(3, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008460f4); };
            c.w64(6, t12);
            return c.call(0x7000870914, 0x70008460fd, 0x70008460f8);
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008460fd); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000846101); };
            c.w64(9, t13);
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(11));
            blk = 12; continue;
        }
        9 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000846110); };
            c.w64(15, t14);
            c.w64(12, c.r64(15).wrapping_mul(4).wrapping_add(0x7));
            let r15 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r15);
            if !c.st32(c.r64(9).wrapping_add(c.r64(12)), c.r32(0)) { return RegfileExit::fault(0x7000846120); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r16 = c.r64(3) & c.r64(9);
            c.w64(3, r16);
            c.logic(r16, 64);
            let Some(t17) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000846132); };
            c.logic(t17 & 0x4u64, 8);
            if c.cc_ne() { blk = 102; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700084613c); };
            c.w64(0, t18);
            c.w64(12, c.r64(15));
            blk = 12; continue;
        }
        12 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000846143); };
            if !c.push64(t19) { return RegfileExit::fault(0x7000846143); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000846146); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700084614a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700084614e); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000846157); };
            c.w64(6, t20);
            return c.call(0x70007f21d4, 0x7000846160, 0x700084615b);
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000846160); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084616c); };
            c.w32(9, t21);
            let r22 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r22);
            let Some(t23) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000846173); };
            c.cmp(t23, 0xbau64, 16);
            if c.cc_b() { blk = 90; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x7000846180); };
            c.w64(3, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000846187); };
            c.w64(6, t25);
            return c.call(0x7000870914, 0x7000846190, 0x700084618b);
        }
        16 => {
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000846190); };
            c.cmp(t26, c.r32(0), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(9, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008461a0); };
            c.cmp(t27, c.r32(0), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008461a9); };
            c.w32(9, t28);
            let r29 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r29);
            let Some(t30) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008461b0); };
            c.logic(t30 & 0x10u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008461b7); };
            c.cmp(t31, c.r32(9), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t32) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008461c4); };
            c.cmp(t32, 0x41u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 49; continue;
        }
        23 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x70008461df); };
            c.w64(3, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008461e6); };
            c.w64(0, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008461ea); };
            c.w64(6, t35);
            return c.call(0x7000870914, 0x70008461f3, 0x70008461ee);
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008461f3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 81; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t36) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008461ff); };
            c.w32(9, t36);
            let r37 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r37);
            let Some(t38) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000846206); };
            c.cmp(t38, 0x40u64, 16);
            if c.cc_ae() { blk = 81; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000846212); };
            c.w64(11, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000846216); };
            c.cmp(c.r64(11), t40, 64);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084621c); };
            c.w64(9, t41);
            blk = 9; continue;
        }
        29 => {
            c.w64(9, c.r64(11));
            if let Some(sr) = c.shr(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r42 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084622e); };
            c.w64(12, t43);
            let Some(t44) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000846232); };
            c.w32(11, t44);
            let r45 = c.add(c.r64(9), 0x10u64, 64);
            c.w64(9, r45);
            if c.cc_e() { blk = 46; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000846241); }
            let Some(t46) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000846245); };
            c.cmp(t46, c.r32(11), 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700084624e); };
            c.w64(11, t47);
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r64(9), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 79; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000846262); };
            c.w64(15, t48);
            let Some(t49) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000846269); };
            c.w64(15, t49);
            c.w64(2, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.w64(3, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000846278); };
            c.w64(1, t50);
            let Some(t51) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700084627f); };
            c.cmp(t51, c.r64(3), 64);
            if c.cc_be() { blk = 77; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 77; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000846295); };
            c.w64(2, t52);
            if !c.st64(c.r64(2), c.r64(3)) { return RegfileExit::fault(0x700084629c); }
            let r53 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r53);
            blk = 36; continue;
        }
        36 => {
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008462a7); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008462ab); }
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008462af); };
            c.w64(11, t54);
            c.w64(3, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.w64(2, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008462c3); }
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_e() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008462cc); };
            c.w64(1, t55);
            blk = 38; continue;
        }
        38 => {
            let r56 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r56);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(1)) { return RegfileExit::fault(0x70008462d7); }
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_ne() { blk = 38; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(2, c.r64(15));
            let r57 = c.r64(2) & 0xfffffffffffc0000u64;
            c.w64(2, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008462ea); };
            c.logic(t58 & 0x4u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_e() { blk = 47; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w32(2, 0x7u64);
            blk = 42; continue;
        }
        42 => {
            let Some(t59) = c.ld32(c.r64(12).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008462ff); };
            c.w32(1, t59);
            if !c.st32(c.r64(15).wrapping_add(c.r64(2)), c.r32(1)) { return RegfileExit::fault(0x7000846303); }
            let r60 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r60);
            c.cmp(c.r64(3), c.r64(2), 64);
            if c.cc_ne() { blk = 42; continue; }
            blk = 43; continue;
        }
        43 => {
            blk = 47; continue;
        }
        44 => {
            c.w64(8, c.r64(11).wrapping_mul(4));
            c.w64(2, c.r64(12).wrapping_add(0x7));
            c.w64(1, c.r64(15).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r61 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r61);
            let r62 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r62);
            c.logic(r62, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700084632e); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000846333); };
            c.w64(0, t63);
            c.w64(10, 0x7000846341u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000846341); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000846345); }
            return c.call(c.r64(0), 0x700084634b, 0x7000846349);
        }
        45 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700084634b); }
            let Some(t64) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000846353); };
            c.w64(4, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000846358); };
            c.w64(11, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084635c); };
            c.w64(0, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000846360); };
            c.w64(9, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000846364); };
            c.w64(15, t68);
            blk = 47; continue;
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700084636a); }
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700084636e); };
            c.w64(11, t69);
            c.w64(15, c.r64(11));
            blk = 47; continue;
        }
        47 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(15));
            blk = 9; continue;
        }
        48 => {
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000846383); };
            c.w64(9, t70);
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700084638d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x7000846391); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000846395); }
            blk = 49; continue;
        }
        49 => {
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000846399); };
            c.w64(9, t71);
            let Some(t72) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084639d); };
            c.w32(9, t72);
            let r73 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r73);
            let Some(t74) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008463a4); };
            c.w32(9, t74);
            let r75 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r75);
            let Some(t76) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008463ab); };
            c.w32(11, t76);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            return RegfileExit::fault(0x70008463bb);
        }
        51 => {
            let Some(t77) = c.ld32(c.r64(9).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008463bc); };
            c.w32(9, t77);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70008463c3); }
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008463c7); };
            c.w64(11, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008463cb); };
            c.w64(12, t79);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_e() { blk = 68; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008463d8); };
            c.w64(12, t80);
            let Some(t81) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008463dc); };
            c.w32(15, t81);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 66; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t82) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008463eb); };
            c.cmp(t82, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008463f4); };
            c.w64(15, t83);
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r64(11), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 98; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(8, c.r64(11).wrapping_mul(4));
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000846414); };
            c.w64(3, t84);
            let Some(t85) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700084641b); };
            c.w64(3, t85);
            c.w64(2, c.r64(8).wrapping_add(0x8));
            c.w64(1, c.r64(2).wrapping_add(c.r64(3)));
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000846426); };
            c.w64(6, t86);
            let Some(t87) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700084642d); };
            c.cmp(t87, c.r64(1), 64);
            if c.cc_be() { blk = 96; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 96; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000846443); };
            c.w64(2, t88);
            if !c.st64(c.r64(2), c.r64(1)) { return RegfileExit::fault(0x700084644a); }
            let r89 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r89);
            blk = 59; continue;
        }
        59 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000846451); }
            c.w64(15, c.r64(3));
            let r90 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r90);
            c.logic(r90, 64);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700084645f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000846462); }
            let Some(t91) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000846466); };
            c.logic(t91 & 0x4u64, 8);
            if c.cc_e() { blk = 64; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(15, c.r64(8).wrapping_add(0x7));
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_e() { blk = 67; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w32(0, 0x7u64);
            blk = 62; continue;
        }
        62 => {
            let Some(t92) = c.ld32(c.r64(12).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x700084647c); };
            c.w32(2, t92);
            if !c.st32(c.r64(3).wrapping_add(c.r64(0)), c.r32(2)) { return RegfileExit::fault(0x7000846480); }
            let r93 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r93);
            c.cmp(c.r64(15), c.r64(0), 64);
            if c.cc_ne() { blk = 62; continue; }
            blk = 63; continue;
        }
        63 => {
            blk = 67; continue;
        }
        64 => {
            c.w64(2, c.r64(12).wrapping_add(0x7));
            c.w64(1, c.r64(3).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r94 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r94);
            let r95 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r95);
            c.logic(r95, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008464a2); }
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x70008464a7); };
            c.w64(0, t96);
            c.w64(10, 0x70008464b5u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008464b5); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008464b9); }
            return c.call(c.r64(0), 0x70008464bf, 0x70008464bd);
        }
        65 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008464bf); }
            let Some(t97) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008464c7); };
            c.w64(4, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008464cc); };
            c.w64(9, t98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008464d0); };
            c.w64(11, t99);
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008464d4); };
            c.w64(3, t100);
            blk = 67; continue;
        }
        66 => {
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008464da); };
            c.w64(3, t101);
            c.w64(12, c.r64(3));
            blk = 67; continue;
        }
        67 => {
            c.w64(12, c.r64(3));
            blk = 69; continue;
        }
        68 => {
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008464e9); };
            c.w64(12, t102);
            blk = 69; continue;
        }
        69 => {
            let r103 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r103);
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008464f0); };
            c.w64(15, t104);
            let Some(t105) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008464f7); };
            c.w64(0, t105);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008464fe); };
            c.w64(2, t106);
            let Some(t107) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000846505); };
            c.cmp(t107, c.r64(3), 64);
            if c.cc_be() { blk = 100; continue; }
            blk = 70; continue;
        }
        70 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x700084650e); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            blk = 71; continue;
        }
        71 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000846515); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x7000846519); }
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700084651d); };
            c.w64(9, t108);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x7000846524); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x7000846528); }
            c.w64(0, c.r64(15));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000846532); }
            return c.ret(0x0, 0x7000846533);
        }
        72 => {
            blk = 94; continue;
        }
        73 => {
            let Some(t109) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000846539); };
            c.logic(t109 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 74; continue;
        }
        74 => {
            blk = 49; continue;
        }
        75 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700084654b); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 49; continue; }
            blk = 76; continue;
        }
        76 => {
            blk = 24; continue;
        }
        77 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000846563); }
            return c.call(0x70007febb4, 0x700084656c, 0x7000846567);
        }
        78 => {
            c.w64(15, c.r64(0));
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084656f); };
            c.w64(12, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000846573); };
            c.w64(0, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000846577); };
            c.w64(9, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700084657b); };
            c.w64(11, t113);
            blk = 36; continue;
        }
        79 => {
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000846584); };
            c.w64(3, t114);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000846594, 0x700084658f);
        }
        80 => {
            return RegfileExit::fault(0x7000846594);
        }
        81 => {
            c.w32(9, 0x8cu64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700084659b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700084659d); }
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700084659e); };
            c.w64(3, t115);
            c.w32(0, 0x2u64);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008465aa); };
            c.w64(6, t116);
            return c.call(0x700086ed74, 0x70008465b3, 0x70008465ae);
        }
        82 => {
            blk = 89; continue;
        }
        83 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008465b5); }
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x9e8)) else { return RegfileExit::fault(0x70008465b9); };
            c.w64(3, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008465c0); };
            c.w64(0, t118);
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008465c4); };
            c.w64(6, t119);
            return c.call(0x7000870914, 0x70008465cd, 0x70008465c8);
        }
        84 => {
            let Some(t120) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008465cd); };
            c.cmp(t120, c.r32(0), 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t121) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008465d6); };
            c.cmp(t121, c.r32(0), 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008465df); };
            if !c.push64(t122) { return RegfileExit::fault(0x70008465df); }
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008465e2); };
            c.w64(6, t123);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2274, 0x70008465f3, 0x70008465ee);
        }
        87 => {
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008465f3); };
            if !c.push64(t124) { return RegfileExit::fault(0x70008465f3); }
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008465f6); };
            c.w64(6, t125);
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x70008465fa); };
            c.w64(3, t126);
            c.w32(0, 0x1u64);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x700084660e, 0x7000846609);
        }
        88 => {
            return RegfileExit::fault(0x700084660e);
        }
        89 => {
            return RegfileExit::fault(0x700084660f);
        }
        90 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000846610); }
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x2200)) else { return RegfileExit::fault(0x7000846611); };
            c.w64(3, t127);
            c.w32(0, 0x1u64);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700084661d); };
            c.w64(6, t128);
            return c.call(0x700086ed74, 0x7000846626, 0x7000846621);
        }
        91 => {
            return RegfileExit::fault(0x7000846626);
        }
        92 => {
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x7000846627); };
            c.w64(3, t129);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000846630); };
            c.w64(6, t130);
            return c.call(0x700086ed74, 0x7000846639, 0x7000846634);
        }
        93 => {
            return RegfileExit::fault(0x7000846639);
        }
        94 => {
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084663a); };
            if !c.push64(t131) { return RegfileExit::fault(0x700084663a); }
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x21f0)) else { return RegfileExit::fault(0x700084663d); };
            c.w64(3, t132);
            c.w32(0, 0x1u64);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000846649); };
            c.w64(6, t133);
            return c.call(0x700086ed74, 0x7000846652, 0x700084664d);
        }
        95 => {
            return RegfileExit::fault(0x7000846652);
        }
        96 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000846653); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x7000846657); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700084665b); }
            return c.call(0x70007febb4, 0x7000846664, 0x700084665f);
        }
        97 => {
            c.w64(3, c.r64(0));
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000846667); };
            c.w64(9, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084666b); };
            c.w64(11, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084666f); };
            c.w64(12, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000846673); };
            c.w64(8, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000846677); };
            c.w64(15, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084667b); };
            c.w64(0, t139);
            blk = 59; continue;
        }
        98 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000846684); };
            c.w64(3, t140);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000846694, 0x700084668f);
        }
        99 => {
            return RegfileExit::fault(0x7000846694);
        }
        100 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x700084669a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700084669e); }
            return c.call(0x70007febf4, 0x70008466a7, 0x70008466a2);
        }
        101 => {
            c.w64(15, c.r64(0));
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008466aa); };
            c.w64(12, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008466ae); };
            c.w64(9, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008466b2); };
            c.w64(11, t143);
            blk = 71; continue;
        }
        102 => {
            c.w32(0, c.r32(0));
            let r144 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r144);
            c.w64(3, 0xfffffffffffc0000u64);
            let r145 = c.r64(3) & c.r64(0);
            c.w64(3, r145);
            c.logic(r145, 64);
            let Some(t146) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008466ca); };
            c.logic(t146 & 0x2u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w64(2, c.r64(9).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008466d8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008466d9); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70008466e6, 0x70008466e1);
        }
        104 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008466e6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008466e8); }
            blk = 11; continue;
        }
        105 => {
            return RegfileExit::jump(0x70008466f0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008466f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008466f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008466f8); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008466fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x7000846702); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084670f); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000846715); };
            c.w32(8, t3);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_b() { blk = 46; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000846725); };
            c.cmp(t4, c.r32(7), 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 20; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(7, c.r16(8));
            c.cmp(c.r32(7), 0xbau64, 32);
            if c.cc_ge() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(10, c.r64(4));
            let r5 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r5);
            let r6 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r6);
            c.logic(r6, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000846754); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(3));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x700084675f); };
            c.w64(0, t7);
            c.w64(10, 0x700084676du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700084676d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000846771); }
            return c.call(c.r64(0), 0x7000846777, 0x7000846775);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000846777); }
            let Some(t8) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700084677f); };
            c.w64(4, t8);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084678a); };
            c.w64(2, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084678e); };
            c.w64(0, t10);
            blk = 15; continue;
        }
        7 => {
            c.w64(2, c.r64(3));
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000846797); };
            c.w32(7, t11);
            let r12 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r12);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008467a3); };
            c.w32(8, t13);
            let r14 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r14);
            let Some(t15) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008467aa); };
            c.w32(8, t15);
            c.cmp(c.r16(8), 0xb0u64, 16);
            if c.cc_e() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r16(8), 0x7bu64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 15; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008467c2); };
            c.w32(8, t16);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x4u64, 64);
            if c.cc_be() { blk = 70; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t17) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008467d6); };
            c.w32(7, t17);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            blk = 15; continue;
        }
        13 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008467e1); };
            c.w32(7, t18);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if let Some(sr) = c.shr(c.r64(7), 0xau64, 64) { c.w64(7, sr); }
            let r19 = c.r64(7) & 0xfffffu64;
            c.w64(7, r19);
            c.logic(r19, 64);
            blk = 15; continue;
        }
        14 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            blk = 15; continue;
        }
        15 => {
            c.w64(8, c.r64(0));
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000846800); };
            c.w32(9, t20);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r21 = c.r64(11) & c.r64(7);
            c.w64(11, r21);
            c.logic(r21, 64);
            let Some(t22) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000846811); };
            c.w32(7, t22);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            blk = 17; continue;
        }
        16 => {
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700084681e); };
            c.w32(7, t23);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(7), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 18; continue;
        }
        18 => {
            let r24 = c.imul2(c.r64(7), 0x3u64, 64);
            c.w64(7, r24);
            let r25 = c.add(c.r64(7), c.r64(9), 64);
            c.w64(7, r25);
            c.w64(11, c.r64(7).wrapping_mul(4));
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000846842); };
            c.cmp(t26, c.r32(2), 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 19; continue;
        }
        19 => {
            blk = 16; continue;
        }
        20 => {
            c.w64(10, c.r64(4));
            let r27 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r27);
            let r28 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r28);
            c.logic(r28, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700084685a); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(3));
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x7000846865); };
            c.w64(0, t29);
            c.w64(10, 0x7000846873u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000846873); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000846877); }
            return c.call(c.r64(0), 0x700084687d, 0x700084687b);
        }
        21 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700084687d); }
            let Some(t30) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000846885); };
            c.w64(4, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700084688a); };
            c.w64(7, t31);
            let Some(t32) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084688e); };
            c.w32(8, t32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(8).wrapping_sub(0x1));
            let r33 = c.r64(11) & c.r64(9);
            c.w64(11, r33);
            c.logic(r33, 64);
            let Some(t34) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008468a5); };
            c.w32(9, t34);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008468b0); }
            blk = 23; continue;
        }
        22 => {
            c.w64(9, c.r64(11));
            let Some(t35) = c.ld32(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008468b9); };
            c.w32(9, t35);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r64(9), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 24; continue;
        }
        24 => {
            let r36 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(9, r36);
            let r37 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r37);
            c.w64(11, c.r64(9).wrapping_mul(4));
            let Some(t38) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008468dd); };
            c.w32(12, t38);
            let r39 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r39);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70008468e5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008468e9); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t40) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008468f3); };
            c.w32(15, t40);
            let r41 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r41);
            let Some(t42) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008468fb); };
            c.cmp(t42, 0x41u64, 16);
            if c.cc_ne() { blk = 22; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000846903); };
            if !c.push64(t43) { return RegfileExit::fault(0x7000846903); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000846906); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x7000846908); };
            c.w64(3, t44);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700084691b, 0x7000846916);
        }
        27 => {
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700084691b); };
            c.cmp(t45, c.r32(0), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000846924); };
            c.w64(7, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000846928); };
            c.w64(11, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084692c); };
            c.w64(8, t48);
            blk = 22; continue;
        }
        29 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000846932); };
            c.w64(7, t49);
            blk = 67; continue;
        }
        30 => {
            c.w64(10, c.r64(4));
            let r50 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r50);
            let r51 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r51);
            c.logic(r51, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000846946); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(3));
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x7000846951); };
            c.w64(0, t52);
            c.w64(10, 0x700084695fu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700084695f); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000846963); }
            return c.call(c.r64(0), 0x7000846969, 0x7000846967);
        }
        31 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000846969); }
            let Some(t53) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000846971); };
            c.w64(4, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000846976); };
            c.w64(7, t54);
            let Some(t55) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084697a); };
            c.w32(8, t55);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000846981); };
            c.w64(2, t56);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000846985); }
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(8).wrapping_sub(0x1));
            let r57 = c.r64(11) & c.r64(9);
            c.w64(11, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700084699a); };
            c.w32(9, t58);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 33; continue;
        }
        32 => {
            let Some(t59) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008469a7); };
            c.w32(9, t59);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r64(9), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 34; continue;
        }
        34 => {
            let r60 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(9, r60);
            let r61 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r61);
            c.w64(11, c.r64(9).wrapping_mul(4));
            let Some(t62) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008469cb); };
            c.w32(12, t62);
            let r63 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r63);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t64) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008469d9); };
            c.w32(15, t64);
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008469de); };
            c.cmp(t65, c.r32(15), 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008469e7); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            if c.cc_e() { blk = 45; continue; }
            blk = 38; continue;
        }
        38 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            if c.cc_e() { blk = 32; continue; }
            blk = 40; continue;
        }
        40 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 45; continue; }
            blk = 41; continue;
        }
        41 => {
            if c.cc_e() { blk = 32; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 45; continue;
        }
        43 => {
            let r66 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r66);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 32; continue; }
            blk = 44; continue;
        }
        44 => {
            if c.cc_ne() { blk = 32; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(7, c.r64(9));
            blk = 67; continue;
        }
        46 => {
            let Some(t67) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000846a24); };
            c.w32(7, t67);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            let r68 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r68);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 50; continue;
        }
        48 => {
            c.w64(10, c.r64(4));
            let r69 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r69);
            let r70 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r70);
            c.logic(r70, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000846a40); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(3));
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x7000846a4b); };
            c.w64(0, t71);
            c.w64(10, 0x7000846a59u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000846a59); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000846a5d); }
            return c.call(c.r64(0), 0x7000846a63, 0x7000846a61);
        }
        49 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000846a63); }
            let Some(t72) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000846a6b); };
            c.w64(4, t72);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000846a76); };
            c.w64(3, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000846a7a); };
            c.w64(0, t74);
            blk = 50; continue;
        }
        50 => {
            c.w64(8, c.r64(0));
            let Some(t75) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000846a81); };
            c.w32(9, t75);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r76 = c.r64(11) & c.r64(7);
            c.w64(11, r76);
            c.logic(r76, 64);
            let Some(t77) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000846a92); };
            c.w32(7, t77);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000846a9d); }
            blk = 52; continue;
        }
        51 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            let Some(t78) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000846aa9); };
            c.w32(8, t78);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(10, c.r64(7));
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(10));
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r64(7), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 53; continue;
        }
        53 => {
            let r79 = c.imul2(c.r64(7), 0x3u64, 64);
            c.w64(7, r79);
            let r80 = c.add(c.r64(7), c.r64(9), 64);
            c.w64(7, r80);
            c.w64(11, c.r64(7).wrapping_mul(4));
            let Some(t81) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000846ad6); };
            c.w32(0, t81);
            let r82 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r82);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000846ade); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000846ae2); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t83) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000846aea); };
            c.w32(12, t83);
            let r84 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r84);
            let Some(t85) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000846af1); };
            c.cmp(t85, 0x40u64, 16);
            if c.cc_ae() { blk = 51; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(3));
            return c.call(0x70007fb534, 0x7000846b04, 0x7000846aff);
        }
        56 => {
            let Some(t86) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000846b04); };
            c.cmp(t86, c.r32(0), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000846b0d); };
            c.w64(8, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000846b11); };
            c.w64(3, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000846b15); };
            c.w64(11, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000846b19); };
            c.w64(9, t90);
            blk = 51; continue;
        }
        58 => {
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000846b1f); };
            c.w64(7, t91);
            blk = 67; continue;
        }
        59 => {
            c.w64(7, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t92) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000846b2e); };
            c.w32(8, t92);
            c.w32(9, c.r32(7));
            let r93 = c.shl(c.r32(7), 0xfu64, 32).unwrap_or(c.r32(7));
            c.w32(7, r93);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let r94 = !c.r32(9);
            c.w32(9, r94);
            let r95 = c.add(c.r32(7), c.r32(9), 32);
            c.w32(7, r95);
            c.w64(9, c.r64(7));
            let r96 = c.shr(c.r32(9), 0xcu64, 32).unwrap_or(c.r32(9));
            c.w32(9, r96);
            let r97 = c.r32(9) ^ c.r32(7);
            c.w32(9, r97);
            c.logic(r97, 32);
            c.w32(7, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(4)));
            c.w64(9, c.r64(7));
            let r98 = c.shr(c.r32(9), 0x4u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r98);
            let r99 = c.r32(9) ^ c.r32(7);
            c.w32(9, r99);
            c.logic(r99, 32);
            let r100 = c.imul2(c.r32(9), 0x809u64, 32);
            c.w32(7, r100);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8).wrapping_sub(0x1));
            c.w64(11, c.r64(7));
            let r101 = c.shr(c.r32(11), 0x10u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r101);
            let r102 = c.r32(11) ^ c.r32(7);
            c.w32(11, r102);
            c.logic(r102, 32);
            let r103 = c.r32(11) & 0x3fffffffu64;
            c.w32(11, r103);
            c.logic(r103, 32);
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            let r104 = c.r64(9) & c.r64(7);
            c.w64(9, r104);
            c.logic(r104, 64);
            let Some(t105) = c.ld32(c.r64(0).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000846b7e); };
            c.w32(7, t105);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(3));
            blk = 61; continue;
        }
        60 => {
            let Some(t106) = c.ld32(c.r64(0).wrapping_add(c.r64(11)).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000846b8e); };
            c.w32(7, t106);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r64(7), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 62; continue;
        }
        62 => {
            let r107 = c.imul2(c.r64(7), 0x3u64, 64);
            c.w64(7, r107);
            let r108 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r108);
            c.w64(11, c.r64(7).wrapping_mul(4));
            let Some(t109) = c.ld32(c.r64(0).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000846bae); };
            c.w32(12, t109);
            let r110 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r110);
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 63; continue;
        }
        63 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t111) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000846bc1); };
            c.w32(15, t111);
            let Some(t112) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000846bc6); };
            c.cmp(t112, c.r32(15), 32);
            if c.cc_ne() { blk = 60; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(15, c.r64(9));
            let r113 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r113);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000846bd5); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(15) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 60; continue; }
            blk = 66; continue;
        }
        66 => {
            if c.cc_ne() { blk = 60; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            blk = 68; continue;
        }
        68 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000846bf3); }
            return c.ret(0x0, 0x7000846bf4);
        }
        69 => {
            c.w64(0, 0xfffffffffffffffeu64);
            blk = 68; continue;
        }
        70 => {
            return RegfileExit::fault(0x7000846bfe);
        }
        71 => {
            return RegfileExit::jump(0x7000846c00);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000846c14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000846c14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000846c18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000846c19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000846c1a); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000846c1f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000846c23); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 137; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(5));
            let Some(t2) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000846c39); };
            c.w64(1, t2);
            c.w64(8, c.r64(1));
            blk = 4; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000846c42); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000846c49); };
            c.w64(1, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000846c50); }
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000846c54); };
            c.cmp(t5, c.r32(2), 32);
            if c.cc_e() { blk = 282; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000846c61); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            let Some(t8) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000846c68); };
            c.w32(9, t8);
            let r9 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000846c6f); };
            c.w32(11, t10);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000846c79); }
            c.cmp(c.r64(11), 0x6fu64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x7000846c83);
        }
        7 => {
            let Some(t11) = c.ld32(c.r64(9).wrapping_add(0x1c3)) else { return RegfileExit::fault(0x7000846c84); };
            c.w32(11, t11);
            let r12 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r12);
            c.cmp(c.r32(11), c.r32(2), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000846c97); };
            c.w32(12, t13);
            let r14 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r14);
            let Some(t15) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000846c9e); };
            c.logic(t15 & 0xffffffffffffff80u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t16) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000846caa); };
            c.w32(12, t16);
            let r17 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r17);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000846cbb); };
            c.w32(15, t18);
            let r19 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r19);
            let Some(t20) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000846cc3); };
            c.cmp(t20, 0xacu64, 16);
            if c.cc_ne() { blk = 36; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t21) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000846cd0); };
            c.cmp(t21, c.r32(11), 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t22) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000846cdb); };
            c.logic(t22 & 0x200000u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000846ce6); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000846ced); };
            c.w64(15, t24);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000846cf6); };
            c.w64(11, t25);
            blk = 23; continue;
        }
        14 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000846d02); };
            c.w64(11, t26);
            let Some(t27) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000846d09); };
            c.w64(15, t27);
            c.w64(0, c.r64(15).wrapping_add(0x4c));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000846d10); };
            c.w64(3, t28);
            let Some(t29) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000846d17); };
            c.cmp(t29, c.r64(0), 64);
            if c.cc_be() { blk = 139; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x7000846d20); }
            let r30 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r30);
            blk = 16; continue;
        }
        16 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x7000846d27); };
            c.w64(0, t31);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000846d2e); }
            c.w32(0, 0x22u64);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x7000846d37); }
            let Some(t32) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000846d3b); };
            c.w32(0, t32);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_be() { blk = 141; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000846d51); }
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000846d55); };
            c.w32(2, t33);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_be() { blk = 142; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x7000846d69); }
            let Some(t34) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000846d6d); };
            c.w32(2, t34);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_be() { blk = 143; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, 0x8u64);
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x7000846d86); }
            let Some(t35) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000846d8a); };
            c.w32(2, t35);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x3u64, 64);
            if c.cc_be() { blk = 144; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(2, 0x2u64);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x7000846da3); }
            let Some(t36) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000846da7); };
            c.w32(2, t36);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x4u64, 64);
            if c.cc_be() { blk = 145; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(0)) { return RegfileExit::fault(0x7000846dbb); }
            c.w64(2, c.r64(15).wrapping_add(0x4b));
            c.w64(7, c.r64(15).wrapping_add(0x1b));
            c.cmp(c.r64(7), c.r64(2), 64);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.st32(c.r64(7), c.r32(1)) { return RegfileExit::fault(0x7000846dcc); }
            let r37 = c.add(c.r64(7), 0x4u64, 64);
            c.w64(7, r37);
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_ne() { blk = 22; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t38) = c.ld8(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000846dd7); };
            c.w32(2, t38);
            if let Some(sr) = c.shl(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let Some(t39) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000846de1); };
            c.w64(7, t39);
            c.w64(8, c.r64(2).wrapping_add(c.r64(7)));
            let Some(t40) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000846de8); };
            c.cmp(t40, c.r64(8), 64);
            if c.cc_be() { blk = 146; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(11), c.r64(8)) { return RegfileExit::fault(0x7000846df1); }
            c.w64(8, c.r64(7).wrapping_add(0x1));
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000846dfc); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x7000846e00); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000846e04); };
            c.w64(15, t41);
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000846e0b); }
            let Some(t42) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000846e0f); };
            c.w32(15, t42);
            c.w64(3, c.r64(15));
            let r43 = c.r32(3) & 0xe0000000u64;
            c.w32(3, r43);
            c.logic(r43, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 27; continue;
        }
        27 => {
            let r44 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r44);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x7000846e29); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 27; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 35; continue;
        }
        29 => {
            let r45 = c.sub(c.r32(15), 0x20000000u64, 32);
            c.w32(15, r45);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x7000846e3c); }
            let Some(t46) = c.ld8(c.r64(12).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000846e41); };
            c.w32(15, t46);
            c.w64(15, c.r64(15).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x7000846e54); };
            c.w64(2, t47);
            blk = 31; continue;
        }
        31 => {
            let r48 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r48);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(2)) { return RegfileExit::fault(0x7000846e5f); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ne() { blk = 31; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let r49 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r49);
            if !c.st32(c.r64(15).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x7000846e72); }
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_ne() { blk = 33; continue; }
            blk = 34; continue;
        }
        34 => {
            let r50 = c.add(c.r32(3), 0xe0000000u64, 32);
            c.w32(3, r50);
            if c.cc_e() { blk = 148; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(0, c.r64(8));
            blk = 37; continue;
        }
        36 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000846e8d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000846e8f); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2408)) else { return RegfileExit::fault(0x7000846e90); };
            c.w64(3, t51);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000846ea1, 0x7000846e9c);
        }
        37 => {
            c.w64(12, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000846ea4); };
            c.w64(6, t52);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000846ea8); };
            c.w64(1, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000846eaf); };
            c.w64(8, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000846eb3); };
            c.w64(9, t55);
            blk = 42; continue;
        }
        38 => {
            let Some(t56) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000846eb9); };
            c.w32(11, t56);
            let r57 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r57);
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000846ec0); };
            c.w64(12, t58);
            let Some(t59) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000846ec7); };
            c.w64(15, t59);
            let Some(t60) = c.ld8(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000846ecb); };
            c.w32(0, t60);
            c.w64(2, c.r64(0).wrapping_mul(4));
            c.w64(0, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000846edc); };
            c.w64(3, t61);
            let Some(t62) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000846ee3); };
            c.cmp(t62, c.r64(0), 64);
            if c.cc_be() { blk = 150; continue; }
            blk = 39; continue;
        }
        39 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x7000846eec); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 40; continue;
        }
        40 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000846ef4); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000846ef9); };
            c.w64(11, t63);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000846f00); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000846f05); }
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let r64 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r64);
            if !c.st32(c.r64(11).wrapping_add(c.r64(12)), c.r32(1)) { return RegfileExit::fault(0x7000846f18); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 41; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(12)) { return RegfileExit::fault(0x7000846f22); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x7000846f30); };
            c.w64(11, t65);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t66) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000846f39); };
            c.cmp(t66, c.r32(3), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(0, c.r64(8));
            let Some(t67) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000846f46); };
            c.w32(8, t67);
            let r68 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r68);
            let Some(t69) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000846f4d); };
            c.cmp(t69, 0x834u64, 16);
            if c.cc_ne() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(15, 0x1u64);
            blk = 47; continue;
        }
        46 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 47; continue;
        }
        47 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t70) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000846f6a); };
            c.w32(15, t70);
            let r71 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r71);
            let r72 = c.r32(15) & 0x3fu64;
            c.w32(15, r72);
            c.logic(r72, 32);
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_a() { blk = 56; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t73) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000846f7d); };
            c.w32(15, t73);
            let r74 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r74);
            let Some(t75) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000846f84); };
            c.w32(15, t75);
            let r76 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r76);
            let Some(t77) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000846f8b); };
            c.w32(2, t77);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x4cu64, 64);
            if c.cc_a() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            return RegfileExit::fault(0x7000846f9b);
        }
        51 => {
            let Some(t78) = c.ld32(c.r64(15).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000846f9c); };
            c.w32(15, t78);
            let Some(t79) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000846fa3); };
            c.cmp(t79, c.r32(15), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000846fa9); };
            c.w64(8, t80);
            let Some(t81) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000846fb0); };
            c.cmp(t81, c.r32(3), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w32(11, 0x1u64);
            blk = 56; continue;
        }
        54 => {
            c.w64(0, c.r64(8));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        55 => {
            c.w64(0, c.r64(8));
            c.w32(3, 0);
            c.logic(0, 32);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            let Some(t82) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000846fd2); };
            c.cmp(t82, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t83) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000846fe1); };
            c.cmp(t83, c.r32(0), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            let r84 = c.r32(8) | c.r32(15);
            c.w32(8, r84);
            c.logic(r84, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(8)) { return RegfileExit::fault(0x7000846ff3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000846ff7); }
            blk = 58; continue;
        }
        57 => {
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000847000); };
            c.w64(1, t85);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000847007); }
            blk = 58; continue;
        }
        58 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700084700b); };
            c.w64(9, t86);
            let Some(t87) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000847012); };
            c.w64(11, t87);
            c.w64(12, c.r64(11).wrapping_add(0x4c));
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x230)) else { return RegfileExit::fault(0x7000847019); };
            c.w64(7, t88);
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000847020); };
            c.w64(15, t89);
            c.w64(8, 0xfffffffffffffffeu64);
            c.w32(0, 0x4u64);
            c.w32(6, 0x22u64);
            let Some(t90) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000847038); };
            c.cmp(t90, c.r64(12), 64);
            if c.cc_be() { blk = 152; continue; }
            blk = 59; continue;
        }
        59 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000847041); }
            let r91 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r91);
            blk = 60; continue;
        }
        60 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000847048); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x700084704c); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000847050); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x7000847054); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x7000847058); }
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(8)) { return RegfileExit::fault(0x700084705c); }
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(8)) { return RegfileExit::fault(0x7000847060); }
            if !c.st32(c.r64(11).wrapping_add(0x1b), c.r32(1)) { return RegfileExit::fault(0x7000847064); }
            if !c.st32(c.r64(11).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x7000847068); }
            if !c.st32(c.r64(11).wrapping_add(0x23), c.r32(1)) { return RegfileExit::fault(0x700084706c); }
            if !c.st32(c.r64(11).wrapping_add(0x27), c.r32(1)) { return RegfileExit::fault(0x7000847070); }
            if !c.st32(c.r64(11).wrapping_add(0x2b), c.r32(1)) { return RegfileExit::fault(0x7000847074); }
            if !c.st32(c.r64(11).wrapping_add(0x2f), c.r32(1)) { return RegfileExit::fault(0x7000847078); }
            if !c.st32(c.r64(11).wrapping_add(0x33), c.r32(1)) { return RegfileExit::fault(0x700084707c); }
            if !c.st32(c.r64(11).wrapping_add(0x37), c.r32(1)) { return RegfileExit::fault(0x7000847080); }
            if !c.st32(c.r64(11).wrapping_add(0x3b), c.r32(1)) { return RegfileExit::fault(0x7000847084); }
            if !c.st32(c.r64(11).wrapping_add(0x3f), c.r32(1)) { return RegfileExit::fault(0x7000847088); }
            if !c.st32(c.r64(11).wrapping_add(0x43), c.r32(1)) { return RegfileExit::fault(0x700084708c); }
            if !c.st32(c.r64(11).wrapping_add(0x47), c.r32(1)) { return RegfileExit::fault(0x7000847090); }
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000847094); };
            c.w64(12, t92);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x7000847098); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r93 = c.r64(9) & c.r64(12);
            c.w64(9, r93);
            c.logic(r93, 64);
            let Some(t94) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008470ad); };
            c.logic(t94 & 0x4u64, 8);
            if c.cc_ne() { blk = 284; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t95) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008470b8); };
            c.cmp(t95, 0x0u64, 32);
            if c.cc_ne() { blk = 136; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008470c2); };
            c.w64(9, t96);
            let Some(t97) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008470c6); };
            c.w32(11, t97);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x53u64, 64);
            if c.cc_be() { blk = 119; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t98) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008470da); };
            c.w32(15, t98);
            let r99 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r99);
            let Some(t100) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008470e2); };
            c.w32(15, t100);
            let r101 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r101);
            let Some(t102) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008470e9); };
            c.w32(15, t102);
            let r103 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r103);
            let Some(t104) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008470f0); };
            c.w32(12, t104);
            let r105 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r105);
            let Some(t106) = c.ld32(c.r64(9).wrapping_add(0x153)) else { return RegfileExit::fault(0x70008470f7); };
            c.cmp(t106, c.r32(15), 32);
            if c.cc_ne() { blk = 158; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t107) = c.ld32(c.r64(12).wrapping_add(0x37)) else { return RegfileExit::fault(0x7000847104); };
            c.w32(12, t107);
            c.w32(12, c.r32(12));
            let r108 = c.r32(12) & 0x8u64;
            c.w32(12, r108);
            c.logic(r108, 32);
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_ne() { blk = 158; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t109) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700084711a); };
            c.cmp(t109, 0x0u64, 32);
            if c.cc_e() { blk = 158; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000847124); };
            c.w64(12, t110);
            let Some(t111) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000847128); };
            c.w32(15, t111);
            let r112 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r112);
            let Some(t113) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000847130); };
            c.w32(2, t113);
            let r114 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r114);
            let Some(t115) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000847138); };
            c.w32(15, t115);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700084713d); }
            c.cmp(c.r64(11), 0xe2u64, 64);
            if c.cc_be() { blk = 120; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t116) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700084714e); };
            c.w32(11, t116);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t117) = c.ld32(c.r64(9).wrapping_add(0x38f)) else { return RegfileExit::fault(0x7000847156); };
            c.w32(12, t117);
            let r118 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r118);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000847163); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000847167); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 136; continue; }
            blk = 69; continue;
        }
        69 => {
            let r119 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r119);
            let r120 = c.r32(15) & 0x3fu64;
            c.w32(15, r120);
            c.logic(r120, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_a() { blk = 121; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(8, c.r64(3));
            c.w64(12, c.r64(2));
            blk = 72; continue;
        }
        71 => {
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000847191); };
            c.w64(12, t121);
            blk = 72; continue;
        }
        72 => {
            let Some(t122) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000847195); };
            c.w32(9, t122);
            let r123 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084719d); };
            c.w64(6, t124);
            let Some(t125) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008471a1); };
            c.cmp(t125, c.r32(9), 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008471aa); };
            c.w64(9, t126);
            blk = 74; continue;
        }
        74 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 115; continue; }
            blk = 75; continue;
        }
        75 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008471bb); }
            let Some(t127) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008471bf); };
            c.w32(8, t127);
            let r128 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r128);
            let Some(t129) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008471c6); };
            c.cmp(t129, 0x834u64, 16);
            if c.cc_ne() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), 0x1u64) { return RegfileExit::fault(0x70008471cf); }
            blk = 78; continue;
        }
        77 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), 0x0u64) { return RegfileExit::fault(0x70008471d9); }
            blk = 78; continue;
        }
        78 => {
            let Some(t130) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008471e1); };
            c.cmp(t130, 0x0u64, 32);
            if c.cc_e() { blk = 114; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t131) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008471eb); };
            c.w32(11, t131);
            let r132 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r132);
            let r133 = c.r32(11) & 0x3fu64;
            c.w32(11, r133);
            c.logic(r133, 32);
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_a() { blk = 114; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t134) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000847202); };
            c.w32(11, t134);
            let r135 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r135);
            let Some(t136) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000847209); };
            c.w32(11, t136);
            let r137 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r137);
            let Some(t138) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000847210); };
            c.w32(12, t138);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x4cu64, 64);
            if c.cc_be() { blk = 127; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t139) = c.ld32(c.r64(11).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000847224); };
            c.w32(11, t139);
            let Some(t140) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084722b); };
            c.cmp(t140, c.r32(11), 32);
            if c.cc_ne() { blk = 114; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000847235); };
            c.w64(8, t141);
            let Some(t142) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700084723c); };
            c.cmp(t142, c.r32(3), 32);
            if c.cc_e() { blk = 114; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t143) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000847246); };
            c.w32(8, t143);
            let r144 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r144);
            let Some(t145) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084724d); };
            c.w32(11, t145);
            let r146 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r146);
            let Some(t147) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000847254); };
            c.w32(9, t147);
            let Some(t148) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000847258); };
            c.w32(12, t148);
            let r149 = c.sub(c.r32(12), 0x77u64, 32);
            c.w32(12, r149);
            c.cmp(c.r32(12), 0xcu64, 32);
            if c.cc_a() { blk = 94; continue; }
            blk = 84; continue;
        }
        84 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_g() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000847271); };
            c.w64(11, t150);
            blk = 89; continue;
        }
        86 => {
            let Some(t151) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084727a); };
            c.w32(11, t151);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 128; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t152) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084728e); };
            c.w32(11, t152);
            let r153 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r153);
            let Some(t154) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000847295); };
            c.cmp(t154, c.r32(11), 32);
            if c.cc_ne() { blk = 89; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700084729e); };
            c.w64(11, t155);
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_g() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t156) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008472ab); };
            c.w64(8, t156);
            blk = 111; continue;
        }
        91 => {
            let Some(t157) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008472b7); };
            c.w32(9, t157);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 129; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t158) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008472cb); };
            c.w32(8, t158);
            let r159 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r159);
            let Some(t160) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008472d2); };
            c.cmp(t160, c.r32(8), 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008472df); };
            c.w64(8, t161);
            blk = 111; continue;
        }
        94 => {
            let Some(t162) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70008472eb); };
            c.cmp(t162, c.r32(11), 32);
            if c.cc_ne() { blk = 133; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_g() { blk = 97; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t163) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008472fe); };
            c.w64(11, t163);
            blk = 103; continue;
        }
        97 => {
            let Some(t164) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000847307); };
            c.w32(11, t164);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 130; continue; }
            blk = 98; continue;
        }
        98 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x7)) { return RegfileExit::fault(0x700084731b); }
            c.w64(11, 0xfff7fffffff7ffffu64);
            let Some(t165) = c.ld64(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084732b); };
            c.cmp(t165, c.r64(11), 64);
            if c.cc_e() { blk = 102; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t166) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000847331); };
            c.w64(11, t166);
            let Some(t167) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000847338); };
            c.w64(12, t167);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t168) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000847340); };
            c.w64(0, t168);
            let Some(t169) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000847347); };
            c.cmp(t169, c.r64(15), 64);
            if c.cc_be() { blk = 154; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000847350); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 101; continue;
        }
        101 => {
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000847358); };
            c.w64(12, t170);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700084735f); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000847363); }
            blk = 103; continue;
        }
        102 => {
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700084736b); };
            c.w64(11, t171);
            blk = 103; continue;
        }
        103 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_g() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000847378); };
            c.w64(8, t172);
            blk = 111; continue;
        }
        105 => {
            let Some(t173) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000847381); };
            c.w32(9, t173);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 131; continue; }
            blk = 106; continue;
        }
        106 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0xf)) { return RegfileExit::fault(0x7000847395); }
            let Some(t174) = c.ld64(0x7000847323u64) else { return RegfileExit::fault(0x700084739b); };
            c.w64(9, t174);
            let Some(t175) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008473a2); };
            c.cmp(t175, c.r64(9), 64);
            if c.cc_e() { blk = 110; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t176) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008473a8); };
            c.w64(8, t176);
            let Some(t177) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008473af); };
            c.w64(9, t177);
            c.w64(12, c.r64(9).wrapping_add(0xc));
            let Some(t178) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008473b6); };
            c.w64(15, t178);
            let Some(t179) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008473bd); };
            c.cmp(t179, c.r64(12), 64);
            if c.cc_be() { blk = 156; continue; }
            blk = 108; continue;
        }
        108 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008473c6); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 109; continue;
        }
        109 => {
            let Some(t180) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008473cd); };
            c.w64(9, t180);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008473d4); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008473d8); }
            blk = 111; continue;
        }
        110 => {
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008473e0); };
            c.w64(8, t181);
            blk = 111; continue;
        }
        111 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008473e7); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008473e9); }
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008473eb); };
            if !c.push64(t182) { return RegfileExit::fault(0x70008473eb); }
            c.w32(0, 0x2u64);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008473f3); };
            c.w64(7, t183);
            return c.call(0x70007f2274, 0x70008473fc, 0x70008473f7);
        }
        112 => {
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008473fc); };
            c.w64(8, t184);
            let r185 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r185);
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000847404); };
            c.w64(1, t186);
            c.cmp(c.r64(1), c.r64(8), 64);
            if c.cc_ne() { blk = 71; continue; }
            blk = 113; continue;
        }
        113 => {
            blk = 136; continue;
        }
        114 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 116; continue;
        }
        115 => {
            c.w32(1, 0x1u64);
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 134; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t187) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000847428); };
            c.w32(1, t187);
            let r188 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r188);
            let Some(t189) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084742f); };
            c.w32(1, t189);
            let r190 = c.sub(c.r32(1), 0xbau64, 32);
            c.w32(1, r190);
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 134; continue; }
            blk = 118; continue;
        }
        118 => {
            blk = 57; continue;
        }
        119 => {
            return RegfileExit::fault(0x700084744a);
        }
        120 => {
            return RegfileExit::fault(0x700084744b);
        }
        121 => {
            let Some(t191) = c.ld64(0x7000847323u64) else { return RegfileExit::fault(0x700084744c); };
            c.w64(8, t191);
            c.w64(9, c.r64(2));
            let Some(t192) = c.ld64(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000847456); };
            c.cmp(t192, c.r64(8), 64);
            if c.cc_e() { blk = 125; continue; }
            blk = 122; continue;
        }
        122 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x7)) { return RegfileExit::fault(0x700084745c); }
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000847462); };
            c.w64(1, t193);
            let Some(t194) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000847469); };
            c.w64(8, t194);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000847470); };
            c.w64(11, t195);
            let Some(t196) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000847477); };
            c.cmp(t196, c.r64(9), 64);
            if c.cc_be() { blk = 280; continue; }
            blk = 123; continue;
        }
        123 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x7000847480); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 124; continue;
        }
        124 => {
            let Some(t197) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000847487); };
            c.w64(8, t197);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700084748e); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000847492); }
            blk = 125; continue;
        }
        125 => {
            c.w32(8, 0x92u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700084749d); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700084749f); }
            let Some(t198) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008474a0); };
            c.w64(3, t198);
            c.w32(0, 0x2u64);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008474ac); };
            c.w64(6, t199);
            return c.call(0x700086ed74, 0x70008474b5, 0x70008474b0);
        }
        126 => {
            return RegfileExit::fault(0x70008474b5);
        }
        127 => {
            return RegfileExit::fault(0x70008474b6);
        }
        128 => {
            return RegfileExit::fault(0x70008474b7);
        }
        129 => {
            return RegfileExit::fault(0x70008474b8);
        }
        130 => {
            return RegfileExit::fault(0x70008474b9);
        }
        131 => {
            return RegfileExit::fault(0x70008474ba);
        }
        132 => {
            blk = 136; continue;
        }
        133 => {
            return RegfileExit::fault(0x70008474bd);
        }
        134 => {
            c.w32(1, 0x92u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008474c3); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008474c4); }
            let Some(t200) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008474c6); };
            c.w64(3, t200);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008474d7, 0x70008474d2);
        }
        135 => {
            return RegfileExit::fault(0x70008474d7);
        }
        136 => {
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008474d8); };
            c.w64(0, t201);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008474df); }
            return c.ret(0x0, 0x70008474e0);
        }
        137 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008474e1); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008474e5); }
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008474e9); };
            c.w64(3, t202);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008474fa, 0x70008474f5);
        }
        138 => {
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008474fa); };
            c.w64(6, t203);
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008474fe); };
            c.w64(2, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000847502); };
            c.w64(0, t205);
            blk = 1; continue;
        }
        139 => {
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000847510); }
            return c.call(0x70007febf4, 0x7000847519, 0x7000847514);
        }
        140 => {
            c.w64(15, c.r64(0));
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084751c); };
            c.w64(6, t206);
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000847520); };
            c.w64(1, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000847527); };
            c.w64(8, t208);
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084752b); };
            c.w64(9, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700084752f); };
            c.w64(12, t210);
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000847533); };
            c.w64(11, t211);
            let Some(t212) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700084753a); };
            c.w64(3, t212);
            blk = 16; continue;
        }
        141 => {
            return RegfileExit::fault(0x7000847546);
        }
        142 => {
            return RegfileExit::fault(0x7000847547);
        }
        143 => {
            return RegfileExit::fault(0x7000847548);
        }
        144 => {
            return RegfileExit::fault(0x7000847549);
        }
        145 => {
            return RegfileExit::fault(0x700084754a);
        }
        146 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x700084754b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x700084754f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000847553); }
            return c.call(0x70007febf4, 0x700084755c, 0x7000847557);
        }
        147 => {
            c.w64(8, c.r64(0));
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084755f); };
            c.w64(6, t213);
            let Some(t214) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000847563); };
            c.w64(1, t214);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084756c); };
            c.w64(9, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000847570); };
            c.w64(12, t216);
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000847574); };
            c.w64(15, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000847578); };
            c.w64(2, t218);
            blk = 25; continue;
        }
        148 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000847581); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(8)) { return RegfileExit::fault(0x7000847583); }
            let Some(t219) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x7000847587); };
            c.w64(3, t219);
            c.w64(6, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700084759e, 0x7000847599);
        }
        149 => {
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084759e); };
            c.w64(6, t220);
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008475a2); };
            c.w64(1, t221);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008475ab); };
            c.w64(9, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008475af); };
            c.w64(8, t223);
            blk = 35; continue;
        }
        150 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008475b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008475bc); }
            return c.call(0x70007febf4, 0x70008475c5, 0x70008475c0);
        }
        151 => {
            c.w64(12, c.r64(0));
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008475c8); };
            c.w64(6, t224);
            let Some(t225) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008475cc); };
            c.w64(1, t225);
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008475d3); };
            c.w64(8, t226);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008475d7); };
            c.w64(9, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008475db); };
            c.w64(2, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008475df); };
            c.w64(11, t229);
            blk = 40; continue;
        }
        152 => {
            c.w32(2, 0x4cu64);
            return c.call(0x70007febb4, 0x70008475f2, 0x70008475ed);
        }
        153 => {
            c.w64(11, c.r64(0));
            let Some(t230) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008475f5); };
            c.w64(1, t230);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0x230)) else { return RegfileExit::fault(0x70008475fe); };
            c.w64(7, t231);
            c.w64(8, 0xfffffffffffffffeu64);
            c.w32(0, 0x4u64);
            c.w32(6, 0x22u64);
            blk = 60; continue;
        }
        154 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x7000847620); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x7000847624); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x7000847629); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x7000847635, 0x7000847630);
        }
        155 => {
            c.w64(11, c.r64(0));
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847638); };
            c.w64(6, t232);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700084763e); };
            c.w64(8, t233);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x7000847642); }
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000847647); };
            c.w64(9, t234);
            blk = 101; continue;
        }
        156 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x7000847655); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x7000847659); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x7000847666, 0x7000847661);
        }
        157 => {
            c.w64(8, c.r64(0));
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847669); };
            c.w64(6, t235);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700084766f); };
            c.w64(11, t236);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x7000847673); }
            blk = 109; continue;
        }
        158 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700084767d); };
            c.w64(6, t237);
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000847681); };
            c.w64(0, t238);
            let Some(t239) = c.ld64(c.r64(13).wrapping_add(0xa38)) else { return RegfileExit::fault(0x7000847685); };
            c.w64(3, t239);
            c.w64(8, c.r64(3));
            return c.call(0x7000870914, 0x7000847694, 0x700084768f);
        }
        159 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000847694); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 278; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t240) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008476a0); };
            c.w32(1, t240);
            let r241 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r241);
            let Some(t242) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008476a6); };
            c.logic(t242 & 0x2u64, 8);
            if c.cc_e() { blk = 278; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t243) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70008476b0); };
            c.w64(3, t243);
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008476b7); };
            c.w64(0, t244);
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008476bb); };
            c.w64(6, t245);
            return c.call(0x7000870914, 0x70008476c4, 0x70008476bf);
        }
        162 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 276; continue; }
            blk = 163; continue;
        }
        163 => {
            let Some(t246) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008476cc); };
            c.w32(1, t246);
            let r247 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r247);
            let Some(t248) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008476d2); };
            c.logic(t248 & 0x2u64, 8);
            if c.cc_e() { blk = 276; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008476dc); };
            if !c.push64(t249) { return RegfileExit::fault(0x70008476dc); }
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008476df); };
            c.w64(6, t250);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x70008476f0, 0x70008476eb);
        }
        165 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008476f0); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 274; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t251) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008476fc); };
            c.w32(1, t251);
            let r252 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r252);
            let Some(t253) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000847702); };
            c.cmp(t253, 0xbau64, 16);
            if c.cc_b() { blk = 274; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t254) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x700084770e); };
            c.w64(3, t254);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847715); };
            c.w64(6, t255);
            return c.call(0x7000870914, 0x700084771e, 0x7000847719);
        }
        168 => {
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700084771e); };
            c.w64(1, t256);
            let Some(t257) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000847722); };
            c.w32(8, t257);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x700084772c); }
            c.cmp(c.r64(8), 0x68u64, 64);
            if c.cc_a() { blk = 170; continue; }
            blk = 169; continue;
        }
        169 => {
            return RegfileExit::fault(0x7000847736);
        }
        170 => {
            let Some(t258) = c.ld32(c.r64(1).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x7000847737); };
            c.w32(1, t258);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x700084773d); }
            blk = 171; continue;
        }
        171 => {
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000847741); };
            if !c.push64(t259) { return RegfileExit::fault(0x7000847741); }
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847744); };
            c.w64(6, t260);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x70007f21d4, 0x7000847755, 0x7000847750);
        }
        172 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000847755); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 272; continue; }
            blk = 173; continue;
        }
        173 => {
            let Some(t261) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000847761); };
            c.w32(1, t261);
            let r262 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r262);
            let Some(t263) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000847767); };
            c.cmp(c.r32(1), t263, 32);
            if c.cc_e() { blk = 191; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t264) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000847770); };
            c.cmp(t264, 0xbau64, 16);
            if c.cc_b() { blk = 272; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t265) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x700084777c); };
            c.w64(3, t265);
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847783); };
            c.w64(6, t266);
            return c.call(0x7000870914, 0x700084778c, 0x7000847787);
        }
        176 => {
            let Some(t267) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700084778c); };
            c.cmp(t267, c.r32(0), 32);
            if c.cc_e() { blk = 190; continue; }
            blk = 177; continue;
        }
        177 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 188; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t268) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700084779f); };
            c.cmp(t268, c.r32(0), 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 179; continue;
        }
        179 => {
            let Some(t269) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008477a8); };
            c.w32(1, t269);
            let r270 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r270);
            let Some(t271) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008477ae); };
            c.logic(t271 & 0x10u64, 8);
            if c.cc_ne() { blk = 186; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t272) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008477b4); };
            c.cmp(t272, c.r32(1), 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t273) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008477bd); };
            c.cmp(t273, 0x41u64, 16);
            if c.cc_ne() { blk = 136; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t274) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008477c8); };
            c.logic(t274 & 0x7ffffffeu64, 32);
            if c.cc_ne() { blk = 136; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008477d5); };
            c.w64(0, t275);
            blk = 202; continue;
        }
        184 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008477de); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 136; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008477f1); };
            c.w64(0, t276);
            blk = 202; continue;
        }
        186 => {
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008477fa); };
            c.w64(0, t277);
            blk = 202; continue;
        }
        187 => {
            let Some(t278) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000847803); };
            c.w64(0, t278);
            blk = 202; continue;
        }
        188 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 136; continue; }
            blk = 189; continue;
        }
        189 => {
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000847815); };
            c.w64(0, t279);
            blk = 202; continue;
        }
        190 => {
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700084781e); };
            c.w64(0, t280);
            blk = 202; continue;
        }
        191 => {
            let Some(t281) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000847827); };
            c.w32(1, t281);
            let r282 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r282);
            let Some(t283) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700084782d); };
            c.cmp(t283, c.r32(1), 32);
            if c.cc_e() { blk = 202; continue; }
            blk = 192; continue;
        }
        192 => {
            c.w32(8, c.r32(1));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 201; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t284) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700084783e); };
            c.cmp(t284, c.r32(1), 32);
            if c.cc_e() { blk = 202; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t285) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000847847); };
            c.w32(8, t285);
            let r286 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r286);
            let Some(t287) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700084784e); };
            c.logic(t287 & 0x10u64, 8);
            if c.cc_ne() { blk = 202; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t288) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000847855); };
            c.cmp(t288, c.r32(8), 32);
            if c.cc_e() { blk = 199; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t289) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084785e); };
            c.cmp(t289, 0x41u64, 16);
            if c.cc_ne() { blk = 136; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t290) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700084786a); };
            c.logic(t290 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 202; continue; }
            blk = 198; continue;
        }
        198 => {
            blk = 136; continue;
        }
        199 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000847878); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 136; continue; }
            blk = 200; continue;
        }
        200 => {
            blk = 202; continue;
        }
        201 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 136; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000847897); };
            c.w64(1, t291);
            let Some(t292) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700084789b); };
            c.cmp(t292, c.r32(1), 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 203; continue;
        }
        203 => {
            let Some(t293) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x70008478a0); };
            c.w64(3, t293);
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008478a7); };
            c.w64(6, t294);
            return c.call(0x7000870914, 0x70008478b0, 0x70008478ab);
        }
        204 => {
            blk = 206; continue;
        }
        205 => {
            let Some(t295) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008478b2); };
            c.w32(0, t295);
            let r296 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r296);
            blk = 206; continue;
        }
        206 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008478b8); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 248; continue; }
            blk = 207; continue;
        }
        207 => {
            let Some(t297) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008478c4); };
            c.w32(1, t297);
            let r298 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r298);
            let Some(t299) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008478ca); };
            c.cmp(t299, 0x834u64, 16);
            if c.cc_ne() { blk = 209; continue; }
            blk = 208; continue;
        }
        208 => {
            c.w32(9, 0x1u64);
            blk = 210; continue;
        }
        209 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 210; continue;
        }
        210 => {
            c.w32(8, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 249; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t300) = c.ld8(c.r64(1).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008478ea); };
            c.w32(9, t300);
            let r301 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r301);
            let r302 = c.r32(9) & 0x3fu64;
            c.w32(9, r302);
            c.logic(r302, 32);
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_a() { blk = 249; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847901); };
            c.w64(6, t303);
            let Some(t304) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000847905); };
            c.w32(9, t304);
            let r305 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r305);
            let Some(t306) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700084790c); };
            c.w32(9, t306);
            let r307 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r307);
            let Some(t308) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000847913); };
            c.w32(11, t308);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4cu64, 64);
            if c.cc_be() { blk = 256; continue; }
            blk = 213; continue;
        }
        213 => {
            let Some(t309) = c.ld32(c.r64(9).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000847927); };
            c.w32(9, t309);
            let Some(t310) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700084792e); };
            c.cmp(t310, c.r32(9), 32);
            if c.cc_ne() { blk = 249; continue; }
            blk = 214; continue;
        }
        214 => {
            let Some(t311) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000847938); };
            c.w64(1, t311);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t312) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000847941); };
            c.cmp(t312, c.r32(3), 32);
            if c.cc_e() { blk = 249; continue; }
            blk = 215; continue;
        }
        215 => {
            let Some(t313) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084794a); };
            c.w32(1, t313);
            let r314 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r314);
            let Some(t315) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000847950); };
            c.w32(8, t315);
            let r316 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r316);
            let Some(t317) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000847957); };
            c.w32(9, t317);
            let Some(t318) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700084795b); };
            c.w32(11, t318);
            let r319 = c.sub(c.r32(11), 0x77u64, 32);
            c.w32(11, r319);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x7000847964); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000847968); }
            c.cmp(c.r32(11), 0xcu64, 32);
            if c.cc_a() { blk = 226; continue; }
            blk = 216; continue;
        }
        216 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_g() { blk = 218; continue; }
            blk = 217; continue;
        }
        217 => {
            let Some(t320) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700084797c); };
            c.w64(8, t320);
            blk = 221; continue;
        }
        218 => {
            let Some(t321) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000847985); };
            c.w32(8, t321);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 257; continue; }
            blk = 219; continue;
        }
        219 => {
            let Some(t322) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000847999); };
            c.w32(8, t322);
            let r323 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r323);
            let Some(t324) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008479a0); };
            c.cmp(t324, c.r32(8), 32);
            if c.cc_ne() { blk = 221; continue; }
            blk = 220; continue;
        }
        220 => {
            let Some(t325) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008479a9); };
            c.w64(8, t325);
            blk = 221; continue;
        }
        221 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_g() { blk = 223; continue; }
            blk = 222; continue;
        }
        222 => {
            let Some(t326) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008479b6); };
            c.w64(1, t326);
            blk = 247; continue;
        }
        223 => {
            let Some(t327) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008479c2); };
            c.w32(9, t327);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 258; continue; }
            blk = 224; continue;
        }
        224 => {
            let Some(t328) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008479d6); };
            c.w32(1, t328);
            let r329 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r329);
            let Some(t330) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008479dc); };
            c.cmp(t330, c.r32(1), 32);
            if c.cc_ne() { blk = 247; continue; }
            blk = 225; continue;
        }
        225 => {
            let Some(t331) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008479e9); };
            c.w64(1, t331);
            blk = 247; continue;
        }
        226 => {
            let Some(t332) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70008479f5); };
            c.cmp(t332, c.r32(8), 32);
            if c.cc_ne() { blk = 261; continue; }
            blk = 227; continue;
        }
        227 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_g() { blk = 229; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t333) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000847a08); };
            c.w64(8, t333);
            blk = 237; continue;
        }
        229 => {
            let Some(t334) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000847a14); };
            c.w32(8, t334);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 259; continue; }
            blk = 230; continue;
        }
        230 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x7)) { return RegfileExit::fault(0x7000847a28); }
            let Some(t335) = c.ld64(0x7000847323u64) else { return RegfileExit::fault(0x7000847a2d); };
            c.w64(8, t335);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x7000847a34); }
            let Some(t336) = c.ld64(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000847a39); };
            c.cmp(t336, c.r64(8), 64);
            if c.cc_e() { blk = 236; continue; }
            blk = 231; continue;
        }
        231 => {
            let Some(t337) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000847a3f); };
            c.w64(8, t337);
            let Some(t338) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000847a46); };
            c.w64(11, t338);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t339) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000847a4d); };
            c.w64(15, t339);
            let Some(t340) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000847a54); };
            c.cmp(t340, c.r64(12), 64);
            if c.cc_a() { blk = 234; continue; }
            blk = 232; continue;
        }
        232 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000847a63, 0x7000847a5e);
        }
        233 => {
            let Some(t341) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000847a63); };
            c.w64(9, t341);
            let Some(t342) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847a67); };
            c.w64(6, t342);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t343) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000847a6d); };
            c.w64(1, t343);
            blk = 235; continue;
        }
        234 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x7000847a73); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 235; continue;
        }
        235 => {
            let Some(t344) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000847a7d); };
            c.w64(8, t344);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000847a84); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x7000847a88); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000847a8d); }
            c.w64(8, c.r64(0));
            blk = 237; continue;
        }
        236 => {
            let Some(t345) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000847a97); };
            c.w64(8, t345);
            blk = 237; continue;
        }
        237 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000847a9e); }
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_g() { blk = 239; continue; }
            blk = 238; continue;
        }
        238 => {
            let Some(t346) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000847aa8); };
            c.w64(1, t346);
            blk = 247; continue;
        }
        239 => {
            let Some(t347) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000847ab4); };
            c.w32(9, t347);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 260; continue; }
            blk = 240; continue;
        }
        240 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0xf)) { return RegfileExit::fault(0x7000847ac8); }
            let Some(t348) = c.ld64(0x7000847323u64) else { return RegfileExit::fault(0x7000847acd); };
            c.w64(9, t348);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x7000847ad4); }
            let Some(t349) = c.ld64(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000847ad9); };
            c.cmp(t349, c.r64(9), 64);
            if c.cc_e() { blk = 246; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t350) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000847adf); };
            c.w64(1, t350);
            let Some(t351) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000847ae6); };
            c.w64(9, t351);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t352) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000847aed); };
            c.w64(12, t352);
            let Some(t353) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000847af4); };
            c.cmp(t353, c.r64(11), 64);
            if c.cc_a() { blk = 244; continue; }
            blk = 242; continue;
        }
        242 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000847b04, 0x7000847aff);
        }
        243 => {
            let Some(t354) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000847b04); };
            c.w64(8, t354);
            let Some(t355) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847b08); };
            c.w64(6, t355);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 245; continue;
        }
        244 => {
            if !c.st64(c.r64(1), c.r64(11)) { return RegfileExit::fault(0x7000847b10); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 245; continue;
        }
        245 => {
            let Some(t356) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000847b1a); };
            c.w64(1, t356);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000847b21); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x7000847b24); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000847b29); }
            c.w64(1, c.r64(0));
            blk = 247; continue;
        }
        246 => {
            let Some(t357) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000847b33); };
            c.w64(1, t357);
            blk = 247; continue;
        }
        247 => {
            c.w64(0, c.r64(1));
            blk = 254; continue;
        }
        248 => {
            c.w32(8, 0x1u64);
            blk = 249; continue;
        }
        249 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 250; continue;
        }
        250 => {
            let Some(t358) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000847b4b); };
            c.w32(1, t358);
            let r359 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r359);
            let Some(t360) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000847b51); };
            c.w32(1, t360);
            let r361 = c.sub(c.r32(1), 0xbau64, 32);
            c.w32(1, r361);
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 263; continue; }
            blk = 251; continue;
        }
        251 => {
            let Some(t362) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847b63); };
            c.w64(6, t362);
            c.w32(3, 0);
            c.logic(0, 32);
            return c.call(0x7000870914, 0x7000847b6e, 0x7000847b69);
        }
        252 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x7000847b6e); }
            c.w32(3, 0x2u64);
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000847b77); };
            c.w64(0, t363);
            let Some(t364) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847b7b); };
            c.w64(6, t364);
            return c.call(0x7000870914, 0x7000847b84, 0x7000847b7f);
        }
        253 => {
            let Some(t365) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847b84); };
            c.w64(6, t365);
            let Some(t366) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000847b88); };
            c.w64(8, t366);
            blk = 254; continue;
        }
        254 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000847b8c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000847b8d); }
            let Some(t367) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000847b8f); };
            if !c.push64(t367) { return RegfileExit::fault(0x7000847b8f); }
            c.w32(0, 0x2u64);
            let Some(t368) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000847b97); };
            c.w64(7, t368);
            return c.call(0x70007f2274, 0x7000847ba0, 0x7000847b9b);
        }
        255 => {
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000847ba0); };
            c.w64(0, t369);
            blk = 171; continue;
        }
        256 => {
            return RegfileExit::fault(0x7000847ba9);
        }
        257 => {
            return RegfileExit::fault(0x7000847baa);
        }
        258 => {
            return RegfileExit::fault(0x7000847bab);
        }
        259 => {
            return RegfileExit::fault(0x7000847bac);
        }
        260 => {
            return RegfileExit::fault(0x7000847bad);
        }
        261 => {
            return RegfileExit::fault(0x7000847bae);
        }
        262 => {
            blk = 265; continue;
        }
        263 => {
            c.w32(1, 0x92u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000847bb6); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000847bb7); }
            let Some(t370) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000847bb8); };
            c.w64(3, t370);
            c.w32(0, 0x2u64);
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847bc4); };
            c.w64(6, t371);
            return c.call(0x700086ed74, 0x7000847bcd, 0x7000847bc8);
        }
        264 => {
            blk = 271; continue;
        }
        265 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000847bcf); }
            let Some(t372) = c.ld64(c.r64(13).wrapping_add(0x9e8)) else { return RegfileExit::fault(0x7000847bd3); };
            c.w64(3, t372);
            let Some(t373) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000847bda); };
            c.w64(0, t373);
            let Some(t374) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847bde); };
            c.w64(6, t374);
            return c.call(0x7000870914, 0x7000847be7, 0x7000847be2);
        }
        266 => {
            let Some(t375) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000847be7); };
            c.cmp(t375, c.r32(0), 32);
            if c.cc_e() { blk = 269; continue; }
            blk = 267; continue;
        }
        267 => {
            let Some(t376) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000847bf0); };
            c.cmp(t376, c.r32(0), 32);
            if c.cc_e() { blk = 269; continue; }
            blk = 268; continue;
        }
        268 => {
            let Some(t377) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000847bf9); };
            if !c.push64(t377) { return RegfileExit::fault(0x7000847bf9); }
            let Some(t378) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847bfc); };
            c.w64(6, t378);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x7000847c0d, 0x7000847c08);
        }
        269 => {
            let Some(t379) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000847c0d); };
            if !c.push64(t379) { return RegfileExit::fault(0x7000847c0d); }
            let Some(t380) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x7000847c10); };
            c.w64(3, t380);
            c.w32(0, 0x1u64);
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847c1c); };
            c.w64(6, t381);
            return c.call(0x700086ed74, 0x7000847c25, 0x7000847c20);
        }
        270 => {
            return RegfileExit::fault(0x7000847c25);
        }
        271 => {
            return RegfileExit::fault(0x7000847c26);
        }
        272 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000847c27); }
            let Some(t382) = c.ld64(c.r64(13).wrapping_add(0x2200)) else { return RegfileExit::fault(0x7000847c28); };
            c.w64(3, t382);
            c.w32(0, 0x1u64);
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847c34); };
            c.w64(6, t383);
            return c.call(0x700086ed74, 0x7000847c3d, 0x7000847c38);
        }
        273 => {
            return RegfileExit::fault(0x7000847c3d);
        }
        274 => {
            let Some(t384) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x7000847c3e); };
            c.w64(3, t384);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t385) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847c47); };
            c.w64(6, t385);
            return c.call(0x700086ed74, 0x7000847c50, 0x7000847c4b);
        }
        275 => {
            return RegfileExit::fault(0x7000847c50);
        }
        276 => {
            let Some(t386) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000847c51); };
            if !c.push64(t386) { return RegfileExit::fault(0x7000847c51); }
            let Some(t387) = c.ld64(c.r64(13).wrapping_add(0x21f0)) else { return RegfileExit::fault(0x7000847c54); };
            c.w64(3, t387);
            c.w32(0, 0x1u64);
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847c60); };
            c.w64(6, t388);
            return c.call(0x700086ed74, 0x7000847c69, 0x7000847c64);
        }
        277 => {
            return RegfileExit::fault(0x7000847c69);
        }
        278 => {
            c.w32(1, 0xf6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000847c6f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000847c70); }
            let Some(t389) = c.ld64(c.r64(13).wrapping_add(0xa38)) else { return RegfileExit::fault(0x7000847c71); };
            c.w64(1, t389);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000847c78); }
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000847c79); };
            if !c.push64(t390) { return RegfileExit::fault(0x7000847c79); }
            let Some(t391) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000847c7c); };
            c.w64(3, t391);
            c.w32(0, 0x4u64);
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000847c88); };
            c.w64(6, t392);
            return c.call(0x700086ed74, 0x7000847c91, 0x7000847c8c);
        }
        279 => {
            return RegfileExit::fault(0x7000847c91);
        }
        280 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000847c97); }
            return c.call(0x70007febf4, 0x7000847ca1, 0x7000847c9c);
        }
        281 => {
            c.w64(1, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000847ca4); }
            blk = 124; continue;
        }
        282 => {
            c.w32(1, 0x52u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000847cb3); }
            let Some(t393) = c.ld64(c.r64(13).wrapping_add(0x848)) else { return RegfileExit::fault(0x7000847cb4); };
            c.w64(1, t393);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000847cbb); }
            let Some(t394) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000847cbc); };
            c.w64(3, t394);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000847ccd, 0x7000847cc8);
        }
        283 => {
            return RegfileExit::fault(0x7000847ccd);
        }
        284 => {
            c.w32(11, c.r32(11));
            let r395 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r395);
            c.w64(9, 0xfffffffffffc0000u64);
            let r396 = c.r64(9) & c.r64(11);
            c.w64(9, r396);
            c.logic(r396, 64);
            let Some(t397) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000847cde); };
            c.logic(t397 & 0x2u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 285; continue;
        }
        285 => {
            c.w64(15, c.r64(12).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000847cee); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000847cef); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000847cfc, 0x7000847cf7);
        }
        286 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000847cfc); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000847cfe); }
            blk = 62; continue;
        }
        287 => {
            return RegfileExit::jump(0x7000847d08);
        }
            _ => unreachable!(),
        }
    }
}
