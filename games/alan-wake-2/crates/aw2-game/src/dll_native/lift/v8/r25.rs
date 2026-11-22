#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_70008c2014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008c2014); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008c2018); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c2019); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c201a); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008c201f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008c2023); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 125; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c202d); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 150; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c2039); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c2040); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 150; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008c204d); };
            c.w64(8, t6);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 127; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c205b); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c2062); };
            c.cmp(t9, 0x40u64, 16);
            if c.cc_ae() { blk = 127; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c206e); };
            c.w32(9, t10);
            let r11 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r11);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008c2075); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008c2079); };
            c.w64(11, t12);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c2083); };
            c.cmp(t13, c.r32(12), 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c208d); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c2094); };
            c.w32(11, t16);
            let r17 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r17);
            let Some(t18) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c209b); };
            c.w32(15, t18);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x9au64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x70008c20ae);
        }
        8 => {
            let Some(t19) = c.ld32(c.r64(11).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008c20af); };
            c.w32(3, t19);
            let r20 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r20);
            let Some(t21) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008c20b9); };
            c.cmp(t21, c.r32(9), 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c20c3); };
            c.w32(3, t22);
            c.logic(c.r32(3) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(15), 0x9fu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008c20db);
        }
        12 => {
            let Some(t23) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c20dc); };
            c.w32(9, t23);
            let r24 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r24);
            let Some(t25) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c20e3); };
            c.w32(9, t25);
            let r26 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r26);
            let Some(t27) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c20ea); };
            c.w32(15, t27);
            let r28 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r28);
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0x283)) else { return RegfileExit::fault(0x70008c20f1); };
            c.cmp(t29, c.r32(9), 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t30) = c.ld32(c.r64(15).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008c20fe); };
            c.w32(9, t30);
            c.w32(9, c.r32(9));
            let r31 = c.r32(9) & 0x8u64;
            c.w32(9, r31);
            c.logic(r31, 32);
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c2113); };
            c.w32(9, t32);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let Some(t33) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c211d); };
            c.w32(11, t33);
            let r34 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r34);
            c.logic(c.r8(9) & 0xffffffffffffff80u64, 8);
            c.w8(15, (c.cc_ne() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008c2130); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 17; continue;
        }
        16 => {
            c.w32(2, 0x1u64);
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(1, (c.cc_ne() as u64));
            c.w32(1, c.r8(1));
            c.w32(3, 0x1u64);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let r35 = c.add(c.r32(2), 0x1u64, 32);
            c.w32(2, r35);
            blk = 19; continue;
        }
        19 => {
            c.logic(c.r8(9) & 0x2u64, 8);
            c.w8(7, (c.cc_ne() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let r36 = c.add(c.r32(2), 0x1u64, 32);
            c.w32(2, r36);
            blk = 21; continue;
        }
        21 => {
            c.logic(c.r8(9) & 0x40u64, 8);
            c.w8(8, (c.cc_ne() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            let r37 = c.add(c.r32(2), 0x1u64, 32);
            c.w32(2, r37);
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(9) & 0x4u64, 8);
            c.w8(0, (c.cc_ne() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            let r38 = c.add(c.r32(2), 0x1u64, 32);
            c.w32(2, r38);
            blk = 25; continue;
        }
        25 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            c.w8(3, (c.cc_ne() as u64));
            c.w32(3, c.r8(3));
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            let r39 = c.add(c.r32(2), 0x1u64, 32);
            c.w32(2, r39);
            blk = 27; continue;
        }
        27 => {
            c.logic(c.r8(9) & 0x10u64, 8);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008c21b3); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            let r40 = c.add(c.r32(2), 0x1u64, 32);
            c.w32(2, r40);
            blk = 29; continue;
        }
        29 => {
            c.logic(c.r8(9) & 0x8u64, 8);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008c21cc); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            let r41 = c.add(c.r32(2), 0x1u64, 32);
            c.w32(2, r41);
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(6, 0xcu64);
            let r42 = c.add(c.r64(11), c.r64(6), 64);
            c.w64(11, r42);
            if c.cc_o() { blk = 37; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c21ef); };
            c.w64(6, t43);
            let Some(t44) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008c21f6); };
            c.w64(6, t44);
            let r45 = c.add(c.r64(11), 0x3u64, 64);
            c.w64(11, r45);
            let r46 = c.r64(11) & 0xfffffffffffffffcu64;
            c.w64(11, r46);
            c.logic(r46, 64);
            c.w64(9, c.r64(6).wrapping_add(c.r64(11)));
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c2205); };
            c.w64(12, t47);
            let Some(t48) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008c220c); };
            c.cmp(t48, c.r64(9), 64);
            if c.cc_be() { blk = 129; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 129; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c2223); };
            c.w64(11, t49);
            if !c.st64(c.r64(11), c.r64(9)) { return RegfileExit::fault(0x70008c222a); }
            c.w64(9, c.r64(6).wrapping_add(0x1));
            blk = 36; continue;
        }
        36 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008c2231); };
            c.w64(11, t50);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008c2238); }
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008c223c); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008c2244); }
            c.w64(2, c.r64(9));
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c224b); };
            c.w64(6, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c224f); };
            c.w64(11, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c2253); };
            c.w64(9, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c2257); };
            c.w64(12, t54);
            blk = 39; continue;
        }
        37 => {
            return RegfileExit::fault(0x70008c225d);
        }
        38 => {
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c225e); };
            c.w64(2, t55);
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(15, 0xbu64);
            blk = 42; continue;
        }
        41 => {
            if !c.st8(c.r64(2).wrapping_add(0xb), 0x64u64) { return RegfileExit::fault(0x70008c2273); }
            c.w32(15, 0xcu64);
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            if !c.st8(c.r64(2).wrapping_add(c.r64(15)), 0x67u64) { return RegfileExit::fault(0x70008c2282); }
            let r56 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r56);
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            if !c.st8(c.r64(2).wrapping_add(c.r64(15)), 0x69u64) { return RegfileExit::fault(0x70008c2290); }
            let r57 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r57);
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            if !c.st8(c.r64(2).wrapping_add(c.r64(15)), 0x6cu64) { return RegfileExit::fault(0x70008c229f); }
            let r58 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r58);
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            if !c.st8(c.r64(2).wrapping_add(c.r64(15)), 0x6du64) { return RegfileExit::fault(0x70008c22ad); }
            let r59 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r59);
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            if !c.st8(c.r64(2).wrapping_add(c.r64(15)), 0x73u64) { return RegfileExit::fault(0x70008c22bb); }
            let r60 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r60);
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(8, c.r64(15).wrapping_add(0x1));
            if !c.st8(c.r64(2).wrapping_add(c.r64(15)), 0x75u64) { return RegfileExit::fault(0x70008c22ce); }
            c.w64(15, c.r64(8));
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            if !c.st8(c.r64(2).wrapping_add(c.r64(15)), 0x79u64) { return RegfileExit::fault(0x70008c22dc); }
            blk = 56; continue;
        }
        56 => {
            let Some(t61) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c22e1); };
            c.w32(8, t61);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x9au64, 64);
            if c.cc_a() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            return RegfileExit::fault(0x70008c22f4);
        }
        58 => {
            let Some(t62) = c.ld32(c.r64(6).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008c22f5); };
            c.w32(8, t62);
            let r63 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r63);
            let Some(t64) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008c22ff); };
            c.w32(8, t64);
            let r65 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r65);
            c.w64(0, c.r64(11));
            let Some(t66) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c2309); };
            c.cmp(t66, c.r32(0), 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 59; continue;
        }
        59 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 131; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t67) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c231a); };
            c.w32(9, t67);
            let r68 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r68);
            let Some(t69) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c2321); };
            c.cmp(t69, 0x40u64, 16);
            if c.cc_b() { blk = 63; continue; }
            blk = 61; continue;
        }
        61 => {
            blk = 131; continue;
        }
        62 => {
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c232e); };
            c.w64(0, t70);
            c.w64(11, c.r64(0));
            blk = 63; continue;
        }
        63 => {
            let Some(t71) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c2338); };
            c.logic(t71 & 0x200000u64, 32);
            if c.cc_ne() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c2345); };
            c.w64(11, t72);
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c234c); };
            c.w64(9, t73);
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c2353); };
            c.w64(15, t74);
            blk = 75; continue;
        }
        65 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c235f); };
            c.w64(9, t75);
            let Some(t76) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c2366); };
            c.w64(11, t76);
            c.w64(12, c.r64(11).wrapping_add(0x4c));
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c236d); };
            c.w64(15, t77);
            let Some(t78) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008c2374); };
            c.cmp(t78, c.r64(12), 64);
            if c.cc_be() { blk = 133; continue; }
            blk = 66; continue;
        }
        66 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008c237d); }
            let r79 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r79);
            blk = 67; continue;
        }
        67 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008c2384); };
            c.w64(12, t80);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c238b); }
            c.w32(12, 0x22u64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008c2395); }
            let Some(t81) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c2399); };
            c.w32(12, t81);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 135; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c23b0); }
            let Some(t82) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c23b4); };
            c.w32(3, t82);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_be() { blk = 136; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70008c23c8); }
            let Some(t83) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c23cc); };
            c.w32(3, t83);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x2u64, 64);
            if c.cc_be() { blk = 137; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w32(3, 0x8u64);
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(3)) { return RegfileExit::fault(0x70008c23e5); }
            let Some(t84) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c23e9); };
            c.w32(3, t84);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x3u64, 64);
            if c.cc_be() { blk = 138; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w32(3, 0x2u64);
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(3)) { return RegfileExit::fault(0x70008c2402); }
            let Some(t85) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c2406); };
            c.w32(3, t85);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x4u64, 64);
            if c.cc_be() { blk = 139; continue; }
            blk = 72; continue;
        }
        72 => {
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x70008c241a); }
            c.w64(3, c.r64(11).wrapping_add(0x4b));
            c.w64(1, c.r64(11).wrapping_add(0x1b));
            c.cmp(c.r64(1), c.r64(3), 64);
            if c.cc_e() { blk = 75; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c242b); };
            c.w64(7, t86);
            blk = 74; continue;
        }
        74 => {
            if !c.st32(c.r64(1), c.r32(7)) { return RegfileExit::fault(0x70008c2432); }
            let r87 = c.add(c.r64(1), 0x4u64, 64);
            c.w64(1, r87);
            c.cmp(c.r64(3), c.r64(1), 64);
            if c.cc_ne() { blk = 74; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t88) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c243d); };
            c.w32(3, t88);
            if let Some(sr) = c.shl(c.r64(3), 0x2u64, 64) { c.w64(3, sr); }
            let Some(t89) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c2446); };
            c.w64(1, t89);
            c.w64(7, c.r64(3).wrapping_add(c.r64(1)));
            let Some(t90) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008c244d); };
            c.cmp(t90, c.r64(7), 64);
            if c.cc_be() { blk = 140; continue; }
            blk = 76; continue;
        }
        76 => {
            if !c.st64(c.r64(9), c.r64(7)) { return RegfileExit::fault(0x70008c2456); }
            c.w64(9, c.r64(1).wrapping_add(0x1));
            blk = 77; continue;
        }
        77 => {
            c.w64(15, c.r64(3).wrapping_sub(0x1));
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008c2461); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008c2465); }
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c2469); };
            c.w64(11, t91);
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008c2470); }
            let Some(t92) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c2474); };
            c.w32(11, t92);
            c.w64(3, c.r64(11));
            let r93 = c.r32(3) & 0xe0000000u64;
            c.w32(3, r93);
            c.logic(r93, 32);
            if c.cc_ne() { blk = 82; continue; }
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_e() { blk = 89; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c2489); };
            c.w64(8, t94);
            blk = 80; continue;
        }
        80 => {
            let r95 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r95);
            if !c.st32(c.r64(15).wrapping_add(c.r64(9)), c.r32(8)) { return RegfileExit::fault(0x70008c2494); }
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_ne() { blk = 80; continue; }
            blk = 81; continue;
        }
        81 => {
            blk = 89; continue;
        }
        82 => {
            let r96 = c.sub(c.r32(11), 0x20000000u64, 32);
            c.w32(11, r96);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008c24a7); }
            let Some(t97) = c.ld8(c.r64(8).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008c24ab); };
            c.w32(11, t97);
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_e() { blk = 85; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x70008c24bd); };
            c.w64(1, t98);
            blk = 84; continue;
        }
        84 => {
            let r99 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r99);
            if !c.st32(c.r64(15).wrapping_add(c.r64(9)), c.r32(1)) { return RegfileExit::fault(0x70008c24c8); }
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_ne() { blk = 84; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 88; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c24d7); };
            c.w64(15, t100);
            blk = 87; continue;
        }
        87 => {
            let r101 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r101);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(15)) { return RegfileExit::fault(0x70008c24e2); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 87; continue; }
            blk = 88; continue;
        }
        88 => {
            let r102 = c.add(c.r32(3), 0xe0000000u64, 32);
            c.w32(3, r102);
            if c.cc_e() { blk = 142; continue; }
            blk = 89; continue;
        }
        89 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008c24f8); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c24fa); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008c24fb); }
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x2670)) else { return RegfileExit::fault(0x70008c24fc); };
            c.w64(3, t103);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008c250d, 0x70008c2508);
        }
        90 => {
            let Some(t104) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c250d); };
            c.w64(8, t104);
            let Some(t105) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c2511); };
            c.w32(8, t105);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(8)) { return RegfileExit::fault(0x70008c2515); }
            let Some(t106) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c2519); };
            c.w32(8, t106);
            c.w32(8, c.r32(8));
            c.w64(9, c.r64(8));
            let r107 = c.r32(9) & 0x20u64;
            c.w32(9, r107);
            c.logic(r107, 32);
            let r108 = c.r32(8) & 0x2u64;
            c.w32(8, r108);
            c.logic(r108, 32);
            let r109 = c.shr(c.r32(9), 0x4u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r109);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r110 = c.r64(8) & 0xfffffffffffffffeu64;
            c.w64(8, r110);
            c.logic(r110, 64);
            let r111 = c.r64(9) & 0xfffffffffffffffeu64;
            c.w64(9, r111);
            c.logic(r111, 64);
            let r112 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r112);
            let r113 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r113);
            blk = 118; continue;
        }
        91 => {
            let Some(t114) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c2548); };
            c.w32(9, t114);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x9au64, 64);
            if c.cc_a() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            return RegfileExit::fault(0x70008c255b);
        }
        93 => {
            let Some(t115) = c.ld32(c.r64(6).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008c255c); };
            c.w32(9, t115);
            let r116 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r116);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008c2566); }
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x70008c256a); };
            c.w64(3, t117);
            return c.call(0x7000870914, 0x70008c2576, 0x70008c2571);
        }
        94 => {
            let Some(t118) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c2576); };
            c.cmp(t118, c.r32(0), 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 95; continue;
        }
        95 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 144; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t119) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c258b); };
            c.w32(8, t119);
            let r120 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r120);
            let Some(t121) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c2592); };
            c.cmp(t121, 0xbau64, 16);
            if c.cc_b() { blk = 144; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x70008c259f); };
            c.w64(3, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c25a6); };
            c.w64(6, t123);
            return c.call(0x7000870914, 0x70008c25af, 0x70008c25aa);
        }
        98 => {
            let Some(t124) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c25af); };
            c.cmp(t124, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t125) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c25be); };
            c.cmp(t125, c.r32(0), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let r126 = c.r32(8) | c.r32(9);
            c.w32(8, r126);
            c.logic(r126, 32);
            if c.cc_ne() { blk = 104; continue; }
            blk = 99; continue;
        }
        99 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 102; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t127) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c25d6); };
            c.w32(8, t127);
            let r128 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r128);
            let Some(t129) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008c25dd); };
            c.logic(t129 & 0x40u64, 8);
            if c.cc_e() { blk = 102; continue; }
            blk = 101; continue;
        }
        101 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008c25e4); }
            blk = 104; continue;
        }
        102 => {
            c.w32(8, 0x26eu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c25f0); }
            let Some(t130) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008c25f2); };
            c.w64(3, t130);
            c.w32(0, 0x1u64);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c25fe); };
            c.w64(6, t131);
            return c.call(0x700086ed74, 0x70008c2607, 0x70008c2602);
        }
        103 => {
            return RegfileExit::fault(0x70008c2607);
        }
        104 => {
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x700)) else { return RegfileExit::fault(0x70008c2608); };
            c.w64(3, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c260f); };
            c.w64(0, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c2613); };
            c.w64(6, t134);
            return c.call(0x7000870914, 0x70008c261c, 0x70008c2617);
        }
        105 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 146; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t135) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c2624); };
            c.w32(8, t135);
            let r136 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r136);
            let Some(t137) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c262b); };
            c.cmp(t137, 0x40u64, 16);
            if c.cc_ae() { blk = 146; continue; }
            blk = 107; continue;
        }
        107 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c2637); }
            let Some(t138) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c2638); };
            if !c.push64(t138) { return RegfileExit::fault(0x70008c2638); }
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c263b); };
            c.w64(8, t139);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c2642); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008c2644); }
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c2648); };
            c.w64(6, t140);
            c.w32(0, 0x2u64);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c2651); };
            c.w64(2, t141);
            c.w64(8, c.r64(0));
            c.w64(7, c.r64(2));
            return c.call(0x70007f7214, 0x70008c2660, 0x70008c265b);
        }
        108 => {
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c2660); };
            c.w64(6, t142);
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008c2664); };
            c.w64(3, t143);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008c266b); }
            let Some(t144) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c266f); };
            c.w64(0, t144);
            return c.call(0x7000870914, 0x70008c2678, 0x70008c2673);
        }
        109 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 112; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c267c); };
            c.w64(6, t145);
            return c.call(0x70008a17f4, 0x70008c2685, 0x70008c2680);
        }
        111 => {
            blk = 114; continue;
        }
        112 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 114; continue;
        }
        114 => {
            c.w64(1, c.r64(0));
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c2691); };
            c.w64(0, t146);
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008c2695); };
            c.w64(3, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c269c); };
            c.w64(6, t148);
            return c.call(0x7000871cb4, 0x70008c26a5, 0x70008c26a0);
        }
        115 => {
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008c26a5); };
            c.w64(3, t149);
            let Some(t150) = c.ld32(c.r64(3).wrapping_add(0xa3)) else { return RegfileExit::fault(0x70008c26ac); };
            c.w32(3, t150);
            let r151 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c26b5); };
            c.w64(0, t152);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c26bb); };
            c.w64(6, t153);
            return c.call(0x70008e8b74, 0x70008c26c4, 0x70008c26bf);
        }
        116 => {
            c.cmp(c.r32(0), 0xfffffffffffffffeu64, 32);
            c.w8(8, (c.cc_ne() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008c26cf); }
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008c26d3); };
            c.w64(3, t154);
            let Some(t155) = c.ld32(c.r64(3).wrapping_add(0xa7)) else { return RegfileExit::fault(0x70008c26da); };
            c.w32(3, t155);
            let r156 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c26e3); };
            c.w64(0, t157);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c26e9); };
            c.w64(6, t158);
            return c.call(0x70008e8b74, 0x70008c26f2, 0x70008c26ed);
        }
        117 => {
            c.cmp(c.r32(0), 0xfffffffffffffffeu64, 32);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c26fd); };
            c.w64(8, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c2701); };
            c.w64(0, t160);
            blk = 118; continue;
        }
        118 => {
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c2705); };
            c.w64(6, t161);
            let Some(t162) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c2709); };
            c.w32(11, t162);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xa6u64, 64);
            if c.cc_a() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            return RegfileExit::fault(0x70008c271c);
        }
        120 => {
            let Some(t163) = c.ld32(c.r64(6).wrapping_add(0x29f)) else { return RegfileExit::fault(0x70008c271d); };
            c.w32(11, t163);
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c2724); };
            c.w64(12, t164);
            let Some(t165) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008c272b); };
            c.w64(15, t165);
            c.w64(3, c.r64(15).wrapping_add(0x18));
            let Some(t166) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c2733); };
            c.w64(2, t166);
            let Some(t167) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008c273a); };
            c.cmp(t167, c.r64(3), 64);
            if c.cc_be() { blk = 148; continue; }
            blk = 121; continue;
        }
        121 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x70008c2743); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 122; continue;
        }
        122 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008c274b); }
            let r168 = c.shl(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r168);
            let r169 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r169);
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c2757); };
            c.w64(11, t170);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008c275e); }
            let r171 = c.r32(9) | c.r32(8);
            c.w32(9, r171);
            c.logic(r171, 32);
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008c2766); }
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008c276f); }
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c2774); };
            c.w64(9, t172);
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008c2778); }
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(8)) { return RegfileExit::fault(0x70008c277d); }
            c.w64(0, c.r64(12));
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c2785); };
            c.w64(1, t173);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008c278c); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 124; continue; }
            blk = 123; continue;
        }
        123 => {
            return c.ret(0x10, 0x70008c2793);
        }
        124 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008c2796); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008c279d); }
            return c.ret(0x0, 0x70008c279f);
        }
        125 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008c27a2); };
            c.w64(3, t174);
            return c.call(0x700086ed74, 0x70008c27ae, 0x70008c27a9);
        }
        126 => {
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c27ae); };
            c.w64(6, t175);
            blk = 1; continue;
        }
        127 => {
            c.w64(0, c.r64(8));
            return c.call(0x700089c794, 0x70008c27bf, 0x70008c27ba);
        }
        128 => {
            c.w64(8, c.r64(0));
            let Some(t176) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c27c2); };
            c.w64(0, t176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c27c6); };
            c.w64(6, t177);
            blk = 5; continue;
        }
        129 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008c27cf); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008c27d3); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008c27d7); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x70008c27db); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(1)) { return RegfileExit::fault(0x70008c27df); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(15)) { return RegfileExit::fault(0x70008c27e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008c27e7); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x70008c27f3, 0x70008c27ee);
        }
        130 => {
            c.w64(9, c.r64(0));
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c27f6); };
            c.w64(3, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c27fa); };
            c.w64(0, t179);
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c27fe); };
            c.w64(8, t180);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c2802); };
            c.w64(7, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c2806); };
            c.w64(1, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c280a); };
            c.w64(15, t183);
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c280e); };
            c.w64(2, t184);
            blk = 36; continue;
        }
        131 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008c2817); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008c281b); }
            return c.call(0x700089c794, 0x70008c2824, 0x70008c281f);
        }
        132 => {
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c2824); };
            c.w64(8, t185);
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c2828); };
            c.w64(6, t186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c282c); };
            c.w64(2, t187);
            blk = 63; continue;
        }
        133 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008c2835); }
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008c283e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008c2842); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008c284e, 0x70008c2849);
        }
        134 => {
            c.w64(11, c.r64(0));
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c2851); };
            c.w64(6, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c2855); };
            c.w64(0, t189);
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c2859); };
            c.w64(2, t190);
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c285d); };
            c.w64(8, t191);
            let Some(t192) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c2861); };
            c.w64(9, t192);
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c2868); };
            c.w64(15, t193);
            blk = 67; continue;
        }
        135 => {
            return RegfileExit::fault(0x70008c2874);
        }
        136 => {
            return RegfileExit::fault(0x70008c2875);
        }
        137 => {
            return RegfileExit::fault(0x70008c2876);
        }
        138 => {
            return RegfileExit::fault(0x70008c2877);
        }
        139 => {
            return RegfileExit::fault(0x70008c2878);
        }
        140 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008c2879); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008c287d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008c2881); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008c2885); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008c2889); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febf4, 0x70008c2895, 0x70008c2890);
        }
        141 => {
            c.w64(9, c.r64(0));
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c2898); };
            c.w64(6, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c289c); };
            c.w64(0, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c28a0); };
            c.w64(2, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c28a4); };
            c.w64(8, t197);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c28a8); };
            c.w64(11, t198);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c28ac); };
            c.w64(3, t199);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 77; continue;
        }
        142 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c28b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008c28ba); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008c28be); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008c28c2); }
            let Some(t200) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x70008c28c6); };
            c.w64(3, t200);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x70008c28da, 0x70008c28d5);
        }
        143 => {
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c28da); };
            c.w64(6, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c28de); };
            c.w64(9, t202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c28e2); };
            c.w64(0, t203);
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c28e6); };
            c.w64(2, t204);
            blk = 89; continue;
        }
        144 => {
            c.w32(8, 0x54u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c28f5); }
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c28f7); };
            c.w64(8, t205);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c28fe); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c2900); }
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008c2901); };
            c.w64(3, t206);
            c.w32(0, 0x3u64);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c290d); };
            c.w64(6, t207);
            return c.call(0x700086ed74, 0x70008c2916, 0x70008c2911);
        }
        145 => {
            return RegfileExit::fault(0x70008c2916);
        }
        146 => {
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c2917); };
            c.w64(6, t208);
            return c.call(0x700089c794, 0x70008c2920, 0x70008c291b);
        }
        147 => {
            blk = 107; continue;
        }
        148 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008c292a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008c292e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008c2932); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008c2936); }
            return c.call(0x70007febf4, 0x70008c293f, 0x70008c293a);
        }
        149 => {
            c.w64(12, c.r64(0));
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c2942); };
            c.w64(0, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c2946); };
            c.w64(8, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c294a); };
            c.w64(9, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c294e); };
            c.w64(11, t212);
            blk = 122; continue;
        }
        150 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c295d); }
            let Some(t213) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008c295f); };
            c.w64(8, t213);
            let Some(t214) = c.ld32(c.r64(8).wrapping_add(0xaf)) else { return RegfileExit::fault(0x70008c2966); };
            c.w32(8, t214);
            let r215 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r215);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c2970); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c2972); }
            let Some(t216) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008c2973); };
            c.w64(3, t216);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008c2984, 0x70008c297f);
        }
        151 => {
            return RegfileExit::fault(0x70008c2984);
        }
        152 => {
            return RegfileExit::jump(0x70008c2988);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008c2994(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008c2994); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008c2998); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c2999); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c299a); }
            let r0 = c.sub(c.r64(4), 0x80u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008c29a2); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008c29a6); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 405; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c29b0); };
            c.w64(11, t2);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 403; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c29be); };
            c.w32(12, t3);
            let r4 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r4);
            let Some(t5) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c29c5); };
            c.cmp(t5, 0x83fu64, 16);
            if c.cc_ne() { blk = 403; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c29d3); };
            c.w32(12, t6);
            c.w32(15, c.r32(12));
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(12)) { return RegfileExit::fault(0x70008c29da); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008c29e1); }
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_ne() { blk = 398; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c29ef); };
            c.w32(0, t7);
            let r8 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c29f6); };
            c.w32(3, t9);
            let r10 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r10);
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c29fd); };
            c.w32(2, t11);
            let r12 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r12);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008c2a03); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008c2a07); }
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008c2a0d); };
            c.w64(7, t13);
            let Some(t14) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c2a14); };
            c.cmp(t14, c.r32(1), 32);
            if c.cc_e() { blk = 217; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c2a1d); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            let Some(t17) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c2a23); };
            c.w32(7, t17);
            let r18 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r18);
            let Some(t19) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c2a29); };
            c.w32(8, t19);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x9au64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70008c2a3c);
        }
        7 => {
            let Some(t20) = c.ld32(c.r64(7).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008c2a3d); };
            c.w32(9, t20);
            let r21 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r21);
            let Some(t22) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008c2a47); };
            c.cmp(t22, c.r32(2), 32);
            if c.cc_ne() { blk = 217; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t23) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c2a51); };
            c.w32(9, t23);
            c.logic(c.r32(9) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 217; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(8), 0x9fu64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x70008c2a6b);
        }
        11 => {
            let Some(t24) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c2a6c); };
            c.w32(2, t24);
            let r25 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r25);
            let Some(t26) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c2a72); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c2a79); };
            c.w32(9, t28);
            let r29 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r29);
            let Some(t30) = c.ld32(c.r64(7).wrapping_add(0x283)) else { return RegfileExit::fault(0x70008c2a80); };
            c.cmp(t30, c.r32(8), 32);
            if c.cc_ne() { blk = 407; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t31) = c.ld32(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008c2a8d); };
            c.w32(11, t31);
            c.w32(11, c.r32(11));
            let r32 = c.r32(11) & 0x8u64;
            c.w32(11, r32);
            c.logic(r32, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_ne() { blk = 407; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t33) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c2aa2); };
            c.w32(11, t33);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let Some(t34) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c2aac); };
            c.w32(2, t34);
            let r35 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r35);
            c.logic(c.r8(11) & 0x9u64, 8);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(2)) { return RegfileExit::fault(0x70008c2abe); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008c2ac2); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(7, c.r64(1));
            blk = 29; continue;
        }
        15 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 216; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t36) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c2ae8); };
            c.w32(8, t36);
            let r37 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r37);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c2af5); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c2b00); }
            blk = 22; continue;
        }
        19 => {
            c.w64(7, c.r64(2));
            let r38 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r38);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            blk = 22; continue;
        }
        20 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 21; continue;
        }
        21 => {
            let r39 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r39);
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c2b1e); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 22; continue;
        }
        22 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            blk = 25; continue;
        }
        24 => {
            c.cmp(c.r32(8), c.r32(2), 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 27; continue;
        }
        26 => {
            c.w32(7, 0x1u64);
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 216; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(7, c.r64(2));
            blk = 29; continue;
        }
        29 => {
            let Some(t40) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c2b4e); };
            c.w32(8, t40);
            let r41 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r41);
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c2b55); };
            c.w32(8, t42);
            let r43 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r43);
            let Some(t44) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c2b5c); };
            c.w32(9, t44);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008c2b66); }
            c.cmp(c.r64(9), 0x9bu64, 64);
            if c.cc_a() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            return RegfileExit::fault(0x70008c2b73);
        }
        31 => {
            let Some(t45) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c2b74); };
            c.w32(9, t45);
            let r46 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r46);
            let Some(t47) = c.ld32(c.r64(8).wrapping_add(0x273)) else { return RegfileExit::fault(0x70008c2b7b); };
            c.w32(8, t47);
            let r48 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r48);
            let Some(t49) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c2b85); };
            c.w32(9, t49);
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(8)) { return RegfileExit::fault(0x70008c2b8a); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 81; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t50) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c2b9e); };
            c.w32(7, t50);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008c2ba4); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(8)) { return RegfileExit::fault(0x70008c2ba8); }
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_b() { blk = 81; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t51) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c2bb5); };
            c.w32(8, t51);
            let r52 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r52);
            let Some(t53) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c2bbc); };
            c.w32(7, t53);
            let r54 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r54);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008c2bc2); }
            c.cmp(c.r32(7), 0x1u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 36; continue;
        }
        36 => {
            blk = 431; continue;
        }
        37 => {
            blk = 40; continue;
        }
        38 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008c2bdc); };
            c.w64(2, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c2be3); };
            c.w64(1, t56);
            return c.call(0x7000859a54, 0x70008c2bec, 0x70008c2be7);
        }
        39 => {
            c.w64(8, c.r64(0));
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c2bef); };
            c.w64(0, t57);
            blk = 82; continue;
        }
        40 => {
            let Some(t58) = c.ld32(c.r64(8).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008c2bf8); };
            c.w32(7, t58);
            c.cmp(c.r32(7), 0x7eu64, 32);
            if c.cc_a() { blk = 445; continue; }
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(9) & 0x7u64, 8);
            if c.cc_e() { blk = 58; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x70008c2c0f); }
            c.w64(9, c.r64(1));
            c.w64(11, c.r64(3));
            blk = 44; continue;
        }
        43 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008c2c1b); }
            blk = 44; continue;
        }
        44 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c2c1f); };
            c.w64(12, t59);
            let r60 = c.r32(12) & 0x7u64;
            c.w32(12, r60);
            c.logic(r60, 32);
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_l() { blk = 49; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 445; continue;
        }
        49 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 445; continue;
        }
        52 => {
            let Some(t61) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c2c59); };
            c.w32(12, t61);
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c2c5d); };
            c.cmp(t62, c.r32(12), 32);
            if c.cc_ne() { blk = 445; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t63) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c2c6a); };
            c.w32(11, t63);
            let r64 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r64);
            let Some(t65) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c2c71); };
            c.w32(12, t65);
            let r66 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r66);
            let Some(t67) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c2c78); };
            c.w32(12, t67);
            blk = 43; continue;
        }
        54 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c2c80); };
            c.w32(12, t68);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r69 = c.add(c.r64(9), c.r64(12), 64);
            c.w64(9, r69);
            let Some(t70) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c2c8d); };
            c.w32(11, t70);
            let r71 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r71);
            let Some(t72) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c2c94); };
            c.w32(12, t72);
            let r73 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r73);
            let Some(t74) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c2c9b); };
            c.w32(12, t74);
            blk = 43; continue;
        }
        55 => {
            let Some(t75) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c2ca6); };
            c.w32(11, t75);
            let r76 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r76);
            let Some(t77) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c2cad); };
            c.w32(12, t77);
            let r78 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r78);
            let Some(t79) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c2cb4); };
            c.w32(12, t79);
            blk = 43; continue;
        }
        56 => {
            c.w64(12, c.r64(11));
            c.w64(11, c.r64(9));
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 59; continue;
        }
        57 => {
            c.w64(12, c.r64(11));
            c.w64(11, c.r64(9));
            c.w32(9, 0x1u64);
            blk = 59; continue;
        }
        58 => {
            c.w64(11, c.r64(1));
            c.w64(12, c.r64(3));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x70008c2cde); }
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 433; continue; }
            blk = 60; continue;
        }
        60 => {
            let r80 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r80);
            blk = 61; continue;
        }
        61 => {
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c2cf3); };
            c.w64(9, t81);
            let r82 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c2cfa); };
            c.w64(15, t83);
            let r84 = c.r32(15) & 0x8u64;
            c.w32(15, r84);
            c.logic(r84, 32);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c2d02); };
            c.w64(3, t85);
            let r86 = c.add(c.r64(11), c.r64(3), 64);
            c.w64(11, r86);
            c.cmp(c.r32(15), 0x8u64, 32);
            if c.cc_ne() { blk = 435; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(15, c.r64(9).wrapping_add(c.r64(12)));
            let r87 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r87);
            let Some(t88) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c2d1a); };
            c.w32(12, t88);
            let r89 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r89);
            blk = 63; continue;
        }
        63 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x70008c2d21); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 444; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t90) = c.ld32(c.r64(12).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008c2d2f); };
            c.w32(9, t90);
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2e70)) else { return RegfileExit::fault(0x70008c2d34); };
            c.w64(11, t91);
            let Some(t92) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c2d3b); };
            c.w64(11, t92);
            c.cmp(c.r32(9), 0xffffffffffffffffu64, 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w32(12, c.r64(9).wrapping_mul(8));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(13));
            let Some(t93) = c.ld64(c.r64(9).wrapping_add(c.r64(12)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008c2d52); };
            c.w64(12, t93);
            blk = 67; continue;
        }
        66 => {
            let r94 = c.add(c.r64(12), 0x3fu64, 64);
            c.w64(12, r94);
            c.w64(9, c.r64(13));
            blk = 67; continue;
        }
        67 => {
            let r95 = c.add(c.r32(7), 0x2u64, 32);
            c.w32(7, r95);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w32(3, c.r32(3));
            c.w64(10, c.r64(4));
            let r96 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r96);
            let r97 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r97);
            c.logic(r97, 64);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x70008c2d79); }
            let r98 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r98);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008c2d80); }
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008c2d84); };
            c.w64(12, t99);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(12)) { return RegfileExit::fault(0x70008c2d8b); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(7)) { return RegfileExit::fault(0x70008c2d90); }
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(11)) { return RegfileExit::fault(0x70008c2d95); }
            if !c.st64(c.r64(4).wrapping_add(0x38), 0x1u64) { return RegfileExit::fault(0x70008c2d9a); }
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(9)) { return RegfileExit::fault(0x70008c2da3); }
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(0)) { return RegfileExit::fault(0x70008c2da8); }
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c2dad); };
            c.w64(11, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c2db1); };
            c.w64(1, t101);
            c.w64(2, c.r64(3));
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c2db8); };
            c.w64(8, t102);
            c.w64(9, c.r64(15));
            c.w64(10, 0x70008c2dc6u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008c2dc6); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008c2dca); }
            return c.call(c.r64(11), 0x70008c2dd1, 0x70008c2dce);
        }
        68 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008c2dd1); }
            let Some(t103) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x70008c2dd9); };
            c.w64(4, t103);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_e() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_ne() { blk = 436; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c2df1); };
            c.w64(6, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c2df5); };
            c.w64(0, t105);
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c2df9); };
            c.w64(3, t106);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c2dff); };
            c.w64(15, t107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c2e03); };
            c.w64(2, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c2e07); };
            c.w64(11, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008c2e0b); };
            c.w64(12, t110);
            blk = 81; continue;
        }
        71 => {
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008c2e17); };
            c.w64(8, t111);
            let Some(t112) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c2e1e); };
            c.w32(11, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c2e22); };
            c.w64(12, t113);
            let Some(t114) = c.ld32(c.r64(12).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008c2e26); };
            c.w32(12, t114);
            let r115 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r115);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r116 = c.sub(c.r32(11), 0x6u64, 32);
            c.w32(11, r116);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_l() { blk = 443; continue; }
            blk = 72; continue;
        }
        72 => {
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c2e42); }
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c2e46); };
            c.w64(0, t117);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008c2e4a); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r118 = c.r64(11) & c.r64(8);
            c.w64(11, r118);
            c.logic(r118, 64);
            let Some(t119) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c2e5c); };
            c.logic(t119 & 0x4u64, 8);
            if c.cc_ne() { blk = 609; continue; }
            blk = 74; continue;
        }
        74 => {
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008c2e67); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 76; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r120 = c.r64(11) & c.r64(8);
            c.w64(11, r120);
            c.logic(r120, 64);
            let Some(t121) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c2e79); };
            c.logic(t121 & 0x4u64, 8);
            if c.cc_ne() { blk = 606; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if c.cc_e() { blk = 82; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w32(12, 0x13u64);
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008c2e95); };
            c.w64(3, t122);
            blk = 79; continue;
        }
        78 => {
            let r123 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r123);
            blk = 79; continue;
        }
        79 => {
            let Some(t124) = c.ld32(c.r64(3).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x70008c2ea2); };
            c.w32(2, t124);
            let r125 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r125);
            if !c.st32(c.r64(8).wrapping_add(c.r64(12)), c.r32(2)) { return RegfileExit::fault(0x70008c2ea8); }
            let r126 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r126);
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ne() { blk = 78; continue; }
            blk = 80; continue;
        }
        80 => {
            blk = 82; continue;
        }
        81 => {
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c2eb7); };
            c.w64(8, t127);
            c.w64(0, c.r64(3));
            blk = 82; continue;
        }
        82 => {
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(8)) { return RegfileExit::fault(0x70008c2ec1); }
            let Some(t128) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c2ec8); };
            c.cmp(t128, c.r32(8), 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t129) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c2ed1); };
            c.cmp(t129, 0x0u64, 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c2ed7); };
            c.w64(11, t130);
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(2)) { return RegfileExit::fault(0x70008c2edd); }
            blk = 85; continue;
        }
        85 => {
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c2ee1); };
            c.w64(6, t131);
            blk = 395; continue;
        }
        86 => {
            let Some(t132) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c2eea); };
            c.cmp(t132, 0x0u64, 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t133) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c2ef0); };
            c.w32(11, t133);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4u64, 64);
            if c.cc_a() { blk = 89; continue; }
            blk = 88; continue;
        }
        88 => {
            return RegfileExit::fault(0x70008c2f00);
        }
        89 => {
            let Some(t134) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c2f01); };
            c.w32(12, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c2f05); };
            c.w64(11, t135);
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x70008c2f09); }
            blk = 90; continue;
        }
        90 => {
            let Some(t136) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c2f0d); };
            c.w32(12, t136);
            let Some(t137) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c2f11); };
            c.w32(3, t137);
            let r138 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r138);
            let Some(t139) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c2f18); };
            c.w32(1, t139);
            let r140 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r140);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008c2f1f); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x70008c2f23); }
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c2f27); };
            c.w64(6, t141);
            return c.call(0x700086fe54, 0x70008c2f30, 0x70008c2f2b);
        }
        91 => {
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c2f30); };
            c.w64(11, t142);
            let Some(t143) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c2f34); };
            c.w32(12, t143);
            let r144 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r144);
            c.logic(c.r16(12) & 0x100u64, 16);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008c2f49); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008c2f4d); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c2f5b); };
            c.w64(6, t145);
            let Some(t146) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c2f5f); };
            c.w32(15, t146);
            let r147 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r147);
            let Some(t148) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c2f66); };
            c.w32(15, t148);
            let r149 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r149);
            let Some(t150) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c2f6d); };
            c.w32(3, t150);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0xa1u64, 64);
            if c.cc_a() { blk = 94; continue; }
            blk = 93; continue;
        }
        93 => {
            return RegfileExit::fault(0x70008c2f80);
        }
        94 => {
            let Some(t151) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c2f81); };
            c.w64(3, (t151 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r152 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r152);
            c.w64(2, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t153) = c.ld32(c.r64(15).wrapping_add(0x28b)) else { return RegfileExit::fault(0x70008c2f90); };
            c.w32(15, t153);
            c.w64(1, c.r64(2));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x70008c2f9d); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_e() { blk = 108; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(2, c.r64(2).wrapping_mul(2).wrapping_add(0x30));
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_be() { blk = 105; continue; }
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r64(1), 0x2000000u64, 64);
            if c.cc_a() { blk = 447; continue; }
            blk = 97; continue;
        }
        97 => {
            c.cmp(c.r64(1), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 452; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t154) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c2fda); };
            c.w64(3, (t154 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r155 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r155);
            c.w64(2, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(2).wrapping_add(c.r64(2)));
            let Some(t156) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c2fed); };
            c.w64(2, t156);
            let Some(t157) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008c2ff4); };
            c.w64(7, t157);
            c.w64(9, c.r64(8).wrapping_add(0x8));
            c.w64(11, c.r64(9).wrapping_add(c.r64(7)));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(3)) { return RegfileExit::fault(0x70008c2fff); }
            let Some(t158) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c3003); };
            c.w64(12, t158);
            let Some(t159) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008c300a); };
            c.cmp(t159, c.r64(11), 64);
            if c.cc_be() { blk = 448; continue; }
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r64(9), 0x20000u64, 64);
            if c.cc_ae() { blk = 448; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.st64(c.r64(2), c.r64(11)) { return RegfileExit::fault(0x70008c3021); }
            c.w64(11, c.r64(7).wrapping_add(0x1));
            blk = 101; continue;
        }
        101 => {
            let r160 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r160);
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c302a); };
            c.w64(7, t161);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008c3031); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008c3035); }
            c.w64(1, c.r64(11).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r162 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r162);
            let r163 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r163);
            c.logic(r163, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008c3048); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(11)) { return RegfileExit::fault(0x70008c304d); }
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008c3056); };
            c.w64(0, t164);
            c.w64(10, 0x70008c3064u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008c3064); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008c3068); }
            return c.call(c.r64(0), 0x70008c306e, 0x70008c306c);
        }
        102 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008c306e); }
            let Some(t165) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008c3076); };
            c.w64(4, t165);
            let Some(t166) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c307b); };
            c.w64(11, t166);
            let Some(t167) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c3082); };
            c.w64(12, t167);
            c.w64(15, c.r64(12).wrapping_add(0x28));
            let Some(t168) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c308a); };
            c.w64(0, t168);
            let Some(t169) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008c3091); };
            c.cmp(t169, c.r64(15), 64);
            if c.cc_be() { blk = 450; continue; }
            blk = 103; continue;
        }
        103 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008c309a); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 104; continue;
        }
        104 => {
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c30a2); };
            c.w64(12, t170);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c30a6); }
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008c30aa); };
            c.w64(12, t171);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70008c30ae); }
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c30b2); };
            c.w64(12, t172);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008c30b9); }
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008c30bd); };
            c.w64(0, t173);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008c30c4); }
            c.w64(2, c.r64(11));
            c.w64(15, c.r64(0));
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c30ce); };
            c.w64(6, t174);
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c30d2); };
            c.w64(11, t175);
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c30d6); };
            c.w64(12, t176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c30da); };
            c.w64(0, t177);
            blk = 130; continue;
        }
        105 => {
            c.w32(3, c.r64(1).wrapping_add(c.r64(1)));
            let Some(t178) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c30e6); };
            c.w64(1, t178);
            let Some(t179) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008c30ed); };
            c.w64(7, t179);
            c.w64(8, c.r64(2).wrapping_add(c.r64(7)));
            let Some(t180) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c30f4); };
            c.w64(9, t180);
            let Some(t181) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c30fb); };
            c.cmp(t181, c.r64(8), 64);
            if c.cc_be() { blk = 454; continue; }
            blk = 106; continue;
        }
        106 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008c3104); }
            c.w64(2, c.r64(7).wrapping_add(0x1));
            blk = 107; continue;
        }
        107 => {
            let Some(t182) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c310b); };
            c.w64(1, (t182 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if let Some(sr) = c.shr(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r183 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r183);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008c3117); }
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008c311b); }
            let Some(t184) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c311e); };
            c.w64(15, t184);
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008c3125); }
            c.w64(15, c.r64(2).wrapping_add(0x28));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008c312d); }
            let Some(t185) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c3131); };
            c.w64(1, t185);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008c3138); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008c313c); }
            blk = 130; continue;
        }
        108 => {
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c3145); };
            c.w64(2, t186);
            let Some(t187) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008c314c); };
            c.w64(1, t187);
            c.w64(7, c.r64(1).wrapping_add(0x28));
            let Some(t188) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c3153); };
            c.w64(8, t188);
            let Some(t189) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008c315a); };
            c.cmp(t189, c.r64(7), 64);
            if c.cc_be() { blk = 456; continue; }
            blk = 109; continue;
        }
        109 => {
            if !c.st64(c.r64(2), c.r64(7)) { return RegfileExit::fault(0x70008c3163); }
            c.w64(2, c.r64(1).wrapping_add(0x1));
            blk = 110; continue;
        }
        110 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008c316a); }
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008c316e); }
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c3171); };
            c.w64(15, t190);
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008c3178); }
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008c317c); }
            blk = 130; continue;
        }
        111 => {
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c3185); };
            c.w64(6, t191);
            let Some(t192) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c3189); };
            c.w32(15, t192);
            let r193 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r193);
            let Some(t194) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c3190); };
            c.w32(15, t194);
            let r195 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r195);
            let Some(t196) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c3197); };
            c.w32(3, t196);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0xa2u64, 64);
            if c.cc_a() { blk = 113; continue; }
            blk = 112; continue;
        }
        112 => {
            return RegfileExit::fault(0x70008c31aa);
        }
        113 => {
            let Some(t197) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c31ab); };
            c.w64(3, (t197 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r198 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r198);
            c.w64(2, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t199) = c.ld32(c.r64(15).wrapping_add(0x28f)) else { return RegfileExit::fault(0x70008c31ba); };
            c.w32(15, t199);
            c.w64(1, c.r64(2));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x70008c31c7); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_e() { blk = 127; continue; }
            blk = 114; continue;
        }
        114 => {
            c.w64(2, c.r64(2).wrapping_mul(2).wrapping_add(0x34));
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_be() { blk = 124; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r64(1), 0x2000000u64, 64);
            if c.cc_a() { blk = 458; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r64(1), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 463; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t200) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c3204); };
            c.w64(3, (t200 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r201 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r201);
            c.w64(2, (c.r32(3) as u32 as i32 as i64 as u64));
            let r202 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r202);
            c.w64(8, c.r64(2).wrapping_add(c.r64(2)));
            c.w64(2, c.r64(8).wrapping_add(0x8));
            let Some(t203) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c321d); };
            c.w64(7, t203);
            let Some(t204) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008c3224); };
            c.w64(9, t204);
            c.w64(11, c.r64(2).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(3)) { return RegfileExit::fault(0x70008c322b); }
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c322f); };
            c.w64(12, t205);
            let Some(t206) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008c3236); };
            c.cmp(t206, c.r64(11), 64);
            if c.cc_be() { blk = 459; continue; }
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 459; continue; }
            blk = 119; continue;
        }
        119 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x70008c324d); }
            c.w64(11, c.r64(9).wrapping_add(0x1));
            blk = 120; continue;
        }
        120 => {
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c3254); };
            c.w64(2, t207);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008c325b); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008c325f); }
            c.w64(1, c.r64(11).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r208 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r208);
            let r209 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r209);
            c.logic(r209, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008c3272); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(11)) { return RegfileExit::fault(0x70008c3277); }
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t210) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008c3280); };
            c.w64(0, t210);
            c.w64(10, 0x70008c328eu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008c328e); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008c3292); }
            return c.call(c.r64(0), 0x70008c3298, 0x70008c3296);
        }
        121 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008c3298); }
            let Some(t211) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008c32a0); };
            c.w64(4, t211);
            let Some(t212) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c32a5); };
            c.w64(11, t212);
            let Some(t213) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c32ac); };
            c.w64(12, t213);
            c.w64(15, c.r64(12).wrapping_add(0x2c));
            let Some(t214) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c32b4); };
            c.w64(0, t214);
            let Some(t215) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008c32bb); };
            c.cmp(t215, c.r64(15), 64);
            if c.cc_be() { blk = 461; continue; }
            blk = 122; continue;
        }
        122 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008c32c4); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 123; continue;
        }
        123 => {
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c32cc); };
            c.w64(12, t216);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c32d0); }
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008c32d4); };
            c.w64(12, t217);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70008c32d8); }
            let Some(t218) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c32dc); };
            c.w64(12, t218);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008c32e3); }
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008c32e7); };
            c.w64(0, t219);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008c32ee); }
            c.w64(2, c.r64(11));
            c.w64(15, c.r64(0));
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c32f8); };
            c.w64(6, t220);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c32fc); };
            c.w64(11, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c3300); };
            c.w64(12, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c3304); };
            c.w64(0, t223);
            blk = 130; continue;
        }
        124 => {
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c330d); };
            c.w64(3, t224);
            let Some(t225) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c3314); };
            c.w64(7, t225);
            c.w64(8, c.r64(2).wrapping_add(c.r64(7)));
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c331b); };
            c.w64(9, t226);
            let Some(t227) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c3322); };
            c.cmp(t227, c.r64(8), 64);
            if c.cc_be() { blk = 465; continue; }
            blk = 125; continue;
        }
        125 => {
            if !c.st64(c.r64(3), c.r64(8)) { return RegfileExit::fault(0x70008c332b); }
            c.w64(3, c.r64(7).wrapping_add(0x1));
            blk = 126; continue;
        }
        126 => {
            let Some(t228) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c3332); };
            c.w64(2, (t228 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if let Some(sr) = c.shr(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r229 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r229);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008c333e); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x70008c3342); }
            let Some(t230) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c3345); };
            c.w64(15, t230);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008c334c); }
            c.w32(15, c.r64(1).wrapping_add(c.r64(1)));
            c.w64(2, c.r64(3).wrapping_add(0x2c));
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008c3358); }
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c335b); };
            c.w64(1, t231);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008c3362); }
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008c3365); }
            c.w64(15, c.r64(2));
            c.w64(2, c.r64(3));
            blk = 130; continue;
        }
        127 => {
            let Some(t232) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c3371); };
            c.w64(2, t232);
            let Some(t233) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008c3378); };
            c.w64(1, t233);
            c.w64(7, c.r64(1).wrapping_add(0x2c));
            let Some(t234) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c337f); };
            c.w64(8, t234);
            let Some(t235) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008c3386); };
            c.cmp(t235, c.r64(7), 64);
            if c.cc_be() { blk = 467; continue; }
            blk = 128; continue;
        }
        128 => {
            if !c.st64(c.r64(2), c.r64(7)) { return RegfileExit::fault(0x70008c338f); }
            c.w64(2, c.r64(1).wrapping_add(0x1));
            blk = 129; continue;
        }
        129 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008c3396); }
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008c339a); }
            let Some(t236) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c339d); };
            c.w64(15, t236);
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008c33a4); }
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008c33a8); }
            blk = 130; continue;
        }
        130 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x70008c33ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(15)) { return RegfileExit::fault(0x70008c33b0); }
            let Some(t237) = c.ld8(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c33b7); };
            c.logic(t237 & 0x1u64, 8);
            if c.cc_e() { blk = 132; continue; }
            blk = 131; continue;
        }
        131 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 133; continue;
        }
        132 => {
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c33c1); };
            c.w64(3, t238);
            blk = 133; continue;
        }
        133 => {
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c33c5); };
            c.w64(1, t239);
            if !c.st32(c.r64(2).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008c33c9); }
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c33cc); };
            c.w64(1, t240);
            if !c.st32(c.r64(2).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x70008c33d0); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r241 = c.r64(7) & c.r64(2);
            c.w64(7, r241);
            c.logic(r241, 64);
            let Some(t242) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c33e2); };
            c.logic(t242 & 0x4u64, 8);
            if c.cc_ne() { blk = 603; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t243) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c33ec); };
            c.w64(7, t243);
            if !c.st32(c.r64(2).wrapping_add(0x17), c.r32(7)) { return RegfileExit::fault(0x70008c33f3); }
            if !c.st32(c.r64(2).wrapping_add(0x1b), c.r32(7)) { return RegfileExit::fault(0x70008c33f6); }
            if !c.st32(c.r64(2).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x70008c33f9); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r244 = c.r64(8) & c.r64(2);
            c.w64(8, r244);
            c.logic(r244, 64);
            let Some(t245) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c340b); };
            c.logic(t245 & 0x4u64, 8);
            if c.cc_ne() { blk = 600; continue; }
            blk = 137; continue;
        }
        137 => {
            if !c.st32(c.r64(2).wrapping_add(0x23), c.r32(3)) { return RegfileExit::fault(0x70008c3416); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 139; continue; }
            blk = 138; continue;
        }
        138 => {
            if !c.st32(c.r64(2).wrapping_add(0x27), c.r32(7)) { return RegfileExit::fault(0x70008c341f); }
            blk = 139; continue;
        }
        139 => {
            let Some(t246) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c3422); };
            c.w64(3, (t246 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r247 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r247);
            c.w64(7, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 142; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t248) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c343f); };
            c.w64(8, t248);
            blk = 141; continue;
        }
        141 => {
            let r249 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r249);
            if !c.st32(c.r64(7).wrapping_add(c.r64(15)), c.r32(8)) { return RegfileExit::fault(0x70008c344a); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 141; continue; }
            blk = 142; continue;
        }
        142 => {
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008c3454); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 144; continue; }
            blk = 143; continue;
        }
        143 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r250 = c.r64(7) & c.r64(15);
            c.w64(7, r250);
            c.logic(r250, 64);
            let Some(t251) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c3466); };
            c.logic(t251 & 0x4u64, 8);
            if c.cc_ne() { blk = 597; continue; }
            blk = 144; continue;
        }
        144 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t252) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c3479); };
            c.w64(0, (t252 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r253 = c.add(c.r64(0), 0x3u64, 64);
            c.w64(0, r253);
            c.w32(3, 0x1u64);
            c.w32(7, 0x5u64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008c348e); }
            blk = 147; continue;
        }
        146 => {
            let r254 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r254);
            c.w64(15, c.r64(12));
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c349b); };
            c.w64(2, t255);
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c349f); };
            c.w64(6, t256);
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c34a3); };
            c.w64(1, t257);
            blk = 147; continue;
        }
        147 => {
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008c34a7); };
            c.w64(8, t258);
            let Some(t259) = c.ld32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c34ae); };
            c.w32(9, t259);
            let r260 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r260);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008c34b6); }
            c.cmp(c.r32(9), 0xfffffffffffffffeu64, 32);
            if c.cc_e() { blk = 152; continue; }
            blk = 148; continue;
        }
        148 => {
            c.w64(11, c.r64(7).wrapping_add(0x1));
            let Some(t261) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c34c4); };
            c.w32(11, t261);
            let r262 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r262);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008c34cc); }
            c.w64(0, c.r64(1));
            c.w64(3, c.r64(9));
            c.w64(1, c.r64(11));
            return c.call(0x700086fe54, 0x70008c34de, 0x70008c34d9);
        }
        149 => {
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c34de); };
            c.w64(3, t263);
            c.w64(11, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008c34ea); };
            c.w64(12, t264);
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008c34f1); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 151; continue; }
            blk = 150; continue;
        }
        150 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r265 = c.r64(15) & c.r64(12);
            c.w64(15, r265);
            c.logic(r265, 64);
            let Some(t266) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c3503); };
            c.logic(t266 & 0x4u64, 8);
            if c.cc_ne() { blk = 594; continue; }
            blk = 151; continue;
        }
        151 => {
            blk = 153; continue;
        }
        152 => {
            c.w64(12, c.r64(15));
            blk = 153; continue;
        }
        153 => {
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c3513); };
            c.w64(7, t267);
            let r268 = c.add(c.r64(7), 0x2u64, 64);
            c.w64(7, r268);
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c351b); };
            c.cmp(c.r64(7), t269, 64);
            if c.cc_b() { blk = 146; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c3525); };
            c.w64(0, t270);
            let Some(t271) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c3529); };
            c.w32(11, t271);
            let r272 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r272);
            let Some(t273) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c3530); };
            c.w32(15, t273);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x9u64, 64);
            if c.cc_be() { blk = 469; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t274) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008c3544); };
            c.w32(11, t274);
            let r275 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r275);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x70008c354b); }
            c.w32(3, 0);
            c.logic(0, 32);
            c.cmp(c.r32(11), c.r32(3), 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t276) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c355a); };
            c.w32(15, t276);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c355e); };
            c.w64(2, t277);
            if !c.st32(c.r64(2).wrapping_add(0x1b), c.r32(11)) { return RegfileExit::fault(0x70008c3562); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 158; continue; }
            blk = 157; continue;
        }
        157 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r278 = c.r64(1) & c.r64(2);
            c.w64(1, r278);
            c.logic(r278, 64);
            let Some(t279) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c3576); };
            c.logic(t279 & 0x4u64, 8);
            if c.cc_ne() { blk = 591; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c3580); };
            c.w64(6, t280);
            let Some(t281) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c3584); };
            c.w32(1, t281);
            let r282 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r282);
            let Some(t283) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c358a); };
            c.w32(1, t283);
            let r284 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r284);
            let Some(t285) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c3590); };
            c.w32(7, t285);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xb1u64, 64);
            if c.cc_a() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            return RegfileExit::fault(0x70008c35a2);
        }
        160 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w64(7, c.r64(15));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7));
            if let Some(sr) = c.shr(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(7, c.r64(8).wrapping_add(c.r64(7)).wrapping_sub(0x1));
            c.w64(8, c.r64(7));
            if let Some(sr) = c.shr(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r286 = c.r64(8) | c.r64(7);
            c.w64(8, r286);
            c.logic(r286, 64);
            c.w64(7, c.r64(8));
            if let Some(sr) = c.shr(c.r64(7), 0x2u64, 64) { c.w64(7, sr); }
            let r287 = c.r64(7) | c.r64(8);
            c.w64(7, r287);
            c.logic(r287, 64);
            c.w64(8, c.r64(7));
            if let Some(sr) = c.shr(c.r64(8), 0x4u64, 64) { c.w64(8, sr); }
            let r288 = c.r64(8) | c.r64(7);
            c.w64(8, r288);
            c.logic(r288, 64);
            c.w64(7, c.r64(8));
            if let Some(sr) = c.shr(c.r64(7), 0x8u64, 64) { c.w64(7, sr); }
            let r289 = c.r64(7) | c.r64(8);
            c.w64(7, r289);
            c.logic(r289, 64);
            c.w64(8, c.r64(7));
            if let Some(sr) = c.shr(c.r64(8), 0x10u64, 64) { c.w64(8, sr); }
            let r290 = c.r64(8) | c.r64(7);
            c.w64(8, r290);
            c.logic(r290, 64);
            c.w64(7, c.r64(8).wrapping_add(0x1));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008c35ef); }
            c.cmp(c.r64(7), 0x4u64, 64);
            if c.cc_ge() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            c.w32(7, 0x4u64);
            blk = 162; continue;
        }
        162 => {
            let Some(t291) = c.ld32(c.r64(1).wrapping_add(0x2cb)) else { return RegfileExit::fault(0x70008c35fe); };
            c.w32(1, t291);
            let r292 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r292);
            c.w64(8, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t293) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c360c); };
            c.w64(9, t293);
            let Some(t294) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c3613); };
            c.w64(9, t294);
            c.w64(0, c.r64(8).wrapping_mul(4).wrapping_add(0x8));
            c.w64(6, c.r64(0).wrapping_add(c.r64(9)));
            let Some(t295) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c3622); };
            c.w64(15, t295);
            let Some(t296) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008c3629); };
            c.cmp(t296, c.r64(6), 64);
            if c.cc_be() { blk = 470; continue; }
            blk = 163; continue;
        }
        163 => {
            c.cmp(c.r64(0), 0x20000u64, 64);
            if c.cc_ae() { blk = 470; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t297) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c363e); };
            c.w64(15, t297);
            if !c.st64(c.r64(15), c.r64(6)) { return RegfileExit::fault(0x70008c3645); }
            c.w64(15, c.r64(9).wrapping_add(0x1));
            blk = 165; continue;
        }
        165 => {
            c.w32(6, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t298) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008c3650); };
            c.w64(8, t298);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008c3657); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x70008c365b); }
            let Some(t299) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c365f); };
            c.w32(6, t299);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_be() { blk = 472; continue; }
            blk = 166; continue;
        }
        166 => {
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008c3673); }
            let Some(t300) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c3677); };
            c.w32(6, t300);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x1u64, 64);
            if c.cc_be() { blk = 473; continue; }
            blk = 167; continue;
        }
        167 => {
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008c368b); }
            let Some(t301) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c368f); };
            c.w32(6, t301);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x2u64, 64);
            if c.cc_be() { blk = 474; continue; }
            blk = 168; continue;
        }
        168 => {
            c.w32(6, c.r64(7).wrapping_add(c.r64(7)));
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(6)) { return RegfileExit::fault(0x70008c36a6); }
            let Some(t302) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c36aa); };
            c.w32(6, t302);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x3u64, 64);
            if c.cc_be() { blk = 475; continue; }
            blk = 169; continue;
        }
        169 => {
            c.w32(6, 0x2u64);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(6)) { return RegfileExit::fault(0x70008c36c3); }
            let Some(t303) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c36c7); };
            c.w32(6, t303);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x4u64, 64);
            if c.cc_be() { blk = 476; continue; }
            blk = 170; continue;
        }
        170 => {
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(3)) { return RegfileExit::fault(0x70008c36db); }
            c.w64(0, c.r64(15).wrapping_add(c.r64(0)).wrapping_sub(0x1));
            c.w64(6, c.r64(15).wrapping_add(0x1b));
            c.cmp(c.r64(6), c.r64(0), 64);
            if c.cc_e() { blk = 173; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t304) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c36ed); };
            c.w64(7, t304);
            blk = 172; continue;
        }
        172 => {
            if !c.st32(c.r64(6), c.r32(7)) { return RegfileExit::fault(0x70008c36f4); }
            let r305 = c.add(c.r64(6), 0x4u64, 64);
            c.w64(6, r305);
            c.cmp(c.r64(0), c.r64(6), 64);
            if c.cc_ne() { blk = 172; continue; }
            blk = 173; continue;
        }
        173 => {
            let Some(t306) = c.ld8(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c36ff); };
            c.w32(0, t306);
            let Some(t307) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c3703); };
            c.w64(6, t307);
            let Some(t308) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008c370a); };
            c.w64(7, t308);
            if let Some(sr) = c.shl(c.r64(0), 0x2u64, 64) { c.w64(0, sr); }
            c.w64(8, c.r64(0).wrapping_add(c.r64(7)));
            let Some(t309) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c3715); };
            c.w64(9, t309);
            let Some(t310) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c371c); };
            c.cmp(t310, c.r64(8), 64);
            if c.cc_be() { blk = 477; continue; }
            blk = 174; continue;
        }
        174 => {
            if !c.st64(c.r64(6), c.r64(8)) { return RegfileExit::fault(0x70008c3725); }
            c.w64(6, c.r64(7).wrapping_add(0x1));
            blk = 175; continue;
        }
        175 => {
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008c372c); }
            if !c.st32(c.r64(6).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008c372f); }
            let Some(t311) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c3733); };
            c.w64(1, t311);
            if !c.st32(c.r64(6).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008c373a); }
            let r312 = c.sub(c.r64(0), 0x1u64, 64);
            c.w64(0, r312);
            c.cmp(c.r64(0), 0xbu64, 64);
            if c.cc_e() { blk = 178; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t313) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c3747); };
            c.w64(1, t313);
            blk = 177; continue;
        }
        177 => {
            let r314 = c.sub(c.r64(0), 0x4u64, 64);
            c.w64(0, r314);
            if !c.st32(c.r64(0).wrapping_add(c.r64(6)), c.r32(1)) { return RegfileExit::fault(0x70008c3752); }
            c.cmp(c.r64(0), 0xbu64, 64);
            if c.cc_ne() { blk = 177; continue; }
            blk = 178; continue;
        }
        178 => {
            if !c.st32(c.r64(2).wrapping_add(0x17), c.r32(6)) { return RegfileExit::fault(0x70008c375b); }
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 180; continue; }
            blk = 179; continue;
        }
        179 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r315 = c.r64(0) & c.r64(2);
            c.w64(0, r315);
            c.logic(r315, 64);
            let Some(t316) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c376e); };
            c.logic(t316 & 0x4u64, 8);
            if c.cc_ne() { blk = 588; continue; }
            blk = 180; continue;
        }
        180 => {
            c.w64(0, c.r64(3));
            blk = 182; continue;
        }
        181 => {
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c377d); };
            c.w64(11, t317);
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008c3781); };
            c.w64(12, t318);
            blk = 182; continue;
        }
        182 => {
            let Some(t319) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c3788); };
            c.w32(6, t319);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(0), c.r64(6), 64);
            if c.cc_ae() { blk = 479; continue; }
            blk = 183; continue;
        }
        183 => {
            c.w64(7, c.r64(0).wrapping_add(0x1));
            let Some(t320) = c.ld32(c.r64(11).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c379f); };
            c.w32(0, t320);
            let r321 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r321);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008c37a7); }
            c.cmp(c.r64(7), c.r64(6), 64);
            if c.cc_ae() { blk = 480; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t322) = c.ld32(c.r64(11).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c37b4); };
            c.w32(6, t322);
            let Some(t323) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c37b9); };
            c.w32(8, t323);
            c.cmp(c.r32(6), c.r32(8), 32);
            if c.cc_ae() { blk = 481; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t324) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c37c7); };
            c.w32(8, t324);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t325) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c37d1); };
            c.w32(9, t325);
            let Some(t326) = c.ld32(c.r64(12).wrapping_add(c.r64(6).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c37d5); };
            c.w32(6, t326);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 482; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t327) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c37e7); };
            c.w32(2, t327);
            let r328 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r328);
            c.w64(11, c.r64(2));
            let r329 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r329);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r330 = c.r64(11) & 0xfffffffffffffffeu64;
            c.w64(11, r330);
            c.logic(r330, 64);
            let r331 = c.add(c.r32(11), c.r32(2), 32);
            c.w32(11, r331);
            c.w32(12, c.r32(9));
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_b() { blk = 489; continue; }
            blk = 187; continue;
        }
        187 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 483; continue; }
            blk = 188; continue;
        }
        188 => {
            let r332 = c.sub(c.r32(12), c.r32(2), 32);
            c.w32(12, r332);
            let r333 = c.shr(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r333);
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t334) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c381d); };
            c.w32(12, t334);
            let r335 = c.r64(11) & 0xfffffffffffffffeu64;
            c.w64(11, r335);
            c.logic(r335, 64);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_b() { blk = 489; continue; }
            blk = 189; continue;
        }
        189 => {
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_be() { blk = 484; continue; }
            blk = 190; continue;
        }
        190 => {
            let Some(t336) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c3838); };
            c.w32(11, t336);
            let r337 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r337);
            c.w32(12, c.r64(11).wrapping_add(0x2));
            c.cmp(c.r32(12), 0xfffffeu64, 32);
            if c.cc_a() { blk = 489; continue; }
            blk = 191; continue;
        }
        191 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(12)) { return RegfileExit::fault(0x70008c3853); }
            let Some(t338) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c3857); };
            c.w32(12, t338);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 485; continue; }
            blk = 192; continue;
        }
        192 => {
            c.w64(12, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c386e); }
            let Some(t339) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c3872); };
            c.w32(12, t339);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r340 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r340);
            let Some(t341) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c3880); };
            c.w32(2, t341);
            let r342 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r342);
            let r343 = c.r64(2) & c.r64(12);
            c.w64(2, r343);
            c.logic(r343, 64);
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 194; continue;
        }
        193 => {
            let r344 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r344);
            c.w64(3, c.r64(8).wrapping_add(c.r64(2)));
            c.w64(2, c.r64(12));
            let r345 = c.r64(2) & c.r64(3);
            c.w64(2, r345);
            c.logic(r345, 64);
            blk = 194; continue;
        }
        194 => {
            c.w64(9, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(3, c.r64(9).wrapping_mul(4));
            c.w64(1, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let Some(t346) = c.ld32(c.r64(15).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c38b1); };
            c.w32(7, t346);
            let Some(t347) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c38b6); };
            c.cmp(t347, c.r32(7), 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t348) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008c38bf); };
            c.cmp(t348, c.r32(7), 32);
            if c.cc_ne() { blk = 193; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t349) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c38c8); };
            c.w32(12, t349);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 486; continue; }
            blk = 197; continue;
        }
        197 => {
            if !c.st32(c.r64(15).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x70008c38db); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 199; continue; }
            blk = 198; continue;
        }
        198 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r350 = c.r64(12) & c.r64(15);
            c.w64(12, r350);
            c.logic(r350, 64);
            let Some(t351) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c38ed); };
            c.logic(t351 & 0x4u64, 8);
            if c.cc_ne() { blk = 585; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t352) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c38f9); };
            c.w32(12, t352);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(2, c.r64(9).wrapping_add(0x1));
            c.cmp(c.r64(2), c.r64(12), 64);
            if c.cc_ae() { blk = 487; continue; }
            blk = 200; continue;
        }
        200 => {
            if let Some(sr) = c.shl(c.r64(11), 0x8u64, 64) { c.w64(11, sr); }
            c.w32(12, c.r32(11));
            c.w64(2, c.r64(9).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(15).wrapping_add(c.r64(2)), c.r32(6)) { return RegfileExit::fault(0x70008c391f); }
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 202; continue; }
            blk = 201; continue;
        }
        201 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r353 = c.r64(1) & c.r64(15);
            c.w64(1, r353);
            c.logic(r353, 64);
            let Some(t354) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c3933); };
            c.logic(t354 & 0x4u64, 8);
            if c.cc_ne() { blk = 582; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t355) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c393d); };
            c.w32(2, t355);
            let Some(t356) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008c3940); };
            c.cmp(t356, c.r32(2), 32);
            if c.cc_ne() { blk = 205; continue; }
            blk = 203; continue;
        }
        203 => {
            let Some(t357) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3949); };
            c.logic(t357 & 0x1u64, 8);
            if c.cc_e() { blk = 205; continue; }
            blk = 204; continue;
        }
        204 => {
            let r358 = c.r32(12) | 0x20u64;
            c.w32(12, r358);
            c.logic(r358, 32);
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 206; continue;
        }
        205 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 206; continue;
        }
        206 => {
            let Some(t359) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c395b); };
            c.w32(12, t359);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(0, c.r64(9).wrapping_add(0x2));
            c.cmp(c.r64(0), c.r64(12), 64);
            if c.cc_ae() { blk = 488; continue; }
            blk = 207; continue;
        }
        207 => {
            if !c.st32(c.r64(15).wrapping_add(c.r64(3)).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008c3972); }
            let Some(t360) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c3977); };
            c.w64(11, t360);
            c.w64(0, c.r64(11).wrapping_add(0x1));
            let Some(t361) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c397f); };
            c.w64(11, t361);
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_g() { blk = 181; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t362) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c398c); };
            c.w64(0, t362);
            blk = 210; continue;
        }
        209 => {
            c.w64(0, c.r64(11));
            blk = 210; continue;
        }
        210 => {
            let Some(t363) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c3995); };
            c.cmp(t363, 0x0u64, 32);
            if c.cc_ne() { blk = 212; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t364) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c399b); };
            c.w64(11, t364);
            blk = 215; continue;
        }
        212 => {
            let Some(t365) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c39a1); };
            c.w64(11, t365);
            let Some(t366) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008c39a5); };
            c.w32(12, t366);
            let r367 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r367);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c39ac); }
            let Some(t368) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008c39ad); };
            if !c.push64(t368) { return RegfileExit::fault(0x70008c39ad); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008c39b3); }
            let Some(t369) = c.ld64(c.r64(13).wrapping_add(0x2640)) else { return RegfileExit::fault(0x70008c39b5); };
            c.w64(3, t369);
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c39bc); };
            c.w64(6, t370);
            c.w32(0, 0x3u64);
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x70008c39cd, 0x70008c39c8);
        }
        213 => {
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c39cd); };
            c.w64(11, t371);
            if !c.st32(c.r64(11).wrapping_add(0x27), c.r32(0)) { return RegfileExit::fault(0x70008c39d1); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 215; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r372 = c.r64(12) & c.r64(11);
            c.w64(12, r372);
            c.logic(r372, 64);
            let Some(t373) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c39e3); };
            c.logic(t373 & 0x4u64, 8);
            if c.cc_ne() { blk = 579; continue; }
            blk = 215; continue;
        }
        215 => {
            c.w32(12, 0x1u64);
            blk = 229; continue;
        }
        216 => {
            c.w64(3, c.r64(1));
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(3)) { return RegfileExit::fault(0x70008c39fd); }
            blk = 395; continue;
        }
        217 => {
            let Some(t374) = c.ld64(c.r64(13).wrapping_add(0x6e8)) else { return RegfileExit::fault(0x70008c3a05); };
            c.w64(2, t374);
            c.w64(3, c.r64(2));
            return c.call(0x7000870914, 0x70008c3a14, 0x70008c3a0f);
        }
        218 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 225; continue; }
            blk = 219; continue;
        }
        219 => {
            let Some(t375) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c3a18); };
            c.w32(11, t375);
            let r376 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r376);
            let Some(t377) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008c3a1f); };
            c.logic(t377 & 0x2u64, 8);
            if c.cc_e() { blk = 225; continue; }
            blk = 220; continue;
        }
        220 => {
            let Some(t378) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c3a26); };
            if !c.push64(t378) { return RegfileExit::fault(0x70008c3a26); }
            let Some(t379) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c3a29); };
            if !c.push64(t379) { return RegfileExit::fault(0x70008c3a29); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c3a34); };
            c.w64(6, t380);
            return c.call(0x70007f2274, 0x70008c3a3d, 0x70008c3a38);
        }
        221 => {
            let Some(t381) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c3a3d); };
            c.cmp(t381, c.r32(0), 32);
            if c.cc_e() { blk = 227; continue; }
            blk = 222; continue;
        }
        222 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 490; continue; }
            blk = 223; continue;
        }
        223 => {
            let Some(t382) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c3a4e); };
            c.w32(11, t382);
            let r383 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r383);
            let Some(t384) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3a55); };
            c.cmp(t384, 0xbau64, 16);
            if c.cc_ae() { blk = 227; continue; }
            blk = 224; continue;
        }
        224 => {
            blk = 490; continue;
        }
        225 => {
            let Some(t385) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c3a63); };
            c.w64(0, t385);
            let Some(t386) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c3a67); };
            c.w32(11, t386);
            let r387 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r387);
            let Some(t388) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3a6e); };
            c.cmp(t388, 0x83eu64, 16);
            if c.cc_ne() { blk = 396; continue; }
            blk = 226; continue;
        }
        226 => {
            let Some(t389) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c3a7b); };
            c.w64(6, t389);
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c3a7f); };
            c.w64(3, t390);
            return c.call(0x70008bf674, 0x70008c3a88, 0x70008c3a83);
        }
        227 => {
            let Some(t391) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c3a88); };
            c.cmp(t391, c.r32(0), 32);
            if c.cc_e() { blk = 394; continue; }
            blk = 228; continue;
        }
        228 => {
            c.w32(12, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(0));
            blk = 229; continue;
        }
        229 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008c3a9b); }
            let Some(t392) = c.ld8(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008c3a9f); };
            c.logic(t392 & 0x4u64, 8);
            if c.cc_e() { blk = 389; continue; }
            blk = 230; continue;
        }
        230 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 315; continue; }
            blk = 231; continue;
        }
        231 => {
            c.w64(0, c.r64(11));
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t393) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c3abb); };
            c.w64(6, t393);
            return c.call(0x7000870914, 0x70008c3ac4, 0x70008c3abf);
        }
        232 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 492; continue; }
            blk = 233; continue;
        }
        233 => {
            let Some(t394) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c3acc); };
            c.w32(11, t394);
            let r395 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r395);
            let Some(t396) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3ad3); };
            c.cmp(t396, 0x40u64, 16);
            if c.cc_ae() { blk = 492; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t397) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c3adf); };
            c.cmp(t397, c.r32(0), 32);
            if c.cc_ne() { blk = 307; continue; }
            blk = 235; continue;
        }
        235 => {
            let Some(t398) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008c3aec); };
            c.w64(3, t398);
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c3af3); };
            c.w64(0, t399);
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c3af7); };
            c.w64(6, t400);
            return c.call(0x7000870914, 0x70008c3b00, 0x70008c3afb);
        }
        236 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 239; continue; }
            blk = 237; continue;
        }
        237 => {
            let Some(t401) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c3b04); };
            c.w64(6, t401);
            return c.call(0x70008a17f4, 0x70008c3b0d, 0x70008c3b08);
        }
        238 => {
            blk = 241; continue;
        }
        239 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 241; continue; }
            blk = 240; continue;
        }
        240 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 241; continue;
        }
        241 => {
            c.w32(11, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 243; continue; }
            blk = 242; continue;
        }
        242 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c3b1d); }
            blk = 246; continue;
        }
        243 => {
            let r402 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r402);
            if c.cc_o() { blk = 245; continue; }
            blk = 244; continue;
        }
        244 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 249; continue;
        }
        245 => {
            c.w64(11, c.r64(0));
            let r403 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r403);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 246; continue;
        }
        246 => {
            let Some(t404) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c3b3e); };
            c.w64(11, t404);
            let Some(t405) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c3b45); };
            c.w64(12, t405);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t406) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c3b4d); };
            c.w64(3, t406);
            let Some(t407) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c3b54); };
            c.cmp(t407, c.r64(15), 64);
            if c.cc_be() { blk = 494; continue; }
            blk = 247; continue;
        }
        247 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008c3b5d); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 248; continue;
        }
        248 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t408) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c3b77); };
            c.w64(12, t408);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c3b7e); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008c3b82); }
            blk = 249; continue;
        }
        249 => {
            let Some(t409) = c.ld8(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008c3b88); };
            c.logic(t409 & 0x8u64, 8);
            if c.cc_e() { blk = 306; continue; }
            blk = 250; continue;
        }
        250 => {
            c.w32(12, c.r32(11));
            c.logic(c.r32(11) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 306; continue; }
            blk = 251; continue;
        }
        251 => {
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c3ba8); };
            c.w64(3, t410);
            let Some(t411) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3bac); };
            c.w32(2, t411);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), c.r64(2), 64);
            if c.cc_ae() { blk = 306; continue; }
            blk = 252; continue;
        }
        252 => {
            let Some(t412) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c3bbb); };
            c.w32(2, t412);
            let r413 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r413);
            let Some(t414) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3bc1); };
            c.w32(2, t414);
            c.logic(c.r8(2) & 0x7u64, 8);
            if c.cc_e() { blk = 268; continue; }
            blk = 253; continue;
        }
        253 => {
            c.w64(6, c.r64(2));
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(3));
            blk = 254; continue;
        }
        254 => {
            c.w64(7, c.r64(6));
            let r415 = c.r32(7) & 0x7u64;
            c.w32(7, r415);
            c.logic(r415, 32);
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_l() { blk = 259; continue; }
            blk = 255; continue;
        }
        255 => {
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_e() { blk = 267; continue; }
            blk = 256; continue;
        }
        256 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 264; continue; }
            blk = 257; continue;
        }
        257 => {
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_e() { blk = 265; continue; }
            blk = 258; continue;
        }
        258 => {
            blk = 496; continue;
        }
        259 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 266; continue; }
            blk = 260; continue;
        }
        260 => {
            c.cmp(c.r32(7), 0x1u64, 32);
            if c.cc_e() { blk = 262; continue; }
            blk = 261; continue;
        }
        261 => {
            blk = 496; continue;
        }
        262 => {
            let Some(t416) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c3c08); };
            c.w32(6, t416);
            let Some(t417) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c3c0b); };
            c.cmp(t417, c.r32(6), 32);
            if c.cc_ne() { blk = 497; continue; }
            blk = 263; continue;
        }
        263 => {
            let Some(t418) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c3c18); };
            c.w32(1, t418);
            let r419 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r419);
            let Some(t420) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c3c1e); };
            c.w32(6, t420);
            let r421 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r421);
            let Some(t422) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3c24); };
            c.w32(6, t422);
            blk = 254; continue;
        }
        264 => {
            let Some(t423) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c3c2a); };
            c.w32(6, t423);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r424 = c.add(c.r64(2), c.r64(6), 64);
            c.w64(2, r424);
            let Some(t425) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c3c36); };
            c.w32(1, t425);
            let r426 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r426);
            let Some(t427) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c3c3c); };
            c.w32(6, t427);
            let r428 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r428);
            let Some(t429) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3c42); };
            c.w32(6, t429);
            blk = 254; continue;
        }
        265 => {
            let Some(t430) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c3c48); };
            c.w32(1, t430);
            let r431 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r431);
            let Some(t432) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c3c4e); };
            c.w32(6, t432);
            let r433 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r433);
            let Some(t434) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3c54); };
            c.w32(6, t434);
            blk = 254; continue;
        }
        266 => {
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(2));
            c.w64(2, c.r64(10));
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(6));
            c.w64(6, c.r64(10));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 269; continue;
        }
        267 => {
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(2));
            c.w64(2, c.r64(10));
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(6));
            c.w64(6, c.r64(10));
            c.w32(7, 0x1u64);
            blk = 269; continue;
        }
        268 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(3));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 269; continue;
        }
        269 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 498; continue; }
            blk = 270; continue;
        }
        270 => {
            c.w64(0, c.r64(6).wrapping_add(0xb));
            blk = 271; continue;
        }
        271 => {
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r435 = c.r32(2) & 0x8u64;
            c.w32(2, r435);
            c.logic(r435, 32);
            let r436 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r436);
            c.cmp(c.r32(2), 0x8u64, 32);
            if c.cc_e() { blk = 273; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t437) = c.ld16(c.r64(0).wrapping_add(c.r64(1).wrapping_mul(2))) else { return RegfileExit::fault(0x70008c3cb1); };
            c.w32(0, t437);
            blk = 274; continue;
        }
        273 => {
            let Some(t438) = c.ld8(c.r64(1).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70008c3cb7); };
            c.w32(0, t438);
            blk = 274; continue;
        }
        274 => {
            let r439 = c.r32(0) & 0xfc00u64;
            c.w32(0, r439);
            c.logic(r439, 32);
            c.cmp(c.r32(0), 0xd800u64, 32);
            if c.cc_ne() { blk = 306; continue; }
            blk = 275; continue;
        }
        275 => {
            let Some(t440) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c3ccb); };
            c.w32(0, t440);
            let r441 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r441);
            let Some(t442) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3cd1); };
            c.w32(0, t442);
            c.logic(c.r8(0) & 0x7u64, 8);
            if c.cc_e() { blk = 291; continue; }
            blk = 276; continue;
        }
        276 => {
            c.w64(1, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(3));
            blk = 277; continue;
        }
        277 => {
            c.w64(6, c.r64(1));
            let r443 = c.r32(6) & 0x7u64;
            c.w32(6, r443);
            c.logic(r443, 32);
            c.cmp(c.r32(6), 0x2u64, 32);
            if c.cc_l() { blk = 282; continue; }
            blk = 278; continue;
        }
        278 => {
            c.cmp(c.r32(6), 0x2u64, 32);
            if c.cc_e() { blk = 290; continue; }
            blk = 279; continue;
        }
        279 => {
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 287; continue; }
            blk = 280; continue;
        }
        280 => {
            c.cmp(c.r32(6), 0x5u64, 32);
            if c.cc_e() { blk = 288; continue; }
            blk = 281; continue;
        }
        281 => {
            blk = 513; continue;
        }
        282 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 289; continue; }
            blk = 283; continue;
        }
        283 => {
            c.cmp(c.r32(6), 0x1u64, 32);
            if c.cc_e() { blk = 285; continue; }
            blk = 284; continue;
        }
        284 => {
            blk = 513; continue;
        }
        285 => {
            let Some(t444) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c3d17); };
            c.w32(1, t444);
            let Some(t445) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c3d1a); };
            c.cmp(t445, c.r32(1), 32);
            if c.cc_ne() { blk = 514; continue; }
            blk = 286; continue;
        }
        286 => {
            let Some(t446) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c3d27); };
            c.w32(2, t446);
            let r447 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r447);
            let Some(t448) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c3d2d); };
            c.w32(1, t448);
            let r449 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r449);
            let Some(t450) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3d33); };
            c.w32(1, t450);
            blk = 277; continue;
        }
        287 => {
            let Some(t451) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c3d39); };
            c.w32(1, t451);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r452 = c.add(c.r64(0), c.r64(1), 64);
            c.w64(0, r452);
            let Some(t453) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c3d45); };
            c.w32(2, t453);
            let r454 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r454);
            let Some(t455) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c3d4b); };
            c.w32(1, t455);
            let r456 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r456);
            let Some(t457) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3d51); };
            c.w32(1, t457);
            blk = 277; continue;
        }
        288 => {
            let Some(t458) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c3d57); };
            c.w32(2, t458);
            let r459 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r459);
            let Some(t460) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c3d5d); };
            c.w32(1, t460);
            let r461 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r461);
            let Some(t462) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3d63); };
            c.w32(1, t462);
            blk = 277; continue;
        }
        289 => {
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(10));
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 292; continue;
        }
        290 => {
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(10));
            c.w32(6, 0x1u64);
            blk = 292; continue;
        }
        291 => {
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(3));
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 292; continue;
        }
        292 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 515; continue; }
            blk = 293; continue;
        }
        293 => {
            c.w64(3, c.r64(1).wrapping_add(0xb));
            blk = 294; continue;
        }
        294 => {
            let r463 = c.add(c.r64(15), c.r64(2), 64);
            c.w64(15, r463);
            let r464 = c.r32(0) & 0x8u64;
            c.w32(0, r464);
            c.logic(r464, 32);
            c.cmp(c.r32(0), 0x8u64, 32);
            if c.cc_e() { blk = 296; continue; }
            blk = 295; continue;
        }
        295 => {
            let Some(t465) = c.ld16(c.r64(3).wrapping_add(c.r64(15).wrapping_mul(2))) else { return RegfileExit::fault(0x70008c3dba); };
            c.w32(15, t465);
            blk = 297; continue;
        }
        296 => {
            let Some(t466) = c.ld8(c.r64(15).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x70008c3dc1); };
            c.w32(15, t466);
            blk = 297; continue;
        }
        297 => {
            let r467 = c.r32(15) & 0xfc00u64;
            c.w32(15, r467);
            c.logic(r467, 32);
            c.cmp(c.r32(15), 0xdc00u64, 32);
            if c.cc_ne() { blk = 306; continue; }
            blk = 298; continue;
        }
        298 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 300; continue; }
            blk = 299; continue;
        }
        299 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c3ddc); }
            blk = 303; continue;
        }
        300 => {
            let r468 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r468);
            if c.cc_o() { blk = 302; continue; }
            blk = 301; continue;
        }
        301 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 306; continue;
        }
        302 => {
            let r469 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r469);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 303; continue;
        }
        303 => {
            let Some(t470) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c3dfb); };
            c.w64(11, t470);
            let Some(t471) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c3e02); };
            c.w64(12, t471);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t472) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c3e0a); };
            c.w64(0, t472);
            let Some(t473) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008c3e11); };
            c.cmp(t473, c.r64(15), 64);
            if c.cc_be() { blk = 530; continue; }
            blk = 304; continue;
        }
        304 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008c3e1a); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 305; continue;
        }
        305 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t474) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c3e34); };
            c.w64(12, t474);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c3e3b); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008c3e3f); }
            blk = 306; continue;
        }
        306 => {
            let Some(t475) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c3e45); };
            c.w64(0, t475);
            let Some(t476) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008c3e49); };
            c.w64(3, t476);
            c.w64(1, c.r64(11));
            let Some(t477) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c3e53); };
            c.w64(6, t477);
            return c.call(0x7000871cb4, 0x70008c3e5c, 0x70008c3e57);
        }
        307 => {
            let Some(t478) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c3e5c); };
            c.w64(6, t478);
            let Some(t479) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c3e60); };
            c.w32(11, t479);
            let r480 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r480);
            let Some(t481) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c3e67); };
            c.w32(11, t481);
            let r482 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r482);
            let Some(t483) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c3e6e); };
            c.w32(12, t483);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x68u64, 64);
            if c.cc_a() { blk = 309; continue; }
            blk = 308; continue;
        }
        308 => {
            return RegfileExit::fault(0x70008c3e7e);
        }
        309 => {
            let Some(t484) = c.ld32(c.r64(11).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x70008c3e7f); };
            c.w32(11, t484);
            let Some(t485) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c3e86); };
            c.w64(12, t485);
            let Some(t486) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008c3e8d); };
            c.w64(15, t486);
            c.w64(0, c.r64(15).wrapping_add(0x14));
            let Some(t487) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c3e95); };
            c.w64(3, t487);
            let Some(t488) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c3e9c); };
            c.cmp(t488, c.r64(0), 64);
            if c.cc_be() { blk = 532; continue; }
            blk = 310; continue;
        }
        310 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008c3ea5); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 311; continue;
        }
        311 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008c3ead); }
            let Some(t489) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c3eb2); };
            c.w64(11, t489);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008c3eb9); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008c3ebe); }
            let Some(t490) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c3ec3); };
            c.w64(0, t490);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008c3ec7); }
            let Some(t491) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008c3ecc); };
            c.w64(11, t491);
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008c3ed3); }
            c.w64(0, c.r64(12));
            blk = 312; continue;
        }
        312 => {
            let Some(t492) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c3edb); };
            c.w64(1, t492);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008c3ee2); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 314; continue; }
            blk = 313; continue;
        }
        313 => {
            return c.ret(0x8, 0x70008c3ee9);
        }
        314 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008c3eec); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008c3ef3); }
            return c.ret(0x0, 0x70008c3ef5);
        }
        315 => {
            let Some(t493) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3ef6); };
            c.w32(12, t493);
            let r494 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r494);
            let Some(t495) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c3efd); };
            c.w32(15, t495);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_a() { blk = 317; continue; }
            blk = 316; continue;
        }
        316 => {
            return RegfileExit::fault(0x70008c3f0e);
        }
        317 => {
            let Some(t496) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3f0f); };
            c.w32(12, t496);
            let Some(t497) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c3f14); };
            c.cmp(t497, c.r32(12), 32);
            if c.cc_ne() { blk = 384; continue; }
            blk = 318; continue;
        }
        318 => {
            let Some(t498) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c3f21); };
            c.w64(12, t498);
            let Some(t499) = c.ld32(c.r64(12).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c3f25); };
            c.w32(15, t499);
            let r500 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r500);
            let Some(t501) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c3f2d); };
            c.w64(0, t501);
            let r502 = c.r32(0) & 0x8u64;
            c.w32(0, r502);
            c.logic(r502, 32);
            c.w32(3, c.r32(15));
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 320; continue; }
            blk = 319; continue;
        }
        319 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c3f3d); }
            blk = 323; continue;
        }
        320 => {
            let r503 = c.add(c.r32(3), 0x2u64, 32);
            c.w32(3, r503);
            if c.cc_o() { blk = 322; continue; }
            blk = 321; continue;
        }
        321 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 326; continue;
        }
        322 => {
            c.w64(3, c.r64(15));
            let r504 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r504);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 323; continue;
        }
        323 => {
            let Some(t505) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c3f5c); };
            c.w64(3, t505);
            let Some(t506) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c3f63); };
            c.w64(2, t506);
            c.w64(1, c.r64(2).wrapping_add(0xc));
            let Some(t507) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c3f6a); };
            c.w64(6, t507);
            let Some(t508) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008c3f71); };
            c.cmp(t508, c.r64(1), 64);
            if c.cc_be() { blk = 534; continue; }
            blk = 324; continue;
        }
        324 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x70008c3f7a); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 325; continue;
        }
        325 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t509) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c3f93); };
            c.w64(2, t509);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008c3f9a); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008c3f9d); }
            blk = 326; continue;
        }
        326 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 383; continue; }
            blk = 327; continue;
        }
        327 => {
            c.w32(0, c.r32(3));
            c.logic(c.r32(3) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 383; continue; }
            blk = 328; continue;
        }
        328 => {
            c.w64(2, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t510) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c3fbc); };
            c.w64(1, t510);
            let Some(t511) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3fc0); };
            c.w32(6, t511);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), c.r64(6), 64);
            if c.cc_ae() { blk = 383; continue; }
            blk = 329; continue;
        }
        329 => {
            let Some(t512) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c3fcf); };
            c.w32(6, t512);
            let r513 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r513);
            let Some(t514) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c3fd5); };
            c.w32(6, t514);
            c.logic(c.r8(6) & 0x7u64, 8);
            if c.cc_e() { blk = 345; continue; }
            blk = 330; continue;
        }
        330 => {
            c.w64(8, c.r64(6));
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(1));
            blk = 331; continue;
        }
        331 => {
            c.w64(9, c.r64(8));
            let r515 = c.r32(9) & 0x7u64;
            c.w32(9, r515);
            c.logic(r515, 32);
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 336; continue; }
            blk = 332; continue;
        }
        332 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 344; continue; }
            blk = 333; continue;
        }
        333 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 341; continue; }
            blk = 334; continue;
        }
        334 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 342; continue; }
            blk = 335; continue;
        }
        335 => {
            blk = 536; continue;
        }
        336 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 343; continue; }
            blk = 337; continue;
        }
        337 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 339; continue; }
            blk = 338; continue;
        }
        338 => {
            blk = 536; continue;
        }
        339 => {
            let Some(t516) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c4024); };
            c.w32(8, t516);
            let Some(t517) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c4028); };
            c.cmp(t517, c.r32(8), 32);
            if c.cc_ne() { blk = 537; continue; }
            blk = 340; continue;
        }
        340 => {
            let Some(t518) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c4035); };
            c.w32(7, t518);
            let r519 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r519);
            let Some(t520) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c403b); };
            c.w32(8, t520);
            let r521 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r521);
            let Some(t522) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c4042); };
            c.w32(8, t522);
            blk = 331; continue;
        }
        341 => {
            let Some(t523) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c4049); };
            c.w32(8, t523);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r524 = c.add(c.r64(6), c.r64(8), 64);
            c.w64(6, r524);
            let Some(t525) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c4056); };
            c.w32(7, t525);
            let r526 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r526);
            let Some(t527) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c405c); };
            c.w32(8, t527);
            let r528 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r528);
            let Some(t529) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c4063); };
            c.w32(8, t529);
            blk = 331; continue;
        }
        342 => {
            let Some(t530) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c406a); };
            c.w32(7, t530);
            let r531 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r531);
            let Some(t532) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c4070); };
            c.w32(8, t532);
            let r533 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r533);
            let Some(t534) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c4077); };
            c.w32(8, t534);
            blk = 331; continue;
        }
        343 => {
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(6));
            c.w64(6, c.r64(10));
            c.w64(10, c.r64(7));
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(10));
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 346; continue;
        }
        344 => {
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(6));
            c.w64(6, c.r64(10));
            c.w64(10, c.r64(7));
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(10));
            c.w32(9, 0x1u64);
            blk = 346; continue;
        }
        345 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(1));
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 346; continue;
        }
        346 => {
            c.w64(11, (c.r32(15) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 538; continue; }
            blk = 347; continue;
        }
        347 => {
            c.w64(15, c.r64(8).wrapping_add(0xb));
            blk = 348; continue;
        }
        348 => {
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r535 = c.r32(6) & 0x8u64;
            c.w32(6, r535);
            c.logic(r535, 32);
            let r536 = c.add(c.r64(11), c.r64(7), 64);
            c.w64(11, r536);
            c.cmp(c.r32(6), 0x8u64, 32);
            if c.cc_e() { blk = 350; continue; }
            blk = 349; continue;
        }
        349 => {
            let Some(t537) = c.ld16(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(2))) else { return RegfileExit::fault(0x70008c40d9); };
            c.w32(11, t537);
            blk = 351; continue;
        }
        350 => {
            let Some(t538) = c.ld8(c.r64(11).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x70008c40e0); };
            c.w32(11, t538);
            blk = 351; continue;
        }
        351 => {
            let r539 = c.r32(11) & 0xfc00u64;
            c.w32(11, r539);
            c.logic(r539, 32);
            c.cmp(c.r32(11), 0xd800u64, 32);
            if c.cc_ne() { blk = 383; continue; }
            blk = 352; continue;
        }
        352 => {
            let Some(t540) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c40f9); };
            c.w32(11, t540);
            let r541 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r541);
            let Some(t542) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c4100); };
            c.w32(11, t542);
            c.logic(c.r8(11) & 0x7u64, 8);
            if c.cc_e() { blk = 368; continue; }
            blk = 353; continue;
        }
        353 => {
            c.w64(6, c.r64(11));
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(1));
            blk = 354; continue;
        }
        354 => {
            c.w64(7, c.r64(6));
            let r543 = c.r32(7) & 0x7u64;
            c.w32(7, r543);
            c.logic(r543, 32);
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_l() { blk = 359; continue; }
            blk = 355; continue;
        }
        355 => {
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_e() { blk = 367; continue; }
            blk = 356; continue;
        }
        356 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 364; continue; }
            blk = 357; continue;
        }
        357 => {
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_e() { blk = 365; continue; }
            blk = 358; continue;
        }
        358 => {
            blk = 554; continue;
        }
        359 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 366; continue; }
            blk = 360; continue;
        }
        360 => {
            c.cmp(c.r32(7), 0x1u64, 32);
            if c.cc_e() { blk = 362; continue; }
            blk = 361; continue;
        }
        361 => {
            blk = 554; continue;
        }
        362 => {
            let Some(t544) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c414a); };
            c.w32(6, t544);
            let Some(t545) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c414e); };
            c.cmp(t545, c.r32(6), 32);
            if c.cc_ne() { blk = 555; continue; }
            blk = 363; continue;
        }
        363 => {
            let Some(t546) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c415b); };
            c.w32(15, t546);
            let r547 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r547);
            let Some(t548) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c4162); };
            c.w32(6, t548);
            let r549 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r549);
            let Some(t550) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c4169); };
            c.w32(6, t550);
            blk = 354; continue;
        }
        364 => {
            let Some(t551) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c416f); };
            c.w32(6, t551);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r552 = c.add(c.r64(11), c.r64(6), 64);
            c.w64(11, r552);
            let Some(t553) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c417c); };
            c.w32(15, t553);
            let r554 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r554);
            let Some(t555) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c4183); };
            c.w32(6, t555);
            let r556 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r556);
            let Some(t557) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c418a); };
            c.w32(6, t557);
            blk = 354; continue;
        }
        365 => {
            let Some(t558) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c4190); };
            c.w32(15, t558);
            let r559 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r559);
            let Some(t560) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c4197); };
            c.w32(6, t560);
            let r561 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r561);
            let Some(t562) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c419e); };
            c.w32(6, t562);
            blk = 354; continue;
        }
        366 => {
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(11));
            c.w64(11, c.r64(10));
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(6));
            c.w64(6, c.r64(10));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 369; continue;
        }
        367 => {
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(11));
            c.w64(11, c.r64(10));
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(6));
            c.w64(6, c.r64(10));
            c.w32(7, 0x1u64);
            blk = 369; continue;
        }
        368 => {
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 369; continue;
        }
        369 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 556; continue; }
            blk = 370; continue;
        }
        370 => {
            c.w64(1, c.r64(6).wrapping_add(0xb));
            blk = 371; continue;
        }
        371 => {
            let r563 = c.add(c.r64(15), c.r64(2), 64);
            c.w64(15, r563);
            let r564 = c.r32(11) & 0x8u64;
            c.w32(11, r564);
            c.logic(r564, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 373; continue; }
            blk = 372; continue;
        }
        372 => {
            let Some(t565) = c.ld16(c.r64(1).wrapping_add(c.r64(15).wrapping_mul(2))) else { return RegfileExit::fault(0x70008c41f8); };
            c.w32(11, t565);
            blk = 374; continue;
        }
        373 => {
            let Some(t566) = c.ld8(c.r64(15).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008c41ff); };
            c.w32(11, t566);
            blk = 374; continue;
        }
        374 => {
            let r567 = c.r32(11) & 0xfc00u64;
            c.w32(11, r567);
            c.logic(r567, 32);
            c.cmp(c.r32(11), 0xdc00u64, 32);
            if c.cc_ne() { blk = 383; continue; }
            blk = 375; continue;
        }
        375 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 377; continue; }
            blk = 376; continue;
        }
        376 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c4219); }
            blk = 380; continue;
        }
        377 => {
            let r568 = c.add(c.r32(0), 0x2u64, 32);
            c.w32(0, r568);
            if c.cc_o() { blk = 379; continue; }
            blk = 378; continue;
        }
        378 => {
            c.w64(3, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 383; continue;
        }
        379 => {
            let r569 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r569);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 380; continue;
        }
        380 => {
            let Some(t570) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c4234); };
            c.w64(11, t570);
            let Some(t571) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c423b); };
            c.w64(15, t571);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t572) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c4242); };
            c.w64(3, t572);
            let Some(t573) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c4249); };
            c.cmp(t573, c.r64(0), 64);
            if c.cc_be() { blk = 571; continue; }
            blk = 381; continue;
        }
        381 => {
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x70008c4252); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            blk = 382; continue;
        }
        382 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t574) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c426b); };
            c.w64(15, t574);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008c4272); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008c4276); }
            c.w64(3, c.r64(11));
            blk = 383; continue;
        }
        383 => {
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(3)) { return RegfileExit::fault(0x70008c427f); }
            blk = 384; continue;
        }
        384 => {
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c4284); };
            c.w64(6, t575);
            let Some(t576) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c4288); };
            c.w32(11, t576);
            let r577 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r577);
            let Some(t578) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c428f); };
            c.w32(11, t578);
            let r579 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r579);
            let Some(t580) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c4296); };
            c.w32(12, t580);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x68u64, 64);
            if c.cc_a() { blk = 386; continue; }
            blk = 385; continue;
        }
        385 => {
            return RegfileExit::fault(0x70008c42a6);
        }
        386 => {
            let Some(t581) = c.ld32(c.r64(11).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x70008c42a7); };
            c.w32(11, t581);
            let Some(t582) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c42ae); };
            c.w64(12, t582);
            let Some(t583) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008c42b5); };
            c.w64(15, t583);
            c.w64(0, c.r64(15).wrapping_add(0x14));
            let Some(t584) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c42bd); };
            c.w64(3, t584);
            let Some(t585) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c42c4); };
            c.cmp(t585, c.r64(0), 64);
            if c.cc_be() { blk = 573; continue; }
            blk = 387; continue;
        }
        387 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008c42cd); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 388; continue;
        }
        388 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008c42d5); }
            let Some(t586) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c42da); };
            c.w64(11, t586);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008c42e1); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008c42e6); }
            let Some(t587) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c42eb); };
            c.w64(11, t587);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008c42ef); }
            let Some(t588) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008c42f4); };
            c.w64(11, t588);
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008c42fb); }
            c.w64(0, c.r64(12));
            blk = 312; continue;
        }
        389 => {
            let Some(t589) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c4308); };
            c.w64(12, t589);
            let Some(t590) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c430c); };
            c.w32(15, t590);
            let r591 = c.r64(15) | 0x2u64;
            c.w64(15, r591);
            c.logic(r591, 64);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(15)) { return RegfileExit::fault(0x70008c4315); }
            let Some(t592) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c431a); };
            c.w64(6, t592);
            let Some(t593) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c431e); };
            c.w32(12, t593);
            let r594 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r594);
            let Some(t595) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c4325); };
            c.w32(12, t595);
            let r596 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r596);
            let Some(t597) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c432d); };
            c.w32(15, t597);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x68u64, 64);
            if c.cc_a() { blk = 391; continue; }
            blk = 390; continue;
        }
        390 => {
            return RegfileExit::fault(0x70008c433e);
        }
        391 => {
            let Some(t598) = c.ld32(c.r64(12).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x70008c433f); };
            c.w32(12, t598);
            let Some(t599) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c4347); };
            c.w64(15, t599);
            let Some(t600) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008c434e); };
            c.w64(0, t600);
            c.w64(3, c.r64(0).wrapping_add(0x14));
            let Some(t601) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c4355); };
            c.w64(2, t601);
            let Some(t602) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008c435c); };
            c.cmp(t602, c.r64(3), 64);
            if c.cc_be() { blk = 575; continue; }
            blk = 392; continue;
        }
        392 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x70008c4365); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            blk = 393; continue;
        }
        393 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c436c); }
            let Some(t603) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c4370); };
            c.w64(12, t603);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008c4377); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c437b); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008c437f); }
            let Some(t604) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008c4383); };
            c.w64(11, t604);
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008c438a); }
            c.w64(0, c.r64(15));
            blk = 312; continue;
        }
        394 => {
            let Some(t605) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c4396); };
            c.w64(6, t605);
            blk = 395; continue;
        }
        395 => {
            let Some(t606) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c439a); };
            c.w64(11, t606);
            let Some(t607) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c439e); };
            c.w32(12, t607);
            let r608 = c.r64(12) | 0x2u64;
            c.w64(12, r608);
            c.logic(r608, 64);
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(12)) { return RegfileExit::fault(0x70008c43a6); }
            blk = 398; continue;
        }
        396 => {
            c.w32(11, 0x80u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008c43b2); }
            let Some(t609) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008c43b4); };
            c.w64(11, t609);
            let Some(t610) = c.ld32(c.r64(11).wrapping_add(0x28b)) else { return RegfileExit::fault(0x70008c43bb); };
            c.w32(11, t610);
            let r611 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r611);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008c43c5); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c43c7); }
            let Some(t612) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008c43c8); };
            c.w64(3, t612);
            c.w32(0, 0x3u64);
            let Some(t613) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c43d4); };
            c.w64(6, t613);
            return c.call(0x700086ed74, 0x70008c43dd, 0x70008c43d8);
        }
        397 => {
            return RegfileExit::fault(0x70008c43dd);
        }
        398 => {
            let Some(t614) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c43de); };
            c.w32(11, t614);
            let r615 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r615);
            let Some(t616) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c43e5); };
            c.w32(11, t616);
            let r617 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r617);
            let Some(t618) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c43ec); };
            c.w32(12, t618);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x68u64, 64);
            if c.cc_a() { blk = 400; continue; }
            blk = 399; continue;
        }
        399 => {
            return RegfileExit::fault(0x70008c43fc);
        }
        400 => {
            let Some(t619) = c.ld32(c.r64(11).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x70008c43fd); };
            c.w32(11, t619);
            let Some(t620) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c4404); };
            c.w64(12, t620);
            let Some(t621) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008c440b); };
            c.w64(15, t621);
            c.w64(0, c.r64(15).wrapping_add(0x14));
            let Some(t622) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c4413); };
            c.w64(3, t622);
            let Some(t623) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c441a); };
            c.cmp(t623, c.r64(0), 64);
            if c.cc_be() { blk = 577; continue; }
            blk = 401; continue;
        }
        401 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008c4423); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 402; continue;
        }
        402 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008c442b); }
            let Some(t624) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c4430); };
            c.w64(11, t624);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008c4437); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008c443c); }
            let Some(t625) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c4441); };
            c.w64(11, t625);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008c4448); }
            let Some(t626) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008c444d); };
            c.w64(11, t626);
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008c4454); }
            c.w64(0, c.r64(12));
            blk = 312; continue;
        }
        403 => {
            c.w32(12, 0x80u64);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008c4467); }
            let Some(t627) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008c4469); };
            c.w64(12, t627);
            let Some(t628) = c.ld32(c.r64(12).wrapping_add(0x28f)) else { return RegfileExit::fault(0x70008c4470); };
            c.w32(12, t628);
            let r629 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r629);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008c447b); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008c447d); }
            let Some(t630) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008c447f); };
            c.w64(3, t630);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008c4490, 0x70008c448b);
        }
        404 => {
            return RegfileExit::fault(0x70008c4490);
        }
        405 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t631) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008c4493); };
            c.w64(3, t631);
            return c.call(0x700086ed74, 0x70008c449f, 0x70008c449a);
        }
        406 => {
            let Some(t632) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c449f); };
            c.w64(6, t632);
            blk = 1; continue;
        }
        407 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008c44a8); }
            let Some(t633) = c.ld16(c.r64(9).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008c44ac); };
            c.cmp(t633, 0x1u64, 16);
            if c.cc_le() { blk = 217; continue; }
            blk = 408; continue;
        }
        408 => {
            let Some(t634) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008c44b8); };
            c.w32(11, t634);
            let r635 = c.r32(11) & 0xfcu64;
            c.w32(11, r635);
            c.logic(r635, 32);
            c.cmp(c.r32(11), 0x40u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(11), 0x3cu64, 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let r636 = c.r32(12) | c.r32(11);
            c.w32(12, r636);
            c.logic(r636, 32);
            if c.cc_ne() { blk = 217; continue; }
            blk = 409; continue;
        }
        409 => {
            let Some(t637) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c44e5); };
            c.cmp(t637, 0x410u64, 16);
            if c.cc_be() { blk = 217; continue; }
            blk = 410; continue;
        }
        410 => {
            let Some(t638) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c44f2); };
            c.logic(t638 & 0x200000u64, 32);
            if c.cc_ne() { blk = 217; continue; }
            blk = 411; continue;
        }
        411 => {
            let Some(t639) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008c4500); };
            c.w32(11, t639);
            let Some(t640) = c.ld32(c.r64(13).wrapping_add(0x6e8)) else { return RegfileExit::fault(0x70008c4504); };
            c.cmp(t640, c.r32(11), 32);
            if c.cc_ne() { blk = 217; continue; }
            blk = 412; continue;
        }
        412 => {
            let Some(t641) = c.ld32(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008c4511); };
            c.w32(11, t641);
            c.w32(12, c.r32(11));
            c.logic(c.r8(11) & 0x4u64, 8);
            if c.cc_e() { blk = 414; continue; }
            blk = 413; continue;
        }
        413 => {
            let Some(t642) = c.ld32(c.r64(9).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008c451e); };
            c.w32(11, t642);
            blk = 427; continue;
        }
        414 => {
            let r643 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r643);
            if let Some(sr) = c.shr(c.r64(11), 0x13u64, 64) { c.w64(11, sr); }
            let Some(t644) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008c452e); };
            c.w32(9, t644);
            let r645 = c.r64(11) & 0x3ffu64;
            c.w64(11, r645);
            c.logic(r645, 64);
            let r646 = c.r32(12) & 0x380u64;
            c.w32(12, r646);
            c.logic(r646, 32);
            let r647 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r647);
            c.cmp(c.r32(12), 0x100u64, 32);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            let Some(t648) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c4553); };
            c.w32(8, t648);
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 419; continue; }
            blk = 415; continue;
        }
        415 => {
            let Some(t649) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c455d); };
            c.w32(2, t649);
            let r650 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r650);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 417; continue; }
            blk = 416; continue;
        }
        416 => {
            let Some(t651) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c4568); };
            c.w64(2, t651);
            blk = 417; continue;
        }
        417 => {
            let r652 = c.sub(c.r64(11), c.r64(8), 64);
            c.w64(11, r652);
            let Some(t653) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c4572); };
            c.w32(11, t653);
            let r654 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r654);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 427; continue; }
            blk = 418; continue;
        }
        418 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c4584); }
            blk = 421; continue;
        }
        419 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 426; continue; }
            blk = 420; continue;
        }
        420 => {
            let Some(t655) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c4592); };
            c.w32(11, t655);
            let r656 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r656);
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c459a); }
            blk = 421; continue;
        }
        421 => {
            let Some(t657) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c45a0); };
            c.w64(11, t657);
            let Some(t658) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c45a7); };
            c.w64(12, t658);
            c.w64(2, c.r64(12).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x70008c45af); }
            let Some(t659) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c45b4); };
            c.w64(8, t659);
            let Some(t660) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008c45bb); };
            c.cmp(t660, c.r64(2), 64);
            if c.cc_a() { blk = 424; continue; }
            blk = 422; continue;
        }
        422 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008c45ca, 0x70008c45c5);
        }
        423 => {
            c.w64(11, c.r64(0));
            let Some(t661) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c45cd); };
            c.w64(6, t661);
            let Some(t662) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c45d1); };
            c.w64(0, t662);
            let Some(t663) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c45d5); };
            c.w64(3, t663);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t664) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c45db); };
            c.w64(15, t664);
            let Some(t665) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c45df); };
            c.w64(7, t665);
            blk = 425; continue;
        }
        424 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x70008c45e5); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 425; continue;
        }
        425 => {
            let Some(t666) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c45ed); };
            c.w64(12, t666);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c45f4); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70008c45f8); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008c45fd); }
            blk = 427; continue;
        }
        426 => {
            let Some(t667) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c4605); };
            c.w32(11, t667);
            blk = 427; continue;
        }
        427 => {
            let Some(t668) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c460a); };
            c.w32(12, t668);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x99u64, 64);
            if c.cc_a() { blk = 429; continue; }
            blk = 428; continue;
        }
        428 => {
            return RegfileExit::fault(0x70008c461d);
        }
        429 => {
            let Some(t669) = c.ld32(c.r64(7).wrapping_add(0x26b)) else { return RegfileExit::fault(0x70008c461e); };
            c.cmp(t669, c.r32(11), 32);
            if c.cc_ne() { blk = 217; continue; }
            blk = 430; continue;
        }
        430 => {
            let Some(t670) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008c462b); };
            c.w64(12, t670);
            blk = 13; continue;
        }
        431 => {
            return RegfileExit::fault(0x70008c4637);
        }
        432 => {
            blk = 445; continue;
        }
        433 => {
            let Some(t671) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c463d); };
            c.w64(9, t671);
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 445; continue; }
            blk = 434; continue;
        }
        434 => {
            let Some(t672) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c464b); };
            c.w64(12, t672);
            blk = 61; continue;
        }
        435 => {
            c.w64(15, c.r64(12).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(11, c.r64(12).wrapping_add(c.r64(11).wrapping_mul(2)));
            let Some(t673) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c465d); };
            c.w32(12, t673);
            let r674 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r674);
            blk = 63; continue;
        }
        436 => {
            c.cmp(c.r64(11), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 441; continue; }
            blk = 437; continue;
        }
        437 => {
            c.cmp(c.r64(11), 0xfffffffffffffffdu64, 64);
            if c.cc_e() { blk = 439; continue; }
            blk = 438; continue;
        }
        438 => {
            let Some(t675) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c4675); };
            c.w64(6, t675);
            let Some(t676) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c4679); };
            c.w64(0, t676);
            let Some(t677) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c467d); };
            c.w64(3, t677);
            blk = 445; continue;
        }
        439 => {
            let Some(t678) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c4683); };
            if !c.push64(t678) { return RegfileExit::fault(0x70008c4683); }
            let Some(t679) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c4686); };
            if !c.push64(t679) { return RegfileExit::fault(0x70008c4686); }
            let Some(t680) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c4689); };
            if !c.push64(t680) { return RegfileExit::fault(0x70008c4689); }
            let Some(t681) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008c468c); };
            if !c.push64(t681) { return RegfileExit::fault(0x70008c468c); }
            let Some(t682) = c.ld64(c.r64(13).wrapping_add(0x2658)) else { return RegfileExit::fault(0x70008c4692); };
            c.w64(3, t682);
            c.w32(0, 0x4u64);
            let Some(t683) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c469e); };
            c.w64(6, t683);
            return c.call(0x700086ed74, 0x70008c46a7, 0x70008c46a2);
        }
        440 => {
            c.w64(8, c.r64(0));
            let Some(t684) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c46aa); };
            c.w64(0, t684);
            blk = 82; continue;
        }
        441 => {
            let Some(t685) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70008c46b3); };
            c.w64(3, t685);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t686) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c46bc); };
            c.w64(6, t686);
            return c.call(0x700086ed74, 0x70008c46c5, 0x70008c46c0);
        }
        442 => {
            return RegfileExit::fault(0x70008c46c5);
        }
        443 => {
            let Some(t687) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c46c6); };
            c.w64(6, t687);
            let Some(t688) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c46ca); };
            c.w64(0, t688);
            let Some(t689) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c46ce); };
            c.w64(3, t689);
            blk = 445; continue;
        }
        444 => {
            let Some(t690) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c46d4); };
            c.w64(3, t690);
            blk = 445; continue;
        }
        445 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c46d8); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008c46d9); }
            let Some(t691) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c46da); };
            if !c.push64(t691) { return RegfileExit::fault(0x70008c46da); }
            let Some(t692) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008c46dd); };
            if !c.push64(t692) { return RegfileExit::fault(0x70008c46dd); }
            c.w32(0, 0x4u64);
            let Some(t693) = c.ld64(c.r64(13).wrapping_add(0x2648)) else { return RegfileExit::fault(0x70008c46e8); };
            c.w64(3, t693);
            c.w64(11, c.r64(3));
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x70008c46fa, 0x70008c46f5);
        }
        446 => {
            c.w64(8, c.r64(0));
            let Some(t694) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c46fd); };
            c.w64(0, t694);
            blk = 82; continue;
        }
        447 => {
            return RegfileExit::fault(0x70008c4706);
        }
        448 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(1)) { return RegfileExit::fault(0x70008c4707); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(8)) { return RegfileExit::fault(0x70008c470b); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febb4, 0x70008c4717, 0x70008c4712);
        }
        449 => {
            c.w64(11, c.r64(0));
            let Some(t695) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c471a); };
            c.w64(1, t695);
            let Some(t696) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c471e); };
            c.w64(8, t696);
            blk = 101; continue;
        }
        450 => {
            c.w32(2, 0x28u64);
            return c.call(0x70007febf4, 0x70008c4731, 0x70008c472c);
        }
        451 => {
            c.w64(11, c.r64(0));
            blk = 104; continue;
        }
        452 => {
            let Some(t697) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008c4739); };
            c.w64(3, t697);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008c4749, 0x70008c4744);
        }
        453 => {
            return RegfileExit::fault(0x70008c4749);
        }
        454 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(3)) { return RegfileExit::fault(0x70008c474a); }
            return c.call(0x70007febf4, 0x70008c4753, 0x70008c474e);
        }
        455 => {
            c.w64(2, c.r64(0));
            let Some(t698) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c4756); };
            c.w64(6, t698);
            let Some(t699) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c475a); };
            c.w64(11, t699);
            let Some(t700) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c475e); };
            c.w64(12, t700);
            let Some(t701) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c4762); };
            c.w64(0, t701);
            let Some(t702) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c4766); };
            c.w64(15, t702);
            let Some(t703) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c476a); };
            c.w64(3, t703);
            blk = 107; continue;
        }
        456 => {
            c.w32(2, 0x28u64);
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(3)) { return RegfileExit::fault(0x70008c4778); }
            return c.call(0x70007febf4, 0x70008c4781, 0x70008c477c);
        }
        457 => {
            c.w64(2, c.r64(0));
            let Some(t704) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c4784); };
            c.w64(6, t704);
            let Some(t705) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c4788); };
            c.w64(11, t705);
            let Some(t706) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c478c); };
            c.w64(12, t706);
            let Some(t707) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c4790); };
            c.w64(0, t707);
            let Some(t708) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c4794); };
            c.w64(15, t708);
            let Some(t709) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c4798); };
            c.w64(3, t709);
            blk = 110; continue;
        }
        458 => {
            return RegfileExit::fault(0x70008c47a1);
        }
        459 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(8)) { return RegfileExit::fault(0x70008c47a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(1)) { return RegfileExit::fault(0x70008c47a6); }
            return c.call(0x70007febb4, 0x70008c47af, 0x70008c47aa);
        }
        460 => {
            c.w64(11, c.r64(0));
            let Some(t710) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c47b2); };
            c.w64(8, t710);
            let Some(t711) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c47b6); };
            c.w64(1, t711);
            blk = 120; continue;
        }
        461 => {
            c.w32(2, 0x2cu64);
            return c.call(0x70007febf4, 0x70008c47c9, 0x70008c47c4);
        }
        462 => {
            c.w64(11, c.r64(0));
            blk = 123; continue;
        }
        463 => {
            let Some(t712) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008c47d1); };
            c.w64(3, t712);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008c47e1, 0x70008c47dc);
        }
        464 => {
            return RegfileExit::fault(0x70008c47e1);
        }
        465 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(1)) { return RegfileExit::fault(0x70008c47e2); }
            return c.call(0x70007febf4, 0x70008c47eb, 0x70008c47e6);
        }
        466 => {
            c.w64(3, c.r64(0));
            let Some(t713) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c47ee); };
            c.w64(6, t713);
            let Some(t714) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c47f2); };
            c.w64(11, t714);
            let Some(t715) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c47f6); };
            c.w64(12, t715);
            let Some(t716) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c47fa); };
            c.w64(0, t716);
            let Some(t717) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c47fe); };
            c.w64(15, t717);
            let Some(t718) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c4802); };
            c.w64(1, t718);
            blk = 126; continue;
        }
        467 => {
            c.w32(2, 0x2cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(3)) { return RegfileExit::fault(0x70008c4810); }
            return c.call(0x70007febf4, 0x70008c4819, 0x70008c4814);
        }
        468 => {
            c.w64(2, c.r64(0));
            let Some(t719) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c481c); };
            c.w64(6, t719);
            let Some(t720) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c4820); };
            c.w64(11, t720);
            let Some(t721) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c4824); };
            c.w64(12, t721);
            let Some(t722) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c4828); };
            c.w64(0, t722);
            let Some(t723) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c482c); };
            c.w64(15, t723);
            let Some(t724) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c4830); };
            c.w64(3, t724);
            blk = 129; continue;
        }
        469 => {
            return RegfileExit::fault(0x70008c4839);
        }
        470 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(1)) { return RegfileExit::fault(0x70008c483a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008c483e); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(7)) { return RegfileExit::fault(0x70008c4842); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(8)) { return RegfileExit::fault(0x70008c4846); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febb4, 0x70008c4852, 0x70008c484d);
        }
        471 => {
            c.w64(15, c.r64(0));
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t725) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c4857); };
            c.w64(2, t725);
            let Some(t726) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008c485b); };
            c.w64(12, t726);
            let Some(t727) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c4862); };
            c.w64(11, t727);
            let Some(t728) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c4866); };
            c.w64(1, t728);
            let Some(t729) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c486a); };
            c.w64(0, t729);
            let Some(t730) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c486e); };
            c.w64(7, t730);
            let Some(t731) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008c4872); };
            c.w64(8, t731);
            blk = 165; continue;
        }
        472 => {
            return RegfileExit::fault(0x70008c487b);
        }
        473 => {
            return RegfileExit::fault(0x70008c487c);
        }
        474 => {
            return RegfileExit::fault(0x70008c487d);
        }
        475 => {
            return RegfileExit::fault(0x70008c487e);
        }
        476 => {
            return RegfileExit::fault(0x70008c487f);
        }
        477 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x70008c4880); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008c4884); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(1)) { return RegfileExit::fault(0x70008c4888); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x70008c4894, 0x70008c488f);
        }
        478 => {
            c.w64(6, c.r64(0));
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t732) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c4899); };
            c.w64(2, t732);
            let Some(t733) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c489d); };
            c.w64(15, t733);
            let Some(t734) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008c48a1); };
            c.w64(12, t734);
            let Some(t735) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c48a8); };
            c.w64(11, t735);
            let Some(t736) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c48ac); };
            c.w64(0, t736);
            let Some(t737) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c48b0); };
            c.w64(1, t737);
            blk = 175; continue;
        }
        479 => {
            return RegfileExit::fault(0x70008c48b9);
        }
        480 => {
            return RegfileExit::fault(0x70008c48ba);
        }
        481 => {
            return RegfileExit::fault(0x70008c48bb);
        }
        482 => {
            return RegfileExit::fault(0x70008c48bc);
        }
        483 => {
            return RegfileExit::fault(0x70008c48bd);
        }
        484 => {
            return RegfileExit::fault(0x70008c48be);
        }
        485 => {
            return RegfileExit::fault(0x70008c48bf);
        }
        486 => {
            return RegfileExit::fault(0x70008c48c0);
        }
        487 => {
            return RegfileExit::fault(0x70008c48c1);
        }
        488 => {
            return RegfileExit::fault(0x70008c48c2);
        }
        489 => {
            return RegfileExit::fault(0x70008c48c3);
        }
        490 => {
            c.w32(11, 0x88u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008c48ca); }
            let Some(t738) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c48cc); };
            c.w64(11, t738);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008c48d3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c48d5); }
            let Some(t739) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008c48d6); };
            c.w64(3, t739);
            c.w32(0, 0x3u64);
            let Some(t740) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c48e2); };
            c.w64(6, t740);
            return c.call(0x700086ed74, 0x70008c48eb, 0x70008c48e6);
        }
        491 => {
            return RegfileExit::fault(0x70008c48eb);
        }
        492 => {
            let Some(t741) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c48ec); };
            c.w64(6, t741);
            return c.call(0x700089c794, 0x70008c48f5, 0x70008c48f0);
        }
        493 => {
            blk = 234; continue;
        }
        494 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x70008c48ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008c4904); }
            return c.call(0x70007febf4, 0x70008c490d, 0x70008c4908);
        }
        495 => {
            c.w64(11, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x70008c4910); }
            let Some(t742) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c4915); };
            c.w64(0, t742);
            blk = 248; continue;
        }
        496 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008c491e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008c4922); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008c4926); }
            blk = 500; continue;
        }
        497 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008c492c); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008c4930); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008c4934); }
            blk = 500; continue;
        }
        498 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008c493a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008c493e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008c4942); }
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 500; continue; }
            blk = 499; continue;
        }
        499 => {
            let Some(t743) = c.ld64(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c494b); };
            c.w64(0, t743);
            blk = 271; continue;
        }
        500 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008c495a); }
            c.cmp(c.r64(0), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 502; continue; }
            blk = 501; continue;
        }
        501 => {
            let r744 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r744);
            blk = 511; continue;
        }
        502 => {
            let Some(t745) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c496d); };
            c.w64(2, t745);
            let Some(t746) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008c4974); };
            c.w64(1, t746);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            let Some(t747) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c497b); };
            c.w64(7, t747);
            let Some(t748) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008c4982); };
            c.cmp(t748, c.r64(6), 64);
            if c.cc_a() { blk = 505; continue; }
            blk = 503; continue;
        }
        503 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008c4991, 0x70008c498c);
        }
        504 => {
            c.w64(2, c.r64(0));
            let Some(t749) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c4994); };
            c.w64(3, t749);
            let Some(t750) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c4998); };
            c.w64(11, t750);
            let Some(t751) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c499c); };
            c.w64(12, t751);
            let Some(t752) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c49a0); };
            c.w64(15, t752);
            let Some(t753) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c49a4); };
            c.w64(0, t753);
            blk = 506; continue;
        }
        505 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x70008c49aa); }
            c.w64(2, c.r64(1).wrapping_add(0x1));
            blk = 506; continue;
        }
        506 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(0) as i64));
            c.logic(c.r64(0) & c.r64(0), 64);
            if c.cc_ns() { blk = 510; continue; }
            blk = 507; continue;
        }
        507 => {
            c.w64(10, c.r64(0));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 509; continue; }
            blk = 508; continue;
        }
        508 => {
            let r754 = c.r64(10) | 0x1u64;
            c.w64(10, r754);
            c.logic(r754, 64);
            blk = 509; continue;
        }
        509 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 510; continue;
        }
        510 => {
            let Some(t755) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c49d8); };
            c.w64(0, t755);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008c49df); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008c49e2); }
            c.w64(0, c.r64(2));
            blk = 511; continue;
        }
        511 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008c49ea); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c49eb); }
            c.w32(0, 0x2u64);
            let Some(t756) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008c49f1); };
            c.w64(2, t756);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(2));
            return c.call(0x700086ed74, 0x70008c4a02, 0x70008c49fd);
        }
        512 => {
            let r757 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r757);
            let Some(t758) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c4a04); };
            c.w64(15, t758);
            let Some(t759) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c4a08); };
            c.w64(3, t759);
            let Some(t760) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c4a0c); };
            c.w64(11, t760);
            let Some(t761) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c4a10); };
            c.w64(12, t761);
            blk = 274; continue;
        }
        513 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008c4a19); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008c4a1d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008c4a21); }
            blk = 517; continue;
        }
        514 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008c4a27); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008c4a2b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008c4a2f); }
            blk = 517; continue;
        }
        515 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008c4a35); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008c4a39); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008c4a3d); }
            c.logic(c.r8(0) & 0x10u64, 8);
            if c.cc_ne() { blk = 517; continue; }
            blk = 516; continue;
        }
        516 => {
            let Some(t762) = c.ld64(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c4a45); };
            c.w64(3, t762);
            blk = 294; continue;
        }
        517 => {
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 519; continue; }
            blk = 518; continue;
        }
        518 => {
            let r763 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r763);
            blk = 528; continue;
        }
        519 => {
            let Some(t764) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c4a5c); };
            c.w64(0, t764);
            let Some(t765) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008c4a63); };
            c.w64(2, t765);
            c.w64(1, c.r64(2).wrapping_add(0xc));
            let Some(t766) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c4a6a); };
            c.w64(6, t766);
            let Some(t767) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008c4a71); };
            c.cmp(t767, c.r64(1), 64);
            if c.cc_a() { blk = 522; continue; }
            blk = 520; continue;
        }
        520 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008c4a80, 0x70008c4a7b);
        }
        521 => {
            let Some(t768) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c4a80); };
            c.w64(15, t768);
            let Some(t769) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c4a84); };
            c.w64(3, t769);
            let Some(t770) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c4a88); };
            c.w64(11, t770);
            let Some(t771) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c4a8c); };
            c.w64(12, t771);
            blk = 523; continue;
        }
        522 => {
            if !c.st64(c.r64(0), c.r64(1)) { return RegfileExit::fault(0x70008c4a92); }
            c.w64(0, c.r64(2).wrapping_add(0x1));
            blk = 523; continue;
        }
        523 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(15) as i64));
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 527; continue; }
            blk = 524; continue;
        }
        524 => {
            c.w64(10, c.r64(15));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 526; continue; }
            blk = 525; continue;
        }
        525 => {
            let r772 = c.r64(10) | 0x1u64;
            c.w64(10, r772);
            c.logic(r772, 64);
            blk = 526; continue;
        }
        526 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 527; continue;
        }
        527 => {
            let Some(t773) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c4ac0); };
            c.w64(15, t773);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008c4ac7); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008c4acb); }
            c.w64(15, c.r64(0));
            blk = 528; continue;
        }
        528 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008c4ad3); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008c4ad4); }
            let Some(t774) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008c4ad6); };
            c.w64(3, t774);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008c4ae9, 0x70008c4ae4);
        }
        529 => {
            let r775 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r775);
            c.w64(15, c.r64(0));
            let Some(t776) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c4aee); };
            c.w64(11, t776);
            let Some(t777) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c4af2); };
            c.w64(12, t777);
            blk = 297; continue;
        }
        530 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x70008c4b00); }
            return c.call(0x70007febf4, 0x70008c4b0a, 0x70008c4b05);
        }
        531 => {
            c.w64(11, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x70008c4b0d); }
            blk = 305; continue;
        }
        532 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008c4b1c); }
            return c.call(0x70007febf4, 0x70008c4b25, 0x70008c4b20);
        }
        533 => {
            c.w64(12, c.r64(0));
            let Some(t778) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c4b28); };
            c.w64(11, t778);
            blk = 311; continue;
        }
        534 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008c4b36); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x70008c4b3a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x70008c4b3f); }
            return c.call(0x70007febf4, 0x70008c4b48, 0x70008c4b43);
        }
        535 => {
            c.w64(3, c.r64(0));
            let Some(t779) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c4b4b); };
            c.w64(12, t779);
            let Some(t780) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c4b4f); };
            c.w64(11, t780);
            let Some(t781) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c4b53); };
            c.w64(0, t781);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x70008c4b57); }
            let Some(t782) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c4b5c); };
            c.w64(15, t782);
            blk = 325; continue;
        }
        536 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008c4b65); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008c4b69); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008c4b6d); }
            blk = 541; continue;
        }
        537 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008c4b73); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008c4b77); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008c4b7b); }
            blk = 541; continue;
        }
        538 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008c4b81); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008c4b85); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008c4b89); }
            c.logic(c.r8(6) & 0x10u64, 8);
            if c.cc_ne() { blk = 540; continue; }
            blk = 539; continue;
        }
        539 => {
            let Some(t783) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c4b93); };
            c.w64(15, t783);
            blk = 348; continue;
        }
        540 => {
            let Some(t784) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c4b9c); };
            c.w64(11, t784);
            blk = 541; continue;
        }
        541 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x70008c4ba6); }
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 543; continue; }
            blk = 542; continue;
        }
        542 => {
            let r785 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r785);
            blk = 552; continue;
        }
        543 => {
            let Some(t786) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c4bbb); };
            c.w64(6, t786);
            let Some(t787) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008c4bc2); };
            c.w64(7, t787);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t788) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c4bc9); };
            c.w64(9, t788);
            let Some(t789) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c4bd0); };
            c.cmp(t789, c.r64(8), 64);
            if c.cc_a() { blk = 546; continue; }
            blk = 544; continue;
        }
        544 => {
            c.w32(2, 0xcu64);
            c.w64(6, c.r64(2));
            return c.call(0x70007febf4, 0x70008c4be2, 0x70008c4bdd);
        }
        545 => {
            c.w64(6, c.r64(0));
            let Some(t790) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c4be5); };
            c.w64(12, t790);
            let Some(t791) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c4be9); };
            c.w64(11, t791);
            let Some(t792) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c4bed); };
            c.w64(3, t792);
            let Some(t793) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c4bf1); };
            c.w64(0, t793);
            let Some(t794) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c4bf5); };
            c.w64(1, t794);
            let Some(t795) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c4bf9); };
            c.w64(2, t795);
            let Some(t796) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c4bfd); };
            c.w64(15, t796);
            blk = 547; continue;
        }
        546 => {
            if !c.st64(c.r64(6), c.r64(8)) { return RegfileExit::fault(0x70008c4c03); }
            c.w64(6, c.r64(7).wrapping_add(0x1));
            blk = 547; continue;
        }
        547 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(15) as i64));
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 551; continue; }
            blk = 548; continue;
        }
        548 => {
            c.w64(10, c.r64(15));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 550; continue; }
            blk = 549; continue;
        }
        549 => {
            let r797 = c.r64(10) | 0x1u64;
            c.w64(10, r797);
            c.logic(r797, 64);
            blk = 550; continue;
        }
        550 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 551; continue;
        }
        551 => {
            let Some(t798) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c4c31); };
            c.w64(15, t798);
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008c4c38); }
            if !c.movsd_mr(c.r64(6).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008c4c3c); }
            c.w64(15, c.r64(6));
            blk = 552; continue;
        }
        552 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008c4c44); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008c4c45); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t799) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008c4c49); };
            c.w64(3, t799);
            c.w32(0, 0x2u64);
            c.w64(15, c.r64(3));
            return c.call(0x700086ed74, 0x70008c4c5d, 0x70008c4c58);
        }
        553 => {
            let r800 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r800);
            c.w64(11, c.r64(0));
            let Some(t801) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c4c62); };
            c.w64(12, t801);
            let Some(t802) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c4c66); };
            c.w64(3, t802);
            let Some(t803) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c4c6a); };
            c.w64(0, t803);
            let Some(t804) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c4c6e); };
            c.w64(1, t804);
            let Some(t805) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c4c72); };
            c.w64(2, t805);
            blk = 351; continue;
        }
        554 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008c4c7b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008c4c7f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008c4c83); }
            blk = 558; continue;
        }
        555 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008c4c89); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008c4c8d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008c4c91); }
            blk = 558; continue;
        }
        556 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008c4c97); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008c4c9b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008c4c9f); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 558; continue; }
            blk = 557; continue;
        }
        557 => {
            let Some(t806) = c.ld64(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c4ca9); };
            c.w64(1, t806);
            blk = 371; continue;
        }
        558 => {
            c.cmp(c.r64(2), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 560; continue; }
            blk = 559; continue;
        }
        559 => {
            c.w32(11, c.r64(2).wrapping_add(c.r64(2)));
            blk = 569; continue;
        }
        560 => {
            let Some(t807) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c4cc4); };
            c.w64(11, t807);
            let Some(t808) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c4ccb); };
            c.w64(15, t808);
            c.w64(6, c.r64(15).wrapping_add(0xc));
            let Some(t809) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c4cd2); };
            c.w64(7, t809);
            let Some(t810) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008c4cd9); };
            c.cmp(t810, c.r64(6), 64);
            if c.cc_a() { blk = 563; continue; }
            blk = 561; continue;
        }
        561 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008c4ceb, 0x70008c4ce6);
        }
        562 => {
            c.w64(11, c.r64(0));
            let Some(t811) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c4cee); };
            c.w64(12, t811);
            let Some(t812) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c4cf2); };
            c.w64(3, t812);
            let Some(t813) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c4cf6); };
            c.w64(0, t813);
            let Some(t814) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c4cfa); };
            c.w64(1, t814);
            let Some(t815) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c4cfe); };
            c.w64(2, t815);
            blk = 564; continue;
        }
        563 => {
            if !c.st64(c.r64(11), c.r64(6)) { return RegfileExit::fault(0x70008c4d04); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            blk = 564; continue;
        }
        564 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(2) as i64));
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 568; continue; }
            blk = 565; continue;
        }
        565 => {
            c.w64(10, c.r64(2));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 567; continue; }
            blk = 566; continue;
        }
        566 => {
            let r816 = c.r64(10) | 0x1u64;
            c.w64(10, r816);
            c.logic(r816, 64);
            blk = 567; continue;
        }
        567 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 568; continue;
        }
        568 => {
            let Some(t817) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c4d32); };
            c.w64(15, t817);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008c4d39); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008c4d3d); }
            blk = 569; continue;
        }
        569 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008c4d43); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008c4d44); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t818) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008c4d48); };
            c.w64(3, t818);
            c.w32(0, 0x2u64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70008c4d5c, 0x70008c4d57);
        }
        570 => {
            let r819 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r819);
            c.w64(11, c.r64(0));
            let Some(t820) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c4d61); };
            c.w64(12, t820);
            let Some(t821) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c4d65); };
            c.w64(3, t821);
            let Some(t822) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c4d69); };
            c.w64(0, t822);
            blk = 374; continue;
        }
        571 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x70008c4d77); }
            return c.call(0x70007febf4, 0x70008c4d81, 0x70008c4d7c);
        }
        572 => {
            c.w64(11, c.r64(0));
            let Some(t823) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c4d84); };
            c.w64(12, t823);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x70008c4d88); }
            blk = 382; continue;
        }
        573 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008c4d97); }
            return c.call(0x70007febf4, 0x70008c4da0, 0x70008c4d9b);
        }
        574 => {
            c.w64(12, c.r64(0));
            let Some(t824) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c4da3); };
            c.w64(11, t824);
            blk = 388; continue;
        }
        575 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x70008c4db1); }
            return c.call(0x70007febf4, 0x70008c4dba, 0x70008c4db5);
        }
        576 => {
            c.w64(15, c.r64(0));
            let Some(t825) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c4dbd); };
            c.w64(11, t825);
            let Some(t826) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c4dc1); };
            c.w64(12, t826);
            blk = 393; continue;
        }
        577 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008c4dcf); }
            return c.call(0x70007febf4, 0x70008c4dd8, 0x70008c4dd3);
        }
        578 => {
            c.w64(12, c.r64(0));
            let Some(t827) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c4ddb); };
            c.w64(11, t827);
            blk = 402; continue;
        }
        579 => {
            c.w32(0, c.r32(0));
            let r828 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r828);
            c.w64(12, 0xfffffffffffc0000u64);
            let r829 = c.r64(12) & c.r64(0);
            c.w64(12, r829);
            c.logic(r829, 64);
            let Some(t830) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c4df3); };
            c.logic(t830 & 0x2u64, 8);
            if c.cc_e() { blk = 215; continue; }
            blk = 580; continue;
        }
        580 => {
            c.w64(15, c.r64(11).wrapping_add(0x27));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c4e03); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c4e04); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008c4e11, 0x70008c4e0c);
        }
        581 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c4e11); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c4e13); }
            blk = 215; continue;
        }
        582 => {
            c.w32(6, c.r32(6));
            let r831 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r831);
            c.w64(1, 0xfffffffffffc0000u64);
            let r832 = c.r64(1) & c.r64(6);
            c.w64(1, r832);
            c.logic(r832, 64);
            let Some(t833) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c4e28); };
            c.logic(t833 & 0x2u64, 8);
            if c.cc_e() { blk = 202; continue; }
            blk = 583; continue;
        }
        583 => {
            c.w64(7, c.r64(15).wrapping_add(c.r64(2)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c4e36); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c4e37); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(15));
            return c.call(0x70007f0e54, 0x70008c4e44, 0x70008c4e3f);
        }
        584 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c4e44); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c4e46); }
            blk = 202; continue;
        }
        585 => {
            c.w32(0, c.r32(0));
            let r834 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r834);
            c.w64(12, 0xfffffffffffc0000u64);
            let r835 = c.r64(12) & c.r64(0);
            c.w64(12, r835);
            c.logic(r835, 64);
            let Some(t836) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c4e5b); };
            c.logic(t836 & 0x2u64, 8);
            if c.cc_e() { blk = 199; continue; }
            blk = 586; continue;
        }
        586 => {
            c.w64(2, c.r64(15).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c4e6b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c4e6c); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70008c4e79, 0x70008c4e74);
        }
        587 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c4e79); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c4e7b); }
            blk = 199; continue;
        }
        588 => {
            c.w32(6, c.r32(6));
            let r837 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r837);
            c.w64(0, 0xfffffffffffc0000u64);
            let r838 = c.r64(0) & c.r64(6);
            c.w64(0, r838);
            c.logic(r838, 64);
            let Some(t839) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c4e90); };
            c.logic(t839 & 0x2u64, 8);
            if c.cc_e() { blk = 180; continue; }
            blk = 589; continue;
        }
        589 => {
            c.w64(1, c.r64(2).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c4e9e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c4e9f); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x70008c4eac, 0x70008c4ea7);
        }
        590 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c4eac); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c4eae); }
            blk = 180; continue;
        }
        591 => {
            c.w32(11, c.r32(11));
            let r840 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r840);
            c.w64(1, 0xfffffffffffc0000u64);
            let r841 = c.r64(1) & c.r64(11);
            c.w64(1, r841);
            c.logic(r841, 64);
            let Some(t842) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c4ec4); };
            c.logic(t842 & 0x2u64, 8);
            if c.cc_e() { blk = 158; continue; }
            blk = 592; continue;
        }
        592 => {
            c.w64(6, c.r64(2).wrapping_add(0x1b));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c4ed2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c4ed3); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x70008c4ee0, 0x70008c4edb);
        }
        593 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c4ee0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c4ee2); }
            blk = 158; continue;
        }
        594 => {
            c.w32(0, c.r32(0));
            let r843 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r843);
            c.w64(15, 0xfffffffffffc0000u64);
            let r844 = c.r64(15) & c.r64(0);
            c.w64(15, r844);
            c.logic(r844, 64);
            let Some(t845) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c4ef7); };
            c.logic(t845 & 0x2u64, 8);
            if c.cc_e() { blk = 151; continue; }
            blk = 595; continue;
        }
        595 => {
            c.w64(2, c.r64(12).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c4f06); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c4f07); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70008c4f14, 0x70008c4f0f);
        }
        596 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c4f14); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c4f16); }
            blk = 151; continue;
        }
        597 => {
            c.w32(0, c.r32(0));
            let r846 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r846);
            c.w64(7, 0xfffffffffffc0000u64);
            let r847 = c.r64(7) & c.r64(0);
            c.w64(7, r847);
            c.logic(r847, 64);
            let Some(t848) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c4f2b); };
            c.logic(t848 & 0x2u64, 8);
            if c.cc_e() { blk = 144; continue; }
            blk = 598; continue;
        }
        598 => {
            c.w64(8, c.r64(15).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c4f39); }
            c.w64(7, c.r64(15));
            return c.call(0x70007f0e54, 0x70008c4f42, 0x70008c4f3d);
        }
        599 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c4f42); }
            blk = 144; continue;
        }
        600 => {
            c.w32(1, c.r32(1));
            let r849 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r849);
            c.w64(8, 0xfffffffffffc0000u64);
            let r850 = c.r64(8) & c.r64(1);
            c.w64(8, r850);
            c.logic(r850, 64);
            let Some(t851) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c4f57); };
            c.logic(t851 & 0x2u64, 8);
            if c.cc_e() { blk = 137; continue; }
            blk = 601; continue;
        }
        601 => {
            c.w64(9, c.r64(2).wrapping_add(0x1f));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c4f66); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c4f67); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008c4f74, 0x70008c4f6f);
        }
        602 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c4f74); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c4f76); }
            blk = 137; continue;
        }
        603 => {
            c.w32(1, c.r32(1));
            let r852 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r852);
            c.w64(7, 0xfffffffffffc0000u64);
            let r853 = c.r64(7) & c.r64(1);
            c.w64(7, r853);
            c.logic(r853, 64);
            let Some(t854) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c4f8b); };
            c.logic(t854 & 0x2u64, 8);
            if c.cc_e() { blk = 135; continue; }
            blk = 604; continue;
        }
        604 => {
            c.w64(8, c.r64(2).wrapping_add(0x13));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c4f99); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x70008c4fa2, 0x70008c4f9d);
        }
        605 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c4fa2); }
            blk = 135; continue;
        }
        606 => {
            c.w32(0, c.r32(0));
            let r855 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r855);
            c.w64(11, 0xfffffffffffc0000u64);
            let r856 = c.r64(11) & c.r64(0);
            c.w64(11, r856);
            c.logic(r856, 64);
            let Some(t857) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c4fb7); };
            c.logic(t857 & 0x2u64, 8);
            if c.cc_e() { blk = 76; continue; }
            blk = 607; continue;
        }
        607 => {
            c.w64(15, c.r64(8).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c4fc6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c4fc7); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008c4fd4, 0x70008c4fcf);
        }
        608 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c4fd4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c4fd6); }
            blk = 76; continue;
        }
        609 => {
            c.w32(0, c.r32(0));
            let r858 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r858);
            c.w64(11, 0xfffffffffffc0000u64);
            let r859 = c.r64(11) & c.r64(0);
            c.w64(11, r859);
            c.logic(r859, 64);
            let Some(t860) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c4feb); };
            c.logic(t860 & 0x2u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 610; continue;
        }
        610 => {
            c.w64(15, c.r64(8).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c4ffa); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c4ffb); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008c5008, 0x70008c5003);
        }
        611 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c5008); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c500a); }
            blk = 74; continue;
        }
        612 => {
            return RegfileExit::jump(0x70008c5014);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008c5034(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008c5034); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008c5038); }
            let r0 = c.sub(c.r64(4), 0x78u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c503e); };
            c.w32(7, t1);
            c.w32(11, c.r32(7));
            c.w64(12, c.r64(11));
            let r2 = c.r32(12) & 0x2u64;
            c.w32(12, r2);
            c.logic(r2, 32);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let r3 = c.r64(12) & 0xfffffffffffffffeu64;
            c.w64(12, r3);
            c.logic(r3, 64);
            let r4 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008c5055); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008c5059); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008c505d); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008c5061); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 216; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(7, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(7)) { return RegfileExit::fault(0x70008c5071); }
            let Some(t5) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c5074); };
            c.w32(12, t5);
            let r6 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c507b); };
            c.w32(15, t7);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x70008c508c);
        }
        3 => {
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c508d); };
            c.w32(2, t8);
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c5099); };
            c.w64(12, t9);
            blk = 8; continue;
        }
        5 => {
            c.cmp(c.r64(15), 0x3u64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70008c50a8);
        }
        7 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c50a9); };
            c.w32(12, t10);
            let r11 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r11);
            c.w32(2, 0x1u64);
            blk = 8; continue;
        }
        8 => {
            c.w32(15, 0x1u64);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x70008c50bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(2)) { return RegfileExit::fault(0x70008c50c0); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c50c4); };
            c.w64(1, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008c50cb); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008c50cf); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008c50d3); }
            blk = 10; continue;
        }
        9 => {
            let r13 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r13);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008c50dd); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008c50e1); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008c50e5); }
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c50e9); };
            c.w64(0, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c50ed); };
            c.w64(6, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c50f1); };
            c.w64(3, t16);
            blk = 10; continue;
        }
        10 => {
            let Some(t17) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c50f5); };
            c.w32(11, t17);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let Some(t18) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c50ff); };
            c.w32(12, t18);
            let r19 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r19);
            c.logic(c.r8(11) & 0x9u64, 8);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008c5112); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        12 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t20) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c5139); };
            c.w32(11, t20);
            let r21 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r21);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c5146); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c5151); }
            blk = 19; continue;
        }
        16 => {
            c.w64(7, c.r64(12));
            let r22 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r22);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            blk = 19; continue;
        }
        17 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            let r23 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r23);
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c5171); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 19; continue;
        }
        19 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            blk = 22; continue;
        }
        21 => {
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_l() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        23 => {
            c.w32(7, 0x1u64);
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 188; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t24) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c51a0); };
            c.w32(7, t24);
            let r25 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r25);
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c51a6); };
            c.w32(7, t26);
            let r27 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r27);
            let Some(t28) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c51ac); };
            c.w32(11, t28);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008c51b6); }
            c.cmp(c.r64(11), 0x9bu64, 64);
            if c.cc_be() { blk = 177; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t29) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c51c7); };
            c.w32(11, t29);
            let r30 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r30);
            let Some(t31) = c.ld32(c.r64(7).wrapping_add(0x273)) else { return RegfileExit::fault(0x70008c51ce); };
            c.w32(7, t31);
            let r32 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r32);
            let Some(t33) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c51d7); };
            c.w32(11, t33);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008c51dc); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 76; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t34) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c51ed); };
            c.w32(12, t34);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(12)) { return RegfileExit::fault(0x70008c51f4); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(7)) { return RegfileExit::fault(0x70008c51f8); }
            c.cmp(c.r64(12), c.r64(7), 64);
            if c.cc_b() { blk = 76; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t35) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c5205); };
            c.w32(7, t35);
            let r36 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r36);
            let Some(t37) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c520b); };
            c.w32(12, t37);
            let r38 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r38);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008c5212); }
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 472; continue;
        }
        32 => {
            blk = 35; continue;
        }
        33 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c522f); };
            c.w64(2, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c5233); };
            c.w64(1, t40);
            return c.call(0x7000859a54, 0x70008c523c, 0x70008c5237);
        }
        34 => {
            c.w64(7, c.r64(0));
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c523f); };
            c.w64(0, t41);
            blk = 77; continue;
        }
        35 => {
            let Some(t42) = c.ld32(c.r64(7).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008c5248); };
            c.w32(12, t42);
            c.cmp(c.r32(12), 0x7eu64, 32);
            if c.cc_a() { blk = 429; continue; }
            blk = 36; continue;
        }
        36 => {
            c.logic(c.r8(11) & 0x7u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(2, c.r64(11));
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(3));
            blk = 39; continue;
        }
        38 => {
            c.w64(2, c.r64(3));
            blk = 39; continue;
        }
        39 => {
            c.w64(3, c.r64(2));
            let r43 = c.r32(3) & 0x7u64;
            c.w32(3, r43);
            c.logic(r43, 32);
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_l() { blk = 44; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 43; continue;
        }
        43 => {
            blk = 429; continue;
        }
        44 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            blk = 429; continue;
        }
        47 => {
            let Some(t44) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c52a0); };
            c.w32(3, t44);
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c52a4); };
            c.cmp(t45, c.r32(3), 32);
            if c.cc_ne() { blk = 429; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t46) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c52b1); };
            c.w32(15, t46);
            let r47 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r47);
            let Some(t48) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c52b8); };
            c.w32(3, t48);
            let r49 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r49);
            let Some(t50) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c52bf); };
            c.w32(3, t50);
            blk = 38; continue;
        }
        49 => {
            let Some(t51) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c52c5); };
            c.w32(3, t51);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r52 = c.add(c.r64(11), c.r64(3), 64);
            c.w64(11, r52);
            let Some(t53) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c52d2); };
            c.w32(15, t53);
            let r54 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r54);
            let Some(t55) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c52d9); };
            c.w32(3, t55);
            let r56 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r56);
            let Some(t57) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c52e0); };
            c.w32(3, t57);
            blk = 38; continue;
        }
        50 => {
            let Some(t58) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c52e6); };
            c.w32(15, t58);
            let r59 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r59);
            let Some(t60) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c52ed); };
            c.w32(3, t60);
            let r61 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r61);
            let Some(t62) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c52f4); };
            c.w32(3, t62);
            blk = 38; continue;
        }
        51 => {
            c.w64(3, c.r64(15));
            c.w64(15, c.r64(2));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 54; continue;
        }
        52 => {
            c.w64(3, c.r64(15));
            c.w64(15, c.r64(2));
            c.w32(2, 0x1u64);
            blk = 54; continue;
        }
        53 => {
            c.w64(15, c.r64(11));
            c.w32(11, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 420; continue; }
            blk = 55; continue;
        }
        55 => {
            let r63 = c.add(c.r64(3), 0xbu64, 64);
            c.w64(3, r63);
            blk = 56; continue;
        }
        56 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c5329); };
            c.w64(2, t64);
            let r65 = c.add(c.r64(2), c.r64(11), 64);
            c.w64(2, r65);
            let r66 = c.r32(15) & 0x8u64;
            c.w32(15, r66);
            c.logic(r66, 32);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008c5334); };
            c.w64(1, t67);
            let r68 = c.add(c.r64(11), c.r64(1), 64);
            c.w64(11, r68);
            c.cmp(c.r32(15), 0x8u64, 32);
            if c.cc_ne() { blk = 422; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t69) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c5345); };
            c.w32(15, t69);
            let r70 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r70);
            let r71 = c.add(c.r64(2), c.r64(3), 64);
            c.w64(2, r71);
            let r72 = c.add(c.r64(11), c.r64(3), 64);
            c.w64(11, r72);
            blk = 58; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(11)) { return RegfileExit::fault(0x70008c5352); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 429; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t73) = c.ld32(c.r64(15).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008c5360); };
            c.w32(3, t73);
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2e70)) else { return RegfileExit::fault(0x70008c5364); };
            c.w64(7, t74);
            let Some(t75) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008c536b); };
            c.w64(7, t75);
            c.cmp(c.r32(3), 0xffffffffffffffffu64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w32(15, c.r64(3).wrapping_mul(8));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(13));
            let Some(t76) = c.ld64(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008c5381); };
            c.w64(15, t76);
            blk = 62; continue;
        }
        61 => {
            let r77 = c.add(c.r64(15), 0x3fu64, 64);
            c.w64(15, r77);
            c.w64(3, c.r64(13));
            blk = 62; continue;
        }
        62 => {
            let r78 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r78);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w32(1, c.r32(1));
            c.w64(10, c.r64(4));
            let r79 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r79);
            let r80 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r80);
            c.logic(r80, 64);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x70008c53a9); }
            let r81 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r81);
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008c53b1); };
            c.w64(11, t82);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(11)) { return RegfileExit::fault(0x70008c53b8); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(12)) { return RegfileExit::fault(0x70008c53bd); }
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(7)) { return RegfileExit::fault(0x70008c53c2); }
            if !c.st64(c.r64(4).wrapping_add(0x38), 0x1u64) { return RegfileExit::fault(0x70008c53c7); }
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(3)) { return RegfileExit::fault(0x70008c53d0); }
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(0)) { return RegfileExit::fault(0x70008c53d5); }
            c.w64(9, c.r64(2));
            c.w64(2, c.r64(1));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c53e0); };
            c.w64(1, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c53e4); };
            c.w64(8, t84);
            c.w64(10, 0x70008c53efu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008c53ef); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008c53f3); }
            return c.call(c.r64(15), 0x70008c53fa, 0x70008c53f7);
        }
        63 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008c53fa); }
            let Some(t85) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x70008c5402); };
            c.w64(4, t85);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_e() { blk = 66; continue; }
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_ne() { blk = 423; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c541a); };
            c.w64(0, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c541e); };
            c.w64(6, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c5422); };
            c.w64(3, t88);
            blk = 76; continue;
        }
        66 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c542b); };
            c.w64(7, t89);
            let Some(t90) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c542f); };
            c.w32(11, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c5433); };
            c.w64(12, t91);
            let Some(t92) = c.ld32(c.r64(12).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008c5437); };
            c.w32(12, t92);
            let r93 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r93);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r94 = c.sub(c.r32(11), 0x6u64, 32);
            c.w32(11, r94);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_l() { blk = 428; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c5456); }
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c545a); };
            c.w64(0, t95);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008c545e); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r96 = c.r64(12) & c.r64(7);
            c.w64(12, r96);
            c.logic(r96, 64);
            let Some(t97) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c546f); };
            c.logic(t97 & 0x4u64, 8);
            if c.cc_ne() { blk = 582; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008c547b); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 71; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r98 = c.r64(12) & c.r64(7);
            c.w64(12, r98);
            c.logic(r98, 64);
            let Some(t99) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c548c); };
            c.logic(t99 & 0x4u64, 8);
            if c.cc_ne() { blk = 579; continue; }
            blk = 71; continue;
        }
        71 => {
            if let Some(sr) = c.shl(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if c.cc_e() { blk = 77; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w32(15, 0);
            c.logic(0, 32);
            c.w32(12, 0x13u64);
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008c54a6); };
            c.w64(3, t100);
            blk = 74; continue;
        }
        73 => {
            let r101 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r101);
            c.w64(15, c.r64(2));
            blk = 74; continue;
        }
        74 => {
            c.w64(2, c.r64(15).wrapping_add(0x4));
            let Some(t102) = c.ld32(c.r64(3).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x70008c54ba); };
            c.w32(15, t102);
            let r103 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r103);
            if !c.st32(c.r64(7).wrapping_add(c.r64(12)), c.r32(15)) { return RegfileExit::fault(0x70008c54c1); }
            c.cmp(c.r64(11), c.r64(2), 64);
            if c.cc_ne() { blk = 73; continue; }
            blk = 75; continue;
        }
        75 => {
            blk = 77; continue;
        }
        76 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c54cc); };
            c.w64(7, t104);
            c.w64(0, c.r64(3));
            blk = 77; continue;
        }
        77 => {
            let Some(t105) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c54d6); };
            c.cmp(t105, c.r32(7), 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t106) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c54e3); };
            c.cmp(t106, 0x0u64, 32);
            if c.cc_ne() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c54e9); };
            c.w64(12, t107);
            blk = 82; continue;
        }
        80 => {
            let Some(t108) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c54ef); };
            c.w32(11, t108);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4u64, 64);
            if c.cc_be() { blk = 181; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t109) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c5503); };
            c.w32(11, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c5507); };
            c.w64(12, t110);
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008c550b); }
            blk = 82; continue;
        }
        82 => {
            let Some(t111) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c5510); };
            c.cmp(t111, 0x0u64, 32);
            if c.cc_ne() { blk = 85; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t112) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c5516); };
            c.w32(3, t112);
            let r113 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r113);
            let Some(t114) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c551c); };
            c.w32(1, t114);
            let r115 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c5522); };
            c.w64(6, t116);
            return c.call(0x700086fe54, 0x70008c552b, 0x70008c5526);
        }
        84 => {
            blk = 86; continue;
        }
        85 => {
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c552d); };
            c.w64(0, t117);
            blk = 86; continue;
        }
        86 => {
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c5531); };
            c.w64(9, t118);
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c5535); };
            c.cmp(c.r64(9), t119, 64);
            if c.cc_e() { blk = 88; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c553b); };
            c.w64(8, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c553f); };
            c.w64(1, t121);
            blk = 105; continue;
        }
        88 => {
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c5548); };
            c.w64(1, t122);
            let Some(t123) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c554c); };
            c.w32(7, t123);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c5555); };
            c.w64(8, t124);
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_a() { blk = 182; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_b() { blk = 182; continue; }
            blk = 90; continue;
        }
        90 => {
            c.w64(7, c.r64(9));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let r125 = c.add(c.r64(7), c.r64(9), 64);
            c.w64(7, r125);
            let r126 = c.add(c.r64(7), 0x10u64, 64);
            c.w64(7, r126);
            if c.cc_e() { blk = 103; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 476; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w32(11, c.r64(7).wrapping_add(c.r64(7)));
            c.w64(12, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w32(15, c.r32(15));
            let r127 = c.imul2(c.r32(15), 0x4u64, 32);
            c.w32(15, r127);
            if c.cc_o() { blk = 186; continue; }
            blk = 93; continue;
        }
        93 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w32(3, 0x8u64);
            let r128 = c.add(c.r64(15), c.r64(3), 64);
            c.w64(15, r128);
            if c.cc_o() { blk = 185; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c55b7); };
            c.w64(2, t129);
            let Some(t130) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008c55be); };
            c.w64(2, t130);
            c.w64(6, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c55c5); };
            c.w64(9, t131);
            let Some(t132) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c55cc); };
            c.cmp(t132, c.r64(6), 64);
            if c.cc_be() { blk = 431; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_ae() { blk = 431; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c55e2); };
            c.w64(15, t133);
            if !c.st64(c.r64(15), c.r64(6)) { return RegfileExit::fault(0x70008c55e9); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            blk = 97; continue;
        }
        97 => {
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c55f0); };
            c.w64(2, t134);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008c55f7); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008c55fb); }
            c.w64(11, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(12, c.r64(3));
            blk = 99; continue;
        }
        98 => {
            c.w64(6, c.r64(3).wrapping_add(0x4));
            if !c.st32(c.r64(15).wrapping_add(c.r64(3)).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008c5610); }
            c.w64(3, c.r64(6));
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_e() { blk = 104; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w64(2, c.r64(8).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(2), c.r64(12), 64);
            if c.cc_e() { blk = 102; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t135) = c.ld32(c.r64(1).wrapping_add(c.r64(12)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c562a); };
            c.w32(2, t135);
            let r136 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r136);
            blk = 98; continue;
        }
        102 => {
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008c5635); };
            c.w64(2, t137);
            blk = 98; continue;
        }
        103 => {
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c563e); };
            c.w64(1, t138);
            c.w64(15, c.r64(1));
            blk = 104; continue;
        }
        104 => {
            c.w64(9, c.r64(7));
            c.w64(1, c.r64(15));
            blk = 105; continue;
        }
        105 => {
            let Some(t139) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c564e); };
            c.w32(7, t139);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008c5657); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008c565b); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ae() { blk = 187; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w64(7, 0xffffffffffffffffu64);
            c.w64(7, c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x8));
            if !c.st32(c.r64(1).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x70008c5674); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 108; continue; }
            blk = 107; continue;
        }
        107 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r140 = c.r64(11) & c.r64(1);
            c.w64(11, r140);
            c.logic(r140, 64);
            let Some(t141) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c5685); };
            c.logic(t141 & 0x4u64, 8);
            if c.cc_ne() { blk = 576; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t142) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c5690); };
            c.w32(7, t142);
            let r143 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r143);
            c.w32(7, c.r32(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c56a0); };
            c.w64(0, t144);
            let Some(t145) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c56a4); };
            c.w32(7, t145);
            let r146 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r146);
            c.w32(11, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 111; continue; }
            blk = 110; continue;
        }
        110 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c56b3); }
            blk = 114; continue;
        }
        111 => {
            let r147 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r147);
            if c.cc_o() { blk = 113; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 117; continue;
        }
        113 => {
            c.w64(11, c.r64(7));
            let r148 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r148);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 114; continue;
        }
        114 => {
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c56d4); };
            c.w64(11, t149);
            let Some(t150) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c56db); };
            c.w64(12, t150);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c56e3); };
            c.w64(3, t151);
            let Some(t152) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c56ea); };
            c.cmp(t152, c.r64(15), 64);
            if c.cc_be() { blk = 433; continue; }
            blk = 115; continue;
        }
        115 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008c56f3); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 116; continue;
        }
        116 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c570d); };
            c.w64(12, t153);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c5714); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008c5718); }
            blk = 117; continue;
        }
        117 => {
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c571e); };
            c.w64(12, t154);
            let r155 = c.r32(12) & 0x20u64;
            c.w32(12, r155);
            c.logic(r155, 32);
            let r156 = c.shr(c.r32(12), 0x4u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r156);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let r157 = c.r64(12) & 0xfffffffffffffffeu64;
            c.w64(12, r157);
            c.logic(r157, 64);
            let r158 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r158);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c573a); };
            c.w64(3, t159);
            blk = 176; continue;
        }
        119 => {
            c.w32(12, c.r32(11));
            c.logic(c.r32(11) & 0x80000001u64, 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c574f); };
            c.w64(3, t160);
            blk = 176; continue;
        }
        121 => {
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c575b); };
            c.w64(3, t161);
            let Some(t162) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c575f); };
            c.w32(2, t162);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), c.r64(2), 64);
            if c.cc_ae() { blk = 176; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t163) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c576e); };
            c.w32(2, t163);
            let r164 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r164);
            let Some(t165) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c5774); };
            c.w32(2, t165);
            c.logic(c.r8(2) & 0x7u64, 8);
            if c.cc_e() { blk = 138; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w64(1, c.r64(3));
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 124; continue;
        }
        124 => {
            c.w64(9, c.r64(2));
            let r166 = c.r32(9) & 0x7u64;
            c.w32(9, r166);
            c.logic(r166, 32);
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 129; continue; }
            blk = 125; continue;
        }
        125 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 126; continue;
        }
        126 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 127; continue;
        }
        127 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 128; continue;
        }
        128 => {
            blk = 435; continue;
        }
        129 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 136; continue; }
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 132; continue; }
            blk = 131; continue;
        }
        131 => {
            blk = 435; continue;
        }
        132 => {
            let Some(t167) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c57bf); };
            c.w32(2, t167);
            let Some(t168) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c57c2); };
            c.cmp(t168, c.r32(2), 32);
            if c.cc_ne() { blk = 436; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t169) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c57cf); };
            c.w32(1, t169);
            let r170 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r170);
            let Some(t171) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c57d5); };
            c.w32(2, t171);
            let r172 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r172);
            let Some(t173) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c57db); };
            c.w32(2, t173);
            blk = 124; continue;
        }
        134 => {
            let Some(t174) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c57e1); };
            c.w32(2, t174);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r175 = c.add(c.r64(6), c.r64(2), 64);
            c.w64(6, r175);
            let Some(t176) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c57ed); };
            c.w32(1, t176);
            let r177 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r177);
            let Some(t178) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c57f3); };
            c.w32(2, t178);
            let r179 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r179);
            let Some(t180) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c57f9); };
            c.w32(2, t180);
            blk = 124; continue;
        }
        135 => {
            let Some(t181) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c57ff); };
            c.w32(1, t181);
            let r182 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r182);
            let Some(t183) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c5805); };
            c.w32(2, t183);
            let r184 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r184);
            let Some(t185) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c580b); };
            c.w32(2, t185);
            blk = 124; continue;
        }
        136 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 139; continue;
        }
        137 => {
            c.w32(9, 0x1u64);
            blk = 139; continue;
        }
        138 => {
            c.w32(9, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(3));
            blk = 139; continue;
        }
        139 => {
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 437; continue; }
            blk = 140; continue;
        }
        140 => {
            c.w64(7, c.r64(1).wrapping_add(0xb));
            blk = 141; continue;
        }
        141 => {
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r186 = c.r32(2) & 0x8u64;
            c.w32(2, r186);
            c.logic(r186, 32);
            c.w64(1, c.r64(8).wrapping_add(c.r64(6)));
            c.cmp(c.r32(2), 0x8u64, 32);
            if c.cc_e() { blk = 143; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t187) = c.ld16(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(2))) else { return RegfileExit::fault(0x70008c5849); };
            c.w32(7, t187);
            blk = 144; continue;
        }
        143 => {
            let Some(t188) = c.ld8(c.r64(1).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008c584f); };
            c.w32(7, t188);
            blk = 144; continue;
        }
        144 => {
            let r189 = c.r32(7) & 0xfc00u64;
            c.w32(7, r189);
            c.logic(r189, 32);
            c.cmp(c.r32(7), 0xd800u64, 32);
            if c.cc_ne() { blk = 176; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t190) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c5865); };
            c.w32(7, t190);
            let r191 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r191);
            let Some(t192) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c586b); };
            c.w32(7, t192);
            c.logic(c.r8(7) & 0x7u64, 8);
            if c.cc_e() { blk = 161; continue; }
            blk = 146; continue;
        }
        146 => {
            c.w64(1, c.r64(3));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 147; continue;
        }
        147 => {
            c.w64(6, c.r64(7));
            let r193 = c.r32(6) & 0x7u64;
            c.w32(6, r193);
            c.logic(r193, 32);
            c.cmp(c.r32(6), 0x2u64, 32);
            if c.cc_l() { blk = 152; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r32(6), 0x2u64, 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 157; continue; }
            blk = 150; continue;
        }
        150 => {
            c.cmp(c.r32(6), 0x5u64, 32);
            if c.cc_e() { blk = 158; continue; }
            blk = 151; continue;
        }
        151 => {
            blk = 453; continue;
        }
        152 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 159; continue; }
            blk = 153; continue;
        }
        153 => {
            c.cmp(c.r32(6), 0x1u64, 32);
            if c.cc_e() { blk = 155; continue; }
            blk = 154; continue;
        }
        154 => {
            blk = 453; continue;
        }
        155 => {
            let Some(t194) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c58b0); };
            c.w32(7, t194);
            let Some(t195) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c58b3); };
            c.cmp(t195, c.r32(7), 32);
            if c.cc_ne() { blk = 454; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t196) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c58c0); };
            c.w32(1, t196);
            let r197 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r197);
            let Some(t198) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c58c6); };
            c.w32(7, t198);
            let r199 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r199);
            let Some(t200) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c58cc); };
            c.w32(7, t200);
            blk = 147; continue;
        }
        157 => {
            let Some(t201) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c58d2); };
            c.w32(7, t201);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let r202 = c.add(c.r64(2), c.r64(7), 64);
            c.w64(2, r202);
            let Some(t203) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c58de); };
            c.w32(1, t203);
            let r204 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r204);
            let Some(t205) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c58e4); };
            c.w32(7, t205);
            let r206 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r206);
            let Some(t207) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c58ea); };
            c.w32(7, t207);
            blk = 147; continue;
        }
        158 => {
            let Some(t208) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c58f0); };
            c.w32(1, t208);
            let r209 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r209);
            let Some(t210) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c58f6); };
            c.w32(7, t210);
            let r211 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r211);
            let Some(t212) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c58fc); };
            c.w32(7, t212);
            blk = 147; continue;
        }
        159 => {
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 162; continue;
        }
        160 => {
            c.w32(6, 0x1u64);
            blk = 162; continue;
        }
        161 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(3));
            blk = 162; continue;
        }
        162 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 455; continue; }
            blk = 163; continue;
        }
        163 => {
            let r213 = c.add(c.r64(1), 0xbu64, 64);
            c.w64(1, r213);
            blk = 164; continue;
        }
        164 => {
            let r214 = c.r32(7) & 0x8u64;
            c.w32(7, r214);
            c.logic(r214, 32);
            let r215 = c.add(c.r64(15), c.r64(2), 64);
            c.w64(15, r215);
            c.cmp(c.r32(7), 0x8u64, 32);
            if c.cc_e() { blk = 166; continue; }
            blk = 165; continue;
        }
        165 => {
            let Some(t216) = c.ld16(c.r64(1).wrapping_add(c.r64(15).wrapping_mul(2))) else { return RegfileExit::fault(0x70008c592f); };
            c.w32(7, t216);
            blk = 167; continue;
        }
        166 => {
            let Some(t217) = c.ld8(c.r64(15).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008c5936); };
            c.w32(7, t217);
            blk = 167; continue;
        }
        167 => {
            let r218 = c.r32(7) & 0xfc00u64;
            c.w32(7, r218);
            c.logic(r218, 32);
            c.cmp(c.r32(7), 0xdc00u64, 32);
            if c.cc_ne() { blk = 176; continue; }
            blk = 168; continue;
        }
        168 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 170; continue; }
            blk = 169; continue;
        }
        169 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c594f); }
            blk = 173; continue;
        }
        170 => {
            let r219 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r219);
            if c.cc_o() { blk = 172; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 176; continue;
        }
        172 => {
            let r220 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r220);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 173; continue;
        }
        173 => {
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c596e); };
            c.w64(7, t221);
            let Some(t222) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008c5975); };
            c.w64(11, t222);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t223) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c597c); };
            c.w64(15, t223);
            let Some(t224) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008c5983); };
            c.cmp(t224, c.r64(12), 64);
            if c.cc_be() { blk = 470; continue; }
            blk = 174; continue;
        }
        174 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x70008c598c); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 175; continue;
        }
        175 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t225) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c59a5); };
            c.w64(11, t225);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008c59ac); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008c59b0); }
            c.w64(11, c.r64(7));
            blk = 176; continue;
        }
        176 => {
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008c59b8); }
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c59bc); };
            c.w64(8, t226);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c59c0); };
            c.w64(1, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c59c4); };
            c.w64(9, t228);
            blk = 9; continue;
        }
        177 => {
            return RegfileExit::fault(0x70008c59cd);
        }
        178 => {
            let Some(t229) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c59ce); };
            c.cmp(t229, 0x0u64, 32);
            if c.cc_ne() { blk = 180; continue; }
            blk = 179; continue;
        }
        179 => {
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 189; continue;
        }
        180 => {
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c59d8); };
            c.w64(0, t230);
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(6)) { return RegfileExit::fault(0x70008c59de); }
            blk = 189; continue;
        }
        181 => {
            return RegfileExit::fault(0x70008c59e3);
        }
        182 => {
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008c59e4); };
            c.w64(2, t231);
            let Some(t232) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008c59eb); };
            c.w32(2, t232);
            let r233 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r233);
            return c.call(0x7000824234, 0x70008c59f9, 0x70008c59f4);
        }
        183 => {
            return RegfileExit::fault(0x70008c59f9);
        }
        184 => {
            return RegfileExit::fault(0x70008c59fa);
        }
        185 => {
            return RegfileExit::fault(0x70008c59fb);
        }
        186 => {
            return RegfileExit::fault(0x70008c59fc);
        }
        187 => {
            return RegfileExit::fault(0x70008c59fd);
        }
        188 => {
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008c5a01); }
            c.w64(6, c.r64(11));
            blk = 189; continue;
        }
        189 => {
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c5a08); };
            c.cmp(t234, 0x0u64, 64);
            if c.cc_e() { blk = 215; continue; }
            blk = 190; continue;
        }
        190 => {
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c5a13); };
            c.w64(11, t235);
            let Some(t236) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c5a17); };
            c.w32(7, t236);
            let r237 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r237);
            let Some(t238) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c5a1e); };
            c.w32(7, t238);
            let r239 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r239);
            let Some(t240) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c5a24); };
            c.w32(11, t240);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 192; continue; }
            blk = 191; continue;
        }
        191 => {
            return RegfileExit::fault(0x70008c5a34);
        }
        192 => {
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c5a35); };
            c.w64(1, t241);
            let Some(t242) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c5a39); };
            c.w32(11, t242);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t243) = c.ld32(c.r64(7).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008c5a40); };
            c.w32(7, t243);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c5a49); };
            c.w64(8, t244);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_a() { blk = 194; continue; }
            blk = 193; continue;
        }
        193 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_ae() { blk = 197; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t245) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008c5a58); };
            c.w64(2, t245);
            let Some(t246) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008c5a5f); };
            c.w32(2, t246);
            let r247 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r247);
            return c.call(0x7000824234, 0x70008c5a6d, 0x70008c5a68);
        }
        195 => {
            return RegfileExit::fault(0x70008c5a6d);
        }
        196 => {
            return RegfileExit::fault(0x70008c5a6e);
        }
        197 => {
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 483; continue; }
            blk = 198; continue;
        }
        198 => {
            c.w32(11, c.r64(8).wrapping_add(c.r64(8)));
            c.w64(12, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w32(15, c.r32(15));
            let r248 = c.imul2(c.r32(15), 0x4u64, 32);
            c.w32(15, r248);
            if c.cc_o() { blk = 214; continue; }
            blk = 199; continue;
        }
        199 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w32(0, 0x8u64);
            let r249 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r249);
            if c.cc_o() { blk = 213; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t250) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c5aa7); };
            c.w64(3, t250);
            let Some(t251) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c5aae); };
            c.w64(3, t251);
            c.w64(2, c.r64(3).wrapping_add(c.r64(15)));
            let Some(t252) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c5ab5); };
            c.w64(6, t252);
            let Some(t253) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008c5abc); };
            c.cmp(t253, c.r64(2), 64);
            if c.cc_be() { blk = 479; continue; }
            blk = 201; continue;
        }
        201 => {
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_ae() { blk = 479; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t254) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c5ad2); };
            c.w64(15, t254);
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x70008c5ad9); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            blk = 203; continue;
        }
        203 => {
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c5ae0); };
            c.w64(3, t255);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008c5ae7); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008c5aeb); }
            c.w64(12, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(0));
            blk = 205; continue;
        }
        204 => {
            if !c.st32(c.r64(15).wrapping_add(c.r64(0)).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008c5afc); }
            let r256 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r256);
            blk = 205; continue;
        }
        205 => {
            c.cmp(c.r64(12), c.r64(0), 64);
            if c.cc_e() { blk = 209; continue; }
            blk = 206; continue;
        }
        206 => {
            c.w64(2, c.r64(8).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_e() { blk = 208; continue; }
            blk = 207; continue;
        }
        207 => {
            let Some(t257) = c.ld32(c.r64(1).wrapping_add(c.r64(3)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c5b17); };
            c.w32(2, t257);
            let r258 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r258);
            blk = 204; continue;
        }
        208 => {
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008c5b21); };
            c.w64(2, t259);
            blk = 204; continue;
        }
        209 => {
            let Some(t260) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c5b2a); };
            c.w64(12, t260);
            let Some(t261) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008c5b31); };
            c.w64(0, t261);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            let Some(t262) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c5b39); };
            c.w64(2, t262);
            let Some(t263) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008c5b40); };
            c.cmp(t263, c.r64(3), 64);
            if c.cc_be() { blk = 481; continue; }
            blk = 210; continue;
        }
        210 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x70008c5b49); }
            c.w64(12, c.r64(0).wrapping_add(0x1));
            blk = 211; continue;
        }
        211 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008c5b51); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008c5b56); }
            let Some(t264) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c5b5b); };
            c.w64(7, t264);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008c5b62); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008c5b67); }
            c.w64(0, c.r64(12));
            blk = 212; continue;
        }
        212 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008c5b72); }
            return c.ret(0x0, 0x70008c5b73);
        }
        213 => {
            return RegfileExit::fault(0x70008c5b74);
        }
        214 => {
            return RegfileExit::fault(0x70008c5b75);
        }
        215 => {
            let Some(t265) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c5b76); };
            c.w64(0, t265);
            blk = 212; continue;
        }
        216 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t266) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c5b85); };
            c.w32(11, t266);
            let r267 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r267);
            c.logic(c.r8(7) & 0x9u64, 8);
            c.w8(7, (c.cc_ne() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008c5b98); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008c5b9c); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 218; continue; }
            blk = 217; continue;
        }
        217 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 232; continue;
        }
        218 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 417; continue; }
            blk = 219; continue;
        }
        219 => {
            let Some(t268) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c5bc3); };
            c.w32(15, t268);
            let r269 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r269);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 223; continue; }
            blk = 220; continue;
        }
        220 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c5bd0); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 222; continue; }
            blk = 221; continue;
        }
        221 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c5bdc); }
            blk = 225; continue;
        }
        222 => {
            c.w64(12, c.r64(11));
            let r270 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r270);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            blk = 225; continue;
        }
        223 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 227; continue; }
            blk = 224; continue;
        }
        224 => {
            let r271 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r271);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c5bfe); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            blk = 225; continue;
        }
        225 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 229; continue; }
            blk = 226; continue;
        }
        226 => {
            blk = 228; continue;
        }
        227 => {
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_l() { blk = 229; continue; }
            blk = 228; continue;
        }
        228 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 230; continue;
        }
        229 => {
            c.w32(12, 0x1u64);
            blk = 230; continue;
        }
        230 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 417; continue; }
            blk = 231; continue;
        }
        231 => {
            c.w64(12, c.r64(11));
            blk = 232; continue;
        }
        232 => {
            let Some(t272) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c5c32); };
            c.w32(15, t272);
            let r273 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r273);
            let Some(t274) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c5c39); };
            c.w32(15, t274);
            let r275 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r275);
            let Some(t276) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c5c40); };
            c.w32(2, t276);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008c5c4a); }
            c.cmp(c.r64(2), 0x9bu64, 64);
            if c.cc_a() { blk = 234; continue; }
            blk = 233; continue;
        }
        233 => {
            return RegfileExit::fault(0x70008c5c57);
        }
        234 => {
            let Some(t277) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c5c58); };
            c.w32(2, t277);
            let r278 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r278);
            let Some(t279) = c.ld32(c.r64(15).wrapping_add(0x273)) else { return RegfileExit::fault(0x70008c5c5e); };
            c.w32(15, t279);
            let r280 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r280);
            let Some(t281) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c5c68); };
            c.w32(2, t281);
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(15)) { return RegfileExit::fault(0x70008c5c6c); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 284; continue; }
            blk = 235; continue;
        }
        235 => {
            c.w64(1, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t282) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c5c7d); };
            c.w32(8, t282);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_b() { blk = 284; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t283) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c5c8d); };
            c.w32(9, t283);
            let r284 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r284);
            let Some(t285) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c5c94); };
            c.w32(12, t285);
            let r286 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r286);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008c5c9b); }
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 237; continue;
        }
        237 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 238; continue;
        }
        238 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 239; continue;
        }
        239 => {
            blk = 485; continue;
        }
        240 => {
            blk = 243; continue;
        }
        241 => {
            c.w64(2, c.r64(15));
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c5cbb); };
            c.w64(1, t287);
            return c.call(0x7000859a54, 0x70008c5cc4, 0x70008c5cbf);
        }
        242 => {
            c.w64(15, c.r64(0));
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c5cc7); };
            c.w64(0, t288);
            blk = 285; continue;
        }
        243 => {
            let Some(t289) = c.ld32(c.r64(9).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008c5cd0); };
            c.w32(12, t289);
            c.cmp(c.r32(12), 0x7eu64, 32);
            if c.cc_a() { blk = 498; continue; }
            blk = 244; continue;
        }
        244 => {
            c.logic(c.r8(2) & 0x7u64, 8);
            if c.cc_e() { blk = 261; continue; }
            blk = 245; continue;
        }
        245 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008c5ce7); }
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(3));
            blk = 247; continue;
        }
        246 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008c5cf2); }
            blk = 247; continue;
        }
        247 => {
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c5cf6); };
            c.w64(11, t290);
            let r291 = c.r32(11) & 0x7u64;
            c.w32(11, r291);
            c.logic(r291, 32);
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_l() { blk = 252; continue; }
            blk = 248; continue;
        }
        248 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 249; continue;
        }
        249 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 257; continue; }
            blk = 250; continue;
        }
        250 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_e() { blk = 258; continue; }
            blk = 251; continue;
        }
        251 => {
            blk = 498; continue;
        }
        252 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 259; continue; }
            blk = 253; continue;
        }
        253 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 255; continue; }
            blk = 254; continue;
        }
        254 => {
            blk = 498; continue;
        }
        255 => {
            let Some(t292) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c5d30); };
            c.w32(11, t292);
            let Some(t293) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c5d34); };
            c.cmp(t293, c.r32(11), 32);
            if c.cc_ne() { blk = 498; continue; }
            blk = 256; continue;
        }
        256 => {
            let Some(t294) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c5d41); };
            c.w32(7, t294);
            let r295 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r295);
            let Some(t296) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c5d47); };
            c.w32(11, t296);
            let r297 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r297);
            let Some(t298) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c5d4e); };
            c.w32(11, t298);
            blk = 246; continue;
        }
        257 => {
            let Some(t299) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c5d55); };
            c.w32(11, t299);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r300 = c.add(c.r64(2), c.r64(11), 64);
            c.w64(2, r300);
            let Some(t301) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c5d62); };
            c.w32(7, t301);
            let r302 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r302);
            let Some(t303) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c5d68); };
            c.w32(11, t303);
            let r304 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r304);
            let Some(t305) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c5d6f); };
            c.w32(11, t305);
            blk = 246; continue;
        }
        258 => {
            let Some(t306) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c5d79); };
            c.w32(7, t306);
            let r307 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r307);
            let Some(t308) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c5d7f); };
            c.w32(11, t308);
            let r309 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r309);
            let Some(t310) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c5d86); };
            c.w32(11, t310);
            blk = 246; continue;
        }
        259 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 262; continue;
        }
        260 => {
            c.w32(11, 0x1u64);
            blk = 262; continue;
        }
        261 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008c5d9d); }
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(3));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 262; continue;
        }
        262 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 487; continue; }
            blk = 263; continue;
        }
        263 => {
            let r311 = c.add(c.r64(7), 0xbu64, 64);
            c.w64(7, r311);
            blk = 264; continue;
        }
        264 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(8)));
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c5dbb); };
            c.w64(8, t312);
            let r313 = c.r32(8) & 0x8u64;
            c.w32(8, r313);
            c.logic(r313, 32);
            let r314 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r314);
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_ne() { blk = 489; continue; }
            blk = 265; continue;
        }
        265 => {
            let r315 = c.add(c.r64(11), c.r64(7), 64);
            c.w64(11, r315);
            let r316 = c.add(c.r64(7), c.r64(2), 64);
            c.w64(7, r316);
            let Some(t317) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c5dd6); };
            c.w32(2, t317);
            let r318 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r318);
            blk = 266; continue;
        }
        266 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008c5ddd); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008c5de1); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 498; continue; }
            blk = 267; continue;
        }
        267 => {
            let Some(t319) = c.ld32(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008c5dee); };
            c.w32(8, t319);
            let Some(t320) = c.ld64(c.r64(13).wrapping_add(0x2e70)) else { return RegfileExit::fault(0x70008c5df2); };
            c.w64(7, t320);
            let Some(t321) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008c5df9); };
            c.w64(7, t321);
            c.cmp(c.r32(8), 0xffffffffffffffffu64, 32);
            if c.cc_e() { blk = 269; continue; }
            blk = 268; continue;
        }
        268 => {
            c.w32(2, c.r64(8).wrapping_mul(8));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(13));
            let Some(t322) = c.ld64(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008c5e10); };
            c.w64(2, t322);
            blk = 270; continue;
        }
        269 => {
            let r323 = c.add(c.r64(2), 0x3fu64, 64);
            c.w64(2, r323);
            c.w64(8, c.r64(13));
            blk = 270; continue;
        }
        270 => {
            let r324 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r324);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w32(1, c.r32(1));
            c.w64(10, c.r64(4));
            let r325 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r325);
            let r326 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r326);
            c.logic(r326, 64);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x70008c5e38); }
            let r327 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r327);
            let Some(t328) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008c5e40); };
            c.w64(11, t328);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(11)) { return RegfileExit::fault(0x70008c5e47); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(12)) { return RegfileExit::fault(0x70008c5e4c); }
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(7)) { return RegfileExit::fault(0x70008c5e51); }
            if !c.st64(c.r64(4).wrapping_add(0x38), 0x1u64) { return RegfileExit::fault(0x70008c5e56); }
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(8)) { return RegfileExit::fault(0x70008c5e5f); }
            c.w64(7, c.r64(0));
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(7)) { return RegfileExit::fault(0x70008c5e67); }
            c.w64(7, c.r64(2));
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(3));
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c5e75); };
            c.w64(8, t329);
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c5e79); };
            c.w64(9, t330);
            c.w64(10, 0x70008c5e84u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008c5e84); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008c5e88); }
            return c.call(c.r64(7), 0x70008c5e8e, 0x70008c5e8c);
        }
        271 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008c5e8e); }
            let Some(t331) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x70008c5e96); };
            c.w64(4, t331);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_e() { blk = 274; continue; }
            blk = 272; continue;
        }
        272 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_ne() { blk = 490; continue; }
            blk = 273; continue;
        }
        273 => {
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c5eae); };
            c.w64(0, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c5eb2); };
            c.w64(6, t333);
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c5eb6); };
            c.w64(3, t334);
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c5eba); };
            c.w64(11, t335);
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c5ebe); };
            c.w64(7, t336);
            blk = 284; continue;
        }
        274 => {
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c5ec7); };
            c.w64(15, t337);
            let Some(t338) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c5ecb); };
            c.w32(7, t338);
            let Some(t339) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c5ecf); };
            c.w64(11, t339);
            let Some(t340) = c.ld32(c.r64(11).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008c5ed3); };
            c.w32(11, t340);
            let r341 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r341);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r342 = c.sub(c.r32(7), 0x6u64, 32);
            c.w32(7, r342);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_l() { blk = 497; continue; }
            blk = 275; continue;
        }
        275 => {
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008c5eed); }
            let Some(t343) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c5ef1); };
            c.w64(0, t343);
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008c5ef5); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 277; continue; }
            blk = 276; continue;
        }
        276 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r344 = c.r64(7) & c.r64(15);
            c.w64(7, r344);
            c.logic(r344, 64);
            let Some(t345) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c5f07); };
            c.logic(t345 & 0x4u64, 8);
            if c.cc_ne() { blk = 573; continue; }
            blk = 277; continue;
        }
        277 => {
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008c5f11); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 279; continue; }
            blk = 278; continue;
        }
        278 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r346 = c.r64(7) & c.r64(15);
            c.w64(7, r346);
            c.logic(r346, 64);
            let Some(t347) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c5f23); };
            c.logic(t347 & 0x4u64, 8);
            if c.cc_ne() { blk = 570; continue; }
            blk = 279; continue;
        }
        279 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if c.cc_e() { blk = 285; continue; }
            blk = 280; continue;
        }
        280 => {
            c.w32(11, 0x13u64);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t348) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008c5f3e); };
            c.w64(3, t348);
            blk = 282; continue;
        }
        281 => {
            let r349 = c.add(c.r64(11), 0x4u64, 64);
            c.w64(11, r349);
            blk = 282; continue;
        }
        282 => {
            let Some(t350) = c.ld32(c.r64(3).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008c5f4b); };
            c.w32(2, t350);
            let r351 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r351);
            if !c.st32(c.r64(15).wrapping_add(c.r64(11)), c.r32(2)) { return RegfileExit::fault(0x70008c5f51); }
            let r352 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r352);
            c.cmp(c.r64(7), c.r64(12), 64);
            if c.cc_ne() { blk = 281; continue; }
            blk = 283; continue;
        }
        283 => {
            blk = 285; continue;
        }
        284 => {
            let Some(t353) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c5f60); };
            c.w64(15, t353);
            c.w64(0, c.r64(3));
            blk = 285; continue;
        }
        285 => {
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(15)) { return RegfileExit::fault(0x70008c5f6a); }
            let Some(t354) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c5f6e); };
            c.cmp(t354, c.r32(15), 32);
            if c.cc_ne() { blk = 288; continue; }
            blk = 286; continue;
        }
        286 => {
            let Some(t355) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c5f77); };
            c.cmp(t355, 0x0u64, 32);
            if c.cc_e() { blk = 418; continue; }
            blk = 287; continue;
        }
        287 => {
            let Some(t356) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c5f81); };
            c.w64(7, t356);
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(2)) { return RegfileExit::fault(0x70008c5f87); }
            blk = 418; continue;
        }
        288 => {
            let Some(t357) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c5f8f); };
            c.cmp(t357, 0x0u64, 32);
            if c.cc_e() { blk = 292; continue; }
            blk = 289; continue;
        }
        289 => {
            let Some(t358) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c5f95); };
            c.w32(7, t358);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x4u64, 64);
            if c.cc_a() { blk = 291; continue; }
            blk = 290; continue;
        }
        290 => {
            return RegfileExit::fault(0x70008c5fa5);
        }
        291 => {
            let Some(t359) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c5fa6); };
            c.w32(11, t359);
            let Some(t360) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c5faa); };
            c.w64(7, t360);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008c5fae); }
            blk = 292; continue;
        }
        292 => {
            let Some(t361) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c5fb2); };
            c.w32(11, t361);
            let Some(t362) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c5fb6); };
            c.w32(3, t362);
            let r363 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r363);
            let Some(t364) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c5fbd); };
            c.w32(1, t364);
            let r365 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r365);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008c5fc4); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008c5fc8); }
            let Some(t366) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c5fcc); };
            c.w64(6, t366);
            return c.call(0x700086fe54, 0x70008c5fd5, 0x70008c5fd0);
        }
        293 => {
            let Some(t367) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c5fd5); };
            c.w64(7, t367);
            let Some(t368) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c5fd9); };
            c.w32(11, t368);
            let r369 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r369);
            c.logic(c.r16(11) & 0x100u64, 16);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008c5fee); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008c5ff2); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 313; continue; }
            blk = 294; continue;
        }
        294 => {
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c6000); };
            c.w64(6, t370);
            let Some(t371) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c6004); };
            c.w32(12, t371);
            let r372 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r372);
            let Some(t373) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c600b); };
            c.w32(12, t373);
            let r374 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r374);
            let Some(t375) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c6013); };
            c.w32(15, t375);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xa1u64, 64);
            if c.cc_a() { blk = 296; continue; }
            blk = 295; continue;
        }
        295 => {
            return RegfileExit::fault(0x70008c6027);
        }
        296 => {
            let Some(t376) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c6028); };
            c.w64(15, (t376 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let Some(t377) = c.ld32(c.r64(12).wrapping_add(0x28b)) else { return RegfileExit::fault(0x70008c602f); };
            c.w32(12, t377);
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r378 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r378);
            c.w64(3, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(3));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x70008c6046); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 310; continue; }
            blk = 297; continue;
        }
        297 => {
            c.w64(15, c.r64(3).wrapping_mul(2).wrapping_add(0x30));
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_be() { blk = 307; continue; }
            blk = 298; continue;
        }
        298 => {
            c.cmp(c.r64(2), 0x2000000u64, 64);
            if c.cc_a() { blk = 500; continue; }
            blk = 299; continue;
        }
        299 => {
            c.cmp(c.r64(2), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 505; continue; }
            blk = 300; continue;
        }
        300 => {
            let Some(t379) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c6083); };
            c.w64(15, (t379 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r380 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r380);
            c.w64(3, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t381) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c6097); };
            c.w64(3, t381);
            let Some(t382) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c609e); };
            c.w64(1, t382);
            c.w64(9, c.r64(8).wrapping_add(0x8));
            c.w64(7, c.r64(9).wrapping_add(c.r64(1)));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(15)) { return RegfileExit::fault(0x70008c60a9); }
            let Some(t383) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c60ad); };
            c.w64(11, t383);
            let Some(t384) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c60b4); };
            c.cmp(t384, c.r64(7), 64);
            if c.cc_be() { blk = 501; continue; }
            blk = 301; continue;
        }
        301 => {
            c.cmp(c.r64(9), 0x20000u64, 64);
            if c.cc_ae() { blk = 501; continue; }
            blk = 302; continue;
        }
        302 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x70008c60ca); }
            c.w64(7, c.r64(1).wrapping_add(0x1));
            blk = 303; continue;
        }
        303 => {
            let r385 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r385);
            let Some(t386) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c60d3); };
            c.w64(1, t386);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008c60da); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008c60dd); }
            c.w64(1, c.r64(7).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r387 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r387);
            let r388 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r388);
            c.logic(r388, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008c60ef); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x70008c60f4); }
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t389) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008c60fa); };
            c.w64(0, t389);
            c.w64(10, 0x70008c6108u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008c6108); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008c610c); }
            return c.call(c.r64(0), 0x70008c6112, 0x70008c6110);
        }
        304 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008c6112); }
            let Some(t390) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008c611a); };
            c.w64(4, t390);
            let Some(t391) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c611f); };
            c.w64(7, t391);
            let Some(t392) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008c6126); };
            c.w64(11, t392);
            c.w64(12, c.r64(11).wrapping_add(0x28));
            let Some(t393) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c612d); };
            c.w64(15, t393);
            let Some(t394) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008c6134); };
            c.cmp(t394, c.r64(12), 64);
            if c.cc_be() { blk = 503; continue; }
            blk = 305; continue;
        }
        305 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x70008c613d); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 306; continue;
        }
        306 => {
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c6144); };
            c.w64(11, t395);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008c6148); }
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c614c); };
            c.w64(11, t396);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008c6150); }
            let Some(t397) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c6154); };
            c.w64(11, t397);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008c615b); }
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c615f); };
            c.w64(0, t398);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008c6163); }
            c.w64(3, c.r64(7));
            c.w64(12, c.r64(0));
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c616c); };
            c.w64(7, t399);
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c6170); };
            c.w64(6, t400);
            let Some(t401) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c6174); };
            c.w64(11, t401);
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c6178); };
            c.w64(0, t402);
            blk = 332; continue;
        }
        307 => {
            c.w32(3, c.r64(2).wrapping_add(c.r64(2)));
            let Some(t403) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c6184); };
            c.w64(2, t403);
            let Some(t404) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008c618b); };
            c.w64(1, t404);
            c.w64(8, c.r64(15).wrapping_add(c.r64(1)));
            let Some(t405) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c6192); };
            c.w64(9, t405);
            let Some(t406) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c6199); };
            c.cmp(t406, c.r64(8), 64);
            if c.cc_be() { blk = 507; continue; }
            blk = 308; continue;
        }
        308 => {
            if !c.st64(c.r64(2), c.r64(8)) { return RegfileExit::fault(0x70008c61a2); }
            c.w64(15, c.r64(1).wrapping_add(0x1));
            blk = 309; continue;
        }
        309 => {
            let Some(t407) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c61a9); };
            c.w64(2, (t407 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if let Some(sr) = c.shr(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r408 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r408);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c61b5); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x70008c61b9); }
            let Some(t409) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c61bd); };
            c.w64(12, t409);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008c61c4); }
            c.w64(12, c.r64(15).wrapping_add(0x28));
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c61cc); }
            let Some(t410) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c61d0); };
            c.w64(2, t410);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008c61d7); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008c61dc); }
            c.w64(3, c.r64(15));
            blk = 332; continue;
        }
        310 => {
            let Some(t411) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c61e9); };
            c.w64(3, t411);
            let Some(t412) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c61f0); };
            c.w64(2, t412);
            c.w64(1, c.r64(2).wrapping_add(0x28));
            let Some(t413) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c61f7); };
            c.w64(8, t413);
            let Some(t414) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008c61fe); };
            c.cmp(t414, c.r64(1), 64);
            if c.cc_be() { blk = 509; continue; }
            blk = 311; continue;
        }
        311 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x70008c6207); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 312; continue;
        }
        312 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c620e); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008c6212); }
            let Some(t415) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c6216); };
            c.w64(12, t415);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008c621d); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c6221); }
            blk = 332; continue;
        }
        313 => {
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c622a); };
            c.w64(6, t416);
            let Some(t417) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c622e); };
            c.w32(12, t417);
            let r418 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r418);
            let Some(t419) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c6235); };
            c.w32(12, t419);
            let r420 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r420);
            let Some(t421) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c623d); };
            c.w32(15, t421);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xa2u64, 64);
            if c.cc_a() { blk = 315; continue; }
            blk = 314; continue;
        }
        314 => {
            return RegfileExit::fault(0x70008c6251);
        }
        315 => {
            let Some(t422) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c6252); };
            c.w64(15, (t422 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r423 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r423);
            c.w64(3, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t424) = c.ld32(c.r64(12).wrapping_add(0x28f)) else { return RegfileExit::fault(0x70008c6262); };
            c.w32(12, t424);
            c.w64(2, c.r64(3));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x70008c6270); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 329; continue; }
            blk = 316; continue;
        }
        316 => {
            c.w64(15, c.r64(3).wrapping_mul(2).wrapping_add(0x34));
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_be() { blk = 326; continue; }
            blk = 317; continue;
        }
        317 => {
            c.cmp(c.r64(2), 0x2000000u64, 64);
            if c.cc_a() { blk = 511; continue; }
            blk = 318; continue;
        }
        318 => {
            c.cmp(c.r64(2), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 516; continue; }
            blk = 319; continue;
        }
        319 => {
            let Some(t425) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c62ad); };
            c.w64(15, (t425 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r426 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r426);
            c.w64(3, (c.r32(15) as u32 as i32 as i64 as u64));
            let r427 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r427);
            c.w64(8, c.r64(3).wrapping_add(c.r64(3)));
            c.w64(3, c.r64(8).wrapping_add(0x8));
            let Some(t428) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c62c7); };
            c.w64(1, t428);
            let Some(t429) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008c62ce); };
            c.w64(9, t429);
            c.w64(7, c.r64(3).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(15)) { return RegfileExit::fault(0x70008c62d5); }
            let Some(t430) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c62d9); };
            c.w64(11, t430);
            let Some(t431) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c62e0); };
            c.cmp(t431, c.r64(7), 64);
            if c.cc_be() { blk = 512; continue; }
            blk = 320; continue;
        }
        320 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_ae() { blk = 512; continue; }
            blk = 321; continue;
        }
        321 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x70008c62f6); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 322; continue;
        }
        322 => {
            let Some(t432) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c62fd); };
            c.w64(3, t432);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008c6304); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008c6307); }
            c.w64(3, c.r64(7).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r433 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r433);
            let r434 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r434);
            c.logic(r434, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008c6319); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x70008c631e); }
            c.w64(1, c.r64(3));
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t435) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008c6327); };
            c.w64(0, t435);
            c.w64(10, 0x70008c6335u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008c6335); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008c6339); }
            return c.call(c.r64(0), 0x70008c633f, 0x70008c633d);
        }
        323 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008c633f); }
            let Some(t436) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008c6347); };
            c.w64(4, t436);
            let Some(t437) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c634c); };
            c.w64(7, t437);
            let Some(t438) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008c6353); };
            c.w64(11, t438);
            c.w64(12, c.r64(11).wrapping_add(0x2c));
            let Some(t439) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c635a); };
            c.w64(15, t439);
            let Some(t440) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008c6361); };
            c.cmp(t440, c.r64(12), 64);
            if c.cc_be() { blk = 514; continue; }
            blk = 324; continue;
        }
        324 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x70008c636a); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 325; continue;
        }
        325 => {
            let Some(t441) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c6371); };
            c.w64(11, t441);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008c6375); }
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c6379); };
            c.w64(11, t442);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008c637d); }
            let Some(t443) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c6381); };
            c.w64(11, t443);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008c6388); }
            let Some(t444) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c638c); };
            c.w64(0, t444);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008c6390); }
            c.w64(3, c.r64(7));
            c.w64(12, c.r64(0));
            let Some(t445) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6399); };
            c.w64(7, t445);
            let Some(t446) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c639d); };
            c.w64(6, t446);
            let Some(t447) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c63a1); };
            c.w64(11, t447);
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c63a5); };
            c.w64(0, t448);
            blk = 332; continue;
        }
        326 => {
            let Some(t449) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c63ae); };
            c.w64(3, t449);
            let Some(t450) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c63b5); };
            c.w64(1, t450);
            c.w64(8, c.r64(15).wrapping_add(c.r64(1)));
            let Some(t451) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c63bc); };
            c.w64(9, t451);
            let Some(t452) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c63c3); };
            c.cmp(t452, c.r64(8), 64);
            if c.cc_be() { blk = 518; continue; }
            blk = 327; continue;
        }
        327 => {
            if !c.st64(c.r64(3), c.r64(8)) { return RegfileExit::fault(0x70008c63cc); }
            c.w64(15, c.r64(1).wrapping_add(0x1));
            blk = 328; continue;
        }
        328 => {
            let Some(t453) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c63d3); };
            c.w64(3, (t453 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r454 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r454);
            let r455 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r455);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c63e1); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008c63e5); }
            let Some(t456) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c63e9); };
            c.w64(12, t456);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008c63f0); }
            c.w64(12, c.r64(15).wrapping_add(0x2c));
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c63f8); }
            let Some(t457) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c63fc); };
            c.w64(3, t457);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008c6403); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008c6408); }
            c.w64(3, c.r64(15));
            blk = 332; continue;
        }
        329 => {
            let Some(t458) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c6412); };
            c.w64(3, t458);
            let Some(t459) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c6419); };
            c.w64(2, t459);
            c.w64(1, c.r64(2).wrapping_add(0x2c));
            let Some(t460) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c6420); };
            c.w64(8, t460);
            let Some(t461) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008c6427); };
            c.cmp(t461, c.r64(1), 64);
            if c.cc_be() { blk = 520; continue; }
            blk = 330; continue;
        }
        330 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x70008c6430); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 331; continue;
        }
        331 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c6437); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008c643b); }
            let Some(t462) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c643f); };
            c.w64(12, t462);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008c6446); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c644a); }
            blk = 332; continue;
        }
        332 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008c644e); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(12)) { return RegfileExit::fault(0x70008c6452); }
            let Some(t463) = c.ld8(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c6456); };
            c.logic(t463 & 0x1u64, 8);
            if c.cc_e() { blk = 334; continue; }
            blk = 333; continue;
        }
        333 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 335; continue;
        }
        334 => {
            let Some(t464) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c6461); };
            c.w64(15, t464);
            blk = 335; continue;
        }
        335 => {
            let Some(t465) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c6465); };
            c.w64(2, t465);
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x70008c6469); }
            let Some(t466) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c646c); };
            c.w64(2, t466);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x70008c6470); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 337; continue; }
            blk = 336; continue;
        }
        336 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r467 = c.r64(1) & c.r64(3);
            c.w64(1, r467);
            c.logic(r467, 64);
            let Some(t468) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c6482); };
            c.logic(t468 & 0x4u64, 8);
            if c.cc_ne() { blk = 567; continue; }
            blk = 337; continue;
        }
        337 => {
            let Some(t469) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c648c); };
            c.w64(1, t469);
            if !c.st32(c.r64(3).wrapping_add(0x17), c.r32(1)) { return RegfileExit::fault(0x70008c6493); }
            if !c.st32(c.r64(3).wrapping_add(0x1b), c.r32(1)) { return RegfileExit::fault(0x70008c6496); }
            if !c.st32(c.r64(3).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x70008c6499); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 339; continue; }
            blk = 338; continue;
        }
        338 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r470 = c.r64(8) & c.r64(3);
            c.w64(8, r470);
            c.logic(r470, 64);
            let Some(t471) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c64ab); };
            c.logic(t471 & 0x4u64, 8);
            if c.cc_ne() { blk = 564; continue; }
            blk = 339; continue;
        }
        339 => {
            if !c.st32(c.r64(3).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x70008c64b6); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 341; continue; }
            blk = 340; continue;
        }
        340 => {
            if !c.st32(c.r64(3).wrapping_add(0x27), c.r32(1)) { return RegfileExit::fault(0x70008c64c0); }
            blk = 341; continue;
        }
        341 => {
            let Some(t472) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c64c3); };
            c.w64(15, (t472 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r473 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r473);
            c.w64(8, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 343; continue; }
            blk = 342; continue;
        }
        342 => {
            let r474 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r474);
            if !c.st32(c.r64(8).wrapping_add(c.r64(12)), c.r32(1)) { return RegfileExit::fault(0x70008c64e5); }
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 342; continue; }
            blk = 343; continue;
        }
        343 => {
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008c64ef); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 345; continue; }
            blk = 344; continue;
        }
        344 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r475 = c.r64(8) & c.r64(12);
            c.w64(8, r475);
            c.logic(r475, 64);
            let Some(t476) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c6502); };
            c.logic(t476 & 0x4u64, 8);
            if c.cc_ne() { blk = 561; continue; }
            blk = 345; continue;
        }
        345 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 411; continue; }
            blk = 346; continue;
        }
        346 => {
            let Some(t477) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c6517); };
            c.w64(15, (t477 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r478 = c.add(c.r64(15), 0x3u64, 64);
            c.w64(15, r478);
            c.w32(0, 0x1u64);
            c.w32(8, 0x5u64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008c652d); }
            blk = 348; continue;
        }
        347 => {
            c.w64(0, c.r64(7).wrapping_add(0x1));
            let Some(t479) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c6537); };
            c.w64(2, t479);
            let Some(t480) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c653b); };
            c.w64(1, t480);
            let Some(t481) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c6542); };
            c.w64(3, t481);
            let Some(t482) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c6546); };
            c.w64(6, t482);
            blk = 348; continue;
        }
        348 => {
            let Some(t483) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c654a); };
            c.w64(9, t483);
            let Some(t484) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c654e); };
            c.w32(7, t484);
            let r485 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r485);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008c6556); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008c655a); }
            c.cmp(c.r32(7), 0xfffffffffffffffeu64, 32);
            if c.cc_e() { blk = 353; continue; }
            blk = 349; continue;
        }
        349 => {
            c.w64(7, c.r64(8).wrapping_add(0x1));
            let Some(t486) = c.ld32(c.r64(9).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c6567); };
            c.w32(7, t486);
            let r487 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r487);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008c656f); }
            c.w64(0, c.r64(2));
            let Some(t488) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c6576); };
            c.w64(3, t488);
            c.w64(1, c.r64(7));
            return c.call(0x700086fe54, 0x70008c6582, 0x70008c657d);
        }
        350 => {
            let Some(t489) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c6582); };
            c.w64(7, t489);
            c.w64(11, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            let Some(t490) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c658e); };
            c.w64(12, t490);
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008c6592); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 352; continue; }
            blk = 351; continue;
        }
        351 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r491 = c.r64(15) & c.r64(12);
            c.w64(15, r491);
            c.logic(r491, 64);
            let Some(t492) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c65a4); };
            c.logic(t492 & 0x4u64, 8);
            if c.cc_ne() { blk = 558; continue; }
            blk = 352; continue;
        }
        352 => {
            blk = 354; continue;
        }
        353 => {
            c.w64(7, c.r64(0));
            blk = 354; continue;
        }
        354 => {
            let Some(t493) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c65b4); };
            c.w64(8, t493);
            let r494 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r494);
            let Some(t495) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c65bc); };
            c.cmp(c.r64(8), t495, 64);
            if c.cc_b() { blk = 347; continue; }
            blk = 355; continue;
        }
        355 => {
            let Some(t496) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c65c6); };
            c.w64(7, t496);
            let Some(t497) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c65ca); };
            c.w32(11, t497);
            let r498 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r498);
            let Some(t499) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c65d1); };
            c.w32(15, t499);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x9u64, 64);
            if c.cc_be() { blk = 522; continue; }
            blk = 356; continue;
        }
        356 => {
            let Some(t500) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008c65e5); };
            c.w32(11, t500);
            let r501 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r501);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008c65ec); }
            c.w32(15, 0);
            c.logic(0, 32);
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_e() { blk = 411; continue; }
            blk = 357; continue;
        }
        357 => {
            let Some(t502) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c65fc); };
            c.w32(0, t502);
            let Some(t503) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c6600); };
            c.w64(3, t503);
            if !c.st32(c.r64(3).wrapping_add(0x1b), c.r32(11)) { return RegfileExit::fault(0x70008c6604); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 359; continue; }
            blk = 358; continue;
        }
        358 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r504 = c.r64(2) & c.r64(3);
            c.w64(2, r504);
            c.logic(r504, 64);
            let Some(t505) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c6618); };
            c.logic(t505 & 0x4u64, 8);
            if c.cc_ne() { blk = 555; continue; }
            blk = 359; continue;
        }
        359 => {
            let Some(t506) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c6622); };
            c.w64(6, t506);
            let Some(t507) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c6626); };
            c.w32(2, t507);
            let r508 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r508);
            let Some(t509) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c662c); };
            c.w32(2, t509);
            let r510 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r510);
            let Some(t511) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c6632); };
            c.w32(1, t511);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0xb1u64, 64);
            if c.cc_a() { blk = 361; continue; }
            blk = 360; continue;
        }
        360 => {
            return RegfileExit::fault(0x70008c6644);
        }
        361 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(1, c.r64(0));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(8, c.r64(1));
            if let Some(sr) = c.shr(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(1, c.r64(8).wrapping_add(c.r64(1)).wrapping_sub(0x1));
            c.w64(8, c.r64(1));
            if let Some(sr) = c.shr(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r512 = c.r64(8) | c.r64(1);
            c.w64(8, r512);
            c.logic(r512, 64);
            c.w64(1, c.r64(8));
            if let Some(sr) = c.shr(c.r64(1), 0x2u64, 64) { c.w64(1, sr); }
            let r513 = c.r64(1) | c.r64(8);
            c.w64(1, r513);
            c.logic(r513, 64);
            c.w64(8, c.r64(1));
            if let Some(sr) = c.shr(c.r64(8), 0x4u64, 64) { c.w64(8, sr); }
            let r514 = c.r64(8) | c.r64(1);
            c.w64(8, r514);
            c.logic(r514, 64);
            c.w64(1, c.r64(8));
            if let Some(sr) = c.shr(c.r64(1), 0x8u64, 64) { c.w64(1, sr); }
            let r515 = c.r64(1) | c.r64(8);
            c.w64(1, r515);
            c.logic(r515, 64);
            c.w64(8, c.r64(1));
            if let Some(sr) = c.shr(c.r64(8), 0x10u64, 64) { c.w64(8, sr); }
            let r516 = c.r64(8) | c.r64(1);
            c.w64(8, r516);
            c.logic(r516, 64);
            c.w64(1, c.r64(8).wrapping_add(0x1));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008c6691); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_ge() { blk = 363; continue; }
            blk = 362; continue;
        }
        362 => {
            c.w32(1, 0x4u64);
            blk = 363; continue;
        }
        363 => {
            let Some(t517) = c.ld32(c.r64(2).wrapping_add(0x2cb)) else { return RegfileExit::fault(0x70008c66a0); };
            c.w32(2, t517);
            let r518 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r518);
            c.w64(8, c.r64(1).wrapping_add(c.r64(1).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t519) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c66ae); };
            c.w64(9, t519);
            let Some(t520) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c66b5); };
            c.w64(9, t520);
            c.w64(7, c.r64(8).wrapping_mul(4).wrapping_add(0x8));
            c.w64(6, c.r64(7).wrapping_add(c.r64(9)));
            let Some(t521) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c66c4); };
            c.w64(0, t521);
            let Some(t522) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008c66cb); };
            c.cmp(t522, c.r64(6), 64);
            if c.cc_be() { blk = 523; continue; }
            blk = 364; continue;
        }
        364 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 523; continue; }
            blk = 365; continue;
        }
        365 => {
            let Some(t523) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c66e1); };
            c.w64(0, t523);
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x70008c66e8); }
            c.w64(0, c.r64(9).wrapping_add(0x1));
            blk = 366; continue;
        }
        366 => {
            c.w32(6, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t524) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008c66f3); };
            c.w64(8, t524);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008c66fa); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x70008c66fe); }
            let Some(t525) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c6701); };
            c.w32(6, t525);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_be() { blk = 525; continue; }
            blk = 367; continue;
        }
        367 => {
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008c6714); }
            let Some(t526) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c6718); };
            c.w32(6, t526);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x1u64, 64);
            if c.cc_be() { blk = 526; continue; }
            blk = 368; continue;
        }
        368 => {
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008c672b); }
            let Some(t527) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c672f); };
            c.w32(6, t527);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x2u64, 64);
            if c.cc_be() { blk = 527; continue; }
            blk = 369; continue;
        }
        369 => {
            let r528 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r528);
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008c6744); }
            let Some(t529) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c6747); };
            c.w32(1, t529);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_be() { blk = 528; continue; }
            blk = 370; continue;
        }
        370 => {
            c.w32(1, 0x2u64);
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x70008c675f); }
            let Some(t530) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c6762); };
            c.w32(1, t530);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_be() { blk = 529; continue; }
            blk = 371; continue;
        }
        371 => {
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70008c6775); }
            c.w64(7, c.r64(0).wrapping_add(c.r64(7)).wrapping_sub(0x1));
            c.w64(1, c.r64(0).wrapping_add(0x1b));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 375; continue; }
            blk = 372; continue;
        }
        372 => {
            let Some(t531) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c6787); };
            c.w64(6, t531);
            blk = 373; continue;
        }
        373 => {
            if !c.st32(c.r64(1), c.r32(6)) { return RegfileExit::fault(0x70008c678e); }
            let r532 = c.add(c.r64(1), 0x4u64, 64);
            c.w64(1, r532);
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_ne() { blk = 373; continue; }
            blk = 374; continue;
        }
        374 => {
            blk = 376; continue;
        }
        375 => {
            let Some(t533) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c679b); };
            c.w64(6, t533);
            blk = 376; continue;
        }
        376 => {
            let Some(t534) = c.ld8(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c67a2); };
            c.w32(7, t534);
            let Some(t535) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c67a6); };
            c.w64(1, t535);
            let Some(t536) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008c67ad); };
            c.w64(8, t536);
            if let Some(sr) = c.shl(c.r64(7), 0x2u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7).wrapping_add(c.r64(8)));
            let Some(t537) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c67b8); };
            c.w64(11, t537);
            let Some(t538) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c67bf); };
            c.cmp(t538, c.r64(9), 64);
            if c.cc_be() { blk = 530; continue; }
            blk = 377; continue;
        }
        377 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008c67c8); }
            c.w64(11, c.r64(8).wrapping_add(0x1));
            blk = 378; continue;
        }
        378 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008c67cf); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008c67d3); }
            let Some(t539) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c67d7); };
            c.w64(2, t539);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008c67de); }
            let r540 = c.sub(c.r64(7), 0x1u64, 64);
            c.w64(7, r540);
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_e() { blk = 380; continue; }
            blk = 379; continue;
        }
        379 => {
            let r541 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r541);
            if !c.st32(c.r64(7).wrapping_add(c.r64(11)), c.r32(6)) { return RegfileExit::fault(0x70008c67f0); }
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_ne() { blk = 379; continue; }
            blk = 380; continue;
        }
        380 => {
            if !c.st32(c.r64(3).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008c67fa); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 382; continue; }
            blk = 381; continue;
        }
        381 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r542 = c.r64(7) & c.r64(3);
            c.w64(7, r542);
            c.logic(r542, 64);
            let Some(t543) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c680e); };
            c.logic(t543 & 0x4u64, 8);
            if c.cc_ne() { blk = 552; continue; }
            blk = 382; continue;
        }
        382 => {
            c.w64(7, c.r64(15));
            blk = 384; continue;
        }
        383 => {
            let Some(t544) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c681d); };
            c.w64(12, t544);
            blk = 384; continue;
        }
        384 => {
            let Some(t545) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c6821); };
            c.w64(11, t545);
            let Some(t546) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c6825); };
            c.w32(1, t546);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_ae() { blk = 532; continue; }
            blk = 385; continue;
        }
        385 => {
            c.w64(6, c.r64(7).wrapping_add(0x1));
            let Some(t547) = c.ld32(c.r64(11).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c683c); };
            c.w32(7, t547);
            let r548 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r548);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008c6844); }
            c.cmp(c.r64(6), c.r64(1), 64);
            if c.cc_ae() { blk = 533; continue; }
            blk = 386; continue;
        }
        386 => {
            let Some(t549) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c6851); };
            c.w32(1, t549);
            let Some(t550) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c6856); };
            c.w32(8, t550);
            c.cmp(c.r32(1), c.r32(8), 32);
            if c.cc_ae() { blk = 534; continue; }
            blk = 387; continue;
        }
        387 => {
            let Some(t551) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c6864); };
            c.w32(8, t551);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t552) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c686e); };
            c.w32(9, t552);
            let Some(t553) = c.ld32(c.r64(12).wrapping_add(c.r64(1).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c6872); };
            c.w32(1, t553);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 535; continue; }
            blk = 388; continue;
        }
        388 => {
            let Some(t554) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c6884); };
            c.w32(3, t554);
            let r555 = c.add(c.r32(3), 0x2u64, 32);
            c.w32(3, r555);
            c.w64(11, c.r64(3));
            let r556 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r556);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r557 = c.r64(11) & 0xfffffffffffffffeu64;
            c.w64(11, r557);
            c.logic(r557, 64);
            let r558 = c.add(c.r32(11), c.r32(3), 32);
            c.w32(11, r558);
            c.w32(12, c.r32(9));
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_b() { blk = 542; continue; }
            blk = 389; continue;
        }
        389 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 536; continue; }
            blk = 390; continue;
        }
        390 => {
            let r559 = c.sub(c.r32(12), c.r32(3), 32);
            c.w32(12, r559);
            let r560 = c.shr(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r560);
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t561) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c68b9); };
            c.w32(12, t561);
            let r562 = c.r64(11) & 0xfffffffffffffffeu64;
            c.w64(11, r562);
            c.logic(r562, 64);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_b() { blk = 542; continue; }
            blk = 391; continue;
        }
        391 => {
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_be() { blk = 537; continue; }
            blk = 392; continue;
        }
        392 => {
            let Some(t563) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c68d4); };
            c.w32(11, t563);
            let r564 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r564);
            c.w32(12, c.r64(11).wrapping_add(0x2));
            c.cmp(c.r32(12), 0xfffffeu64, 32);
            if c.cc_a() { blk = 542; continue; }
            blk = 393; continue;
        }
        393 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(12)) { return RegfileExit::fault(0x70008c68ef); }
            let Some(t565) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c68f3); };
            c.w32(12, t565);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 538; continue; }
            blk = 394; continue;
        }
        394 => {
            c.w64(12, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c690a); }
            let Some(t566) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c690e); };
            c.w32(12, t566);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r567 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r567);
            let Some(t568) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c691c); };
            c.w32(3, t568);
            let r569 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r569);
            let r570 = c.r64(3) & c.r64(12);
            c.w64(3, r570);
            c.logic(r570, 64);
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 396; continue;
        }
        395 => {
            let r571 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r571);
            c.w64(15, c.r64(8).wrapping_add(c.r64(3)));
            c.w64(3, c.r64(12));
            let r572 = c.r64(3) & c.r64(15);
            c.w64(3, r572);
            c.logic(r572, 64);
            blk = 396; continue;
        }
        396 => {
            c.w64(9, c.r64(3).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(15, c.r64(9).wrapping_mul(4));
            c.w64(2, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let Some(t573) = c.ld32(c.r64(0).wrapping_add(c.r64(15)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c694d); };
            c.w32(6, t573);
            let Some(t574) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c6952); };
            c.cmp(t574, c.r32(6), 32);
            if c.cc_e() { blk = 398; continue; }
            blk = 397; continue;
        }
        397 => {
            let Some(t575) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008c695b); };
            c.cmp(t575, c.r32(6), 32);
            if c.cc_ne() { blk = 395; continue; }
            blk = 398; continue;
        }
        398 => {
            let Some(t576) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c6964); };
            c.w32(12, t576);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 539; continue; }
            blk = 399; continue;
        }
        399 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(2)), c.r32(7)) { return RegfileExit::fault(0x70008c6977); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 401; continue; }
            blk = 400; continue;
        }
        400 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r577 = c.r64(12) & c.r64(0);
            c.w64(12, r577);
            c.logic(r577, 64);
            let Some(t578) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c698a); };
            c.logic(t578 & 0x4u64, 8);
            if c.cc_ne() { blk = 549; continue; }
            blk = 401; continue;
        }
        401 => {
            let Some(t579) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c6996); };
            c.w32(12, t579);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(3, c.r64(9).wrapping_add(0x1));
            c.cmp(c.r64(3), c.r64(12), 64);
            if c.cc_ae() { blk = 540; continue; }
            blk = 402; continue;
        }
        402 => {
            if let Some(sr) = c.shl(c.r64(11), 0x8u64, 64) { c.w64(11, sr); }
            c.w32(12, c.r32(11));
            c.w64(3, c.r64(9).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(0).wrapping_add(c.r64(3)), c.r32(1)) { return RegfileExit::fault(0x70008c69bc); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 404; continue; }
            blk = 403; continue;
        }
        403 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r580 = c.r64(2) & c.r64(0);
            c.w64(2, r580);
            c.logic(r580, 64);
            let Some(t581) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c69ce); };
            c.logic(t581 & 0x4u64, 8);
            if c.cc_ne() { blk = 546; continue; }
            blk = 404; continue;
        }
        404 => {
            let Some(t582) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c69d8); };
            c.w32(3, t582);
            let Some(t583) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008c69db); };
            c.cmp(t583, c.r32(3), 32);
            if c.cc_ne() { blk = 407; continue; }
            blk = 405; continue;
        }
        405 => {
            let Some(t584) = c.ld8(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c69e4); };
            c.logic(t584 & 0x1u64, 8);
            if c.cc_e() { blk = 407; continue; }
            blk = 406; continue;
        }
        406 => {
            let r585 = c.r32(12) | 0x20u64;
            c.w32(12, r585);
            c.logic(r585, 32);
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 408; continue;
        }
        407 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 408; continue;
        }
        408 => {
            let Some(t586) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c69f6); };
            c.w32(11, t586);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(9).wrapping_add(0x2));
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_ae() { blk = 541; continue; }
            blk = 409; continue;
        }
        409 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(15)).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008c6a0d); }
            let Some(t587) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c6a12); };
            c.w64(7, t587);
            let r588 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r588);
            let Some(t589) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c6a1a); };
            c.w64(11, t589);
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_g() { blk = 383; continue; }
            blk = 410; continue;
        }
        410 => {
            let Some(t590) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6a27); };
            c.w64(7, t590);
            blk = 411; continue;
        }
        411 => {
            let Some(t591) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c6a2b); };
            c.cmp(t591, 0x0u64, 32);
            if c.cc_ne() { blk = 413; continue; }
            blk = 412; continue;
        }
        412 => {
            let Some(t592) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c6a31); };
            c.w64(7, t592);
            blk = 416; continue;
        }
        413 => {
            let Some(t593) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c6a37); };
            c.w64(0, t593);
            let Some(t594) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008c6a3b); };
            c.w32(11, t594);
            let r595 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r595);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c6a42); }
            let Some(t596) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c6a43); };
            if !c.push64(t596) { return RegfileExit::fault(0x70008c6a43); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008c6a46); }
            let Some(t597) = c.ld64(c.r64(13).wrapping_add(0x2640)) else { return RegfileExit::fault(0x70008c6a48); };
            c.w64(3, t597);
            let Some(t598) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c6a4f); };
            c.w64(6, t598);
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008c6a60, 0x70008c6a5b);
        }
        414 => {
            let Some(t599) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c6a60); };
            c.w64(7, t599);
            if !c.st32(c.r64(7).wrapping_add(0x27), c.r32(0)) { return RegfileExit::fault(0x70008c6a64); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 416; continue; }
            blk = 415; continue;
        }
        415 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r600 = c.r64(11) & c.r64(7);
            c.w64(11, r600);
            c.logic(r600, 64);
            let Some(t601) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c6a75); };
            c.logic(t601 & 0x4u64, 8);
            if c.cc_ne() { blk = 543; continue; }
            blk = 416; continue;
        }
        416 => {
            c.w64(0, c.r64(7));
            blk = 212; continue;
        }
        417 => {
            c.w64(7, c.r64(0));
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70008c6a8e); }
            blk = 418; continue;
        }
        418 => {
            let Some(t602) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c6a92); };
            c.w64(0, t602);
            blk = 212; continue;
        }
        419 => {
            blk = 429; continue;
        }
        420 => {
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 429; continue; }
            blk = 421; continue;
        }
        421 => {
            let Some(t603) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c6aa6); };
            c.w64(3, t603);
            blk = 56; continue;
        }
        422 => {
            let Some(t604) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c6aaf); };
            c.w32(15, t604);
            let r605 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r605);
            c.w64(2, c.r64(3).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(11, c.r64(3).wrapping_add(c.r64(11).wrapping_mul(2)));
            blk = 58; continue;
        }
        423 => {
            c.cmp(c.r64(7), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 473; continue; }
            blk = 424; continue;
        }
        424 => {
            c.cmp(c.r64(7), 0xfffffffffffffffdu64, 64);
            if c.cc_e() { blk = 426; continue; }
            blk = 425; continue;
        }
        425 => {
            let Some(t606) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6ad3); };
            c.w64(0, t606);
            let Some(t607) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c6ad7); };
            c.w64(6, t607);
            blk = 429; continue;
        }
        426 => {
            let Some(t608) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6add); };
            if !c.push64(t608) { return RegfileExit::fault(0x70008c6add); }
            let Some(t609) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c6ae0); };
            if !c.push64(t609) { return RegfileExit::fault(0x70008c6ae0); }
            let Some(t610) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c6ae3); };
            if !c.push64(t610) { return RegfileExit::fault(0x70008c6ae3); }
            let Some(t611) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c6ae6); };
            if !c.push64(t611) { return RegfileExit::fault(0x70008c6ae6); }
            let Some(t612) = c.ld64(c.r64(13).wrapping_add(0x2658)) else { return RegfileExit::fault(0x70008c6ae9); };
            c.w64(3, t612);
            c.w32(0, 0x4u64);
            let Some(t613) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c6af5); };
            c.w64(6, t613);
            return c.call(0x700086ed74, 0x70008c6afe, 0x70008c6af9);
        }
        427 => {
            c.w64(7, c.r64(0));
            let Some(t614) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c6b01); };
            c.w64(0, t614);
            blk = 77; continue;
        }
        428 => {
            let Some(t615) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6b0a); };
            c.w64(0, t615);
            let Some(t616) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c6b0e); };
            c.w64(6, t616);
            blk = 429; continue;
        }
        429 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c6b12); }
            let Some(t617) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c6b13); };
            if !c.push64(t617) { return RegfileExit::fault(0x70008c6b13); }
            let Some(t618) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c6b16); };
            if !c.push64(t618) { return RegfileExit::fault(0x70008c6b16); }
            let Some(t619) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c6b19); };
            if !c.push64(t619) { return RegfileExit::fault(0x70008c6b19); }
            let Some(t620) = c.ld64(c.r64(13).wrapping_add(0x2648)) else { return RegfileExit::fault(0x70008c6b1c); };
            c.w64(3, t620);
            c.w32(0, 0x4u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008c6b30, 0x70008c6b2b);
        }
        430 => {
            c.w64(7, c.r64(0));
            let Some(t621) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c6b33); };
            c.w64(0, t621);
            blk = 77; continue;
        }
        431 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008c6b3c); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008c6b40); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008c6b44); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008c6b48); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febb4, 0x70008c6b54, 0x70008c6b4f);
        }
        432 => {
            c.w64(15, c.r64(0));
            let Some(t622) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c6b57); };
            c.w64(1, t622);
            let Some(t623) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c6b5b); };
            c.w64(8, t623);
            let Some(t624) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c6b5f); };
            c.w64(0, t624);
            let Some(t625) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c6b63); };
            c.w64(7, t625);
            c.w32(3, 0x8u64);
            let Some(t626) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c6b6c); };
            c.w64(12, t626);
            let Some(t627) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c6b70); };
            c.w64(11, t627);
            blk = 97; continue;
        }
        433 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x18), 0) { return RegfileExit::fault(0x70008c6b7e); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008c6b83); }
            return c.call(0x70007febf4, 0x70008c6b8c, 0x70008c6b87);
        }
        434 => {
            c.w64(11, c.r64(0));
            let Some(t628) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6b8f); };
            c.w64(0, t628);
            let Some(t629) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c6b93); };
            c.w64(8, t629);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x18)) { return RegfileExit::fault(0x70008c6b97); }
            let Some(t630) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c6b9c); };
            c.w64(7, t630);
            let Some(t631) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c6ba0); };
            c.w64(1, t631);
            let Some(t632) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c6ba4); };
            c.w64(9, t632);
            blk = 116; continue;
        }
        435 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008c6bad); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008c6bb1); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008c6bb5); }
            blk = 440; continue;
        }
        436 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008c6bbb); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008c6bbf); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008c6bc3); }
            blk = 440; continue;
        }
        437 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008c6bc9); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008c6bcd); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008c6bd1); }
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 439; continue; }
            blk = 438; continue;
        }
        438 => {
            let Some(t633) = c.ld64(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c6bda); };
            c.w64(7, t633);
            blk = 141; continue;
        }
        439 => {
            let Some(t634) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c6be3); };
            c.w64(8, t634);
            blk = 440; continue;
        }
        440 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008c6bed); }
            c.cmp(c.r64(7), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 442; continue; }
            blk = 441; continue;
        }
        441 => {
            let r635 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r635);
            blk = 451; continue;
        }
        442 => {
            let Some(t636) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c6c01); };
            c.w64(2, t636);
            let Some(t637) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008c6c08); };
            c.w64(1, t637);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            let Some(t638) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c6c0f); };
            c.w64(9, t638);
            let Some(t639) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c6c16); };
            c.cmp(t639, c.r64(6), 64);
            if c.cc_a() { blk = 445; continue; }
            blk = 443; continue;
        }
        443 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008c6c25, 0x70008c6c20);
        }
        444 => {
            c.w64(2, c.r64(0));
            let Some(t640) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6c28); };
            c.w64(0, t640);
            let Some(t641) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c6c2c); };
            c.w64(3, t641);
            let Some(t642) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c6c30); };
            c.w64(8, t642);
            let Some(t643) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c6c34); };
            c.w64(11, t643);
            let Some(t644) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c6c38); };
            c.w64(12, t644);
            let Some(t645) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c6c3c); };
            c.w64(15, t645);
            let Some(t646) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c6c40); };
            c.w64(7, t646);
            blk = 446; continue;
        }
        445 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x70008c6c46); }
            c.w64(2, c.r64(1).wrapping_add(0x1));
            blk = 446; continue;
        }
        446 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(7) as i64));
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 450; continue; }
            blk = 447; continue;
        }
        447 => {
            c.w64(10, c.r64(7));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 449; continue; }
            blk = 448; continue;
        }
        448 => {
            let r647 = c.r64(10) | 0x1u64;
            c.w64(10, r647);
            c.logic(r647, 64);
            blk = 449; continue;
        }
        449 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 450; continue;
        }
        450 => {
            let Some(t648) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c6c74); };
            c.w64(7, t648);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008c6c7b); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008c6c7e); }
            c.w64(7, c.r64(2));
            blk = 451; continue;
        }
        451 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008c6c86); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c6c87); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t649) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008c6c8a); };
            c.w64(3, t649);
            c.w32(0, 0x2u64);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x70008c6c9e, 0x70008c6c99);
        }
        452 => {
            let r650 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r650);
            c.w64(7, c.r64(0));
            let Some(t651) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6ca3); };
            c.w64(0, t651);
            let Some(t652) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c6ca7); };
            c.w64(3, t652);
            let Some(t653) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c6cab); };
            c.w64(11, t653);
            let Some(t654) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c6caf); };
            c.w64(12, t654);
            let Some(t655) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c6cb3); };
            c.w64(15, t655);
            blk = 144; continue;
        }
        453 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008c6cbc); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008c6cc0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008c6cc4); }
            blk = 457; continue;
        }
        454 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008c6cca); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008c6cce); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008c6cd2); }
            blk = 457; continue;
        }
        455 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008c6cd8); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008c6cdc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008c6ce0); }
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_ne() { blk = 457; continue; }
            blk = 456; continue;
        }
        456 => {
            let Some(t656) = c.ld64(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c6cea); };
            c.w64(1, t656);
            blk = 164; continue;
        }
        457 => {
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 459; continue; }
            blk = 458; continue;
        }
        458 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            blk = 468; continue;
        }
        459 => {
            let Some(t657) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c6d05); };
            c.w64(7, t657);
            let Some(t658) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008c6d0c); };
            c.w64(2, t658);
            c.w64(1, c.r64(2).wrapping_add(0xc));
            let Some(t659) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c6d13); };
            c.w64(6, t659);
            let Some(t660) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008c6d1a); };
            c.cmp(t660, c.r64(1), 64);
            if c.cc_a() { blk = 462; continue; }
            blk = 460; continue;
        }
        460 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008c6d29, 0x70008c6d24);
        }
        461 => {
            c.w64(7, c.r64(0));
            let Some(t661) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6d2c); };
            c.w64(0, t661);
            let Some(t662) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c6d30); };
            c.w64(3, t662);
            let Some(t663) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c6d34); };
            c.w64(11, t663);
            let Some(t664) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c6d38); };
            c.w64(12, t664);
            let Some(t665) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c6d3c); };
            c.w64(15, t665);
            blk = 463; continue;
        }
        462 => {
            if !c.st64(c.r64(7), c.r64(1)) { return RegfileExit::fault(0x70008c6d42); }
            c.w64(7, c.r64(2).wrapping_add(0x1));
            blk = 463; continue;
        }
        463 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(15) as i64));
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 467; continue; }
            blk = 464; continue;
        }
        464 => {
            c.w64(10, c.r64(15));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 466; continue; }
            blk = 465; continue;
        }
        465 => {
            let r666 = c.r64(10) | 0x1u64;
            c.w64(10, r666);
            c.logic(r666, 64);
            blk = 466; continue;
        }
        466 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 467; continue;
        }
        467 => {
            let Some(t667) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c6d70); };
            c.w64(15, t667);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008c6d77); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008c6d7b); }
            blk = 468; continue;
        }
        468 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008c6d80); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c6d81); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t668) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008c6d84); };
            c.w64(3, t668);
            c.w32(0, 0x2u64);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x70008c6d98, 0x70008c6d93);
        }
        469 => {
            let r669 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r669);
            c.w64(7, c.r64(0));
            let Some(t670) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6d9d); };
            c.w64(0, t670);
            let Some(t671) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c6da1); };
            c.w64(3, t671);
            let Some(t672) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c6da5); };
            c.w64(11, t672);
            let Some(t673) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c6da9); };
            c.w64(12, t673);
            blk = 167; continue;
        }
        470 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x18), 0) { return RegfileExit::fault(0x70008c6db7); }
            return c.call(0x70007febf4, 0x70008c6dc1, 0x70008c6dbc);
        }
        471 => {
            c.w64(7, c.r64(0));
            let Some(t674) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6dc4); };
            c.w64(0, t674);
            let Some(t675) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c6dc8); };
            c.w64(3, t675);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x18)) { return RegfileExit::fault(0x70008c6dcc); }
            blk = 175; continue;
        }
        472 => {
            return RegfileExit::fault(0x70008c6dd6);
        }
        473 => {
            let Some(t676) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70008c6dd7); };
            c.w64(3, t676);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t677) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c6de0); };
            c.w64(6, t677);
            return c.call(0x700086ed74, 0x70008c6de9, 0x70008c6de4);
        }
        474 => {
            return RegfileExit::fault(0x70008c6de9);
        }
        475 => {
            blk = 182; continue;
        }
        476 => {
            let Some(t678) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008c6def); };
            c.w64(3, t678);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008c6dff, 0x70008c6dfa);
        }
        477 => {
            return RegfileExit::fault(0x70008c6dff);
        }
        478 => {
            blk = 194; continue;
        }
        479 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008c6e05); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x70008c6e09); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008c6e0d); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febb4, 0x70008c6e19, 0x70008c6e14);
        }
        480 => {
            c.w64(15, c.r64(0));
            let Some(t679) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c6e1c); };
            c.w64(7, t679);
            let Some(t680) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c6e20); };
            c.w64(1, t680);
            let Some(t681) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c6e24); };
            c.w64(8, t681);
            c.w32(0, 0x8u64);
            let Some(t682) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c6e2d); };
            c.w64(12, t682);
            let Some(t683) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c6e31); };
            c.w64(11, t683);
            blk = 203; continue;
        }
        481 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008c6e3f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008c6e43); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008c6e47); }
            return c.call(0x70007febf4, 0x70008c6e50, 0x70008c6e4b);
        }
        482 => {
            c.w64(12, c.r64(0));
            let Some(t684) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6e53); };
            c.w64(15, t684);
            let Some(t685) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c6e57); };
            c.w64(7, t685);
            let Some(t686) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c6e5b); };
            c.w64(11, t686);
            blk = 211; continue;
        }
        483 => {
            let Some(t687) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008c6e64); };
            c.w64(3, t687);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008c6e72, 0x70008c6e6d);
        }
        484 => {
            return RegfileExit::fault(0x70008c6e72);
        }
        485 => {
            return RegfileExit::fault(0x70008c6e73);
        }
        486 => {
            blk = 498; continue;
        }
        487 => {
            let Some(t688) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c6e79); };
            c.w64(11, t688);
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 498; continue; }
            blk = 488; continue;
        }
        488 => {
            let Some(t689) = c.ld64(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c6e87); };
            c.w64(7, t689);
            blk = 264; continue;
        }
        489 => {
            c.w64(11, c.r64(7).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(7, c.r64(7).wrapping_add(c.r64(2).wrapping_mul(2)));
            let Some(t690) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c6e98); };
            c.w32(2, t690);
            let r691 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r691);
            blk = 266; continue;
        }
        490 => {
            c.cmp(c.r64(7), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 495; continue; }
            blk = 491; continue;
        }
        491 => {
            c.cmp(c.r64(7), 0xfffffffffffffffdu64, 64);
            if c.cc_e() { blk = 493; continue; }
            blk = 492; continue;
        }
        492 => {
            let Some(t692) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6eb0); };
            c.w64(0, t692);
            let Some(t693) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c6eb4); };
            c.w64(6, t693);
            let Some(t694) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c6eb8); };
            c.w64(3, t694);
            let Some(t695) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c6ebc); };
            c.w64(15, t695);
            blk = 498; continue;
        }
        493 => {
            let Some(t696) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6ec2); };
            if !c.push64(t696) { return RegfileExit::fault(0x70008c6ec2); }
            let Some(t697) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c6ec5); };
            if !c.push64(t697) { return RegfileExit::fault(0x70008c6ec5); }
            let Some(t698) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c6ec8); };
            if !c.push64(t698) { return RegfileExit::fault(0x70008c6ec8); }
            let Some(t699) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c6ecb); };
            if !c.push64(t699) { return RegfileExit::fault(0x70008c6ecb); }
            let Some(t700) = c.ld64(c.r64(13).wrapping_add(0x2658)) else { return RegfileExit::fault(0x70008c6ece); };
            c.w64(3, t700);
            c.w32(0, 0x4u64);
            let Some(t701) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c6eda); };
            c.w64(6, t701);
            return c.call(0x700086ed74, 0x70008c6ee3, 0x70008c6ede);
        }
        494 => {
            c.w64(15, c.r64(0));
            let Some(t702) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c6ee6); };
            c.w64(0, t702);
            blk = 285; continue;
        }
        495 => {
            let Some(t703) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70008c6eef); };
            c.w64(3, t703);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t704) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c6ef8); };
            c.w64(6, t704);
            return c.call(0x700086ed74, 0x70008c6f01, 0x70008c6efc);
        }
        496 => {
            return RegfileExit::fault(0x70008c6f01);
        }
        497 => {
            let Some(t705) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6f02); };
            c.w64(0, t705);
            let Some(t706) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c6f06); };
            c.w64(6, t706);
            let Some(t707) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c6f0a); };
            c.w64(3, t707);
            blk = 498; continue;
        }
        498 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c6f0e); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008c6f0f); }
            let Some(t708) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c6f10); };
            if !c.push64(t708) { return RegfileExit::fault(0x70008c6f10); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008c6f13); }
            c.w32(0, 0x4u64);
            let Some(t709) = c.ld64(c.r64(13).wrapping_add(0x2648)) else { return RegfileExit::fault(0x70008c6f1a); };
            c.w64(3, t709);
            c.w64(7, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008c6f2c, 0x70008c6f27);
        }
        499 => {
            c.w64(15, c.r64(0));
            let Some(t710) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c6f2f); };
            c.w64(0, t710);
            blk = 285; continue;
        }
        500 => {
            return RegfileExit::fault(0x70008c6f38);
        }
        501 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x70008c6f39); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008c6f3d); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febb4, 0x70008c6f49, 0x70008c6f44);
        }
        502 => {
            c.w64(7, c.r64(0));
            let Some(t711) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c6f4c); };
            c.w64(2, t711);
            let Some(t712) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c6f50); };
            c.w64(8, t712);
            blk = 303; continue;
        }
        503 => {
            c.w32(2, 0x28u64);
            return c.call(0x70007febf4, 0x70008c6f63, 0x70008c6f5e);
        }
        504 => {
            c.w64(7, c.r64(0));
            blk = 306; continue;
        }
        505 => {
            let Some(t713) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008c6f6b); };
            c.w64(3, t713);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008c6f7b, 0x70008c6f76);
        }
        506 => {
            return RegfileExit::fault(0x70008c6f7b);
        }
        507 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x70008c6f7c); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x70008c6f88, 0x70008c6f83);
        }
        508 => {
            c.w64(15, c.r64(0));
            let Some(t714) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6f8b); };
            c.w64(7, t714);
            let Some(t715) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c6f8f); };
            c.w64(6, t715);
            let Some(t716) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c6f93); };
            c.w64(11, t716);
            let Some(t717) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c6f97); };
            c.w64(0, t717);
            let Some(t718) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c6f9b); };
            c.w64(12, t718);
            let Some(t719) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c6f9f); };
            c.w64(3, t719);
            blk = 309; continue;
        }
        509 => {
            c.w32(2, 0x28u64);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x70008c6fad); }
            return c.call(0x70007febf4, 0x70008c6fb6, 0x70008c6fb1);
        }
        510 => {
            c.w64(3, c.r64(0));
            let Some(t720) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c6fb9); };
            c.w64(7, t720);
            let Some(t721) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c6fbd); };
            c.w64(6, t721);
            let Some(t722) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c6fc1); };
            c.w64(11, t722);
            let Some(t723) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c6fc5); };
            c.w64(0, t723);
            let Some(t724) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c6fc9); };
            c.w64(12, t724);
            let Some(t725) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c6fcd); };
            c.w64(15, t725);
            blk = 312; continue;
        }
        511 => {
            return RegfileExit::fault(0x70008c6fd6);
        }
        512 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x70008c6fd7); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x70008c6fdb); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x70008c6fe7, 0x70008c6fe2);
        }
        513 => {
            c.w64(7, c.r64(0));
            let Some(t726) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c6fea); };
            c.w64(8, t726);
            let Some(t727) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c6fee); };
            c.w64(2, t727);
            blk = 322; continue;
        }
        514 => {
            c.w32(2, 0x2cu64);
            return c.call(0x70007febf4, 0x70008c7001, 0x70008c6ffc);
        }
        515 => {
            c.w64(7, c.r64(0));
            blk = 325; continue;
        }
        516 => {
            let Some(t728) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008c7009); };
            c.w64(3, t728);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008c7019, 0x70008c7014);
        }
        517 => {
            return RegfileExit::fault(0x70008c7019);
        }
        518 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x70008c701a); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x70008c7026, 0x70008c7021);
        }
        519 => {
            c.w64(15, c.r64(0));
            let Some(t729) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c7029); };
            c.w64(7, t729);
            let Some(t730) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c702d); };
            c.w64(6, t730);
            let Some(t731) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c7031); };
            c.w64(11, t731);
            let Some(t732) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c7035); };
            c.w64(0, t732);
            let Some(t733) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c7039); };
            c.w64(12, t733);
            let Some(t734) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c703d); };
            c.w64(2, t734);
            blk = 328; continue;
        }
        520 => {
            c.w32(2, 0x2cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x70008c704b); }
            return c.call(0x70007febf4, 0x70008c7054, 0x70008c704f);
        }
        521 => {
            c.w64(3, c.r64(0));
            let Some(t735) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c7057); };
            c.w64(7, t735);
            let Some(t736) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c705b); };
            c.w64(6, t736);
            let Some(t737) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c705f); };
            c.w64(11, t737);
            let Some(t738) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c7063); };
            c.w64(0, t738);
            let Some(t739) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c7067); };
            c.w64(12, t739);
            let Some(t740) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c706b); };
            c.w64(15, t740);
            blk = 331; continue;
        }
        522 => {
            return RegfileExit::fault(0x70008c7074);
        }
        523 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008c7075); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008c7079); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x70008c707d); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008c7081); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x70008c708d, 0x70008c7088);
        }
        524 => {
            let Some(t741) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c708d); };
            c.w64(8, t741);
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t742) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c7094); };
            c.w64(3, t742);
            let Some(t743) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c7098); };
            c.w64(12, t743);
            let Some(t744) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c709c); };
            c.w64(11, t744);
            let Some(t745) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c70a0); };
            c.w64(2, t745);
            let Some(t746) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c70a4); };
            c.w64(7, t746);
            let Some(t747) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c70a8); };
            c.w64(1, t747);
            blk = 366; continue;
        }
        525 => {
            return RegfileExit::fault(0x70008c70b1);
        }
        526 => {
            return RegfileExit::fault(0x70008c70b2);
        }
        527 => {
            return RegfileExit::fault(0x70008c70b3);
        }
        528 => {
            return RegfileExit::fault(0x70008c70b4);
        }
        529 => {
            return RegfileExit::fault(0x70008c70b5);
        }
        530 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008c70b6); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008c70ba); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008c70be); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febf4, 0x70008c70ca, 0x70008c70c5);
        }
        531 => {
            c.w64(11, c.r64(0));
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t748) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c70d0); };
            c.w64(3, t748);
            let Some(t749) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c70d4); };
            c.w64(0, t749);
            let Some(t750) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c70d8); };
            c.w64(12, t750);
            let Some(t751) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c70dc); };
            c.w64(6, t751);
            let Some(t752) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c70e3); };
            c.w64(7, t752);
            let Some(t753) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c70e7); };
            c.w64(2, t753);
            blk = 378; continue;
        }
        532 => {
            return RegfileExit::fault(0x70008c70f0);
        }
        533 => {
            return RegfileExit::fault(0x70008c70f1);
        }
        534 => {
            return RegfileExit::fault(0x70008c70f2);
        }
        535 => {
            return RegfileExit::fault(0x70008c70f3);
        }
        536 => {
            return RegfileExit::fault(0x70008c70f4);
        }
        537 => {
            return RegfileExit::fault(0x70008c70f5);
        }
        538 => {
            return RegfileExit::fault(0x70008c70f6);
        }
        539 => {
            return RegfileExit::fault(0x70008c70f7);
        }
        540 => {
            return RegfileExit::fault(0x70008c70f8);
        }
        541 => {
            return RegfileExit::fault(0x70008c70f9);
        }
        542 => {
            return RegfileExit::fault(0x70008c70fa);
        }
        543 => {
            c.w32(0, c.r32(0));
            let r754 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r754);
            c.w64(11, 0xfffffffffffc0000u64);
            let r755 = c.r64(11) & c.r64(0);
            c.w64(11, r755);
            c.logic(r755, 64);
            let Some(t756) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c710a); };
            c.logic(t756 & 0x2u64, 8);
            if c.cc_e() { blk = 416; continue; }
            blk = 544; continue;
        }
        544 => {
            c.w64(12, c.r64(7).wrapping_add(0x27));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c7119); }
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008c7123, 0x70008c711e);
        }
        545 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c7123); }
            blk = 416; continue;
        }
        546 => {
            c.w32(1, c.r32(1));
            let r757 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r757);
            c.w64(2, 0xfffffffffffc0000u64);
            let r758 = c.r64(2) & c.r64(1);
            c.w64(2, r758);
            c.logic(r758, 64);
            let Some(t759) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c7139); };
            c.logic(t759 & 0x2u64, 8);
            if c.cc_e() { blk = 404; continue; }
            blk = 547; continue;
        }
        547 => {
            c.w64(6, c.r64(0).wrapping_add(c.r64(3)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c7147); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c7148); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x70008c7155, 0x70008c7150);
        }
        548 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c7155); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c7157); }
            blk = 404; continue;
        }
        549 => {
            c.w32(7, c.r32(7));
            let r760 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r760);
            c.w64(12, 0xfffffffffffc0000u64);
            let r761 = c.r64(12) & c.r64(7);
            c.w64(12, r761);
            c.logic(r761, 64);
            let Some(t762) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c716c); };
            c.logic(t762 & 0x2u64, 8);
            if c.cc_e() { blk = 401; continue; }
            blk = 550; continue;
        }
        550 => {
            c.w64(3, c.r64(0).wrapping_add(c.r64(2)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c717c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c717d); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008c718a, 0x70008c7185);
        }
        551 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c718a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c718c); }
            blk = 401; continue;
        }
        552 => {
            c.w32(11, c.r32(11));
            let r763 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r763);
            c.w64(7, 0xfffffffffffc0000u64);
            let r764 = c.r64(7) & c.r64(11);
            c.w64(7, r764);
            c.logic(r764, 64);
            let Some(t765) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c71a2); };
            c.logic(t765 & 0x2u64, 8);
            if c.cc_e() { blk = 382; continue; }
            blk = 553; continue;
        }
        553 => {
            c.w64(2, c.r64(3).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c71b0); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c71b1); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70008c71be, 0x70008c71b9);
        }
        554 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c71be); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c71c0); }
            blk = 382; continue;
        }
        555 => {
            c.w32(11, c.r32(11));
            let r766 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r766);
            c.w64(2, 0xfffffffffffc0000u64);
            let r767 = c.r64(2) & c.r64(11);
            c.w64(2, r767);
            c.logic(r767, 64);
            let Some(t768) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c71d6); };
            c.logic(t768 & 0x2u64, 8);
            if c.cc_e() { blk = 359; continue; }
            blk = 556; continue;
        }
        556 => {
            c.w64(1, c.r64(3).wrapping_add(0x1b));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c71e4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c71e5); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x70008c71f2, 0x70008c71ed);
        }
        557 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c71f2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c71f4); }
            blk = 359; continue;
        }
        558 => {
            c.w32(0, c.r32(0));
            let r769 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r769);
            c.w64(15, 0xfffffffffffc0000u64);
            let r770 = c.r64(15) & c.r64(0);
            c.w64(15, r770);
            c.logic(r770, 64);
            let Some(t771) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c7209); };
            c.logic(t771 & 0x2u64, 8);
            if c.cc_e() { blk = 352; continue; }
            blk = 559; continue;
        }
        559 => {
            c.w64(3, c.r64(12).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c7218); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c7219); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008c7226, 0x70008c7221);
        }
        560 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c7226); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c7228); }
            blk = 352; continue;
        }
        561 => {
            c.w32(0, c.r32(0));
            let r772 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r772);
            c.w64(8, 0xfffffffffffc0000u64);
            let r773 = c.r64(8) & c.r64(0);
            c.w64(8, r773);
            c.logic(r773, 64);
            let Some(t774) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c723d); };
            c.logic(t774 & 0x2u64, 8);
            if c.cc_e() { blk = 345; continue; }
            blk = 562; continue;
        }
        562 => {
            c.w64(9, c.r64(12).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c724d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c724e); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008c725b, 0x70008c7256);
        }
        563 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c725b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c725d); }
            blk = 345; continue;
        }
        564 => {
            c.w32(2, c.r32(2));
            let r775 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r775);
            c.w64(8, 0xfffffffffffc0000u64);
            let r776 = c.r64(8) & c.r64(2);
            c.w64(8, r776);
            c.logic(r776, 64);
            let Some(t777) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c7272); };
            c.logic(t777 & 0x2u64, 8);
            if c.cc_e() { blk = 339; continue; }
            blk = 565; continue;
        }
        565 => {
            c.w64(9, c.r64(3).wrapping_add(0x1f));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c7281); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c7282); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008c728f, 0x70008c728a);
        }
        566 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c728f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c7291); }
            blk = 339; continue;
        }
        567 => {
            c.w32(2, c.r32(2));
            let r778 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r778);
            c.w64(1, 0xfffffffffffc0000u64);
            let r779 = c.r64(1) & c.r64(2);
            c.w64(1, r779);
            c.logic(r779, 64);
            let Some(t780) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c72a6); };
            c.logic(t780 & 0x2u64, 8);
            if c.cc_e() { blk = 337; continue; }
            blk = 568; continue;
        }
        568 => {
            c.w64(8, c.r64(3).wrapping_add(0x13));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c72b4); }
            c.w64(7, c.r64(3));
            return c.call(0x70007f0e54, 0x70008c72bd, 0x70008c72b8);
        }
        569 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c72bd); }
            blk = 337; continue;
        }
        570 => {
            c.w32(0, c.r32(0));
            let r781 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r781);
            c.w64(7, 0xfffffffffffc0000u64);
            let r782 = c.r64(7) & c.r64(0);
            c.w64(7, r782);
            c.logic(r782, 64);
            let Some(t783) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c72d2); };
            c.logic(t783 & 0x2u64, 8);
            if c.cc_e() { blk = 279; continue; }
            blk = 571; continue;
        }
        571 => {
            c.w64(12, c.r64(15).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c72e0); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c72e1); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008c72ee, 0x70008c72e9);
        }
        572 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c72ee); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c72f0); }
            blk = 279; continue;
        }
        573 => {
            c.w32(0, c.r32(0));
            let r784 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r784);
            c.w64(7, 0xfffffffffffc0000u64);
            let r785 = c.r64(7) & c.r64(0);
            c.w64(7, r785);
            c.logic(r785, 64);
            let Some(t786) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c7305); };
            c.logic(t786 & 0x2u64, 8);
            if c.cc_e() { blk = 277; continue; }
            blk = 574; continue;
        }
        574 => {
            c.w64(12, c.r64(15).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c7313); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c7314); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008c7321, 0x70008c731c);
        }
        575 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c7321); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c7323); }
            blk = 277; continue;
        }
        576 => {
            c.w32(0, c.r32(0));
            let r787 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r787);
            c.w64(11, 0xfffffffffffc0000u64);
            let r788 = c.r64(11) & c.r64(0);
            c.w64(11, r788);
            c.logic(r788, 64);
            let Some(t789) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c7338); };
            c.logic(t789 & 0x2u64, 8);
            if c.cc_e() { blk = 108; continue; }
            blk = 577; continue;
        }
        577 => {
            c.w64(12, c.r64(1).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c7347); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c7348); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008c7355, 0x70008c7350);
        }
        578 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c7355); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c7357); }
            blk = 108; continue;
        }
        579 => {
            c.w32(0, c.r32(0));
            let r790 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r790);
            c.w64(12, 0xfffffffffffc0000u64);
            let r791 = c.r64(12) & c.r64(0);
            c.w64(12, r791);
            c.logic(r791, 64);
            let Some(t792) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c736c); };
            c.logic(t792 & 0x2u64, 8);
            if c.cc_e() { blk = 71; continue; }
            blk = 580; continue;
        }
        580 => {
            c.w64(15, c.r64(7).wrapping_add(0xf));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c737c); }
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008c7386, 0x70008c7381);
        }
        581 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c7386); }
            blk = 71; continue;
        }
        582 => {
            c.w32(0, c.r32(0));
            let r793 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r793);
            c.w64(12, 0xfffffffffffc0000u64);
            let r794 = c.r64(12) & c.r64(0);
            c.w64(12, r794);
            c.logic(r794, 64);
            let Some(t795) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c739c); };
            c.logic(t795 & 0x2u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 583; continue;
        }
        583 => {
            c.w64(15, c.r64(7).wrapping_add(0xb));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c73ac); }
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008c73b6, 0x70008c73b1);
        }
        584 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c73b6); }
            blk = 69; continue;
        }
        585 => {
            return RegfileExit::jump(0x70008c73c0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008c73d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008c73d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008c73d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c73d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c73da); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008c73df); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008c73e3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 202; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c73ed); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 270; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c73f9); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c7400); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 270; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008c740d); };
            c.w64(8, t6);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 204; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c741b); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c7422); };
            c.cmp(t9, 0x40u64, 16);
            if c.cc_ae() { blk = 204; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c742e); };
            c.w32(9, t10);
            let r11 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r11);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008c7435); }
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008c743b); };
            c.w64(11, t12);
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c7442); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c744c); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c7453); };
            c.w32(11, t16);
            let r17 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r17);
            let Some(t18) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c745a); };
            c.w32(12, t18);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x9au64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x70008c746d);
        }
        8 => {
            let Some(t19) = c.ld32(c.r64(11).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008c746e); };
            c.w32(15, t19);
            let r20 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r20);
            let Some(t21) = c.ld32(c.r64(15).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008c7478); };
            c.cmp(t21, c.r32(9), 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c747e); };
            c.w32(15, t22);
            c.logic(c.r32(15) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(12), 0x9fu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008c7494);
        }
        12 => {
            let Some(t23) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c7495); };
            c.w32(9, t23);
            let r24 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r24);
            let Some(t25) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c749c); };
            c.w32(9, t25);
            let r26 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r26);
            let Some(t27) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c74a3); };
            c.w32(12, t27);
            let r28 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r28);
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0x283)) else { return RegfileExit::fault(0x70008c74aa); };
            c.cmp(t29, c.r32(9), 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t30) = c.ld32(c.r64(12).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008c74b3); };
            c.w32(9, t30);
            c.w32(9, c.r32(9));
            let r31 = c.r32(9) & 0x8u64;
            c.w32(9, r31);
            c.logic(r31, 32);
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(3, c.r64(8));
            return c.call(0x70008c5034, 0x70008c74cd, 0x70008c74c8);
        }
        15 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c74cd); };
            c.w64(1, t32);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008c74d4); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return c.ret(0x10, 0x70008c74db);
        }
        17 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008c74de); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008c74e5); }
            return c.ret(0x0, 0x70008c74e7);
        }
        18 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x768)) else { return RegfileExit::fault(0x70008c74e8); };
            c.w64(3, t33);
            return c.call(0x7000870914, 0x70008c74f4, 0x70008c74ef);
        }
        19 => {
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008c74f4); };
            c.cmp(t34, c.r32(0), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c7504); };
            c.cmp(t35, c.r32(0), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t36) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c750d); };
            c.w32(8, t36);
            let r37 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r37);
            let Some(t38) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008c7514); };
            c.logic(t38 & 0x10u64, 8);
            if c.cc_ne() { blk = 28; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c751b); };
            c.cmp(t39, c.r32(8), 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t40) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c7528); };
            c.cmp(t40, 0x41u64, 16);
            if c.cc_e() { blk = 206; continue; }
            blk = 25; continue;
        }
        25 => {
            blk = 27; continue;
        }
        26 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(0, 0x1u64);
            blk = 29; continue;
        }
        28 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 189; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0xb40)) else { return RegfileExit::fault(0x70008c754e); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c7555); };
            c.w64(0, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c7559); };
            c.w64(6, t43);
            return c.call(0x7000870914, 0x70008c7562, 0x70008c755d);
        }
        31 => {
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008c7562); };
            c.cmp(t44, c.r32(0), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 38; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c7572); };
            c.cmp(t45, c.r32(0), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t46) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c757b); };
            c.w32(8, t46);
            let r47 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r47);
            let Some(t48) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008c7582); };
            c.logic(t48 & 0x10u64, 8);
            if c.cc_ne() { blk = 40; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t49) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c7589); };
            c.cmp(t49, c.r32(8), 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t50) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c7596); };
            c.cmp(t50, 0x41u64, 16);
            if c.cc_e() { blk = 210; continue; }
            blk = 37; continue;
        }
        37 => {
            blk = 39; continue;
        }
        38 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(8, 0x1u64);
            blk = 41; continue;
        }
        40 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 41; continue;
        }
        41 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008c75b5); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008c75b9); };
            c.w64(3, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c75c0); };
            c.w64(0, t52);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c75c6); };
            c.w64(6, t53);
            return c.call(0x7000871cb4, 0x70008c75cf, 0x70008c75ca);
        }
        42 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c75cf); };
            c.w64(8, t54);
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(8));
            c.w64(12, c.r64(11));
            blk = 44; continue;
        }
        43 => {
            let r55 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r55);
            c.w64(2, c.r64(8));
            blk = 44; continue;
        }
        44 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c75e8); };
            c.w64(0, t56);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008c75ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008c75f0); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x70008c75f4); }
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x6e8)) else { return RegfileExit::fault(0x70008c75f8); };
            c.w64(3, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c75ff); };
            c.w64(6, t58);
            return c.call(0x7000870914, 0x70008c7608, 0x70008c7603);
        }
        45 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t59) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c760c); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008c7613); };
            c.logic(t61 & 0x2u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c761a); };
            if !c.push64(t62) { return RegfileExit::fault(0x70008c761a); }
            let Some(t63) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c761d); };
            if !c.push64(t63) { return RegfileExit::fault(0x70008c761d); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c7628); };
            c.w64(6, t64);
            return c.call(0x70007f2274, 0x70008c7631, 0x70008c762c);
        }
        48 => {
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c7631); };
            c.cmp(t65, c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 49; continue;
        }
        49 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 256; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t66) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c7642); };
            c.w32(8, t66);
            let r67 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r67);
            let Some(t68) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c7649); };
            c.cmp(t68, 0xbau64, 16);
            if c.cc_ae() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 256; continue;
        }
        52 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c7657); };
            c.w64(0, t69);
            let Some(t70) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c765b); };
            c.w32(8, t70);
            let r71 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r71);
            let Some(t72) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c7662); };
            c.cmp(t72, 0x83eu64, 16);
            if c.cc_ne() { blk = 187; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c766f); };
            c.w64(6, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c7673); };
            c.w64(3, t74);
            return c.call(0x70008bf674, 0x70008c767c, 0x70008c7677);
        }
        54 => {
            let Some(t75) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c767c); };
            c.cmp(t75, c.r32(0), 32);
            if c.cc_e() { blk = 161; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c7689); };
            c.w64(6, t76);
            c.w32(3, 0);
            c.logic(0, 32);
            return c.call(0x7000870914, 0x70008c7694, 0x70008c768f);
        }
        56 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 214; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t77) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c769c); };
            c.w32(8, t77);
            let r78 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r78);
            let Some(t79) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c76a3); };
            c.cmp(t79, 0x40u64, 16);
            if c.cc_ae() { blk = 214; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c76af); };
            c.w64(12, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c76b3); };
            c.cmp(c.r64(12), t81, 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c76b9); };
            c.w64(9, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c76bd); };
            c.w64(11, t83);
            blk = 78; continue;
        }
        60 => {
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c76c6); };
            c.w64(9, t84);
            let Some(t85) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c76ca); };
            c.w32(8, t85);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c76d4); };
            c.w64(11, t86);
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_a() { blk = 155; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_b() { blk = 155; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(8, c.r64(12));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r87 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r87);
            let r88 = c.add(c.r64(8), 0x10u64, 64);
            c.w64(8, r88);
            if c.cc_e() { blk = 76; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 259; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w32(12, c.r64(8).wrapping_add(c.r64(8)));
            c.w64(15, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w32(3, c.r32(3));
            let r89 = c.imul2(c.r32(3), 0x4u64, 32);
            c.w32(3, r89);
            if c.cc_o() { blk = 159; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w32(2, 0x8u64);
            let r90 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r90);
            if c.cc_o() { blk = 158; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c7734); };
            c.w64(1, t91);
            let Some(t92) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008c773b); };
            c.w64(1, t92);
            c.w64(6, c.r64(1).wrapping_add(c.r64(3)));
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c7742); };
            c.w64(7, t93);
            let Some(t94) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008c7749); };
            c.cmp(t94, c.r64(6), 64);
            if c.cc_be() { blk = 216; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_ae() { blk = 216; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c775f); };
            c.w64(3, t95);
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008c7766); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 69; continue;
        }
        69 => {
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c776d); };
            c.w64(1, t96);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008c7774); }
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008c7777); }
            c.w64(12, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.w64(15, c.r64(2));
            blk = 71; continue;
        }
        70 => {
            if !c.st32(c.r64(3).wrapping_add(c.r64(2)).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008c7788); }
            let r97 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r97);
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r64(12), c.r64(2), 64);
            if c.cc_e() { blk = 75; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(1, c.r64(11).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(1), c.r64(15), 64);
            if c.cc_e() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t98) = c.ld32(c.r64(9).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c77a2); };
            c.w32(1, t98);
            let r99 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r99);
            blk = 70; continue;
        }
        74 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008c77ad); };
            c.w64(1, t100);
            blk = 70; continue;
        }
        75 => {
            c.w64(9, c.r64(3));
            blk = 77; continue;
        }
        76 => {
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c77bb); };
            c.w64(9, t101);
            blk = 77; continue;
        }
        77 => {
            c.w64(12, c.r64(8));
            blk = 78; continue;
        }
        78 => {
            let Some(t102) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c77c5); };
            c.w32(8, t102);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008c77cf); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x70008c77d3); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 160; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(8, 0xffffffffffffffffu64);
            c.w64(15, c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x8));
            if !c.st32(c.r64(9).wrapping_add(c.r64(15)), c.r32(0)) { return RegfileExit::fault(0x70008c77ec); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 81; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r103 = c.r64(3) & c.r64(9);
            c.w64(3, r103);
            c.logic(r103, 64);
            let Some(t104) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c77fe); };
            c.logic(t104 & 0x4u64, 8);
            if c.cc_ne() { blk = 272; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t105) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c7808); };
            c.w32(15, t105);
            let r106 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r106);
            c.w32(15, c.r32(15));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c781c); };
            c.w64(0, t107);
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008c7820); };
            c.w64(3, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c7827); };
            c.w64(6, t109);
            return c.call(0x7000870914, 0x70008c7830, 0x70008c782b);
        }
        83 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 86; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c7834); };
            c.w64(6, t110);
            return c.call(0x70008a17f4, 0x70008c783d, 0x70008c7838);
        }
        85 => {
            blk = 88; continue;
        }
        86 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 88; continue; }
            blk = 87; continue;
        }
        87 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 88; continue;
        }
        88 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c784d); }
            blk = 93; continue;
        }
        90 => {
            let r111 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r111);
            if c.cc_o() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 96; continue;
        }
        92 => {
            c.w64(8, c.r64(0));
            let r112 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r112);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 93; continue;
        }
        93 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c786e); };
            c.w64(8, t113);
            let Some(t114) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008c7875); };
            c.w64(9, t114);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c787c); };
            c.w64(12, t115);
            let Some(t116) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008c7883); };
            c.cmp(t116, c.r64(11), 64);
            if c.cc_be() { blk = 218; continue; }
            blk = 94; continue;
        }
        94 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008c788d); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 95; continue;
        }
        95 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c78a6); };
            c.w64(9, t117);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008c78ad); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008c78b1); }
            blk = 96; continue;
        }
        96 => {
            let Some(t118) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c78b7); };
            c.cmp(t118, 0x0u64, 32);
            if c.cc_e() { blk = 153; continue; }
            blk = 97; continue;
        }
        97 => {
            c.w32(9, c.r32(8));
            c.logic(c.r32(8) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 153; continue; }
            blk = 98; continue;
        }
        98 => {
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c78d4); };
            c.w64(3, t119);
            let Some(t120) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c78d8); };
            c.w32(12, t120);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ae() { blk = 153; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t121) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c78e8); };
            c.w32(12, t121);
            let r122 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r122);
            let Some(t123) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c78ef); };
            c.w32(12, t123);
            c.logic(c.r8(12) & 0x7u64, 8);
            if c.cc_e() { blk = 115; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w64(2, c.r64(12));
            c.w32(12, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(3));
            blk = 101; continue;
        }
        101 => {
            c.w64(1, c.r64(2));
            let r124 = c.r32(1) & 0x7u64;
            c.w32(1, r124);
            c.logic(r124, 32);
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_l() { blk = 106; continue; }
            blk = 102; continue;
        }
        102 => {
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_e() { blk = 114; continue; }
            blk = 103; continue;
        }
        103 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 111; continue; }
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(1), 0x5u64, 32);
            if c.cc_e() { blk = 112; continue; }
            blk = 105; continue;
        }
        105 => {
            blk = 220; continue;
        }
        106 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 113; continue; }
            blk = 107; continue;
        }
        107 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_e() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            blk = 220; continue;
        }
        109 => {
            let Some(t125) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c793a); };
            c.w32(2, t125);
            let Some(t126) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c793e); };
            c.cmp(t126, c.r32(2), 32);
            if c.cc_ne() { blk = 221; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t127) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c794b); };
            c.w32(15, t127);
            let r128 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r128);
            let Some(t129) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c7952); };
            c.w32(2, t129);
            let r130 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r130);
            let Some(t131) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c7959); };
            c.w32(2, t131);
            blk = 101; continue;
        }
        111 => {
            let Some(t132) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c795f); };
            c.w32(2, t132);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r133 = c.add(c.r64(12), c.r64(2), 64);
            c.w64(12, r133);
            let Some(t134) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c796c); };
            c.w32(15, t134);
            let r135 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r135);
            let Some(t136) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c7973); };
            c.w32(2, t136);
            let r137 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r137);
            let Some(t138) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c797a); };
            c.w32(2, t138);
            blk = 101; continue;
        }
        112 => {
            let Some(t139) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c7980); };
            c.w32(15, t139);
            let r140 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r140);
            let Some(t141) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c7987); };
            c.w32(2, t141);
            let r142 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r142);
            let Some(t143) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c798e); };
            c.w32(2, t143);
            blk = 101; continue;
        }
        113 => {
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(12));
            c.w64(12, c.r64(10));
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(2));
            c.w64(2, c.r64(10));
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 116; continue;
        }
        114 => {
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(12));
            c.w64(12, c.r64(10));
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(2));
            c.w64(2, c.r64(10));
            c.w32(1, 0x1u64);
            blk = 116; continue;
        }
        115 => {
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(3));
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 116; continue;
        }
        116 => {
            c.w64(6, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 222; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w64(0, c.r64(2).wrapping_add(0xb));
            blk = 118; continue;
        }
        118 => {
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r144 = c.r32(12) & 0x8u64;
            c.w32(12, r144);
            c.logic(r144, 32);
            let r145 = c.add(c.r64(15), c.r64(6), 64);
            c.w64(15, r145);
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t146) = c.ld16(c.r64(0).wrapping_add(c.r64(15).wrapping_mul(2))) else { return RegfileExit::fault(0x70008c79ee); };
            c.w32(12, t146);
            blk = 121; continue;
        }
        120 => {
            let Some(t147) = c.ld8(c.r64(15).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70008c79f5); };
            c.w32(12, t147);
            blk = 121; continue;
        }
        121 => {
            let r148 = c.r32(12) & 0xfc00u64;
            c.w32(12, r148);
            c.logic(r148, 32);
            c.cmp(c.r32(12), 0xd800u64, 32);
            if c.cc_ne() { blk = 153; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t149) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c7a0e); };
            c.w32(12, t149);
            let r150 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r150);
            let Some(t151) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c7a15); };
            c.w32(12, t151);
            c.logic(c.r8(12) & 0x7u64, 8);
            if c.cc_e() { blk = 138; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w64(0, c.r64(12));
            c.w32(12, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(3));
            blk = 124; continue;
        }
        124 => {
            c.w64(2, c.r64(0));
            let r152 = c.r32(2) & 0x7u64;
            c.w32(2, r152);
            c.logic(r152, 32);
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_l() { blk = 129; continue; }
            blk = 125; continue;
        }
        125 => {
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 126; continue;
        }
        126 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 127; continue;
        }
        127 => {
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 128; continue;
        }
        128 => {
            blk = 237; continue;
        }
        129 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 136; continue; }
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r32(2), 0x1u64, 32);
            if c.cc_e() { blk = 132; continue; }
            blk = 131; continue;
        }
        131 => {
            blk = 237; continue;
        }
        132 => {
            let Some(t153) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c7a60); };
            c.w32(0, t153);
            let Some(t154) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c7a64); };
            c.cmp(t154, c.r32(0), 32);
            if c.cc_ne() { blk = 238; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t155) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c7a71); };
            c.w32(15, t155);
            let r156 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r156);
            let Some(t157) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c7a78); };
            c.w32(0, t157);
            let r158 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r158);
            let Some(t159) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c7a7f); };
            c.w32(0, t159);
            blk = 124; continue;
        }
        134 => {
            let Some(t160) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c7a85); };
            c.w32(0, t160);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r161 = c.add(c.r64(12), c.r64(0), 64);
            c.w64(12, r161);
            let Some(t162) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c7a92); };
            c.w32(15, t162);
            let r163 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r163);
            let Some(t164) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c7a99); };
            c.w32(0, t164);
            let r165 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r165);
            let Some(t166) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c7aa0); };
            c.w32(0, t166);
            blk = 124; continue;
        }
        135 => {
            let Some(t167) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c7aa6); };
            c.w32(15, t167);
            let r168 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r168);
            let Some(t169) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c7aad); };
            c.w32(0, t169);
            let r170 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r170);
            let Some(t171) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c7ab4); };
            c.w32(0, t171);
            blk = 124; continue;
        }
        136 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(12));
            c.w64(12, c.r64(10));
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(10));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 139; continue;
        }
        137 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(12));
            c.w64(12, c.r64(10));
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(10));
            c.w32(2, 0x1u64);
            blk = 139; continue;
        }
        138 => {
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(3));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 139; continue;
        }
        139 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 239; continue; }
            blk = 140; continue;
        }
        140 => {
            let r172 = c.add(c.r64(0), 0xbu64, 64);
            c.w64(0, r172);
            blk = 141; continue;
        }
        141 => {
            let r173 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r173);
            let r174 = c.r32(12) & 0x8u64;
            c.w32(12, r174);
            c.logic(r174, 32);
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 143; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t175) = c.ld16(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(2))) else { return RegfileExit::fault(0x70008c7b0e); };
            c.w32(11, t175);
            blk = 144; continue;
        }
        143 => {
            let Some(t176) = c.ld8(c.r64(11).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70008c7b15); };
            c.w32(11, t176);
            blk = 144; continue;
        }
        144 => {
            let r177 = c.r32(11) & 0xfc00u64;
            c.w32(11, r177);
            c.logic(r177, 32);
            c.cmp(c.r32(11), 0xdc00u64, 32);
            if c.cc_ne() { blk = 153; continue; }
            blk = 145; continue;
        }
        145 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 147; continue; }
            blk = 146; continue;
        }
        146 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c7b30); }
            blk = 150; continue;
        }
        147 => {
            let r178 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r178);
            if c.cc_o() { blk = 149; continue; }
            blk = 148; continue;
        }
        148 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 153; continue;
        }
        149 => {
            let r179 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r179);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 150; continue;
        }
        150 => {
            let Some(t180) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c7b4f); };
            c.w64(8, t180);
            let Some(t181) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008c7b56); };
            c.w64(9, t181);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t182) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c7b5d); };
            c.w64(12, t182);
            let Some(t183) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008c7b64); };
            c.cmp(t183, c.r64(11), 64);
            if c.cc_be() { blk = 254; continue; }
            blk = 151; continue;
        }
        151 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008c7b6e); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 152; continue;
        }
        152 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t184) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c7b87); };
            c.w64(9, t184);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008c7b8e); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008c7b92); }
            blk = 153; continue;
        }
        153 => {
            let Some(t185) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c7b98); };
            c.w64(0, t185);
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008c7b9c); };
            c.w64(3, t186);
            c.w64(1, c.r64(8));
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c7ba6); };
            c.w64(6, t187);
            return c.call(0x7000871cb4, 0x70008c7baf, 0x70008c7baa);
        }
        154 => {
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c7baf); };
            c.w64(11, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c7bb3); };
            c.w64(9, t189);
            c.w64(8, 0xffffffffffffffffu64);
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c7bbe); };
            c.w64(12, t190);
            blk = 43; continue;
        }
        155 => {
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008c7bc7); };
            c.w64(2, t191);
            let Some(t192) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008c7bce); };
            c.w32(2, t192);
            let r193 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r193);
            return c.call(0x7000824234, 0x70008c7bdc, 0x70008c7bd7);
        }
        156 => {
            return RegfileExit::fault(0x70008c7bdc);
        }
        157 => {
            return RegfileExit::fault(0x70008c7bdd);
        }
        158 => {
            return RegfileExit::fault(0x70008c7bde);
        }
        159 => {
            return RegfileExit::fault(0x70008c7bdf);
        }
        160 => {
            return RegfileExit::fault(0x70008c7be0);
        }
        161 => {
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c7be1); };
            c.cmp(t194, 0x0u64, 64);
            if c.cc_e() { blk = 186; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c7bec); };
            c.w64(6, t195);
            let Some(t196) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c7bf0); };
            c.w32(8, t196);
            let r197 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r197);
            let Some(t198) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c7bf7); };
            c.w32(8, t198);
            let r199 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r199);
            let Some(t200) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c7bfe); };
            c.w32(9, t200);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 164; continue; }
            blk = 163; continue;
        }
        163 => {
            return RegfileExit::fault(0x70008c7c0e);
        }
        164 => {
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c7c0f); };
            c.w64(9, t201);
            let Some(t202) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c7c13); };
            c.w32(11, t202);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t203) = c.ld32(c.r64(8).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008c7c1a); };
            c.w32(8, t203);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c7c24); };
            c.w64(12, t204);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 166; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_ae() { blk = 169; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008c7c33); };
            c.w64(2, t205);
            let Some(t206) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008c7c3a); };
            c.w32(2, t206);
            let r207 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r207);
            return c.call(0x7000824234, 0x70008c7c48, 0x70008c7c43);
        }
        167 => {
            return RegfileExit::fault(0x70008c7c48);
        }
        168 => {
            return RegfileExit::fault(0x70008c7c49);
        }
        169 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 266; continue; }
            blk = 170; continue;
        }
        170 => {
            c.w32(11, c.r64(12).wrapping_add(c.r64(12)));
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(15));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w32(0, c.r32(0));
            let r208 = c.imul2(c.r32(0), 0x4u64, 32);
            c.w32(0, r208);
            if c.cc_o() { blk = 185; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w32(3, 0x8u64);
            let r209 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r209);
            if c.cc_o() { blk = 184; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t210) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c7c80); };
            c.w64(2, t210);
            let Some(t211) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008c7c87); };
            c.w64(2, t211);
            c.w64(1, c.r64(2).wrapping_add(c.r64(0)));
            let Some(t212) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c7c8e); };
            c.w64(6, t212);
            let Some(t213) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008c7c95); };
            c.cmp(t213, c.r64(1), 64);
            if c.cc_be() { blk = 262; continue; }
            blk = 173; continue;
        }
        173 => {
            c.cmp(c.r64(0), 0x20000u64, 64);
            if c.cc_ae() { blk = 262; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t214) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c7caa); };
            c.w64(0, t214);
            if !c.st64(c.r64(0), c.r64(1)) { return RegfileExit::fault(0x70008c7cb1); }
            c.w64(0, c.r64(2).wrapping_add(0x1));
            blk = 175; continue;
        }
        175 => {
            let Some(t215) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c7cb8); };
            c.w64(2, t215);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008c7cbf); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008c7cc2); }
            c.w64(15, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.w64(2, c.r64(3));
            blk = 177; continue;
        }
        176 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(3)).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008c7cd3); }
            let r216 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r216);
            blk = 177; continue;
        }
        177 => {
            c.cmp(c.r64(15), c.r64(3), 64);
            if c.cc_e() { blk = 181; continue; }
            blk = 178; continue;
        }
        178 => {
            c.w64(1, c.r64(12).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(1), c.r64(2), 64);
            if c.cc_e() { blk = 180; continue; }
            blk = 179; continue;
        }
        179 => {
            let Some(t217) = c.ld32(c.r64(9).wrapping_add(c.r64(2)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c7ced); };
            c.w32(1, t217);
            let r218 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r218);
            blk = 176; continue;
        }
        180 => {
            let Some(t219) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008c7cf8); };
            c.w64(1, t219);
            blk = 176; continue;
        }
        181 => {
            let Some(t220) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c7d01); };
            c.w64(9, t220);
            let Some(t221) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c7d08); };
            c.w64(12, t221);
            c.w64(15, c.r64(12).wrapping_add(0x10));
            let Some(t222) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c7d10); };
            c.w64(3, t222);
            let Some(t223) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c7d17); };
            c.cmp(t223, c.r64(15), 64);
            if c.cc_be() { blk = 264; continue; }
            blk = 182; continue;
        }
        182 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008c7d20); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 183; continue;
        }
        183 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008c7d28); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008c7d2c); }
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c7d30); };
            c.w64(8, t224);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008c7d37); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008c7d3b); }
            c.w64(0, c.r64(9));
            blk = 15; continue;
        }
        184 => {
            return RegfileExit::fault(0x70008c7d47);
        }
        185 => {
            return RegfileExit::fault(0x70008c7d48);
        }
        186 => {
            let Some(t225) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c7d49); };
            c.w64(0, t225);
            blk = 15; continue;
        }
        187 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c7d5b); }
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008c7d5d); };
            c.w64(8, t226);
            let Some(t227) = c.ld32(c.r64(8).wrapping_add(0x28b)) else { return RegfileExit::fault(0x70008c7d64); };
            c.w32(8, t227);
            let r228 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r228);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c7d6e); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c7d70); }
            let Some(t229) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008c7d71); };
            c.w64(3, t229);
            c.w32(0, 0x3u64);
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c7d7d); };
            c.w64(6, t230);
            return c.call(0x700086ed74, 0x70008c7d86, 0x70008c7d81);
        }
        188 => {
            return RegfileExit::fault(0x70008c7d86);
        }
        189 => {
            let Some(t231) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c7d87); };
            c.w64(0, t231);
            let Some(t232) = c.ld64(c.r64(13).wrapping_add(0x6e8)) else { return RegfileExit::fault(0x70008c7d8b); };
            c.w64(3, t232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c7d92); };
            c.w64(6, t233);
            return c.call(0x7000870914, 0x70008c7d9b, 0x70008c7d96);
        }
        190 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 197; continue; }
            blk = 191; continue;
        }
        191 => {
            let Some(t234) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c7d9f); };
            c.w32(8, t234);
            let r235 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r235);
            let Some(t236) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008c7da6); };
            c.logic(t236 & 0x2u64, 8);
            if c.cc_e() { blk = 197; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c7dad); };
            if !c.push64(t237) { return RegfileExit::fault(0x70008c7dad); }
            let Some(t238) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c7db0); };
            if !c.push64(t238) { return RegfileExit::fault(0x70008c7db0); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c7dbb); };
            c.w64(6, t239);
            return c.call(0x70007f2274, 0x70008c7dc4, 0x70008c7dbf);
        }
        193 => {
            let Some(t240) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c7dc4); };
            c.cmp(t240, c.r32(0), 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 194; continue;
        }
        194 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 268; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t241) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c7dd5); };
            c.w32(8, t241);
            let r242 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r242);
            let Some(t243) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c7ddc); };
            c.cmp(t243, 0xbau64, 16);
            if c.cc_b() { blk = 268; continue; }
            blk = 196; continue;
        }
        196 => {
            blk = 15; continue;
        }
        197 => {
            let Some(t244) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c7dee); };
            c.w64(0, t244);
            let Some(t245) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c7df2); };
            c.w32(8, t245);
            let r246 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r246);
            let Some(t247) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c7df9); };
            c.cmp(t247, 0x83eu64, 16);
            if c.cc_ne() { blk = 200; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c7e02); };
            c.w64(6, t248);
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c7e06); };
            c.w64(3, t249);
            return c.call(0x70008bf674, 0x70008c7e0f, 0x70008c7e0a);
        }
        199 => {
            blk = 15; continue;
        }
        200 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c7e1a); }
            let Some(t250) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008c7e1c); };
            c.w64(8, t250);
            let Some(t251) = c.ld32(c.r64(8).wrapping_add(0x28b)) else { return RegfileExit::fault(0x70008c7e23); };
            c.w32(8, t251);
            let r252 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r252);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c7e2d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c7e2f); }
            let Some(t253) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008c7e30); };
            c.w64(3, t253);
            c.w32(0, 0x3u64);
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c7e3c); };
            c.w64(6, t254);
            return c.call(0x700086ed74, 0x70008c7e45, 0x70008c7e40);
        }
        201 => {
            return RegfileExit::fault(0x70008c7e45);
        }
        202 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008c7e48); };
            c.w64(3, t255);
            return c.call(0x700086ed74, 0x70008c7e54, 0x70008c7e4f);
        }
        203 => {
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c7e54); };
            c.w64(6, t256);
            blk = 1; continue;
        }
        204 => {
            c.w64(0, c.r64(8));
            return c.call(0x700089c794, 0x70008c7e65, 0x70008c7e60);
        }
        205 => {
            c.w64(8, c.r64(0));
            let Some(t257) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c7e68); };
            c.w64(0, t257);
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c7e6c); };
            c.w64(6, t258);
            blk = 5; continue;
        }
        206 => {
            let Some(t259) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c7e75); };
            c.logic(t259 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 207; continue;
        }
        207 => {
            blk = 27; continue;
        }
        208 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c7e87); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 27; continue; }
            blk = 209; continue;
        }
        209 => {
            blk = 28; continue;
        }
        210 => {
            let Some(t260) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c7e9f); };
            c.logic(t260 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 211; continue;
        }
        211 => {
            blk = 39; continue;
        }
        212 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c7eb1); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 39; continue; }
            blk = 213; continue;
        }
        213 => {
            blk = 40; continue;
        }
        214 => {
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c7ec9); };
            c.w64(6, t261);
            return c.call(0x700089c794, 0x70008c7ed2, 0x70008c7ecd);
        }
        215 => {
            blk = 58; continue;
        }
        216 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008c7ed7); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x70008c7edb); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008c7edf); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008c7ee3); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x70008c7eef, 0x70008c7eea);
        }
        217 => {
            c.w64(3, c.r64(0));
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c7ef2); };
            c.w64(9, t262);
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c7ef6); };
            c.w64(11, t263);
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c7efa); };
            c.w64(0, t264);
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c7efe); };
            c.w64(8, t265);
            c.w32(2, 0x8u64);
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c7f07); };
            c.w64(15, t266);
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c7f0b); };
            c.w64(12, t267);
            blk = 69; continue;
        }
        218 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x70008c7f19); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008c7f1e); }
            return c.call(0x70007febf4, 0x70008c7f27, 0x70008c7f22);
        }
        219 => {
            c.w64(8, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x70008c7f2a); }
            let Some(t268) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c7f2f); };
            c.w64(0, t268);
            blk = 95; continue;
        }
        220 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008c7f38); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008c7f3c); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008c7f40); }
            blk = 224; continue;
        }
        221 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008c7f46); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008c7f4a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008c7f4e); }
            blk = 224; continue;
        }
        222 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008c7f54); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008c7f58); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008c7f5c); }
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 224; continue; }
            blk = 223; continue;
        }
        223 => {
            let Some(t269) = c.ld64(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c7f66); };
            c.w64(0, t269);
            blk = 118; continue;
        }
        224 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008c7f75); }
            c.cmp(c.r64(12), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 226; continue; }
            blk = 225; continue;
        }
        225 => {
            let r270 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r270);
            blk = 235; continue;
        }
        226 => {
            let Some(t271) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c7f8a); };
            c.w64(15, t271);
            let Some(t272) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008c7f91); };
            c.w64(0, t272);
            c.w64(2, c.r64(0).wrapping_add(0xc));
            let Some(t273) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c7f98); };
            c.w64(1, t273);
            let Some(t274) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008c7f9f); };
            c.cmp(t274, c.r64(2), 64);
            if c.cc_a() { blk = 229; continue; }
            blk = 227; continue;
        }
        227 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008c7fae, 0x70008c7fa9);
        }
        228 => {
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c7fae); };
            c.w64(12, t275);
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c7fb2); };
            c.w64(3, t276);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c7fb6); };
            c.w64(8, t277);
            let Some(t278) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c7fba); };
            c.w64(9, t278);
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c7fbe); };
            c.w64(11, t279);
            blk = 230; continue;
        }
        229 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x70008c7fc4); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(15));
            blk = 230; continue;
        }
        230 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(12) as i64));
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 234; continue; }
            blk = 231; continue;
        }
        231 => {
            c.w64(10, c.r64(12));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 233; continue; }
            blk = 232; continue;
        }
        232 => {
            let r280 = c.r64(10) | 0x1u64;
            c.w64(10, r280);
            c.logic(r280, 64);
            blk = 233; continue;
        }
        233 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 234; continue;
        }
        234 => {
            let Some(t281) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c7ff5); };
            c.w64(12, t281);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c7ffc); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008c8000); }
            c.w64(12, c.r64(0));
            blk = 235; continue;
        }
        235 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008c8008); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008c8009); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t282) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008c8012); };
            c.w64(3, t282);
            c.w64(12, c.r64(3));
            return c.call(0x700086ed74, 0x70008c8021, 0x70008c801c);
        }
        236 => {
            let r283 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r283);
            c.w64(12, c.r64(0));
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c8026); };
            c.w64(3, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c802a); };
            c.w64(8, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c802e); };
            c.w64(9, t286);
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c8032); };
            c.w64(11, t287);
            blk = 121; continue;
        }
        237 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008c803b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008c803f); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008c8043); }
            blk = 241; continue;
        }
        238 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008c8049); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008c804d); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008c8051); }
            blk = 241; continue;
        }
        239 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008c8057); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008c805b); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008c805f); }
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 241; continue; }
            blk = 240; continue;
        }
        240 => {
            let Some(t288) = c.ld64(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c8069); };
            c.w64(0, t288);
            blk = 141; continue;
        }
        241 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 243; continue; }
            blk = 242; continue;
        }
        242 => {
            let r289 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r289);
            blk = 252; continue;
        }
        243 => {
            let Some(t290) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c8083); };
            c.w64(12, t290);
            let Some(t291) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008c808a); };
            c.w64(15, t291);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t292) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c8092); };
            c.w64(2, t292);
            let Some(t293) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008c8099); };
            c.cmp(t293, c.r64(0), 64);
            if c.cc_a() { blk = 246; continue; }
            blk = 244; continue;
        }
        244 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008c80a8, 0x70008c80a3);
        }
        245 => {
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c80a8); };
            c.w64(11, t294);
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c80ac); };
            c.w64(3, t295);
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c80b0); };
            c.w64(8, t296);
            let Some(t297) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c80b4); };
            c.w64(9, t297);
            blk = 247; continue;
        }
        246 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008c80ba); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(12));
            blk = 247; continue;
        }
        247 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 251; continue; }
            blk = 248; continue;
        }
        248 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 250; continue; }
            blk = 249; continue;
        }
        249 => {
            let r298 = c.r64(10) | 0x1u64;
            c.w64(10, r298);
            c.logic(r298, 64);
            blk = 250; continue;
        }
        250 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 251; continue;
        }
        251 => {
            let Some(t299) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c80ec); };
            c.w64(11, t299);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008c80f3); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008c80f7); }
            c.w64(11, c.r64(0));
            blk = 252; continue;
        }
        252 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008c80ff); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008c8100); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t300) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008c8109); };
            c.w64(3, t300);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70008c8118, 0x70008c8113);
        }
        253 => {
            let r301 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r301);
            c.w64(11, c.r64(0));
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c811d); };
            c.w64(3, t302);
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c8121); };
            c.w64(8, t303);
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c8125); };
            c.w64(9, t304);
            blk = 144; continue;
        }
        254 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x70008c8133); }
            return c.call(0x70007febf4, 0x70008c813d, 0x70008c8138);
        }
        255 => {
            c.w64(8, c.r64(0));
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c8140); };
            c.w64(3, t305);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x70008c8144); }
            blk = 152; continue;
        }
        256 => {
            c.w32(8, 0x88u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c8154); }
            let Some(t306) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c8156); };
            c.w64(8, t306);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c815d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c815f); }
            let Some(t307) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008c8160); };
            c.w64(3, t307);
            c.w32(0, 0x3u64);
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c816c); };
            c.w64(6, t308);
            return c.call(0x700086ed74, 0x70008c8175, 0x70008c8170);
        }
        257 => {
            return RegfileExit::fault(0x70008c8175);
        }
        258 => {
            blk = 155; continue;
        }
        259 => {
            let Some(t309) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008c817b); };
            c.w64(3, t309);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008c818b, 0x70008c8186);
        }
        260 => {
            return RegfileExit::fault(0x70008c818b);
        }
        261 => {
            blk = 166; continue;
        }
        262 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008c8191); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008c8195); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008c8199); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febb4, 0x70008c81a5, 0x70008c81a0);
        }
        263 => {
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c81a5); };
            c.w64(11, t310);
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c81a9); };
            c.w64(8, t311);
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c81ad); };
            c.w64(9, t312);
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c81b1); };
            c.w64(12, t313);
            c.w32(3, 0x8u64);
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c81ba); };
            c.w64(15, t314);
            blk = 175; continue;
        }
        264 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008c81c8); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008c81cc); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008c81d0); }
            return c.call(0x70007febf4, 0x70008c81d9, 0x70008c81d4);
        }
        265 => {
            c.w64(9, c.r64(0));
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c81dc); };
            c.w64(0, t315);
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c81e0); };
            c.w64(8, t316);
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c81e4); };
            c.w64(11, t317);
            blk = 183; continue;
        }
        266 => {
            let Some(t318) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008c81ed); };
            c.w64(3, t318);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008c81fd, 0x70008c81f8);
        }
        267 => {
            return RegfileExit::fault(0x70008c81fd);
        }
        268 => {
            c.w32(8, 0x88u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c8204); }
            let Some(t319) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c8206); };
            c.w64(8, t319);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c820d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c820f); }
            let Some(t320) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008c8210); };
            c.w64(3, t320);
            c.w32(0, 0x3u64);
            let Some(t321) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c821c); };
            c.w64(6, t321);
            return c.call(0x700086ed74, 0x70008c8225, 0x70008c8220);
        }
        269 => {
            return RegfileExit::fault(0x70008c8225);
        }
        270 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c822c); }
            let Some(t322) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008c822e); };
            c.w64(8, t322);
            let Some(t323) = c.ld32(c.r64(8).wrapping_add(0x293)) else { return RegfileExit::fault(0x70008c8235); };
            c.w32(8, t323);
            let r324 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r324);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c823f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c8241); }
            let Some(t325) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008c8242); };
            c.w64(3, t325);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008c8253, 0x70008c824e);
        }
        271 => {
            return RegfileExit::fault(0x70008c8253);
        }
        272 => {
            c.w32(0, c.r32(0));
            let r326 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r326);
            c.w64(3, 0xfffffffffffc0000u64);
            let r327 = c.r64(3) & c.r64(0);
            c.w64(3, r327);
            c.logic(r327, 64);
            let Some(t328) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c8263); };
            c.logic(t328 & 0x2u64, 8);
            if c.cc_e() { blk = 81; continue; }
            blk = 273; continue;
        }
        273 => {
            c.w64(2, c.r64(9).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c8271); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c8272); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70008c827f, 0x70008c827a);
        }
        274 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c827f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c8281); }
            blk = 81; continue;
        }
        275 => {
            return RegfileExit::jump(0x70008c8288);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008c8294(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008c8294); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008c8298); }
            let r0 = c.sub(c.r64(4), 0x78u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008c829e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008c82a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(1)) { return RegfileExit::fault(0x70008c82a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008c82aa); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 65; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c82b7); };
            c.w32(11, t1);
            let r2 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r2);
            let Some(t3) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008c82be); };
            c.logic(t3 & 0x2u64, 8);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 66; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c82cd); };
            c.w32(11, t4);
            c.w32(11, c.r32(11));
            let r5 = c.r32(11) & 0x2u64;
            c.w32(11, r5);
            c.logic(r5, 32);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r6 = c.r64(11) & 0xfffffffffffffffeu64;
            c.w64(11, r6);
            c.logic(r6, 64);
            let r7 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r7);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008c82e8); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c82e9); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008c82ea); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2690)) else { return RegfileExit::fault(0x70008c82eb); };
            c.w64(3, t8);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008c82fc, 0x70008c82f7);
        }
        5 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008c82ff); }
            return c.ret(0x0, 0x70008c8300);
        }
        6 => {
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008c8304); }
            let Some(t9) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c8308); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c830f); };
            c.w32(11, t11);
            let r12 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r12);
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c8316); };
            c.w32(12, t13);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x9bu64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x70008c8329);
        }
        8 => {
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x273)) else { return RegfileExit::fault(0x70008c832a); };
            c.w32(15, t14);
            let r15 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r15);
            c.cmp(c.r64(12), 0x6bu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x70008c833a);
        }
        10 => {
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008c833b); };
            c.w32(11, t16);
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c8342); };
            c.w64(12, t17);
            let Some(t18) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008c8349); };
            c.w64(2, t18);
            c.w64(7, c.r64(2).wrapping_add(0x58));
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c8351); };
            c.w64(8, t19);
            let Some(t20) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008c8358); };
            c.cmp(t20, c.r64(7), 64);
            if c.cc_be() { blk = 333; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(12), c.r64(7)) { return RegfileExit::fault(0x70008c8361); }
            c.w64(12, c.r64(2).wrapping_add(0x1));
            blk = 12; continue;
        }
        12 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008c8369); }
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008c8371); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c8376); };
            c.w64(11, t21);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008c837d); }
            c.w64(11, c.r64(12).wrapping_add(0x10));
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008c8387); }
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c838c); };
            c.w64(2, t22);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008c8393); }
            c.w32(2, 0x20u64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008c839c); }
            c.w32(2, 0x47u64);
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008c83a5); };
            c.w64(7, t23);
            blk = 13; continue;
        }
        13 => {
            let r24 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r24);
            if !c.st32(c.r64(2).wrapping_add(c.r64(11)), c.r32(7)) { return RegfileExit::fault(0x70008c83b0); }
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_ne() { blk = 13; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c83ba); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008c83bb); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008c83bc); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008c83be); }
            c.w32(0, 0x4u64);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2668)) else { return RegfileExit::fault(0x70008c83c5); };
            c.w64(3, t25);
            c.w64(11, c.r64(3));
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x70008c83d7, 0x70008c83d2);
        }
        15 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c83d7); };
            c.w64(12, t26);
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008c83de); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008c83e3); }
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c83e7); };
            c.cmp(t27, c.r32(0), 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t28) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c83f4); };
            c.w32(11, t28);
            let r29 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r29);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(11)) { return RegfileExit::fault(0x70008c83fb); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 63; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c8409); };
            c.w64(6, t30);
            let Some(t31) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c840d); };
            c.w32(12, t31);
            let r32 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r32);
            let Some(t33) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c8414); };
            c.w32(12, t33);
            let r34 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r34);
            let Some(t35) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c841c); };
            c.w32(15, t35);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t36) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8424); };
            c.w32(3, t36);
            let r37 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r37);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008c842d); }
            c.cmp(c.r64(15), 0x9bu64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x70008c843a);
        }
        19 => {
            let Some(t38) = c.ld32(c.r64(12).wrapping_add(0x273)) else { return RegfileExit::fault(0x70008c843b); };
            c.w32(12, t38);
            let r39 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r39);
            let Some(t40) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c8446); };
            c.w32(15, t40);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            return RegfileExit::fault(0x70008c8457);
        }
        21 => {
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t41) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c845b); };
            c.w32(12, t41);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x70008c8463); }
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        23 => {
            c.w64(12, c.r64(15).wrapping_add(0x1));
            c.w64(3, c.r64(11));
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c847d); };
            c.w64(6, t42);
            blk = 24; continue;
        }
        24 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c8481); };
            c.cmp(c.r64(12), t43, 64);
            if c.cc_ge() { blk = 57; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t44) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c848b); };
            c.w32(2, t44);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(12), c.r64(2), 64);
            if c.cc_ae() { blk = 37; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(2, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            let Some(t45) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c84a5); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008c84ad); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t47) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c84ba); };
            c.w32(7, t47);
            let r48 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r48);
            let Some(t49) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c84c0); };
            c.cmp(t49, 0x834u64, 16);
            if c.cc_ne() { blk = 36; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t50) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c84cc); };
            c.w32(7, t50);
            let r51 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r51);
            let Some(t52) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c84d2); };
            c.w32(7, t52);
            let r53 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r53);
            let Some(t54) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c84d8); };
            c.w32(8, t54);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x9u64, 64);
            if c.cc_be() { blk = 38; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t55) = c.ld32(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008c84ec); };
            c.w32(7, t55);
            let r56 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r56);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008c84f2); }
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c84f3); };
            c.w64(1, t57);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008c84fa); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008c84fb); };
            if !c.push64(t58) { return RegfileExit::fault(0x70008c84fb); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008c84fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008c84ff); }
            c.w32(0, 0x3u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2134, 0x70008c8510, 0x70008c850b);
        }
        30 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c8510); };
            c.w64(11, t59);
            let Some(t60) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c8514); };
            c.w32(12, t60);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c851e); };
            c.w64(15, t61);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_ae() { blk = 39; continue; }
            blk = 31; continue;
        }
        31 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 335; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t62) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c852f); };
            c.w32(12, t62);
            let r63 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r63);
            let Some(t64) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8536); };
            c.cmp(t64, 0x40u64, 16);
            if c.cc_ae() { blk = 335; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c8543); };
            c.w64(12, t65);
            if !c.st32(c.r64(11).wrapping_add(c.r64(12)), c.r32(0)) { return RegfileExit::fault(0x70008c8547); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r66 = c.r64(3) & c.r64(11);
            c.w64(3, r66);
            c.logic(r66, 64);
            let Some(t67) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c8559); };
            c.logic(t67 & 0x4u64, 8);
            if c.cc_ne() { blk = 446; continue; }
            blk = 35; continue;
        }
        35 => {
            blk = 23; continue;
        }
        36 => {
            c.w64(11, c.r64(3));
            c.w64(15, c.r64(12));
            blk = 23; continue;
        }
        37 => {
            return RegfileExit::fault(0x70008c8573);
        }
        38 => {
            return RegfileExit::fault(0x70008c8574);
        }
        39 => {
            return RegfileExit::fault(0x70008c8575);
        }
        40 => {
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(2));
            blk = 42; continue;
        }
        41 => {
            let r68 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r68);
            blk = 42; continue;
        }
        42 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c8581); };
            c.cmp(c.r64(12), t69, 64);
            if c.cc_ge() { blk = 57; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t70) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c858b); };
            c.w32(1, t70);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(12), c.r64(1), 64);
            if c.cc_ae() { blk = 59; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(7, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            let Some(t71) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c85a5); };
            c.w32(8, t71);
            let r72 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r72);
            c.w32(9, c.r32(8));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008c85b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008c85b4); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_g() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let r73 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r73);
            c.cmp(c.r64(12), c.r64(1), 64);
            if c.cc_ae() { blk = 60; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t74) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c85d1); };
            c.w32(2, t74);
            c.w32(2, c.r32(2));
            let r75 = c.sub(c.r32(2), c.r32(9), 32);
            c.w32(2, r75);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 41; continue;
        }
        48 => {
            c.w64(2, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.w64(1, c.r64(2));
            let r76 = c.r64(1) & 0x7ffu64;
            c.w64(1, r76);
            c.logic(r76, 64);
            if let Some(sr) = c.sar(c.r64(2), 0xbu64, 64) { c.w64(2, sr); }
            let r77 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r77);
            let r78 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r78);
            blk = 41; continue;
        }
        49 => {
            let Some(t79) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c85fb); };
            c.w32(1, t79);
            let r80 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r80);
            let Some(t81) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8602); };
            c.cmp(t81, 0x3fu64, 16);
            if c.cc_a() { blk = 62; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c860d); };
            if !c.push64(t82) { return RegfileExit::fault(0x70008c860d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008c8610); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c8611); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c8613); };
            c.w64(1, t83);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008c861a); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008c861b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008c861f); }
            c.w32(0, 0x3u64);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008c8628); };
            c.w64(7, t84);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2134, 0x70008c8634, 0x70008c862f);
        }
        51 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 337; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t85) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c863c); };
            c.w32(11, t85);
            let r86 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r86);
            let Some(t87) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8643); };
            c.cmp(t87, 0x40u64, 16);
            if c.cc_ae() { blk = 337; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c864f); };
            c.w64(11, t88);
            let Some(t89) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c8653); };
            c.w32(12, t89);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c865d); };
            c.w64(15, t90);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_ae() { blk = 61; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c8666); };
            c.w64(12, t91);
            if !c.st32(c.r64(11).wrapping_add(c.r64(12)), c.r32(0)) { return RegfileExit::fault(0x70008c866a); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r92 = c.r64(3) & c.r64(11);
            c.w64(3, r92);
            c.logic(r92, 64);
            let Some(t93) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c867c); };
            c.logic(t93 & 0x4u64, 8);
            if c.cc_ne() { blk = 443; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c8686); };
            c.w64(12, t94);
            let Some(t95) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c868a); };
            c.w32(12, t95);
            let r96 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c8692); };
            c.w64(2, t97);
            let r98 = c.add(c.r32(12), c.r32(2), 32);
            c.w32(12, r98);
            c.w64(2, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(11));
            c.w64(12, c.r64(15));
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c86a2); };
            c.w64(6, t99);
            blk = 41; continue;
        }
        57 => {
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c86ab); };
            if !c.push64(t100) { return RegfileExit::fault(0x70008c86ab); }
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c86ae); };
            if !c.push64(t101) { return RegfileExit::fault(0x70008c86ae); }
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c86b1); };
            if !c.push64(t102) { return RegfileExit::fault(0x70008c86b1); }
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x2768)) else { return RegfileExit::fault(0x70008c86b4); };
            c.w64(3, t103);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008c86c5, 0x70008c86c0);
        }
        58 => {
            blk = 5; continue;
        }
        59 => {
            return RegfileExit::fault(0x70008c86ca);
        }
        60 => {
            return RegfileExit::fault(0x70008c86cb);
        }
        61 => {
            return RegfileExit::fault(0x70008c86cc);
        }
        62 => {
            return RegfileExit::fault(0x70008c86cd);
        }
        63 => {
            return RegfileExit::fault(0x70008c86ce);
        }
        64 => {
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c86cf); };
            c.w64(0, t104);
            blk = 5; continue;
        }
        65 => {
            c.w32(12, 0x1u64);
            blk = 66; continue;
        }
        66 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 339; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(12, c.r64(1));
            let Some(t105) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c86ee); };
            c.w32(15, t105);
            let r106 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r106);
            let Some(t107) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c86f6); };
            c.cmp(t107, 0x40u64, 16);
            if c.cc_ae() { blk = 340; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t108) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c8702); };
            c.w32(15, t108);
            let r109 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r109);
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(12)) { return RegfileExit::fault(0x70008c8709); }
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008c870d); };
            c.w64(2, t110);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t111) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c8716); };
            c.cmp(t111, c.r32(1), 32);
            if c.cc_e() { blk = 331; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t112) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c871f); };
            c.w32(1, t112);
            let r113 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r113);
            let Some(t114) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c8725); };
            c.w32(1, t114);
            let r115 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r115);
            let Some(t116) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c872b); };
            c.w32(7, t116);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x9au64, 64);
            if c.cc_a() { blk = 71; continue; }
            blk = 70; continue;
        }
        70 => {
            return RegfileExit::fault(0x70008c873d);
        }
        71 => {
            let Some(t117) = c.ld32(c.r64(1).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008c873e); };
            c.w32(8, t117);
            let r118 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r118);
            let Some(t119) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008c8748); };
            c.cmp(t119, c.r32(15), 32);
            if c.cc_ne() { blk = 331; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t120) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c8752); };
            c.w32(8, t120);
            c.logic(c.r32(8) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 331; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r64(7), 0x9fu64, 64);
            if c.cc_a() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            return RegfileExit::fault(0x70008c876c);
        }
        75 => {
            let Some(t121) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c876d); };
            c.w32(15, t121);
            let r122 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r122);
            let Some(t123) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c8774); };
            c.w32(15, t123);
            let r124 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r124);
            let Some(t125) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c877b); };
            c.w32(7, t125);
            let r126 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r126);
            let Some(t127) = c.ld32(c.r64(1).wrapping_add(0x283)) else { return RegfileExit::fault(0x70008c8782); };
            c.cmp(t127, c.r32(15), 32);
            if c.cc_ne() { blk = 331; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t128) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008c878f); };
            c.w32(15, t128);
            c.w32(15, c.r32(15));
            let r129 = c.r32(15) & 0x8u64;
            c.w32(15, r129);
            c.logic(r129, 32);
            c.cmp(c.r32(15), 0x8u64, 32);
            if c.cc_ne() { blk = 331; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t130) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008c87a6); };
            c.w64(3, t130);
            let Some(t131) = c.ld32(c.r64(3).wrapping_add(0xb3)) else { return RegfileExit::fault(0x70008c87ad); };
            c.w32(3, t131);
            let r132 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r132);
            c.w64(0, c.r64(12));
            c.w64(15, c.r64(3));
            return c.call(0x70008e8b74, 0x70008c87c1, 0x70008c87bc);
        }
        78 => {
            c.cmp(c.r32(0), 0xfffffffffffffffeu64, 32);
            if c.cc_ne() { blk = 436; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c87ca); };
            c.w64(0, t133);
            let Some(t134) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c87ce); };
            c.w32(11, t134);
            c.w32(11, c.r32(11));
            c.w64(12, c.r64(11));
            let r135 = c.r32(12) & 0x2u64;
            c.w32(12, r135);
            c.logic(r135, 32);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let r136 = c.r64(12) & 0xfffffffffffffffeu64;
            c.w64(12, r136);
            c.logic(r136, 64);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008c87e3); };
            c.w64(1, t137);
            let Some(t138) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c87e7); };
            c.w32(15, t138);
            let r139 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r139);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(15)) { return RegfileExit::fault(0x70008c87ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x70008c87f2); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 81; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 82; continue;
        }
        81 => {
            let r140 = c.r32(11) & 0x20u64;
            c.w32(11, r140);
            c.logic(r140, 32);
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(3)) { return RegfileExit::fault(0x70008c8809); }
            let r141 = c.shr(c.r32(11), 0x4u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r141);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r142 = c.r64(11) & 0xfffffffffffffffeu64;
            c.w64(11, r142);
            c.logic(r142, 64);
            let r143 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r143);
            blk = 82; continue;
        }
        82 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008c881a); }
            c.w64(2, c.r64(3));
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c8821); };
            c.w64(10, t144);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(10)) { return RegfileExit::fault(0x70008c8828); }
            blk = 84; continue;
        }
        83 => {
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c882e); };
            c.w64(2, t145);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008c8832); }
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c8836); };
            c.w64(0, t146);
            blk = 84; continue;
        }
        84 => {
            let Some(t147) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c883a); };
            c.w32(11, t147);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let Some(t148) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c8844); };
            c.w32(12, t148);
            let r149 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r149);
            c.logic(c.r8(11) & 0x9u64, 8);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c8857); };
            c.w64(7, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c885b); };
            c.w64(8, t151);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008c885f); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 99; continue;
        }
        86 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 326; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t152) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8887); };
            c.w32(15, t152);
            let r153 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r153);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 91; continue; }
            blk = 88; continue;
        }
        88 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c8894); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c88a0); }
            blk = 93; continue;
        }
        90 => {
            c.w64(11, c.r64(12));
            let r154 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r154);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 93; continue;
        }
        91 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 95; continue; }
            blk = 92; continue;
        }
        92 => {
            let r155 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r155);
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c88c3); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            blk = 93; continue;
        }
        93 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 97; continue; }
            blk = 94; continue;
        }
        94 => {
            blk = 96; continue;
        }
        95 => {
            c.cmp(c.r32(15), c.r32(12), 32);
            if c.cc_l() { blk = 97; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 98; continue;
        }
        97 => {
            c.w32(11, 0x1u64);
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 326; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t156) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c88f5); };
            c.w32(15, t156);
            let r157 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r157);
            let Some(t158) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c88fc); };
            c.w32(15, t158);
            let r159 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r159);
            let Some(t160) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c8903); };
            c.w32(11, t160);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008c890d); }
            c.cmp(c.r64(11), 0x9bu64, 64);
            if c.cc_be() { blk = 316; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t161) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c891e); };
            c.w32(11, t161);
            let r162 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r162);
            let Some(t163) = c.ld32(c.r64(15).wrapping_add(0x273)) else { return RegfileExit::fault(0x70008c8925); };
            c.w32(15, t163);
            let r164 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r164);
            let Some(t165) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c892f); };
            c.w32(11, t165);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008c8934); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 150; continue; }
            blk = 101; continue;
        }
        101 => {
            c.w64(15, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t166) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8945); };
            c.w32(12, t166);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x70008c894c); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(15)) { return RegfileExit::fault(0x70008c8950); }
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_b() { blk = 149; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t167) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c895d); };
            c.w32(15, t167);
            let r168 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r168);
            let Some(t169) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8964); };
            c.w32(12, t169);
            let r170 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r170);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x70008c896b); }
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 103; continue;
        }
        103 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 109; continue; }
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 109; continue; }
            blk = 105; continue;
        }
        105 => {
            blk = 433; continue;
        }
        106 => {
            blk = 109; continue;
        }
        107 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008c8988); }
            c.w64(3, c.r64(7));
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c898f); };
            c.w64(1, t171);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c8993); };
            c.w64(2, t172);
            c.w64(6, c.r64(8));
            return c.call(0x7000859a54, 0x70008c899f, 0x70008c899a);
        }
        108 => {
            c.w64(15, c.r64(0));
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c89a2); };
            c.w64(0, t173);
            blk = 152; continue;
        }
        109 => {
            let Some(t174) = c.ld32(c.r64(15).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008c89ab); };
            c.w32(12, t174);
            c.cmp(c.r32(12), 0x7eu64, 32);
            if c.cc_a() { blk = 353; continue; }
            blk = 110; continue;
        }
        110 => {
            c.logic(c.r8(11) & 0x7u64, 8);
            if c.cc_e() { blk = 126; continue; }
            blk = 111; continue;
        }
        111 => {
            c.w32(3, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(11));
            c.w64(11, c.r64(7));
            blk = 112; continue;
        }
        112 => {
            c.w64(6, c.r64(1));
            let r175 = c.r32(6) & 0x7u64;
            c.w32(6, r175);
            c.logic(r175, 32);
            c.cmp(c.r32(6), 0x2u64, 32);
            if c.cc_l() { blk = 117; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(6), 0x2u64, 32);
            if c.cc_e() { blk = 125; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 122; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(6), 0x5u64, 32);
            if c.cc_e() { blk = 123; continue; }
            blk = 116; continue;
        }
        116 => {
            blk = 342; continue;
        }
        117 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 124; continue; }
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r32(6), 0x1u64, 32);
            if c.cc_e() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            blk = 342; continue;
        }
        120 => {
            let Some(t176) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c89fd); };
            c.w32(1, t176);
            let Some(t177) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c8a01); };
            c.cmp(t177, c.r32(1), 32);
            if c.cc_ne() { blk = 343; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t178) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c8a0e); };
            c.w32(11, t178);
            let r179 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r179);
            let Some(t180) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c8a15); };
            c.w32(1, t180);
            let r181 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r181);
            let Some(t182) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8a1c); };
            c.w32(1, t182);
            blk = 112; continue;
        }
        122 => {
            let Some(t183) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c8a22); };
            c.w32(1, t183);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r184 = c.add(c.r64(3), c.r64(1), 64);
            c.w64(3, r184);
            let Some(t185) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c8a2f); };
            c.w32(11, t185);
            let r186 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r186);
            let Some(t187) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c8a36); };
            c.w32(1, t187);
            let r188 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r188);
            let Some(t189) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8a3d); };
            c.w32(1, t189);
            blk = 112; continue;
        }
        123 => {
            let Some(t190) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c8a43); };
            c.w32(11, t190);
            let r191 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r191);
            let Some(t192) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c8a4a); };
            c.w32(1, t192);
            let r193 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r193);
            let Some(t194) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8a51); };
            c.w32(1, t194);
            blk = 112; continue;
        }
        124 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(11));
            c.w64(11, c.r64(10));
            blk = 127; continue;
        }
        125 => {
            c.w32(6, 0x1u64);
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(11));
            c.w64(11, c.r64(10));
            blk = 127; continue;
        }
        126 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(3, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(7));
            blk = 127; continue;
        }
        127 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008c8a7e); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 344; continue; }
            blk = 128; continue;
        }
        128 => {
            let r195 = c.add(c.r64(1), 0xbu64, 64);
            c.w64(1, r195);
            blk = 129; continue;
        }
        129 => {
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c8a8f); };
            c.w64(6, t196);
            let r197 = c.add(c.r64(6), c.r64(3), 64);
            c.w64(6, r197);
            let r198 = c.r32(11) & 0x8u64;
            c.w32(11, r198);
            c.logic(r198, 32);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c8a9a); };
            c.w64(2, t199);
            let r200 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r200);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_ne() { blk = 346; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(11, c.r64(6).wrapping_add(c.r64(1)));
            let r201 = c.add(c.r64(3), c.r64(1), 64);
            c.w64(3, r201);
            let Some(t202) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c8ab2); };
            c.w32(1, t202);
            let r203 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r203);
            blk = 131; continue;
        }
        131 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x70008c8ab9); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 354; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t204) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008c8ac6); };
            c.w32(6, t204);
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x2e70)) else { return RegfileExit::fault(0x70008c8ac9); };
            c.w64(11, t205);
            let Some(t206) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c8ad0); };
            c.w64(11, t206);
            c.cmp(c.r32(6), 0xffffffffffffffffu64, 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 133; continue;
        }
        133 => {
            c.w32(1, c.r64(6).wrapping_mul(8));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(13));
            let Some(t207) = c.ld64(c.r64(6).wrapping_add(c.r64(1)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008c8ae5); };
            c.w64(1, t207);
            blk = 135; continue;
        }
        134 => {
            let r208 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r208);
            c.w64(6, c.r64(13));
            blk = 135; continue;
        }
        135 => {
            let r209 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r209);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w32(2, c.r32(2));
            c.w64(10, c.r64(4));
            let r210 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r210);
            let r211 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r211);
            c.logic(r211, 64);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x70008c8b0d); }
            let r212 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r212);
            let Some(t213) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008c8b15); };
            c.w64(15, t213);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(15)) { return RegfileExit::fault(0x70008c8b1c); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(12)) { return RegfileExit::fault(0x70008c8b21); }
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(11)) { return RegfileExit::fault(0x70008c8b26); }
            if !c.st64(c.r64(4).wrapping_add(0x38), 0x1u64) { return RegfileExit::fault(0x70008c8b2b); }
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(6)) { return RegfileExit::fault(0x70008c8b34); }
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(0)) { return RegfileExit::fault(0x70008c8b39); }
            c.w64(11, c.r64(1));
            c.w64(1, c.r64(7));
            c.w64(8, c.r64(3));
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c8b47); };
            c.w64(9, t214);
            c.w64(10, 0x70008c8b52u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008c8b52); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008c8b56); }
            return c.call(c.r64(11), 0x70008c8b5d, 0x70008c8b5a);
        }
        136 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008c8b5d); }
            let Some(t215) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x70008c8b65); };
            c.w64(4, t215);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_e() { blk = 139; continue; }
            blk = 137; continue;
        }
        137 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_ne() { blk = 347; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c8b7d); };
            c.w64(0, t216);
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c8b81); };
            c.w64(7, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c8b85); };
            c.w64(8, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c8b89); };
            c.w64(2, t219);
            blk = 151; continue;
        }
        139 => {
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c8b92); };
            c.w64(15, t220);
            let Some(t221) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c8b96); };
            c.w32(11, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c8b9a); };
            c.w64(12, t222);
            let Some(t223) = c.ld32(c.r64(12).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008c8b9e); };
            c.w32(12, t223);
            let r224 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r224);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r225 = c.sub(c.r32(11), 0x6u64, 32);
            c.w32(11, r225);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_l() { blk = 352; continue; }
            blk = 140; continue;
        }
        140 => {
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c8bba); }
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c8bbe); };
            c.w64(0, t226);
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008c8bc2); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 142; continue; }
            blk = 141; continue;
        }
        141 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r227 = c.r64(11) & c.r64(15);
            c.w64(11, r227);
            c.logic(r227, 64);
            let Some(t228) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c8bd4); };
            c.logic(t228 & 0x4u64, 8);
            if c.cc_ne() { blk = 440; continue; }
            blk = 142; continue;
        }
        142 => {
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008c8bdf); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 144; continue; }
            blk = 143; continue;
        }
        143 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r229 = c.r64(11) & c.r64(15);
            c.w64(11, r229);
            c.logic(r229, 64);
            let Some(t230) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c8bf1); };
            c.logic(t230 & 0x4u64, 8);
            if c.cc_ne() { blk = 437; continue; }
            blk = 144; continue;
        }
        144 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if c.cc_e() { blk = 152; continue; }
            blk = 145; continue;
        }
        145 => {
            c.w32(12, 0x13u64);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008c8c0c); };
            c.w64(2, t231);
            blk = 147; continue;
        }
        146 => {
            let r232 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r232);
            blk = 147; continue;
        }
        147 => {
            let Some(t233) = c.ld32(c.r64(2).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x70008c8c19); };
            c.w32(1, t233);
            let r234 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r234);
            if !c.st32(c.r64(15).wrapping_add(c.r64(12)), c.r32(1)) { return RegfileExit::fault(0x70008c8c1e); }
            let r235 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r235);
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_ne() { blk = 146; continue; }
            blk = 148; continue;
        }
        148 => {
            blk = 152; continue;
        }
        149 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008c8c2d); }
            blk = 151; continue;
        }
        150 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008c8c33); }
            blk = 151; continue;
        }
        151 => {
            let Some(t236) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c8c37); };
            c.w64(15, t236);
            c.w64(0, c.r64(7));
            blk = 152; continue;
        }
        152 => {
            let Some(t237) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c8c41); };
            c.cmp(t237, c.r32(15), 32);
            if c.cc_e() { blk = 317; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t238) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c8c4e); };
            c.cmp(t238, 0x0u64, 32);
            if c.cc_ne() { blk = 155; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c8c54); };
            c.w64(12, t239);
            blk = 157; continue;
        }
        155 => {
            let Some(t240) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c8c5a); };
            c.w32(11, t240);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4u64, 64);
            if c.cc_be() { blk = 320; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t241) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c8c6e); };
            c.w32(11, t241);
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c8c72); };
            c.w64(12, t242);
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008c8c76); }
            blk = 157; continue;
        }
        157 => {
            let Some(t243) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c8c7b); };
            c.w32(11, t243);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x3u64, 64);
            if c.cc_be() { blk = 321; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t244) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c8c8f); };
            c.w32(1, t244);
            let r245 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r245);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008c8c96); }
            c.cmp(c.r64(11), 0x4u64, 64);
            if c.cc_be() { blk = 322; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t246) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c8ca4); };
            c.w32(2, t246);
            let r247 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r247);
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c8cab); };
            c.w64(11, t248);
            c.w32(15, c.r64(11).wrapping_add(c.r64(11)));
            c.w32(15, c.r32(15));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008c8cb6); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008c8cba); }
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c8cbe); };
            c.w64(3, t249);
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c8cc2); };
            c.w64(6, t250);
            return c.call(0x700086fe54, 0x70008c8ccb, 0x70008c8cc6);
        }
        160 => {
            c.w64(3, c.r64(0));
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c8cce); };
            c.w64(0, t251);
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c8cd2); };
            c.w64(6, t252);
            return c.call(0x700086f9d4, 0x70008c8cdb, 0x70008c8cd6);
        }
        161 => {
            let Some(t253) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c8cdb); };
            c.cmp(t253, 0x0u64, 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c8ce1); };
            c.w64(6, t254);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008c8ce5); };
            c.w64(3, t255);
            return c.call(0x700086f9d4, 0x70008c8cee, 0x70008c8ce9);
        }
        163 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008c8cee); }
            let Some(t256) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c8cf2); };
            c.cmp(t256, 0x0u64, 32);
            if c.cc_e() { blk = 325; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c8cfc); };
            c.w64(11, t257);
            let Some(t258) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c8d00); };
            c.cmp(c.r32(11), t258, 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 165; continue;
        }
        165 => {
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c8d0a); };
            c.w64(11, t259);
            let Some(t260) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c8d0e); };
            c.w32(12, t260);
            let r261 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r261);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t262) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008c8d17); };
            c.w64(15, t262);
            let Some(t263) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c8d1e); };
            c.cmp(t263, c.r32(6), 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c8d28); };
            c.w64(3, t264);
            let Some(t265) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c8d2c); };
            c.w32(2, t265);
            let r266 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r266);
            let Some(t267) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c8d32); };
            c.w32(2, t267);
            let r268 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r268);
            let Some(t269) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c8d38); };
            c.w32(1, t269);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x9au64, 64);
            if c.cc_be() { blk = 323; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t270) = c.ld32(c.r64(2).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008c8d4e); };
            c.w32(7, t270);
            let r271 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r271);
            let Some(t272) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008c8d57); };
            c.cmp(t272, c.r32(12), 32);
            if c.cc_ne() { blk = 242; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t273) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c8d61); };
            c.w32(7, t273);
            let r274 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r274);
            c.w32(8, c.r32(7));
            c.logic(c.r32(7) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 242; continue; }
            blk = 169; continue;
        }
        169 => {
            c.cmp(c.r64(1), 0x9fu64, 64);
            if c.cc_be() { blk = 324; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t275) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c8d84); };
            c.w32(12, t275);
            let r276 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r276);
            let Some(t277) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c8d8c); };
            c.w32(12, t277);
            let r278 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r278);
            let Some(t279) = c.ld32(c.r64(12).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c8d94); };
            c.w32(1, t279);
            let r280 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r280);
            let Some(t281) = c.ld32(c.r64(2).wrapping_add(0x283)) else { return RegfileExit::fault(0x70008c8d9c); };
            c.cmp(t281, c.r32(12), 32);
            if c.cc_ne() { blk = 242; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t282) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008c8da9); };
            c.w32(12, t282);
            c.w32(12, c.r32(12));
            let r283 = c.r32(12) & 0x8u64;
            c.w32(12, r283);
            c.logic(r283, 32);
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_ne() { blk = 242; continue; }
            blk = 172; continue;
        }
        172 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 174; continue; }
            blk = 173; continue;
        }
        173 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c8dc4); }
            blk = 177; continue;
        }
        174 => {
            let r284 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r284);
            if c.cc_o() { blk = 176; continue; }
            blk = 175; continue;
        }
        175 => {
            c.w64(12, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 180; continue;
        }
        176 => {
            c.w64(12, c.r64(7));
            let r285 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r285);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 177; continue;
        }
        177 => {
            let Some(t286) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c8de5); };
            c.w64(12, t286);
            let Some(t287) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008c8dec); };
            c.w64(2, t287);
            c.w64(1, c.r64(2).wrapping_add(0xc));
            let Some(t288) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c8df4); };
            c.w64(6, t288);
            let Some(t289) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008c8dfb); };
            c.cmp(t289, c.r64(1), 64);
            if c.cc_be() { blk = 356; continue; }
            blk = 178; continue;
        }
        178 => {
            if !c.st64(c.r64(12), c.r64(1)) { return RegfileExit::fault(0x70008c8e04); }
            c.w64(12, c.r64(2).wrapping_add(0x1));
            blk = 179; continue;
        }
        179 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t290) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c8e1e); };
            c.w64(2, t290);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008c8e25); }
            if !c.movsd_mr(c.r64(12).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008c8e2a); }
            blk = 180; continue;
        }
        180 => {
            let Some(t291) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c8e31); };
            c.cmp(t291, 0x0u64, 32);
            if c.cc_ne() { blk = 182; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c8e37); };
            c.w64(8, t292);
            blk = 240; continue;
        }
        182 => {
            c.w32(2, c.r32(12));
            c.logic(c.r32(12) & 0x80000001u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c8e4c); };
            c.w64(8, t293);
            blk = 240; continue;
        }
        184 => {
            c.w64(1, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c8e58); };
            c.w64(8, t294);
            let Some(t295) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8e5c); };
            c.w32(6, t295);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), c.r64(6), 64);
            if c.cc_ae() { blk = 240; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t296) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c8e6c); };
            c.w32(6, t296);
            let r297 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r297);
            let Some(t298) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8e73); };
            c.w32(6, t298);
            c.logic(c.r8(6) & 0x7u64, 8);
            if c.cc_e() { blk = 202; continue; }
            blk = 186; continue;
        }
        186 => {
            c.w64(15, c.r64(8));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x70008c8e84); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(6));
            blk = 188; continue;
        }
        187 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008c8e8f); }
            blk = 188; continue;
        }
        188 => {
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c8e93); };
            c.w64(0, t299);
            let r300 = c.r32(0) & 0x7u64;
            c.w32(0, r300);
            c.logic(r300, 32);
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_l() { blk = 193; continue; }
            blk = 189; continue;
        }
        189 => {
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 190; continue;
        }
        190 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 191; continue;
        }
        191 => {
            c.cmp(c.r32(0), 0x5u64, 32);
            if c.cc_e() { blk = 199; continue; }
            blk = 192; continue;
        }
        192 => {
            blk = 358; continue;
        }
        193 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 200; continue; }
            blk = 194; continue;
        }
        194 => {
            c.cmp(c.r32(0), 0x1u64, 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 195; continue;
        }
        195 => {
            blk = 358; continue;
        }
        196 => {
            let Some(t301) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c8ec6); };
            c.w32(0, t301);
            let Some(t302) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c8eca); };
            c.cmp(t302, c.r32(0), 32);
            if c.cc_ne() { blk = 359; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t303) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c8ed7); };
            c.w32(15, t303);
            let r304 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r304);
            let Some(t305) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c8ede); };
            c.w32(0, t305);
            let r306 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r306);
            let Some(t307) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8ee5); };
            c.w32(0, t307);
            blk = 187; continue;
        }
        198 => {
            let Some(t308) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c8eeb); };
            c.w32(0, t308);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r309 = c.add(c.r64(6), c.r64(0), 64);
            c.w64(6, r309);
            let Some(t310) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c8ef8); };
            c.w32(15, t310);
            let r311 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r311);
            let Some(t312) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c8eff); };
            c.w32(0, t312);
            let r313 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r313);
            let Some(t314) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8f06); };
            c.w32(0, t314);
            blk = 187; continue;
        }
        199 => {
            let Some(t315) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c8f0c); };
            c.w32(15, t315);
            let r316 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r316);
            let Some(t317) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c8f13); };
            c.w32(0, t317);
            let r318 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r318);
            let Some(t319) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8f1a); };
            c.w32(0, t319);
            blk = 187; continue;
        }
        200 => {
            c.w64(0, c.r64(15));
            c.w64(15, c.r64(6));
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 203; continue;
        }
        201 => {
            c.w64(0, c.r64(15));
            c.w64(15, c.r64(6));
            c.w32(6, 0x1u64);
            blk = 203; continue;
        }
        202 => {
            c.w32(9, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(8));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x70008c8f40); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(9));
            blk = 203; continue;
        }
        203 => {
            c.w64(3, (c.r32(7) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 360; continue; }
            blk = 204; continue;
        }
        204 => {
            let r320 = c.add(c.r64(0), 0xbu64, 64);
            c.w64(0, r320);
            blk = 205; continue;
        }
        205 => {
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t321) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c8f5c); };
            c.w64(6, t321);
            let r322 = c.r32(6) & 0x8u64;
            c.w32(6, r322);
            c.logic(r322, 32);
            let r323 = c.add(c.r64(15), c.r64(3), 64);
            c.w64(15, r323);
            c.cmp(c.r32(6), 0x8u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 206; continue;
        }
        206 => {
            let Some(t324) = c.ld16(c.r64(0).wrapping_add(c.r64(15).wrapping_mul(2))) else { return RegfileExit::fault(0x70008c8f6b); };
            c.w32(15, t324);
            blk = 208; continue;
        }
        207 => {
            let Some(t325) = c.ld8(c.r64(15).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70008c8f72); };
            c.w32(15, t325);
            blk = 208; continue;
        }
        208 => {
            let r326 = c.r32(15) & 0xfc00u64;
            c.w32(15, r326);
            c.logic(r326, 32);
            c.cmp(c.r32(15), 0xd800u64, 32);
            if c.cc_ne() { blk = 240; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t327) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c8f8b); };
            c.w32(15, t327);
            let r328 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r328);
            let Some(t329) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8f92); };
            c.w32(15, t329);
            c.logic(c.r8(15) & 0x7u64, 8);
            if c.cc_e() { blk = 225; continue; }
            blk = 210; continue;
        }
        210 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(8));
            c.w64(3, c.r64(15));
            c.w64(15, c.r64(6));
            blk = 211; continue;
        }
        211 => {
            c.w64(7, c.r64(3));
            let r330 = c.r32(7) & 0x7u64;
            c.w32(7, r330);
            c.logic(r330, 32);
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_l() { blk = 216; continue; }
            blk = 212; continue;
        }
        212 => {
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_e() { blk = 224; continue; }
            blk = 213; continue;
        }
        213 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 221; continue; }
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 215; continue;
        }
        215 => {
            blk = 376; continue;
        }
        216 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 223; continue; }
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r32(7), 0x1u64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 218; continue;
        }
        218 => {
            blk = 376; continue;
        }
        219 => {
            let Some(t331) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c8fde); };
            c.w32(3, t331);
            let Some(t332) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c8fe1); };
            c.cmp(t332, c.r32(3), 32);
            if c.cc_ne() { blk = 377; continue; }
            blk = 220; continue;
        }
        220 => {
            let Some(t333) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c8fee); };
            c.w32(0, t333);
            let r334 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r334);
            let Some(t335) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c8ff4); };
            c.w32(3, t335);
            let r336 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r336);
            let Some(t337) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c8ffa); };
            c.w32(3, t337);
            blk = 211; continue;
        }
        221 => {
            let Some(t338) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c9000); };
            c.w32(3, t338);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r339 = c.add(c.r64(15), c.r64(3), 64);
            c.w64(15, r339);
            let Some(t340) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c900c); };
            c.w32(0, t340);
            let r341 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r341);
            let Some(t342) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c9012); };
            c.w32(3, t342);
            let r343 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r343);
            let Some(t344) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c9018); };
            c.w32(3, t344);
            blk = 211; continue;
        }
        222 => {
            let Some(t345) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c901e); };
            c.w32(0, t345);
            let r346 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r346);
            let Some(t347) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c9024); };
            c.w32(3, t347);
            let r348 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r348);
            let Some(t349) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c902a); };
            c.w32(3, t349);
            blk = 211; continue;
        }
        223 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(15));
            c.w64(15, c.r64(10));
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(10));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 226; continue;
        }
        224 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(15));
            c.w64(15, c.r64(10));
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(10));
            c.w32(7, 0x1u64);
            blk = 226; continue;
        }
        225 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(8));
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(0));
            blk = 226; continue;
        }
        226 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 378; continue; }
            blk = 227; continue;
        }
        227 => {
            let r350 = c.add(c.r64(3), 0xbu64, 64);
            c.w64(3, r350);
            blk = 228; continue;
        }
        228 => {
            let r351 = c.add(c.r64(0), c.r64(1), 64);
            c.w64(0, r351);
            let r352 = c.r32(15) & 0x8u64;
            c.w32(15, r352);
            c.logic(r352, 32);
            c.cmp(c.r32(15), 0x8u64, 32);
            if c.cc_e() { blk = 230; continue; }
            blk = 229; continue;
        }
        229 => {
            let Some(t353) = c.ld16(c.r64(3).wrapping_add(c.r64(0).wrapping_mul(2))) else { return RegfileExit::fault(0x70008c9086); };
            c.w32(15, t353);
            blk = 231; continue;
        }
        230 => {
            let Some(t354) = c.ld8(c.r64(0).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x70008c908d); };
            c.w32(15, t354);
            blk = 231; continue;
        }
        231 => {
            let r355 = c.r32(15) & 0xfc00u64;
            c.w32(15, r355);
            c.logic(r355, 32);
            c.cmp(c.r32(15), 0xdc00u64, 32);
            if c.cc_ne() { blk = 240; continue; }
            blk = 232; continue;
        }
        232 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 234; continue; }
            blk = 233; continue;
        }
        233 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c90a8); }
            blk = 237; continue;
        }
        234 => {
            let r356 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r356);
            if c.cc_o() { blk = 236; continue; }
            blk = 235; continue;
        }
        235 => {
            c.w64(12, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 240; continue;
        }
        236 => {
            let r357 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r357);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 237; continue;
        }
        237 => {
            let Some(t358) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c90c7); };
            c.w64(12, t358);
            let Some(t359) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008c90ce); };
            c.w64(15, t359);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t360) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c90d6); };
            c.w64(3, t360);
            let Some(t361) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c90dd); };
            c.cmp(t361, c.r64(0), 64);
            if c.cc_be() { blk = 393; continue; }
            blk = 238; continue;
        }
        238 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008c90e6); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 239; continue;
        }
        239 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t362) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c9100); };
            c.w64(15, t362);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008c9107); }
            if !c.movsd_mr(c.r64(12).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008c910c); }
            blk = 240; continue;
        }
        240 => {
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x70008c9113); }
            blk = 315; continue;
        }
        241 => {
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c911c); };
            c.w64(3, t363);
            blk = 242; continue;
        }
        242 => {
            c.w64(6, c.r64(3));
            c.w64(0, c.r64(11));
            let Some(t364) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008c9126); };
            c.w64(3, t364);
            c.w64(12, c.r64(3));
            return c.call(0x7000870914, 0x70008c9135, 0x70008c9130);
        }
        243 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 246; continue; }
            blk = 244; continue;
        }
        244 => {
            let Some(t365) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c9139); };
            c.w64(6, t365);
            return c.call(0x70008a17f4, 0x70008c9142, 0x70008c913d);
        }
        245 => {
            blk = 248; continue;
        }
        246 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 248; continue; }
            blk = 247; continue;
        }
        247 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 248; continue;
        }
        248 => {
            c.w32(11, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 250; continue; }
            blk = 249; continue;
        }
        249 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c9152); }
            blk = 253; continue;
        }
        250 => {
            let r366 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r366);
            if c.cc_o() { blk = 252; continue; }
            blk = 251; continue;
        }
        251 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 256; continue;
        }
        252 => {
            c.w64(11, c.r64(0));
            let r367 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r367);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 253; continue;
        }
        253 => {
            let Some(t368) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c9173); };
            c.w64(11, t368);
            let Some(t369) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c917a); };
            c.w64(12, t369);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t370) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c9182); };
            c.w64(3, t370);
            let Some(t371) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c9189); };
            c.cmp(t371, c.r64(15), 64);
            if c.cc_be() { blk = 395; continue; }
            blk = 254; continue;
        }
        254 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008c9192); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 255; continue;
        }
        255 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t372) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c91ac); };
            c.w64(12, t372);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c91b3); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008c91b7); }
            blk = 256; continue;
        }
        256 => {
            let Some(t373) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c91bd); };
            c.cmp(t373, 0x0u64, 32);
            if c.cc_e() { blk = 313; continue; }
            blk = 257; continue;
        }
        257 => {
            c.w32(12, c.r32(11));
            c.logic(c.r32(11) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 313; continue; }
            blk = 258; continue;
        }
        258 => {
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t374) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c91da); };
            c.w64(3, t374);
            let Some(t375) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c91de); };
            c.w32(2, t375);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), c.r64(2), 64);
            if c.cc_ae() { blk = 313; continue; }
            blk = 259; continue;
        }
        259 => {
            let Some(t376) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c91ed); };
            c.w32(2, t376);
            let r377 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r377);
            let Some(t378) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c91f3); };
            c.w32(2, t378);
            c.logic(c.r8(2) & 0x7u64, 8);
            if c.cc_e() { blk = 275; continue; }
            blk = 260; continue;
        }
        260 => {
            c.w64(6, c.r64(2));
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(3));
            blk = 261; continue;
        }
        261 => {
            c.w64(7, c.r64(6));
            let r379 = c.r32(7) & 0x7u64;
            c.w32(7, r379);
            c.logic(r379, 32);
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_l() { blk = 266; continue; }
            blk = 262; continue;
        }
        262 => {
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_e() { blk = 274; continue; }
            blk = 263; continue;
        }
        263 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 271; continue; }
            blk = 264; continue;
        }
        264 => {
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_e() { blk = 272; continue; }
            blk = 265; continue;
        }
        265 => {
            blk = 397; continue;
        }
        266 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 273; continue; }
            blk = 267; continue;
        }
        267 => {
            c.cmp(c.r32(7), 0x1u64, 32);
            if c.cc_e() { blk = 269; continue; }
            blk = 268; continue;
        }
        268 => {
            blk = 397; continue;
        }
        269 => {
            let Some(t380) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c923a); };
            c.w32(6, t380);
            let Some(t381) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c923d); };
            c.cmp(t381, c.r32(6), 32);
            if c.cc_ne() { blk = 398; continue; }
            blk = 270; continue;
        }
        270 => {
            let Some(t382) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c924a); };
            c.w32(1, t382);
            let r383 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r383);
            let Some(t384) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c9250); };
            c.w32(6, t384);
            let r385 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r385);
            let Some(t386) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c9256); };
            c.w32(6, t386);
            blk = 261; continue;
        }
        271 => {
            let Some(t387) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c925c); };
            c.w32(6, t387);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r388 = c.add(c.r64(2), c.r64(6), 64);
            c.w64(2, r388);
            let Some(t389) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c9268); };
            c.w32(1, t389);
            let r390 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r390);
            let Some(t391) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c926e); };
            c.w32(6, t391);
            let r392 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r392);
            let Some(t393) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c9274); };
            c.w32(6, t393);
            blk = 261; continue;
        }
        272 => {
            let Some(t394) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c927a); };
            c.w32(1, t394);
            let r395 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r395);
            let Some(t396) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c9280); };
            c.w32(6, t396);
            let r397 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r397);
            let Some(t398) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c9286); };
            c.w32(6, t398);
            blk = 261; continue;
        }
        273 => {
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(2));
            c.w64(2, c.r64(10));
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(6));
            c.w64(6, c.r64(10));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 276; continue;
        }
        274 => {
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(2));
            c.w64(2, c.r64(10));
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(6));
            c.w64(6, c.r64(10));
            c.w32(7, 0x1u64);
            blk = 276; continue;
        }
        275 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(3));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 276; continue;
        }
        276 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 399; continue; }
            blk = 277; continue;
        }
        277 => {
            c.w64(0, c.r64(6).wrapping_add(0xb));
            blk = 278; continue;
        }
        278 => {
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r399 = c.r32(2) & 0x8u64;
            c.w32(2, r399);
            c.logic(r399, 32);
            let r400 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r400);
            c.cmp(c.r32(2), 0x8u64, 32);
            if c.cc_e() { blk = 280; continue; }
            blk = 279; continue;
        }
        279 => {
            let Some(t401) = c.ld16(c.r64(0).wrapping_add(c.r64(1).wrapping_mul(2))) else { return RegfileExit::fault(0x70008c92e3); };
            c.w32(0, t401);
            blk = 281; continue;
        }
        280 => {
            let Some(t402) = c.ld8(c.r64(1).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70008c92e9); };
            c.w32(0, t402);
            blk = 281; continue;
        }
        281 => {
            let r403 = c.r32(0) & 0xfc00u64;
            c.w32(0, r403);
            c.logic(r403, 32);
            c.cmp(c.r32(0), 0xd800u64, 32);
            if c.cc_ne() { blk = 313; continue; }
            blk = 282; continue;
        }
        282 => {
            let Some(t404) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c92fd); };
            c.w32(0, t404);
            let r405 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r405);
            let Some(t406) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c9303); };
            c.w32(0, t406);
            c.logic(c.r8(0) & 0x7u64, 8);
            if c.cc_e() { blk = 298; continue; }
            blk = 283; continue;
        }
        283 => {
            c.w64(1, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(3));
            blk = 284; continue;
        }
        284 => {
            c.w64(6, c.r64(1));
            let r407 = c.r32(6) & 0x7u64;
            c.w32(6, r407);
            c.logic(r407, 32);
            c.cmp(c.r32(6), 0x2u64, 32);
            if c.cc_l() { blk = 289; continue; }
            blk = 285; continue;
        }
        285 => {
            c.cmp(c.r32(6), 0x2u64, 32);
            if c.cc_e() { blk = 297; continue; }
            blk = 286; continue;
        }
        286 => {
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 294; continue; }
            blk = 287; continue;
        }
        287 => {
            c.cmp(c.r32(6), 0x5u64, 32);
            if c.cc_e() { blk = 295; continue; }
            blk = 288; continue;
        }
        288 => {
            blk = 414; continue;
        }
        289 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 296; continue; }
            blk = 290; continue;
        }
        290 => {
            c.cmp(c.r32(6), 0x1u64, 32);
            if c.cc_e() { blk = 292; continue; }
            blk = 291; continue;
        }
        291 => {
            blk = 414; continue;
        }
        292 => {
            let Some(t408) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c9349); };
            c.w32(1, t408);
            let Some(t409) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c934c); };
            c.cmp(t409, c.r32(1), 32);
            if c.cc_ne() { blk = 415; continue; }
            blk = 293; continue;
        }
        293 => {
            let Some(t410) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c9359); };
            c.w32(2, t410);
            let r411 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r411);
            let Some(t412) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c935f); };
            c.w32(1, t412);
            let r413 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r413);
            let Some(t414) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c9365); };
            c.w32(1, t414);
            blk = 284; continue;
        }
        294 => {
            let Some(t415) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c936b); };
            c.w32(1, t415);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r416 = c.add(c.r64(0), c.r64(1), 64);
            c.w64(0, r416);
            let Some(t417) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c9377); };
            c.w32(2, t417);
            let r418 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r418);
            let Some(t419) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c937d); };
            c.w32(1, t419);
            let r420 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r420);
            let Some(t421) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c9383); };
            c.w32(1, t421);
            blk = 284; continue;
        }
        295 => {
            let Some(t422) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c9389); };
            c.w32(2, t422);
            let r423 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r423);
            let Some(t424) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c938f); };
            c.w32(1, t424);
            let r425 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r425);
            let Some(t426) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c9395); };
            c.w32(1, t426);
            blk = 284; continue;
        }
        296 => {
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(10));
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 299; continue;
        }
        297 => {
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(10));
            c.w32(6, 0x1u64);
            blk = 299; continue;
        }
        298 => {
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(3));
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 299; continue;
        }
        299 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 416; continue; }
            blk = 300; continue;
        }
        300 => {
            let r427 = c.add(c.r64(1), 0xbu64, 64);
            c.w64(1, r427);
            blk = 301; continue;
        }
        301 => {
            let r428 = c.add(c.r64(15), c.r64(2), 64);
            c.w64(15, r428);
            let r429 = c.r32(0) & 0x8u64;
            c.w32(0, r429);
            c.logic(r429, 32);
            c.cmp(c.r32(0), 0x8u64, 32);
            if c.cc_e() { blk = 303; continue; }
            blk = 302; continue;
        }
        302 => {
            let Some(t430) = c.ld16(c.r64(1).wrapping_add(c.r64(15).wrapping_mul(2))) else { return RegfileExit::fault(0x70008c93ec); };
            c.w32(15, t430);
            blk = 304; continue;
        }
        303 => {
            let Some(t431) = c.ld8(c.r64(15).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008c93f3); };
            c.w32(15, t431);
            blk = 304; continue;
        }
        304 => {
            let r432 = c.r32(15) & 0xfc00u64;
            c.w32(15, r432);
            c.logic(r432, 32);
            c.cmp(c.r32(15), 0xdc00u64, 32);
            if c.cc_ne() { blk = 313; continue; }
            blk = 305; continue;
        }
        305 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 307; continue; }
            blk = 306; continue;
        }
        306 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c940e); }
            blk = 310; continue;
        }
        307 => {
            let r433 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r433);
            if c.cc_o() { blk = 309; continue; }
            blk = 308; continue;
        }
        308 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 313; continue;
        }
        309 => {
            let r434 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r434);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 310; continue;
        }
        310 => {
            let Some(t435) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c942d); };
            c.w64(11, t435);
            let Some(t436) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c9434); };
            c.w64(12, t436);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t437) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c943c); };
            c.w64(0, t437);
            let Some(t438) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008c9443); };
            c.cmp(t438, c.r64(15), 64);
            if c.cc_be() { blk = 431; continue; }
            blk = 311; continue;
        }
        311 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008c944c); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 312; continue;
        }
        312 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t439) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c9466); };
            c.w64(12, t439);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c946d); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008c9471); }
            blk = 313; continue;
        }
        313 => {
            let Some(t440) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c9477); };
            c.w64(0, t440);
            let Some(t441) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008c947b); };
            c.w64(3, t441);
            c.w64(1, c.r64(11));
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c9485); };
            c.w64(6, t442);
            return c.call(0x7000871cb4, 0x70008c948e, 0x70008c9489);
        }
        314 => {
            let Some(t443) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c948e); };
            c.w64(11, t443);
            let Some(t444) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c9492); };
            c.w64(8, t444);
            blk = 315; continue;
        }
        315 => {
            let Some(t445) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c9496); };
            c.w64(0, t445);
            blk = 83; continue;
        }
        316 => {
            return RegfileExit::fault(0x70008c949f);
        }
        317 => {
            let Some(t446) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c94a0); };
            c.cmp(t446, 0x0u64, 32);
            if c.cc_e() { blk = 319; continue; }
            blk = 318; continue;
        }
        318 => {
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t447) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c94a9); };
            c.w64(12, t447);
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008c94ad); }
            blk = 319; continue;
        }
        319 => {
            c.w64(7, c.r64(0));
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c94b5); };
            c.w64(8, t448);
            let Some(t449) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c94b9); };
            c.w64(2, t449);
            blk = 327; continue;
        }
        320 => {
            return RegfileExit::fault(0x70008c94bf);
        }
        321 => {
            return RegfileExit::fault(0x70008c94c0);
        }
        322 => {
            return RegfileExit::fault(0x70008c94c1);
        }
        323 => {
            return RegfileExit::fault(0x70008c94c2);
        }
        324 => {
            return RegfileExit::fault(0x70008c94c3);
        }
        325 => {
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c94c4); };
            c.w64(8, t450);
            let Some(t451) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c94c8); };
            c.w64(11, t451);
            c.w64(2, c.r64(0));
            let Some(t452) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c94cf); };
            c.w64(7, t452);
            blk = 328; continue;
        }
        326 => {
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008c94d8); }
            blk = 327; continue;
        }
        327 => {
            c.w64(11, c.r64(2));
            let Some(t453) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c94df); };
            c.w64(2, t453);
            blk = 328; continue;
        }
        328 => {
            c.w64(12, c.r64(7));
            let Some(t454) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c94e6); };
            c.w32(15, t454);
            c.w32(1, c.r64(15).wrapping_add(c.r64(15)));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008c94ef); }
            c.w64(0, c.r64(12));
            c.w64(3, c.r64(11));
            c.w64(6, c.r64(8));
            return c.call(0x700086fe54, 0x70008c9501, 0x70008c94fc);
        }
        329 => {
            c.w64(3, c.r64(0));
            let Some(t455) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c9504); };
            c.w64(0, t455);
            let Some(t456) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c9508); };
            c.w64(6, t456);
            return c.call(0x700086f9d4, 0x70008c9511, 0x70008c950c);
        }
        330 => {
            blk = 5; continue;
        }
        331 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c9516); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008c9517); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008c9518); }
            let Some(t457) = c.ld64(c.r64(13).wrapping_add(0x2678)) else { return RegfileExit::fault(0x70008c951a); };
            c.w64(3, t457);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008c952b, 0x70008c9526);
        }
        332 => {
            blk = 5; continue;
        }
        333 => {
            c.w32(2, 0x58u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008c9535); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008c9539); }
            return c.call(0x70007febf4, 0x70008c9542, 0x70008c953d);
        }
        334 => {
            c.w64(12, c.r64(0));
            let Some(t458) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c9545); };
            c.w64(0, t458);
            let Some(t459) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c9549); };
            c.w64(3, t459);
            let Some(t460) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c954d); };
            c.w64(6, t460);
            let Some(t461) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008c9551); };
            c.w64(1, t461);
            let Some(t462) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c9555); };
            c.w64(15, t462);
            let Some(t463) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c9559); };
            c.w64(11, t463);
            blk = 12; continue;
        }
        335 => {
            let Some(t464) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c9562); };
            c.w64(6, t464);
            return c.call(0x700089c794, 0x70008c956b, 0x70008c9566);
        }
        336 => {
            let Some(t465) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c956b); };
            c.w64(15, t465);
            let Some(t466) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c956f); };
            c.w64(11, t466);
            blk = 33; continue;
        }
        337 => {
            let Some(t467) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c9578); };
            c.w64(6, t467);
            return c.call(0x700089c794, 0x70008c9581, 0x70008c957c);
        }
        338 => {
            blk = 53; continue;
        }
        339 => {
            c.w64(12, c.r64(1));
            blk = 340; continue;
        }
        340 => {
            c.w64(0, c.r64(12));
            return c.call(0x700089c794, 0x70008c9591, 0x70008c958c);
        }
        341 => {
            c.w64(12, c.r64(0));
            let Some(t468) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c9594); };
            c.w64(0, t468);
            let Some(t469) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c9598); };
            c.w64(3, t469);
            let Some(t470) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c959c); };
            c.w64(6, t470);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 68; continue;
        }
        342 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008c95a8); }
            blk = 354; continue;
        }
        343 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008c95b1); }
            blk = 354; continue;
        }
        344 => {
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 354; continue; }
            blk = 345; continue;
        }
        345 => {
            let Some(t471) = c.ld64(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c95c4); };
            c.w64(1, t471);
            blk = 129; continue;
        }
        346 => {
            c.w64(11, c.r64(1).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(3, c.r64(1).wrapping_add(c.r64(3).wrapping_mul(2)));
            let Some(t472) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c95d5); };
            c.w32(1, t472);
            let r473 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r473);
            blk = 131; continue;
        }
        347 => {
            c.cmp(c.r64(11), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 434; continue; }
            blk = 348; continue;
        }
        348 => {
            c.cmp(c.r64(11), 0xfffffffffffffffdu64, 64);
            if c.cc_e() { blk = 350; continue; }
            blk = 349; continue;
        }
        349 => {
            let Some(t474) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c95f1); };
            c.w64(0, t474);
            let Some(t475) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c95f5); };
            c.w64(7, t475);
            let Some(t476) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c95f9); };
            c.w64(8, t476);
            blk = 354; continue;
        }
        350 => {
            let Some(t477) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c95ff); };
            if !c.push64(t477) { return RegfileExit::fault(0x70008c95ff); }
            let Some(t478) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c9602); };
            if !c.push64(t478) { return RegfileExit::fault(0x70008c9602); }
            let Some(t479) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c9605); };
            if !c.push64(t479) { return RegfileExit::fault(0x70008c9605); }
            let Some(t480) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c9608); };
            if !c.push64(t480) { return RegfileExit::fault(0x70008c9608); }
            let Some(t481) = c.ld64(c.r64(13).wrapping_add(0x2658)) else { return RegfileExit::fault(0x70008c960b); };
            c.w64(3, t481);
            c.w32(0, 0x4u64);
            let Some(t482) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c9617); };
            c.w64(6, t482);
            return c.call(0x700086ed74, 0x70008c9620, 0x70008c961b);
        }
        351 => {
            c.w64(15, c.r64(0));
            let Some(t483) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c9623); };
            c.w64(0, t483);
            blk = 152; continue;
        }
        352 => {
            let Some(t484) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c962c); };
            c.w64(0, t484);
            let Some(t485) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c9630); };
            c.w64(7, t485);
            let Some(t486) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c9634); };
            c.w64(8, t486);
            blk = 354; continue;
        }
        353 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008c963a); }
            blk = 354; continue;
        }
        354 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c963e); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c963f); }
            let Some(t487) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c9640); };
            if !c.push64(t487) { return RegfileExit::fault(0x70008c9640); }
            let Some(t488) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c9643); };
            if !c.push64(t488) { return RegfileExit::fault(0x70008c9643); }
            let Some(t489) = c.ld64(c.r64(13).wrapping_add(0x2648)) else { return RegfileExit::fault(0x70008c9646); };
            c.w64(3, t489);
            c.w64(6, c.r64(8));
            c.w32(0, 0x4u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008c965d, 0x70008c9658);
        }
        355 => {
            c.w64(15, c.r64(0));
            let Some(t490) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c9660); };
            c.w64(0, t490);
            blk = 152; continue;
        }
        356 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008c966e); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008c9673); }
            return c.call(0x70007febf4, 0x70008c967c, 0x70008c9677);
        }
        357 => {
            c.w64(12, c.r64(0));
            let Some(t491) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c967f); };
            c.w64(11, t491);
            let Some(t492) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c9683); };
            c.w64(3, t492);
            let Some(t493) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c9687); };
            c.w64(0, t493);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008c968b); }
            let Some(t494) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c9690); };
            c.w64(7, t494);
            let Some(t495) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008c9694); };
            c.w64(15, t495);
            blk = 179; continue;
        }
        358 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008c96a0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008c96a4); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008c96a8); }
            blk = 363; continue;
        }
        359 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008c96ae); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008c96b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008c96b6); }
            blk = 363; continue;
        }
        360 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008c96bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008c96c0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008c96c4); }
            let Some(t496) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c96c8); };
            c.w64(6, t496);
            c.logic(c.r8(6) & 0x10u64, 8);
            if c.cc_ne() { blk = 362; continue; }
            blk = 361; continue;
        }
        361 => {
            let Some(t497) = c.ld64(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c96d2); };
            c.w64(0, t497);
            blk = 205; continue;
        }
        362 => {
            let Some(t498) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c96db); };
            c.w64(3, t498);
            blk = 363; continue;
        }
        363 => {
            c.w64(15, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x70008c96e5); }
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 365; continue; }
            blk = 364; continue;
        }
        364 => {
            let r499 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r499);
            blk = 374; continue;
        }
        365 => {
            let Some(t500) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c96fa); };
            c.w64(0, t500);
            let Some(t501) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008c9701); };
            c.w64(6, t501);
            c.w64(7, c.r64(6).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(6)) { return RegfileExit::fault(0x70008c9708); }
            let Some(t502) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c970c); };
            c.w64(6, t502);
            let Some(t503) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008c9713); };
            c.cmp(t503, c.r64(7), 64);
            if c.cc_a() { blk = 368; continue; }
            blk = 366; continue;
        }
        366 => {
            c.w32(0, 0xcu64);
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x70008c9725, 0x70008c9720);
        }
        367 => {
            let Some(t504) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c9725); };
            c.w64(15, t504);
            let Some(t505) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c9729); };
            c.w64(11, t505);
            let Some(t506) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c972d); };
            c.w64(8, t506);
            let Some(t507) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c9731); };
            c.w64(3, t507);
            let Some(t508) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c9735); };
            c.w64(12, t508);
            let Some(t509) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c9739); };
            c.w64(2, t509);
            let Some(t510) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c973d); };
            c.w64(1, t510);
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 369; continue;
        }
        368 => {
            if !c.st64(c.r64(0), c.r64(7)) { return RegfileExit::fault(0x70008c9746); }
            let Some(t511) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c9749); };
            c.w64(0, t511);
            let r512 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r512);
            blk = 369; continue;
        }
        369 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(15) as i64));
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 373; continue; }
            blk = 370; continue;
        }
        370 => {
            c.w64(10, c.r64(15));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 372; continue; }
            blk = 371; continue;
        }
        371 => {
            let r513 = c.r64(10) | 0x1u64;
            c.w64(10, r513);
            c.logic(r513, 64);
            blk = 372; continue;
        }
        372 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 373; continue;
        }
        373 => {
            let Some(t514) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c9778); };
            c.w64(15, t514);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008c977f); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008c9783); }
            c.w64(15, c.r64(0));
            blk = 374; continue;
        }
        374 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c978b); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008c978d); }
            c.w64(6, c.r64(9));
            c.w32(0, 0x2u64);
            let Some(t515) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008c9797); };
            c.w64(3, t515);
            c.w64(15, c.r64(3));
            return c.call(0x700086ed74, 0x70008c97a6, 0x70008c97a1);
        }
        375 => {
            let r516 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r516);
            c.w64(15, c.r64(0));
            let Some(t517) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c97ab); };
            c.w64(11, t517);
            let Some(t518) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c97af); };
            c.w64(8, t518);
            let Some(t519) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c97b3); };
            c.w64(12, t519);
            let Some(t520) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c97b7); };
            c.w64(2, t520);
            let Some(t521) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c97bb); };
            c.w64(1, t521);
            blk = 208; continue;
        }
        376 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008c97c4); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008c97c8); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008c97cc); }
            blk = 380; continue;
        }
        377 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008c97d2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008c97d6); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008c97da); }
            blk = 380; continue;
        }
        378 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008c97e0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008c97e4); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008c97e8); }
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 380; continue; }
            blk = 379; continue;
        }
        379 => {
            let Some(t522) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c97f2); };
            c.w64(3, t522);
            blk = 228; continue;
        }
        380 => {
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 382; continue; }
            blk = 381; continue;
        }
        381 => {
            c.w32(15, c.r64(1).wrapping_add(c.r64(1)));
            blk = 391; continue;
        }
        382 => {
            let Some(t523) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c980d); };
            c.w64(15, t523);
            let Some(t524) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008c9814); };
            c.w64(0, t524);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t525) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c981b); };
            c.w64(7, t525);
            let Some(t526) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008c9822); };
            c.cmp(t526, c.r64(3), 64);
            if c.cc_a() { blk = 385; continue; }
            blk = 383; continue;
        }
        383 => {
            c.w32(2, 0xcu64);
            c.w64(15, c.r64(2));
            return c.call(0x70007febf4, 0x70008c9834, 0x70008c982f);
        }
        384 => {
            let Some(t527) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c9834); };
            c.w64(1, t527);
            let Some(t528) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c9838); };
            c.w64(11, t528);
            let Some(t529) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c983c); };
            c.w64(8, t529);
            let Some(t530) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c9840); };
            c.w64(12, t530);
            let Some(t531) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c9844); };
            c.w64(2, t531);
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 386; continue;
        }
        385 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x70008c984c); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(15));
            blk = 386; continue;
        }
        386 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(1) as i64));
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 390; continue; }
            blk = 387; continue;
        }
        387 => {
            c.w64(10, c.r64(1));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 389; continue; }
            blk = 388; continue;
        }
        388 => {
            let r532 = c.r64(10) | 0x1u64;
            c.w64(10, r532);
            c.logic(r532, 64);
            blk = 389; continue;
        }
        389 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 390; continue;
        }
        390 => {
            let Some(t533) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c987d); };
            c.w64(15, t533);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008c9884); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008c9888); }
            c.w64(15, c.r64(0));
            blk = 391; continue;
        }
        391 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c9890); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008c9892); }
            let Some(t534) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008c9894); };
            c.w64(3, t534);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008c98a5, 0x70008c98a0);
        }
        392 => {
            let r535 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r535);
            c.w64(15, c.r64(0));
            let Some(t536) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c98aa); };
            c.w64(11, t536);
            let Some(t537) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c98ae); };
            c.w64(8, t537);
            let Some(t538) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c98b2); };
            c.w64(12, t538);
            let Some(t539) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c98b6); };
            c.w64(2, t539);
            blk = 231; continue;
        }
        393 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008c98c4); }
            return c.call(0x70007febf4, 0x70008c98ce, 0x70008c98c9);
        }
        394 => {
            c.w64(12, c.r64(0));
            let Some(t540) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c98d1); };
            c.w64(11, t540);
            let Some(t541) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c98d5); };
            c.w64(8, t541);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008c98d9); }
            blk = 239; continue;
        }
        395 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008c98e8); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008c98ed); }
            return c.call(0x70007febf4, 0x70008c98f6, 0x70008c98f1);
        }
        396 => {
            c.w64(11, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008c98f9); }
            let Some(t542) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c98fe); };
            c.w64(0, t542);
            blk = 255; continue;
        }
        397 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008c9907); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008c990b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008c990f); }
            blk = 401; continue;
        }
        398 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008c9915); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008c9919); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008c991d); }
            blk = 401; continue;
        }
        399 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008c9923); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008c9927); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008c992b); }
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 401; continue; }
            blk = 400; continue;
        }
        400 => {
            let Some(t543) = c.ld64(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c9934); };
            c.w64(0, t543);
            blk = 278; continue;
        }
        401 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008c9943); }
            c.cmp(c.r64(0), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 403; continue; }
            blk = 402; continue;
        }
        402 => {
            let r544 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r544);
            blk = 412; continue;
        }
        403 => {
            let Some(t545) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c9956); };
            c.w64(2, t545);
            let Some(t546) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008c995d); };
            c.w64(1, t546);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            let Some(t547) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c9964); };
            c.w64(7, t547);
            let Some(t548) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008c996b); };
            c.cmp(t548, c.r64(6), 64);
            if c.cc_a() { blk = 406; continue; }
            blk = 404; continue;
        }
        404 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008c997a, 0x70008c9975);
        }
        405 => {
            c.w64(2, c.r64(0));
            let Some(t549) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c997d); };
            c.w64(3, t549);
            let Some(t550) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c9981); };
            c.w64(11, t550);
            let Some(t551) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c9985); };
            c.w64(12, t551);
            let Some(t552) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c9989); };
            c.w64(15, t552);
            let Some(t553) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c998d); };
            c.w64(0, t553);
            blk = 407; continue;
        }
        406 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x70008c9993); }
            c.w64(2, c.r64(1).wrapping_add(0x1));
            blk = 407; continue;
        }
        407 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(0) as i64));
            c.logic(c.r64(0) & c.r64(0), 64);
            if c.cc_ns() { blk = 411; continue; }
            blk = 408; continue;
        }
        408 => {
            c.w64(10, c.r64(0));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 410; continue; }
            blk = 409; continue;
        }
        409 => {
            let r554 = c.r64(10) | 0x1u64;
            c.w64(10, r554);
            c.logic(r554, 64);
            blk = 410; continue;
        }
        410 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 411; continue;
        }
        411 => {
            let Some(t555) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c99c1); };
            c.w64(0, t555);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008c99c8); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008c99cb); }
            c.w64(0, c.r64(2));
            blk = 412; continue;
        }
        412 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008c99d3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c99d4); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t556) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008c99dc); };
            c.w64(3, t556);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008c99eb, 0x70008c99e6);
        }
        413 => {
            let r557 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r557);
            let Some(t558) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c99ed); };
            c.w64(15, t558);
            let Some(t559) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c99f1); };
            c.w64(3, t559);
            let Some(t560) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c99f5); };
            c.w64(11, t560);
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c99f9); };
            c.w64(12, t561);
            blk = 281; continue;
        }
        414 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008c9a02); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008c9a06); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008c9a0a); }
            blk = 418; continue;
        }
        415 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008c9a10); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008c9a14); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008c9a18); }
            blk = 418; continue;
        }
        416 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008c9a1e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008c9a22); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008c9a26); }
            c.logic(c.r8(0) & 0x10u64, 8);
            if c.cc_ne() { blk = 418; continue; }
            blk = 417; continue;
        }
        417 => {
            let Some(t562) = c.ld64(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c9a2e); };
            c.w64(1, t562);
            blk = 301; continue;
        }
        418 => {
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 420; continue; }
            blk = 419; continue;
        }
        419 => {
            let r563 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r563);
            blk = 429; continue;
        }
        420 => {
            let Some(t564) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c9a45); };
            c.w64(0, t564);
            let Some(t565) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008c9a4c); };
            c.w64(2, t565);
            c.w64(1, c.r64(2).wrapping_add(0xc));
            let Some(t566) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c9a53); };
            c.w64(6, t566);
            let Some(t567) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008c9a5a); };
            c.cmp(t567, c.r64(1), 64);
            if c.cc_a() { blk = 423; continue; }
            blk = 421; continue;
        }
        421 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008c9a69, 0x70008c9a64);
        }
        422 => {
            let Some(t568) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c9a69); };
            c.w64(15, t568);
            let Some(t569) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c9a6d); };
            c.w64(3, t569);
            let Some(t570) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c9a71); };
            c.w64(11, t570);
            let Some(t571) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c9a75); };
            c.w64(12, t571);
            blk = 424; continue;
        }
        423 => {
            if !c.st64(c.r64(0), c.r64(1)) { return RegfileExit::fault(0x70008c9a7b); }
            c.w64(0, c.r64(2).wrapping_add(0x1));
            blk = 424; continue;
        }
        424 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(15) as i64));
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 428; continue; }
            blk = 425; continue;
        }
        425 => {
            c.w64(10, c.r64(15));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 427; continue; }
            blk = 426; continue;
        }
        426 => {
            let r572 = c.r64(10) | 0x1u64;
            c.w64(10, r572);
            c.logic(r572, 64);
            blk = 427; continue;
        }
        427 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 428; continue;
        }
        428 => {
            let Some(t573) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008c9aa9); };
            c.w64(15, t573);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008c9ab0); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008c9ab4); }
            c.w64(15, c.r64(0));
            blk = 429; continue;
        }
        429 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008c9abc); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008c9abd); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t574) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008c9ac6); };
            c.w64(3, t574);
            c.w64(15, c.r64(3));
            return c.call(0x700086ed74, 0x70008c9ad5, 0x70008c9ad0);
        }
        430 => {
            let r575 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r575);
            c.w64(15, c.r64(0));
            let Some(t576) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c9ada); };
            c.w64(3, t576);
            let Some(t577) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c9ade); };
            c.w64(11, t577);
            let Some(t578) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c9ae2); };
            c.w64(12, t578);
            blk = 304; continue;
        }
        431 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008c9af0); }
            return c.call(0x70007febf4, 0x70008c9afa, 0x70008c9af5);
        }
        432 => {
            c.w64(11, c.r64(0));
            let Some(t579) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c9afd); };
            c.w64(3, t579);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008c9b01); }
            blk = 312; continue;
        }
        433 => {
            return RegfileExit::fault(0x70008c9b0b);
        }
        434 => {
            let Some(t580) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70008c9b0c); };
            c.w64(3, t580);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t581) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c9b15); };
            c.w64(6, t581);
            return c.call(0x700086ed74, 0x70008c9b1e, 0x70008c9b19);
        }
        435 => {
            return RegfileExit::fault(0x70008c9b1e);
        }
        436 => {
            let Some(t582) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c9b1f); };
            c.w64(0, t582);
            let Some(t583) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c9b23); };
            c.w64(3, t583);
            let Some(t584) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008c9b27); };
            c.w64(12, t584);
            let Some(t585) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c9b2b); };
            c.w64(6, t585);
            blk = 331; continue;
        }
        437 => {
            c.w32(0, c.r32(0));
            let r586 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r586);
            c.w64(11, 0xfffffffffffc0000u64);
            let r587 = c.r64(11) & c.r64(0);
            c.w64(11, r587);
            c.logic(r587, 64);
            let Some(t588) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c9b43); };
            c.logic(t588 & 0x2u64, 8);
            if c.cc_e() { blk = 144; continue; }
            blk = 438; continue;
        }
        438 => {
            c.w64(3, c.r64(15).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c9b52); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c9b53); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008c9b60, 0x70008c9b5b);
        }
        439 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c9b60); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c9b62); }
            blk = 144; continue;
        }
        440 => {
            c.w32(0, c.r32(0));
            let r589 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r589);
            c.w64(11, 0xfffffffffffc0000u64);
            let r590 = c.r64(11) & c.r64(0);
            c.w64(11, r590);
            c.logic(r590, 64);
            let Some(t591) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c9b77); };
            c.logic(t591 & 0x2u64, 8);
            if c.cc_e() { blk = 142; continue; }
            blk = 441; continue;
        }
        441 => {
            c.w64(3, c.r64(15).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c9b86); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c9b87); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008c9b94, 0x70008c9b8f);
        }
        442 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c9b94); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c9b96); }
            blk = 142; continue;
        }
        443 => {
            c.w32(0, c.r32(0));
            let r592 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r592);
            c.w64(3, 0xfffffffffffc0000u64);
            let r593 = c.r64(3) & c.r64(0);
            c.w64(3, r593);
            c.logic(r593, 64);
            let Some(t594) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c9bab); };
            c.logic(t594 & 0x2u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 444; continue;
        }
        444 => {
            c.w64(2, c.r64(11).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c9bb9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c9bba); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70008c9bc7, 0x70008c9bc2);
        }
        445 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c9bc7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c9bc9); }
            blk = 56; continue;
        }
        446 => {
            c.w32(0, c.r32(0));
            let r595 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r595);
            c.w64(3, 0xfffffffffffc0000u64);
            let r596 = c.r64(3) & c.r64(0);
            c.w64(3, r596);
            c.logic(r596, 64);
            let Some(t597) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c9bde); };
            c.logic(t597 & 0x2u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 447; continue;
        }
        447 => {
            c.w64(2, c.r64(11).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c9bec); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c9bed); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70008c9bfa, 0x70008c9bf5);
        }
        448 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c9bfa); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c9bfc); }
            blk = 35; continue;
        }
        449 => {
            return RegfileExit::jump(0x70008c9c04);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008c9c14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008c9c14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008c9c18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c9c19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c9c1a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008c9c1f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008c9c23); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 27; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(5));
            c.w64(8, c.r64(7).wrapping_add(0x18));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(7)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008c9c3e); };
            c.w64(9, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008c9c43); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008c9c47); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008c9c51); };
            c.w64(7, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c9c57); };
            c.w64(7, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c9c64); };
            c.w64(8, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c9c6a); };
            c.w64(8, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008c9c71); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c9c7f); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c9c86); };
            c.w32(11, t9);
            let r10 = c.sub(c.r32(11), 0xbau64, 32);
            c.w32(11, r10);
            c.cmp(c.r32(11), 0x78eu64, 32);
            if c.cc_a() { blk = 25; continue; }
            blk = 9; continue;
        }
        9 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t11) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c9ca9); };
            c.w32(11, t11);
            let r12 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r12);
            let Some(t13) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c9cb0); };
            c.cmp(t13, 0x40u64, 16);
            if c.cc_ae() { blk = 29; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t14) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c9cbc); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008c9cc3); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008c9cc7); };
            c.w64(12, t16);
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c9cd1); };
            c.cmp(t17, c.r32(15), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t18) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c9cdc); };
            c.w32(12, t18);
            let r19 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r19);
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c9ce3); };
            c.w32(12, t20);
            let r21 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r21);
            let Some(t22) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c9ceb); };
            c.w32(15, t22);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x9au64, 64);
            if c.cc_a() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return RegfileExit::fault(0x70008c9cff);
        }
        14 => {
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008c9d00); };
            c.w32(0, t23);
            let r24 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r24);
            let Some(t25) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008c9d0b); };
            c.cmp(t25, c.r32(11), 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t26) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c9d11); };
            c.w32(0, t26);
            c.logic(c.r32(0) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(15), 0x9fu64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x70008c9d25);
        }
        18 => {
            let Some(t27) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c9d26); };
            c.w32(11, t27);
            let r28 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r28);
            let Some(t29) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c9d2d); };
            c.w32(11, t29);
            let r30 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r30);
            let Some(t31) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c9d34); };
            c.w32(15, t31);
            let r32 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r32);
            c.w64(0, c.r64(2).wrapping_add(0x1));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008c9d3f); }
            let Some(t33) = c.ld32(c.r64(12).wrapping_add(0x283)) else { return RegfileExit::fault(0x70008c9d43); };
            c.cmp(t33, c.r32(11), 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t34) = c.ld32(c.r64(15).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008c9d4d); };
            c.w32(11, t34);
            c.w32(11, c.r32(11));
            let r35 = c.r32(11) & 0x8u64;
            c.w32(11, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(0, c.r64(9));
            c.w64(3, c.r64(7));
            c.w64(1, c.r64(8));
            return c.call(0x70008c8294, 0x70008c9d6c, 0x70008c9d67);
        }
        21 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c9d6c); };
            c.w64(2, t36);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008c9d73); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008c9d74); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008c9d7a); }
            return c.ret(0x0, 0x70008c9d7c);
        }
        22 => {
            c.w32(11, 0xa0u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008c9d83); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x70008c9d85); };
            c.w64(3, t37);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008c9d96, 0x70008c9d91);
        }
        23 => {
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c9d96); };
            if !c.push64(t38) { return RegfileExit::fault(0x70008c9d96); }
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c9d99); };
            if !c.push64(t39) { return RegfileExit::fault(0x70008c9d99); }
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c9d9c); };
            if !c.push64(t40) { return RegfileExit::fault(0x70008c9d9c); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2678)) else { return RegfileExit::fault(0x70008c9d9f); };
            c.w64(3, t41);
            c.w32(0, 0x3u64);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c9dab); };
            c.w64(6, t42);
            return c.call(0x700086ed74, 0x70008c9db4, 0x70008c9daf);
        }
        24 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c9db4); };
            c.w64(2, t43);
            let r44 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r44);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008c9dbf); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008c9dc0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008c9dc6); }
            return c.ret(0x0, 0x70008c9dc8);
        }
        25 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008c9dce); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008c9dcf); };
            c.w64(2, t45);
            let Some(t46) = c.ld32(c.r64(2).wrapping_add(0x297)) else { return RegfileExit::fault(0x70008c9dd6); };
            c.w32(2, t46);
            let r47 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r47);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008c9ddf); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008c9de0); };
            c.w64(3, t48);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008c9df1, 0x70008c9dec);
        }
        26 => {
            return RegfileExit::fault(0x70008c9df1);
        }
        27 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008c9df2); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008c9df6); };
            c.w64(3, t49);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008c9e07, 0x70008c9e02);
        }
        28 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c9e07); };
            c.w64(6, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c9e0b); };
            c.w64(0, t51);
            blk = 1; continue;
        }
        29 => {
            c.w64(0, c.r64(7));
            return c.call(0x700089c794, 0x70008c9e1c, 0x70008c9e17);
        }
        30 => {
            c.w64(7, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c9e1f); };
            c.w64(6, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c9e23); };
            c.w64(2, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c9e27); };
            c.w64(9, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c9e2b); };
            c.w64(8, t55);
            blk = 11; continue;
        }
        31 => {
            return RegfileExit::jump(0x70008c9e38);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008c9e54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008c9e54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008c9e58); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c9e5e); };
            c.w32(7, t1);
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008c9e64); }
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c9e68); };
            c.w32(12, t2);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c9e72); };
            c.w32(15, t3);
            let r4 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r4);
            c.logic(c.r8(12) & 0x9u64, 8);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008c9e85); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008c9e89); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70008c9e8d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008c9e91); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x70008c9e95); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(15, c.r64(11));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 15; continue;
        }
        2 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c9ebc); };
            c.w32(1, t5);
            let r6 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r6);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c9ec6); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.movsd_rm(1, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c9ed0); }
            blk = 9; continue;
        }
        6 => {
            c.w64(2, c.r64(15));
            let r7 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r7);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 9; continue;
        }
        7 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 8; continue;
        }
        8 => {
            let r8 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r8);
            if !c.movsd_rm(1, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c9eee); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 9; continue;
        }
        9 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 12; continue;
        }
        11 => {
            c.cmp(c.r32(1), c.r32(15), 32);
            if c.cc_l() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 14; continue;
        }
        13 => {
            c.w32(1, 0x1u64);
            blk = 14; continue;
        }
        14 => {
            c.w32(2, 0);
            c.logic(0, 32);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 82; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t9) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c9f1d); };
            c.w32(1, t9);
            let r10 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r10);
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c9f23); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c9f29); };
            c.w32(8, t13);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008c9f33); }
            c.cmp(c.r64(8), 0x9bu64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x70008c9f40);
        }
        17 => {
            let Some(t14) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c9f41); };
            c.w32(8, t14);
            let r15 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r15);
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x273)) else { return RegfileExit::fault(0x70008c9f48); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r17);
            let Some(t18) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c9f51); };
            c.w32(8, t18);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008c9f56); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 67; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(9, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c9f67); };
            c.w32(15, t19);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_b() { blk = 67; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t20) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c9f77); };
            c.w32(3, t20);
            let r21 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r21);
            let Some(t22) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c9f7d); };
            c.w32(1, t22);
            let r23 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008c9f82); }
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 84; continue;
        }
        23 => {
            blk = 26; continue;
        }
        24 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c9f9c); };
            c.w64(2, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c9fa0); };
            c.w64(3, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c9fa4); };
            c.w64(1, t26);
            return c.call(0x7000859a54, 0x70008c9fad, 0x70008c9fa8);
        }
        25 => {
            c.w64(1, c.r64(0));
            blk = 68; continue;
        }
        26 => {
            let Some(t27) = c.ld32(c.r64(3).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008c9fb5); };
            c.w32(1, t27);
            c.cmp(c.r32(1), 0x7eu64, 32);
            if c.cc_a() { blk = 97; continue; }
            blk = 27; continue;
        }
        27 => {
            c.logic(c.r8(8) & 0x7u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008c9fcb); }
            c.w64(8, c.r64(11));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c9fd2); };
            c.w64(7, t28);
            blk = 30; continue;
        }
        29 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008c9fd8); }
            blk = 30; continue;
        }
        30 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c9fdc); };
            c.w64(12, t29);
            let r30 = c.r32(12) & 0x7u64;
            c.w32(12, r30);
            c.logic(r30, 32);
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_l() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 97; continue;
        }
        35 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            blk = 97; continue;
        }
        38 => {
            let Some(t31) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ca016); };
            c.w32(12, t31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008ca01a); };
            c.cmp(t32, c.r32(12), 32);
            if c.cc_ne() { blk = 97; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t33) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ca027); };
            c.w32(7, t33);
            let r34 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r34);
            let Some(t35) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca02d); };
            c.w32(12, t35);
            let r36 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r36);
            let Some(t37) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ca034); };
            c.w32(12, t37);
            blk = 29; continue;
        }
        40 => {
            let Some(t38) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ca03c); };
            c.w32(12, t38);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r39 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r39);
            let Some(t40) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ca049); };
            c.w32(7, t40);
            let r41 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r41);
            let Some(t42) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca04f); };
            c.w32(12, t42);
            let r43 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r43);
            let Some(t44) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ca056); };
            c.w32(12, t44);
            blk = 29; continue;
        }
        41 => {
            let Some(t45) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ca061); };
            c.w32(7, t45);
            let r46 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r46);
            let Some(t47) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca067); };
            c.w32(12, t47);
            let r48 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r48);
            let Some(t49) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ca06e); };
            c.w32(12, t49);
            blk = 29; continue;
        }
        42 => {
            c.w64(12, c.r64(7));
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(2));
            blk = 45; continue;
        }
        43 => {
            c.w64(12, c.r64(7));
            c.w64(7, c.r64(8));
            c.w32(8, 0x1u64);
            blk = 45; continue;
        }
        44 => {
            c.w64(7, c.r64(11));
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca095); };
            c.w64(12, t50);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008ca099); }
            c.w64(8, c.r64(2));
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 46; continue;
        }
        46 => {
            let r51 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r51);
            blk = 47; continue;
        }
        47 => {
            let r52 = c.add(c.r64(15), c.r64(7), 64);
            c.w64(15, r52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ca0b1); };
            c.w64(8, t53);
            let r54 = c.r32(8) & 0x8u64;
            c.w32(8, r54);
            c.logic(r54, 32);
            let r55 = c.add(c.r64(7), c.r64(9), 64);
            c.w64(7, r55);
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 48; continue;
        }
        48 => {
            let r56 = c.add(c.r64(15), c.r64(12), 64);
            c.w64(15, r56);
            let r57 = c.add(c.r64(7), c.r64(12), 64);
            c.w64(7, r57);
            let Some(t58) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ca0cc); };
            c.w32(12, t58);
            let r59 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r59);
            blk = 49; continue;
        }
        49 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008ca0d3); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 97; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t60) = c.ld32(c.r64(12).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008ca0e1); };
            c.w32(8, t60);
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2e70)) else { return RegfileExit::fault(0x70008ca0e6); };
            c.w64(7, t61);
            let Some(t62) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ca0ed); };
            c.w64(7, t62);
            c.cmp(c.r32(8), 0xffffffffffffffffu64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w32(12, c.r64(8).wrapping_mul(8));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(13));
            let Some(t63) = c.ld64(c.r64(8).wrapping_add(c.r64(12)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008ca104); };
            c.w64(12, t63);
            blk = 53; continue;
        }
        52 => {
            let r64 = c.add(c.r64(12), 0x3fu64, 64);
            c.w64(12, r64);
            c.w64(8, c.r64(13));
            blk = 53; continue;
        }
        53 => {
            let r65 = c.add(c.r32(1), 0x2u64, 32);
            c.w32(1, r65);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w32(9, c.r32(9));
            c.w64(10, c.r64(4));
            let r66 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r66);
            let r67 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r67);
            c.logic(r67, 64);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x70008ca12c); }
            let r68 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r68);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008ca133); };
            c.w64(11, t69);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(11)) { return RegfileExit::fault(0x70008ca13a); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(1)) { return RegfileExit::fault(0x70008ca13f); }
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(7)) { return RegfileExit::fault(0x70008ca144); }
            if !c.st64(c.r64(4).wrapping_add(0x38), 0x1u64) { return RegfileExit::fault(0x70008ca149); }
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(8)) { return RegfileExit::fault(0x70008ca152); }
            c.w64(7, c.r64(0));
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(7)) { return RegfileExit::fault(0x70008ca15a); }
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca15f); };
            c.w64(1, t70);
            c.w64(2, c.r64(9));
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ca166); };
            c.w64(8, t71);
            c.w64(9, c.r64(15));
            c.w64(10, 0x70008ca174u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008ca174); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008ca178); }
            return c.call(c.r64(12), 0x70008ca17f, 0x70008ca17c);
        }
        54 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008ca17f); }
            let Some(t72) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x70008ca187); };
            c.w64(4, t72);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_e() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_ne() { blk = 89; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ca19f); };
            c.w64(0, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ca1a3); };
            c.w64(7, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ca1a7); };
            c.w64(12, t75);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 67; continue;
        }
        57 => {
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ca1b3); };
            c.w64(1, t76);
            let Some(t77) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ca1b7); };
            c.w32(7, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ca1ba); };
            c.w64(11, t78);
            let Some(t79) = c.ld32(c.r64(11).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008ca1be); };
            c.w32(11, t79);
            let r80 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r80);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r81 = c.sub(c.r32(7), 0x6u64, 32);
            c.w32(7, r81);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_l() { blk = 96; continue; }
            blk = 58; continue;
        }
        58 => {
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008ca1d8); }
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca1dc); };
            c.w64(7, t82);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008ca1e0); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r83 = c.r64(12) & c.r64(1);
            c.w64(12, r83);
            c.logic(r83, 64);
            let Some(t84) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ca1f3); };
            c.logic(t84 & 0x4u64, 8);
            if c.cc_ne() { blk = 102; continue; }
            blk = 60; continue;
        }
        60 => {
            if !c.st32(c.r64(1).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008ca1ff); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r85 = c.r64(12) & c.r64(1);
            c.w64(12, r85);
            c.logic(r85, 64);
            let Some(t86) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ca212); };
            c.logic(t86 & 0x4u64, 8);
            if c.cc_ne() { blk = 99; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if c.cc_e() { blk = 68; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w32(11, 0x13u64);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008ca22f); };
            c.w64(15, t87);
            blk = 65; continue;
        }
        64 => {
            let r88 = c.add(c.r64(11), 0x4u64, 64);
            c.w64(11, r88);
            blk = 65; continue;
        }
        65 => {
            let Some(t89) = c.ld32(c.r64(15).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008ca23c); };
            c.w32(0, t89);
            let r90 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r90);
            if !c.st32(c.r64(1).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008ca242); }
            let r91 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r91);
            c.cmp(c.r64(7), c.r64(12), 64);
            if c.cc_ne() { blk = 64; continue; }
            blk = 66; continue;
        }
        66 => {
            blk = 68; continue;
        }
        67 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008ca251); };
            c.w64(1, t92);
            blk = 68; continue;
        }
        68 => {
            let Some(t93) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008ca258); };
            c.cmp(t93, c.r32(1), 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t94) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ca261); };
            c.cmp(t94, 0x0u64, 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ca267); };
            c.w64(7, t95);
            blk = 72; continue;
        }
        71 => {
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ca26d); };
            c.w64(7, t96);
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008ca274); }
            blk = 72; continue;
        }
        72 => {
            c.w64(0, c.r64(7));
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ca27b); };
            c.w64(7, t97);
            blk = 83; continue;
        }
        73 => {
            let Some(t98) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ca281); };
            c.cmp(t98, 0x0u64, 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ca287); };
            c.w64(7, t99);
            blk = 78; continue;
        }
        75 => {
            let Some(t100) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ca28d); };
            c.w32(7, t100);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x4u64, 64);
            if c.cc_a() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            return RegfileExit::fault(0x70008ca29c);
        }
        77 => {
            let Some(t101) = c.ld32(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008ca29d); };
            c.w32(11, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ca2a1); };
            c.w64(7, t102);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008ca2a5); }
            blk = 78; continue;
        }
        78 => {
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ca2a9); };
            c.w64(12, t103);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x70008ca2ad); }
            let Some(t104) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ca2b1); };
            c.w32(7, t104);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x3u64, 64);
            if c.cc_a() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            return RegfileExit::fault(0x70008ca2c0);
        }
        80 => {
            let Some(t105) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ca2c1); };
            c.w32(0, t105);
            let r106 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r106);
            blk = 81; continue;
        }
        81 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ca2ca); }
            return c.ret(0x0, 0x70008ca2cb);
        }
        82 => {
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008ca2cc); }
            blk = 83; continue;
        }
        83 => {
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(7)) { return RegfileExit::fault(0x70008ca2d0); }
            c.w64(0, 0xfffffffffffffffeu64);
            blk = 81; continue;
        }
        84 => {
            return RegfileExit::fault(0x70008ca2dc);
        }
        85 => {
            blk = 97; continue;
        }
        86 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ca2e2); };
            c.w64(8, t107);
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 97; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t108) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ca2f0); };
            c.w64(12, t108);
            blk = 47; continue;
        }
        88 => {
            c.w64(15, c.r64(12).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.w64(7, c.r64(12).wrapping_add(c.r64(7).wrapping_mul(2)));
            let Some(t109) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008ca302); };
            c.w32(12, t109);
            let r110 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r110);
            blk = 49; continue;
        }
        89 => {
            c.cmp(c.r64(7), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 94; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r64(7), 0xfffffffffffffffdu64, 64);
            if c.cc_e() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ca31a); };
            c.w64(0, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ca31e); };
            c.w64(6, t112);
            blk = 97; continue;
        }
        92 => {
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ca324); };
            if !c.push64(t113) { return RegfileExit::fault(0x70008ca324); }
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca327); };
            if !c.push64(t114) { return RegfileExit::fault(0x70008ca327); }
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ca32a); };
            if !c.push64(t115) { return RegfileExit::fault(0x70008ca32a); }
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ca32d); };
            if !c.push64(t116) { return RegfileExit::fault(0x70008ca32d); }
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x2658)) else { return RegfileExit::fault(0x70008ca330); };
            c.w64(3, t117);
            c.w32(0, 0x4u64);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ca33c); };
            c.w64(6, t118);
            return c.call(0x700086ed74, 0x70008ca345, 0x70008ca340);
        }
        93 => {
            c.w64(1, c.r64(0));
            blk = 68; continue;
        }
        94 => {
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70008ca34d); };
            c.w64(3, t119);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ca356); };
            c.w64(6, t120);
            return c.call(0x700086ed74, 0x70008ca35f, 0x70008ca35a);
        }
        95 => {
            return RegfileExit::fault(0x70008ca35f);
        }
        96 => {
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ca360); };
            c.w64(0, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ca364); };
            c.w64(6, t122);
            blk = 97; continue;
        }
        97 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ca368); }
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca369); };
            if !c.push64(t123) { return RegfileExit::fault(0x70008ca369); }
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ca36c); };
            if !c.push64(t124) { return RegfileExit::fault(0x70008ca36c); }
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ca36f); };
            if !c.push64(t125) { return RegfileExit::fault(0x70008ca36f); }
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x2648)) else { return RegfileExit::fault(0x70008ca372); };
            c.w64(3, t126);
            c.w32(0, 0x4u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008ca386, 0x70008ca381);
        }
        98 => {
            c.w64(1, c.r64(0));
            blk = 68; continue;
        }
        99 => {
            c.w32(7, c.r32(7));
            let r127 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r127);
            c.w64(12, 0xfffffffffffc0000u64);
            let r128 = c.r64(12) & c.r64(7);
            c.w64(12, r128);
            c.logic(r128, 64);
            let Some(t129) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ca39d); };
            c.logic(t129 & 0x2u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w64(15, c.r64(1).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ca3ad); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ca3ae); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008ca3bb, 0x70008ca3b6);
        }
        101 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ca3bb); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ca3bd); }
            blk = 62; continue;
        }
        102 => {
            c.w32(7, c.r32(7));
            let r130 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r130);
            c.w64(12, 0xfffffffffffc0000u64);
            let r131 = c.r64(12) & c.r64(7);
            c.w64(12, r131);
            c.logic(r131, 64);
            let Some(t132) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ca3d2); };
            c.logic(t132 & 0x2u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w64(15, c.r64(1).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ca3e2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ca3e3); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008ca3f0, 0x70008ca3eb);
        }
        104 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ca3f0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ca3f2); }
            blk = 60; continue;
        }
        105 => {
            return RegfileExit::jump(0x70008ca3fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ca414(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ca414); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ca418); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ca419); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ca41a); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ca41f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ca423); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 95; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ca42d); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca439); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ca440); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 125; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ca44d); };
            c.w64(8, t6);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 97; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca45b); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ca462); };
            c.cmp(t9, 0x40u64, 16);
            if c.cc_ae() { blk = 97; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca46e); };
            c.w32(9, t10);
            let r11 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r11);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008ca475); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008ca479); };
            c.w64(11, t12);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ca482); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca48c); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ca493); };
            c.w32(11, t16);
            let r17 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r17);
            let Some(t18) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ca49a); };
            c.w32(12, t18);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x9au64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x70008ca4ad);
        }
        8 => {
            let Some(t19) = c.ld32(c.r64(11).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008ca4ae); };
            c.w32(15, t19);
            let r20 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r20);
            let Some(t21) = c.ld32(c.r64(15).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008ca4b8); };
            c.cmp(t21, c.r32(9), 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008ca4be); };
            c.w32(15, t22);
            c.logic(c.r32(15) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(12), 0x9fu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008ca4d4);
        }
        12 => {
            let Some(t23) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ca4d5); };
            c.w32(9, t23);
            let r24 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r24);
            let Some(t25) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca4dc); };
            c.w32(12, t25);
            let r26 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r26);
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008ca4e3); };
            c.w32(15, t27);
            let r28 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r28);
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0x283)) else { return RegfileExit::fault(0x70008ca4eb); };
            c.cmp(t29, c.r32(12), 32);
            if c.cc_ne() { blk = 99; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t30) = c.ld32(c.r64(15).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008ca4f8); };
            c.w32(3, t30);
            c.w32(3, c.r32(3));
            let r31 = c.r32(3) & 0x8u64;
            c.w32(3, r31);
            c.logic(r31, 32);
            c.cmp(c.r32(3), 0x8u64, 32);
            if c.cc_ne() { blk = 99; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(3, c.r64(8));
            return c.call(0x70008c9e54, 0x70008ca512, 0x70008ca50d);
        }
        15 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ca512); };
            c.w64(1, t32);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ca519); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return c.ret(0x10, 0x70008ca520);
        }
        17 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ca523); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ca52a); }
            return c.ret(0x0, 0x70008ca52c);
        }
        18 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008ca52d); };
            c.w64(3, t33);
            return c.call(0x7000870914, 0x70008ca539, 0x70008ca534);
        }
        19 => {
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008ca53b); }
            c.cmp(c.r32(0), c.r32(1), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t34) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca547); };
            c.w32(8, t34);
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ca54b); };
            c.cmp(t35, c.r32(8), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ca554); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            blk = 26; continue;
        }
        25 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        27 => {
            c.w32(8, 0x1u64);
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ca593); };
            c.w64(0, t36);
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008ca597); };
            c.w64(3, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ca59e); };
            c.w64(6, t38);
            return c.call(0x7000871cb4, 0x70008ca5a7, 0x70008ca5a2);
        }
        30 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x6e8)) else { return RegfileExit::fault(0x70008ca5a7); };
            c.w64(3, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ca5ae); };
            c.w64(0, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ca5b2); };
            c.w64(6, t41);
            return c.call(0x7000870914, 0x70008ca5bb, 0x70008ca5b6);
        }
        31 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 38; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t42) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca5bf); };
            c.w32(8, t42);
            let r43 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r43);
            let Some(t44) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008ca5c6); };
            c.logic(t44 & 0x2u64, 8);
            if c.cc_e() { blk = 38; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ca5cd); };
            if !c.push64(t45) { return RegfileExit::fault(0x70008ca5cd); }
            let Some(t46) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ca5d0); };
            if !c.push64(t46) { return RegfileExit::fault(0x70008ca5d0); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ca5db); };
            c.w64(6, t47);
            return c.call(0x70007f2274, 0x70008ca5e4, 0x70008ca5df);
        }
        34 => {
            let Some(t48) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008ca5e4); };
            c.cmp(t48, c.r32(0), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 123; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca5f5); };
            c.w32(8, t49);
            let r50 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r50);
            let Some(t51) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ca5fc); };
            c.cmp(t51, 0xbau64, 16);
            if c.cc_ae() { blk = 40; continue; }
            blk = 37; continue;
        }
        37 => {
            blk = 123; continue;
        }
        38 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ca60a); };
            c.w64(0, t52);
            let Some(t53) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca60e); };
            c.w32(8, t53);
            let r54 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r54);
            let Some(t55) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ca615); };
            c.cmp(t55, 0x83eu64, 16);
            if c.cc_ne() { blk = 93; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ca622); };
            c.w64(6, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ca626); };
            c.w64(3, t57);
            return c.call(0x70008bf674, 0x70008ca62f, 0x70008ca62a);
        }
        40 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008ca62f); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ca633); };
            c.w64(0, t58);
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008ca637); };
            c.w64(3, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ca63e); };
            c.w64(6, t60);
            return c.call(0x7000870914, 0x70008ca647, 0x70008ca642);
        }
        41 => {
            let Some(t61) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca647); };
            c.cmp(c.r32(0), t61, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t62) = c.ld8(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca650); };
            c.logic(t62 & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 63; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 60; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t63) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca66e); };
            c.w32(8, t63);
            let r64 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r64);
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ca675); };
            c.cmp(t65, c.r32(8), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t66) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ca682); };
            c.w32(8, t66);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_b() { blk = 53; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca695); };
            c.w64(1, t67);
            blk = 76; continue;
        }
        48 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca69e); };
            c.w64(8, t68);
            let Some(t69) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca6a2); };
            c.w32(9, t69);
            let r70 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r70);
            let Some(t71) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ca6a9); };
            c.cmp(t71, 0x41u64, 16);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(1, c.r64(8));
            blk = 76; continue;
        }
        50 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ca6b9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ca6ba); }
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x70008ca6bc); };
            c.w64(3, t72);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ca6cf, 0x70008ca6ca);
        }
        51 => {
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008ca6cf); };
            c.cmp(t73, c.r32(0), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca6dc); };
            c.w64(1, t74);
            blk = 76; continue;
        }
        53 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca6e5); };
            c.w64(8, t75);
            let Some(t76) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca6e9); };
            c.w32(9, t76);
            let r77 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r77);
            let Some(t78) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ca6f0); };
            c.cmp(t78, 0x40u64, 16);
            if c.cc_b() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(1, c.r64(8));
            blk = 76; continue;
        }
        55 => {
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(8));
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x70007fb534, 0x70008ca70d, 0x70008ca708);
        }
        56 => {
            let Some(t79) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008ca70d); };
            c.cmp(t79, c.r32(0), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca71a); };
            c.w64(1, t80);
            blk = 76; continue;
        }
        58 => {
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca723); };
            c.w64(1, t81);
            let Some(t82) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca727); };
            c.w32(8, t82);
            let Some(t83) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ca72b); };
            c.cmp(t83, c.r32(8), 32);
            if c.cc_ne() { blk = 76; continue; }
            blk = 59; continue;
        }
        59 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ca738); }
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ca73d); }
            blk = 66; continue;
        }
        60 => {
            let Some(t84) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca744); };
            c.w32(8, t84);
            let Some(t85) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ca748); };
            c.cmp(t85, c.r32(8), 32);
            if c.cc_ne() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca751); };
            c.w64(1, t86);
            let r87 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r87);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ca757); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca764); };
            c.w64(1, t88);
            blk = 66; continue;
        }
        62 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca76a); };
            c.w64(1, t89);
            blk = 76; continue;
        }
        63 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca776); };
            c.w64(1, t90);
            let Some(t91) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca77a); };
            c.w32(8, t91);
            let Some(t92) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ca77e); };
            c.cmp(t92, c.r32(8), 32);
            if c.cc_ne() { blk = 76; continue; }
            blk = 65; continue;
        }
        65 => {
            let r93 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r93);
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ca789); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 66; continue;
        }
        66 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            if c.cc_e() { blk = 73; continue; }
            blk = 68; continue;
        }
        68 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            if c.cc_e() { blk = 76; continue; }
            blk = 70; continue;
        }
        70 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 77; continue; }
            blk = 71; continue;
        }
        71 => {
            if c.cc_e() { blk = 76; continue; }
            blk = 72; continue;
        }
        72 => {
            blk = 77; continue;
        }
        73 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 74; continue;
        }
        74 => {
            blk = 76; continue;
        }
        75 => {
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca7c9); };
            c.w64(1, t94);
            blk = 76; continue;
        }
        76 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 78; continue;
        }
        77 => {
            c.w32(8, 0x1u64);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ca7d8); };
            c.w64(1, t95);
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t96) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ca7e2); };
            c.w64(0, t96);
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008ca7e6); };
            c.w64(3, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ca7ed); };
            c.w64(6, t98);
            return c.call(0x7000871cb4, 0x70008ca7f6, 0x70008ca7f1);
        }
        80 => {
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ca7f6); };
            c.w64(0, t99);
            let Some(t100) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008ca7fa); };
            c.cmp(t100, c.r32(0), 32);
            if c.cc_e() { blk = 92; continue; }
            blk = 81; continue;
        }
        81 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 89; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ca807); };
            c.w64(6, t101);
            let Some(t102) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca80b); };
            c.w32(8, t102);
            let r103 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r103);
            let Some(t104) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ca812); };
            c.w32(8, t104);
            let r105 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r105);
            let Some(t106) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ca819); };
            c.w32(9, t106);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t107) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca820); };
            c.w32(11, t107);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xa1u64, 64);
            if c.cc_a() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            return RegfileExit::fault(0x70008ca830);
        }
        84 => {
            let Some(t108) = c.ld32(c.r64(8).wrapping_add(0x28b)) else { return RegfileExit::fault(0x70008ca831); };
            c.cmp(t108, c.r32(11), 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r64(9), 0xa2u64, 64);
            if c.cc_a() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            return RegfileExit::fault(0x70008ca843);
        }
        87 => {
            let Some(t109) = c.ld32(c.r64(8).wrapping_add(0x28f)) else { return RegfileExit::fault(0x70008ca844); };
            c.cmp(t109, c.r32(11), 32);
            if c.cc_ne() { blk = 90; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t110) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ca84d); };
            c.w32(0, t110);
            let r111 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r111);
            blk = 15; continue;
        }
        89 => {
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ca858); };
            c.w64(6, t112);
            blk = 90; continue;
        }
        90 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x7b0)) else { return RegfileExit::fault(0x70008ca85c); };
            c.w64(3, t113);
            return c.call(0x7000870914, 0x70008ca868, 0x70008ca863);
        }
        91 => {
            blk = 15; continue;
        }
        92 => {
            c.w64(0, 0xfffffffffffffffeu64);
            blk = 15; continue;
        }
        93 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ca87f); }
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008ca881); };
            c.w64(8, t114);
            let Some(t115) = c.ld32(c.r64(8).wrapping_add(0x28b)) else { return RegfileExit::fault(0x70008ca888); };
            c.w32(8, t115);
            let r116 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r116);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ca892); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ca894); }
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008ca895); };
            c.w64(3, t117);
            c.w32(0, 0x3u64);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ca8a1); };
            c.w64(6, t118);
            return c.call(0x700086ed74, 0x70008ca8aa, 0x70008ca8a5);
        }
        94 => {
            return RegfileExit::fault(0x70008ca8aa);
        }
        95 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ca8ad); };
            c.w64(3, t119);
            return c.call(0x700086ed74, 0x70008ca8b9, 0x70008ca8b4);
        }
        96 => {
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ca8b9); };
            c.w64(6, t120);
            blk = 1; continue;
        }
        97 => {
            c.w64(0, c.r64(8));
            return c.call(0x700089c794, 0x70008ca8ca, 0x70008ca8c5);
        }
        98 => {
            c.w64(8, c.r64(0));
            let Some(t121) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ca8cd); };
            c.w64(0, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ca8d1); };
            c.w64(6, t122);
            blk = 5; continue;
        }
        99 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008ca8da); }
            let Some(t123) = c.ld16(c.r64(15).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008ca8de); };
            c.cmp(t123, 0x1u64, 16);
            if c.cc_le() { blk = 18; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t124) = c.ld8(c.r64(12).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008ca8ea); };
            c.w32(3, t124);
            let r125 = c.r32(3) & 0xfcu64;
            c.w32(3, r125);
            c.logic(r125, 32);
            c.cmp(c.r32(3), 0x40u64, 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.cmp(c.r32(3), 0x3cu64, 32);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            let r126 = c.r32(2) | c.r32(3);
            c.w32(2, r126);
            c.logic(r126, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t127) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ca910); };
            c.cmp(t127, 0x410u64, 16);
            if c.cc_be() { blk = 18; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t128) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ca91e); };
            c.logic(t128 & 0x200000u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t129) = c.ld32(c.r64(15).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008ca92d); };
            c.w32(3, t129);
            let Some(t130) = c.ld32(c.r64(13).wrapping_add(0x6e8)) else { return RegfileExit::fault(0x70008ca931); };
            c.cmp(t130, c.r32(3), 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t131) = c.ld32(c.r64(15).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008ca93e); };
            c.w32(3, t131);
            c.w32(2, c.r32(3));
            c.logic(c.r8(3) & 0x4u64, 8);
            if c.cc_e() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t132) = c.ld32(c.r64(15).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ca949); };
            c.w32(9, t132);
            blk = 119; continue;
        }
        106 => {
            let r133 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r133);
            if let Some(sr) = c.shr(c.r64(3), 0x13u64, 64) { c.w64(3, sr); }
            let Some(t134) = c.ld8(c.r64(12).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008ca958); };
            c.w32(15, t134);
            let r135 = c.r64(3) & 0x3ffu64;
            c.w64(3, r135);
            c.logic(r135, 64);
            let r136 = c.r32(2) & 0x380u64;
            c.w32(2, r136);
            c.logic(r136, 32);
            let r137 = c.add(c.r64(15), c.r64(3), 64);
            c.w64(15, r137);
            c.cmp(c.r32(2), 0x100u64, 32);
            c.w8(3, (c.cc_ne() as u64));
            c.w32(3, c.r8(3));
            let Some(t138) = c.ld8(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ca97a); };
            c.w32(12, t138);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_b() { blk = 111; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t139) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ca985); };
            c.w32(9, t139);
            let r140 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r140);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008ca992); };
            c.w64(9, t141);
            blk = 109; continue;
        }
        109 => {
            let r142 = c.sub(c.r64(15), c.r64(12), 64);
            c.w64(15, r142);
            let Some(t143) = c.ld32(c.r64(9).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ca99c); };
            c.w32(9, t143);
            let r144 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r144);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 119; continue; }
            blk = 110; continue;
        }
        110 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ca9ad); }
            blk = 113; continue;
        }
        111 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 118; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t145) = c.ld32(c.r64(9).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ca9ba); };
            c.w32(9, t145);
            let r146 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r146);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ca9c2); }
            blk = 113; continue;
        }
        113 => {
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ca9c8); };
            c.w64(9, t147);
            let Some(t148) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ca9cf); };
            c.w64(12, t148);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x70008ca9d7); }
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ca9dc); };
            c.w64(3, t149);
            let Some(t150) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008ca9e3); };
            c.cmp(t150, c.r64(15), 64);
            if c.cc_a() { blk = 116; continue; }
            blk = 114; continue;
        }
        114 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ca9f2, 0x70008ca9ed);
        }
        115 => {
            c.w64(9, c.r64(0));
            let Some(t151) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ca9f5); };
            c.w64(0, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ca9f9); };
            c.w64(6, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ca9fd); };
            c.w64(8, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008caa01); };
            c.w64(11, t154);
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 117; continue;
        }
        116 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008caa09); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 117; continue;
        }
        117 => {
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008caa11); };
            c.w64(12, t155);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008caa18); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x70008caa1c); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008caa21); }
            blk = 119; continue;
        }
        118 => {
            let Some(t156) = c.ld32(c.r64(9).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008caa29); };
            c.w32(9, t156);
            blk = 119; continue;
        }
        119 => {
            let Some(t157) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008caa2e); };
            c.w32(12, t157);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x99u64, 64);
            if c.cc_a() { blk = 121; continue; }
            blk = 120; continue;
        }
        120 => {
            return RegfileExit::fault(0x70008caa41);
        }
        121 => {
            let Some(t158) = c.ld32(c.r64(11).wrapping_add(0x26b)) else { return RegfileExit::fault(0x70008caa42); };
            c.cmp(t158, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 122; continue;
        }
        122 => {
            blk = 18; continue;
        }
        123 => {
            c.w32(8, 0x88u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008caa5a); }
            let Some(t159) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008caa5c); };
            c.w64(8, t159);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008caa63); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008caa65); }
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008caa66); };
            c.w64(3, t160);
            c.w32(0, 0x3u64);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008caa72); };
            c.w64(6, t161);
            return c.call(0x700086ed74, 0x70008caa7b, 0x70008caa76);
        }
        124 => {
            return RegfileExit::fault(0x70008caa7b);
        }
        125 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008caa82); }
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008caa84); };
            c.w64(8, t162);
            let Some(t163) = c.ld32(c.r64(8).wrapping_add(0x29b)) else { return RegfileExit::fault(0x70008caa8b); };
            c.w32(8, t163);
            let r164 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r164);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008caa95); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008caa97); }
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008caa98); };
            c.w64(3, t165);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008caaa9, 0x70008caaa4);
        }
        126 => {
            return RegfileExit::fault(0x70008caaa9);
        }
        127 => {
            return RegfileExit::jump(0x70008caaac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008caab4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008caab4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008caab8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008caab9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008caaba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008caabf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008caac9); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008caad2); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008caad8); };
            c.cmp(t5, 0x83eu64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008caae0); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008caae6); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008caaed); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x8, 0x70008caaf4);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008caaf7); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008caafe); }
            return c.ret(0x0, 0x70008cab00);
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cab01); };
            c.w32(1, t9);
            let r10 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r10);
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cab07); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cab0d); };
            c.w32(7, t13);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x9au64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008cab1f);
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008cab20); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008cab29); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cab2f); };
            c.cmp(t18, c.r32(2), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x998)) else { return RegfileExit::fault(0x70008cab34); };
            c.w64(0, t19);
            blk = 4; continue;
        }
        11 => {
            c.w32(2, 0x146u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cab42); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cab43); };
            c.w64(2, t20);
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(0x29f)) else { return RegfileExit::fault(0x70008cab4a); };
            c.w32(2, t21);
            let r22 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r22);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cab53); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008cab54); };
            c.w64(3, t23);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008cab65, 0x70008cab60);
        }
        12 => {
            return RegfileExit::fault(0x70008cab65);
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008cab66); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cab6a); };
            c.w64(3, t24);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008cab78, 0x70008cab73);
        }
        14 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cab78); };
            c.w64(6, t25);
            blk = 1; continue;
        }
        15 => {
            return RegfileExit::jump(0x70008cab84);
        }
            _ => unreachable!(),
        }
    }
}
