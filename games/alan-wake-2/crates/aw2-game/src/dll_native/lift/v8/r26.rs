#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_70008cab94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cab94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008cab98); }
            let r0 = c.sub(c.r64(4), 0x90u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008caba1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008caba5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008caba9); }
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cabad); };
            c.cmp(t1, c.r32(1), 32);
            if c.cc_e() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r32(1) & 0x80000001u64, 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cabbe); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008cabbf); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008cabc0); }
            c.w32(0, 0x3u64);
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x2680)) else { return RegfileExit::fault(0x70008cabc6); };
            c.w64(3, t2);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008cabd5, 0x70008cabd0);
        }
        3 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cabd8); }
            return c.ret(0x0, 0x70008cabd9);
        }
        4 => {
            c.w32(1, 0x7ffffffeu64);
            blk = 5; continue;
        }
        5 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cabdf); };
            c.w32(7, t3);
            c.w32(7, c.r32(7));
            let r4 = c.r32(7) & 0x10u64;
            c.w32(7, r4);
            c.logic(r4, 32);
            let r5 = c.shr(c.r32(7), 0x3u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r5);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r6 = c.r64(7) & 0xfffffffffffffffeu64;
            c.w64(7, r6);
            c.logic(r6, 64);
            let r7 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r7);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008cabf3); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 254; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cac00); };
            c.w32(7, t8);
            let r9 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cac06); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cac0c); };
            c.w32(11, t12);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008cac16); }
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x70008cac20);
        }
        8 => {
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008cac21); };
            c.w32(12, t13);
            c.w32(15, c.r32(1));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 345; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cac35); };
            c.w32(15, t14);
            let r15 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r15);
            c.w32(2, c.r32(15));
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(15)) { return RegfileExit::fault(0x70008cac3f); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 334; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008cac52); };
            c.w64(12, t16);
            c.w64(11, c.r64(9));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008cac5c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008cac60); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008cac64); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008cac68); }
            blk = 12; continue;
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008cac6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008cac72); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008cac76); }
            c.w64(9, c.r64(11));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008cac7d); }
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cac81); };
            c.w64(0, t17);
            c.w64(3, c.r64(15));
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cac88); };
            c.w64(6, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cac8c); };
            c.w64(7, t19);
            blk = 12; continue;
        }
        12 => {
            c.w32(11, c.r32(9));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x70008cac93); }
            let Some(t20) = c.ld32(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008cac97); };
            c.cmp(c.r32(9), t20, 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t21) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008caca4); };
            c.w32(11, t21);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x9bu64, 64);
            if c.cc_be() { blk = 203; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t22) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cacbb); };
            c.w32(11, t22);
            let r23 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r23);
            let Some(t24) = c.ld32(c.r64(7).wrapping_add(0x273)) else { return RegfileExit::fault(0x70008cacc2); };
            c.w32(12, t24);
            let r25 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r25);
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let Some(t26) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cacd8); };
            c.w32(11, t26);
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(12)) { return RegfileExit::fault(0x70008cacdd); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(7)) { return RegfileExit::fault(0x70008cace4); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x70008cace8); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t27) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cacf8); };
            c.w32(11, t27);
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(11)) { return RegfileExit::fault(0x70008cacff); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_b() { blk = 65; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t28) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cad0c); };
            c.w32(12, t28);
            let r29 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r29);
            let Some(t30) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cad13); };
            c.w32(11, t30);
            let r31 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r31);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008cad1b); }
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 19; continue;
        }
        19 => {
            blk = 314; continue;
        }
        20 => {
            blk = 23; continue;
        }
        21 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x70008cad38); }
            c.w64(1, c.r64(9));
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008cad3f); };
            c.w64(2, t32);
            return c.call(0x7000859a54, 0x70008cad4b, 0x70008cad46);
        }
        22 => {
            c.w64(12, c.r64(0));
            blk = 68; continue;
        }
        23 => {
            let Some(t33) = c.ld32(c.r64(12).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008cad53); };
            c.w32(11, t33);
            c.cmp(c.r32(11), 0x7eu64, 32);
            if c.cc_a() { blk = 268; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t34) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cad62); };
            c.logic(t34 & 0x7u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(3));
            blk = 27; continue;
        }
        26 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008cad74); }
            blk = 27; continue;
        }
        27 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cad78); };
            c.w64(7, t35);
            let r36 = c.r32(7) & 0x7u64;
            c.w32(7, r36);
            c.logic(r36, 32);
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_l() { blk = 32; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 256; continue;
        }
        32 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(7), 0x1u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 256; continue;
        }
        35 => {
            let Some(t37) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cadab); };
            c.w32(7, t37);
            let Some(t38) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cadae); };
            c.cmp(t38, c.r32(7), 32);
            if c.cc_ne() { blk = 257; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t39) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cadbb); };
            c.w32(1, t39);
            let r40 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r40);
            let Some(t41) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cadc1); };
            c.w32(7, t41);
            let r42 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r42);
            let Some(t43) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cadc7); };
            c.w32(7, t43);
            blk = 26; continue;
        }
        37 => {
            let Some(t44) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cadcd); };
            c.w32(7, t44);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let r45 = c.add(c.r64(15), c.r64(7), 64);
            c.w64(15, r45);
            let Some(t46) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cadd9); };
            c.w32(1, t46);
            let r47 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r47);
            let Some(t48) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008caddf); };
            c.w32(7, t48);
            let r49 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r49);
            let Some(t50) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cade5); };
            c.w32(7, t50);
            blk = 26; continue;
        }
        38 => {
            let Some(t51) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cadeb); };
            c.w32(1, t51);
            let r52 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r52);
            let Some(t53) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cadf1); };
            c.w32(7, t53);
            let r54 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r54);
            let Some(t55) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cadf7); };
            c.w32(7, t55);
            blk = 26; continue;
        }
        39 => {
            c.w64(7, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 42; continue;
        }
        40 => {
            c.w64(7, c.r64(1));
            c.w32(1, 0x1u64);
            blk = 42; continue;
        }
        41 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(3));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x70008cae19); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 258; continue; }
            blk = 43; continue;
        }
        43 => {
            let r56 = c.add(c.r64(7), 0xbu64, 64);
            c.w64(7, r56);
            blk = 44; continue;
        }
        44 => {
            c.w64(1, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008cae30); };
            c.w64(3, t57);
            c.w64(2, c.r64(15).wrapping_add(c.r64(3)));
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cae38); };
            c.w64(3, t58);
            let r59 = c.r32(3) & 0x8u64;
            c.w32(3, r59);
            c.logic(r59, 32);
            let r60 = c.add(c.r64(15), c.r64(1), 64);
            c.w64(15, r60);
            c.cmp(c.r32(3), 0x8u64, 32);
            if c.cc_ne() { blk = 260; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(3, c.r64(2).wrapping_add(c.r64(7)));
            let r61 = c.add(c.r64(7), c.r64(15), 64);
            c.w64(7, r61);
            let Some(t62) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cae52); };
            c.w32(15, t62);
            let r63 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r63);
            blk = 46; continue;
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008cae5a); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 267; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(2, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let Some(t64) = c.ld32(c.r64(15).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008cae6e); };
            c.w32(1, t64);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2e70)) else { return RegfileExit::fault(0x70008cae72); };
            c.w64(7, t65);
            let Some(t66) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008cae79); };
            c.w64(7, t66);
            c.w32(2, c.r32(2));
            c.cmp(c.r32(1), 0xffffffffffffffffu64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(15, c.r64(1).wrapping_mul(8));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(13));
            let Some(t67) = c.ld64(c.r64(1).wrapping_add(c.r64(15)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008cae91); };
            c.w64(15, t67);
            blk = 50; continue;
        }
        49 => {
            let r68 = c.add(c.r64(15), 0x3fu64, 64);
            c.w64(15, r68);
            c.w64(1, c.r64(13));
            blk = 50; continue;
        }
        50 => {
            let r69 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r69);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(10, c.r64(4));
            let r70 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r70);
            let r71 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r71);
            c.logic(r71, 64);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x70008caeb7); }
            let r72 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r72);
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008caebf); };
            c.w64(12, t73);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(12)) { return RegfileExit::fault(0x70008caec6); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(11)) { return RegfileExit::fault(0x70008caecb); }
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(7)) { return RegfileExit::fault(0x70008caed0); }
            if !c.st64(c.r64(4).wrapping_add(0x38), 0x1u64) { return RegfileExit::fault(0x70008caed5); }
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(1)) { return RegfileExit::fault(0x70008caede); }
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(0)) { return RegfileExit::fault(0x70008caee3); }
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008caee8); };
            c.w64(1, t74);
            c.w64(9, c.r64(3));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008caeef); };
            c.w64(8, t75);
            c.w64(10, 0x70008caefau64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008caefa); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008caefe); }
            return c.call(c.r64(15), 0x70008caf05, 0x70008caf02);
        }
        51 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008caf05); }
            let Some(t76) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x70008caf0d); };
            c.w64(4, t76);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_e() { blk = 54; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_e() { blk = 67; continue; }
            blk = 53; continue;
        }
        53 => {
            blk = 261; continue;
        }
        54 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008caf2a); };
            c.w64(7, t77);
            let Some(t78) = c.ld32(c.r64(7)) else { return RegfileExit::fault(0x70008caf31); };
            c.w64(11, (t78 as u32 as i32 as i64 as u64));
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008caf34); };
            c.w64(12, t79);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_be() { blk = 67; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008caf41); };
            c.w64(12, t80);
            let Some(t81) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008caf48); };
            c.w32(11, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008caf4d); };
            c.w64(15, t82);
            let Some(t83) = c.ld32(c.r64(15).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008caf51); };
            c.w32(15, t83);
            let r84 = c.add(c.r32(15), 0x2u64, 32);
            c.w32(15, r84);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r85 = c.sub(c.r32(11), 0x6u64, 32);
            c.w32(11, r85);
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_l() { blk = 266; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(11, (c.r32(15) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008caf6f); }
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008caf74); };
            c.w64(0, t86);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008caf78); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r87 = c.r64(15) & c.r64(12);
            c.w64(15, r87);
            c.logic(r87, 64);
            let Some(t88) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008caf8b); };
            c.logic(t88 & 0x4u64, 8);
            if c.cc_ne() { blk = 366; continue; }
            blk = 58; continue;
        }
        58 => {
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008caf96); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r89 = c.r64(15) & c.r64(12);
            c.w64(15, r89);
            c.logic(r89, 64);
            let Some(t90) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cafa9); };
            c.logic(t90 & 0x4u64, 8);
            if c.cc_ne() { blk = 363; continue; }
            blk = 60; continue;
        }
        60 => {
            if let Some(sr) = c.shl(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if c.cc_e() { blk = 68; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w32(15, 0x13u64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 63; continue;
        }
        62 => {
            let r91 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r91);
            c.w64(3, c.r64(2));
            blk = 63; continue;
        }
        63 => {
            c.w64(2, c.r64(3).wrapping_add(0x4));
            let Some(t92) = c.ld32(c.r64(7).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x70008cafce); };
            c.w32(3, t92);
            let r93 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r93);
            if !c.st32(c.r64(12).wrapping_add(c.r64(15)), c.r32(3)) { return RegfileExit::fault(0x70008cafd3); }
            c.cmp(c.r64(11), c.r64(2), 64);
            if c.cc_ne() { blk = 62; continue; }
            blk = 64; continue;
        }
        64 => {
            blk = 68; continue;
        }
        65 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x70008cafde); }
            blk = 67; continue;
        }
        66 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x70008cafe4); }
            blk = 67; continue;
        }
        67 => {
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008cafe8); };
            c.w64(12, t94);
            blk = 68; continue;
        }
        68 => {
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(12)) { return RegfileExit::fault(0x70008cafef); }
            let Some(t95) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008caff6); };
            c.cmp(t95, c.r32(12), 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t96) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cb003); };
            c.w32(1, t96);
            let r97 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r97);
            let Some(t98) = c.ld32(c.r64(12).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008cb00b); };
            c.w32(7, t98);
            let r99 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r99);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x70008cb013); }
            let Some(t100) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cb017); };
            c.cmp(c.r32(7), t100, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cb01c); };
            c.w64(3, t101);
            blk = 72; continue;
        }
        71 => {
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cb022); };
            c.w64(3, t102);
            c.cmp(c.r32(3), c.r32(7), 32);
            if c.cc_e() { blk = 133; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cb02e); };
            c.w64(0, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cb032); };
            c.w64(6, t104);
            return c.call(0x700086fe54, 0x70008cb03b, 0x70008cb036);
        }
        73 => {
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cb03b); };
            c.w64(12, t105);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008cb03f); }
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cb043); };
            c.cmp(c.r64(12), t106, 64);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w64(7, c.r64(12));
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cb04c); };
            c.w64(12, t107);
            blk = 95; continue;
        }
        75 => {
            c.w64(7, c.r64(12));
            if let Some(sr) = c.shr(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let r108 = c.add(c.r64(7), c.r64(12), 64);
            c.w64(7, r108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cb05e); };
            c.w64(11, t109);
            let Some(t110) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cb062); };
            c.w32(12, t110);
            let r111 = c.add(c.r64(7), 0x10u64, 64);
            c.w64(7, r111);
            if c.cc_e() { blk = 93; continue; }
            blk = 76; continue;
        }
        76 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008cb070); }
            let Some(t112) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008cb074); };
            c.cmp(t112, c.r32(12), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008cb07d); };
            c.w64(12, t113);
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 317; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cb091); };
            c.w64(15, t114);
            c.w64(3, c.r64(15).wrapping_mul(4).wrapping_add(0x7));
            c.w32(2, c.r64(7).wrapping_add(c.r64(7)));
            c.w64(1, c.r64(7).wrapping_mul(4).wrapping_add(0x8));
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cb0a8); };
            c.w64(6, t115);
            let Some(t116) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008cb0af); };
            c.w64(6, t116);
            c.w64(8, c.r64(1).wrapping_add(c.r64(6)));
            c.w64(9, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008cb0be); };
            c.w64(7, t117);
            let Some(t118) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008cb0c5); };
            c.cmp(t118, c.r64(8), 64);
            if c.cc_be() { blk = 271; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 271; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cb0db); };
            c.w64(7, t119);
            if !c.st64(c.r64(7), c.r64(8)) { return RegfileExit::fault(0x70008cb0e2); }
            c.w64(7, c.r64(6).wrapping_add(0x1));
            blk = 82; continue;
        }
        82 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008cb0e9); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008cb0ed); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008cb0f0); }
            c.cmp(c.r64(9), c.r64(3), 64);
            if c.cc_e() { blk = 85; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008cb0f9); };
            c.w64(12, t120);
            blk = 84; continue;
        }
        84 => {
            let r121 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r121);
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(12)) { return RegfileExit::fault(0x70008cb104); }
            c.cmp(c.r64(9), c.r64(3), 64);
            if c.cc_ne() { blk = 84; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w64(12, c.r64(7));
            let r122 = c.r64(12) & 0xfffffffffffc0000u64;
            c.w64(12, r122);
            c.logic(r122, 64);
            let Some(t123) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cb117); };
            c.logic(t123 & 0x4u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_e() { blk = 92; continue; }
            blk = 87; continue;
        }
        87 => {
            c.w32(12, 0x7u64);
            blk = 88; continue;
        }
        88 => {
            let Some(t124) = c.ld32(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008cb12b); };
            c.w32(2, t124);
            if !c.st32(c.r64(7).wrapping_add(c.r64(12)), c.r32(2)) { return RegfileExit::fault(0x70008cb12f); }
            let r125 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r125);
            c.cmp(c.r64(3), c.r64(12), 64);
            if c.cc_ne() { blk = 88; continue; }
            blk = 89; continue;
        }
        89 => {
            blk = 92; continue;
        }
        90 => {
            c.w64(8, c.r64(15).wrapping_mul(4));
            c.w64(2, c.r64(11).wrapping_add(0x7));
            c.w64(1, c.r64(7).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r126 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r126);
            let r127 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r127);
            c.logic(r127, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008cb159); }
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x70008cb15e); };
            c.w64(0, t128);
            c.w64(10, 0x70008cb16cu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008cb16c); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008cb170); }
            return c.call(c.r64(0), 0x70008cb176, 0x70008cb174);
        }
        91 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008cb176); }
            let Some(t129) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008cb17e); };
            c.w64(4, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cb183); };
            c.w64(15, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cb187); };
            c.w64(0, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cb18b); };
            c.w64(7, t132);
            blk = 92; continue;
        }
        92 => {
            c.w64(11, c.r64(7));
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cb192); };
            c.w64(7, t133);
            blk = 94; continue;
        }
        93 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008cb198); }
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008cb19c); };
            c.w64(11, t134);
            c.w64(12, c.r64(11));
            blk = 94; continue;
        }
        94 => {
            c.w64(12, c.r64(11));
            blk = 95; continue;
        }
        95 => {
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cb1a9); };
            c.w64(15, t135);
            c.w64(11, c.r64(15).wrapping_mul(4).wrapping_add(0x7));
            let Some(t136) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cb1b5); };
            c.w64(3, (t136 as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(15).wrapping_add(0x1));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008cb1c0); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 97; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r137 = c.r64(1) & c.r64(12);
            c.w64(1, r137);
            c.logic(r137, 64);
            let Some(t138) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cb1d2); };
            c.logic(t138 & 0x4u64, 8);
            if c.cc_ne() { blk = 360; continue; }
            blk = 97; continue;
        }
        97 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x70008cb1dc); }
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_e() { blk = 232; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008cb1e9); };
            c.w64(11, t139);
            let Some(t140) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cb1f0); };
            c.w32(0, t140);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_be() { blk = 319; continue; }
            blk = 99; continue;
        }
        99 => {
            let r141 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r141);
            let Some(t142) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cb208); };
            c.w32(0, t142);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008cb212); }
            c.cmp(c.r64(0), 0x2u64, 64);
            if c.cc_le() { blk = 132; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w32(2, 0x2u64);
            blk = 101; continue;
        }
        101 => {
            let Some(t143) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cb225); };
            c.w32(1, t143);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(6, c.r64(2).wrapping_add(0x3));
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x70008cb233); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008cb237); }
            c.cmp(c.r64(6), c.r64(1), 64);
            if c.cc_ae() { blk = 320; continue; }
            blk = 102; continue;
        }
        102 => {
            c.w64(6, c.r64(2).wrapping_add(0x4));
            c.w64(8, c.r64(2).wrapping_mul(4));
            let Some(t144) = c.ld32(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cb250); };
            c.w32(9, t144);
            let r145 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r145);
            c.cmp(c.r64(6), c.r64(1), 64);
            if c.cc_ae() { blk = 321; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t146) = c.ld32(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008cb261); };
            c.w32(1, t146);
            let r147 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r147);
            c.cmp(c.r32(1), 0xfffffffffffffffeu64, 32);
            if c.cc_e() { blk = 106; continue; }
            blk = 104; continue;
        }
        104 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008cb26e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008cb272); }
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cb276); };
            c.w64(0, t148);
            c.w64(3, c.r64(9));
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cb27d); };
            c.w64(6, t149);
            return c.call(0x700086fe54, 0x70008cb286, 0x70008cb281);
        }
        105 => {
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008cb286); };
            c.w64(11, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cb28d); };
            c.w64(2, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cb291); };
            c.w64(15, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cb295); };
            c.w64(7, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cb299); };
            c.w64(12, t154);
            blk = 107; continue;
        }
        106 => {
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cb29f); };
            c.w64(0, t155);
            blk = 107; continue;
        }
        107 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(0)) { return RegfileExit::fault(0x70008cb2a6); }
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_ne() { blk = 128; continue; }
            blk = 108; continue;
        }
        108 => {
            c.w64(1, c.r64(7));
            if let Some(sr) = c.shr(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r156 = c.add(c.r64(7), c.r64(1), 64);
            c.w64(7, r156);
            let Some(t157) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cb2bc); };
            c.w32(1, t157);
            let r158 = c.add(c.r64(7), 0x10u64, 64);
            c.w64(7, r158);
            if c.cc_e() { blk = 127; continue; }
            blk = 109; continue;
        }
        109 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008cb2cb); }
            let Some(t159) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008cb2cf); };
            c.cmp(t159, c.r32(1), 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008cb2d8); };
            c.w64(1, t160);
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 322; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w64(6, c.r64(15).wrapping_mul(4).wrapping_add(0x7));
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cb2f4); };
            c.w64(8, t161);
            let Some(t162) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008cb2fb); };
            c.w64(8, t162);
            c.w64(9, c.r64(7).wrapping_mul(4).wrapping_add(0x8));
            c.w64(11, c.r64(9).wrapping_add(c.r64(8)));
            let Some(t163) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008cb30a); };
            c.w64(0, t163);
            let Some(t164) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008cb311); };
            c.cmp(t164, c.r64(11), 64);
            if c.cc_be() { blk = 273; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r64(9), 0x20000u64, 64);
            if c.cc_ae() { blk = 273; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cb327); };
            c.w64(0, t165);
            if !c.st64(c.r64(0), c.r64(11)) { return RegfileExit::fault(0x70008cb32e); }
            c.w64(11, c.r64(8).wrapping_add(0x1));
            blk = 115; continue;
        }
        115 => {
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008cb338); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008cb33c); }
            c.w64(0, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(11)) { return RegfileExit::fault(0x70008cb348); }
            c.cmp(c.r64(0), c.r64(6), 64);
            if c.cc_e() { blk = 118; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t166) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008cb351); };
            c.w64(1, t166);
            blk = 117; continue;
        }
        117 => {
            let r167 = c.sub(c.r64(0), 0x4u64, 64);
            c.w64(0, r167);
            if !c.st32(c.r64(0).wrapping_add(c.r64(11)), c.r32(1)) { return RegfileExit::fault(0x70008cb35c); }
            c.cmp(c.r64(0), c.r64(6), 64);
            if c.cc_ne() { blk = 117; continue; }
            blk = 118; continue;
        }
        118 => {
            c.w64(0, c.r64(11));
            let r168 = c.r64(0) & 0xfffffffffffc0000u64;
            c.w64(0, r168);
            c.logic(r168, 64);
            let Some(t169) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cb36e); };
            c.logic(t169 & 0x4u64, 8);
            if c.cc_e() { blk = 124; continue; }
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r64(6), 0x7u64, 64);
            if c.cc_e() { blk = 126; continue; }
            blk = 120; continue;
        }
        120 => {
            c.w32(0, 0x7u64);
            blk = 122; continue;
        }
        121 => {
            c.w64(0, c.r64(1));
            blk = 122; continue;
        }
        122 => {
            c.w64(1, c.r64(0).wrapping_add(0x4));
            let Some(t170) = c.ld32(c.r64(12).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70008cb388); };
            c.w32(8, t170);
            if !c.st32(c.r64(11).wrapping_add(c.r64(0)), c.r32(8)) { return RegfileExit::fault(0x70008cb38c); }
            c.cmp(c.r64(6), c.r64(1), 64);
            if c.cc_ne() { blk = 121; continue; }
            blk = 123; continue;
        }
        123 => {
            blk = 126; continue;
        }
        124 => {
            c.w64(8, c.r64(15).wrapping_mul(4));
            let r171 = c.add(c.r64(12), 0x7u64, 64);
            c.w64(12, r171);
            c.w64(1, c.r64(11).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r172 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r172);
            let r173 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r173);
            c.logic(r173, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008cb3b2); }
            c.w64(2, c.r64(12));
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x70008cb3ba); };
            c.w64(0, t174);
            c.w64(10, 0x70008cb3c8u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008cb3c8); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008cb3cc); }
            return c.call(c.r64(0), 0x70008cb3d2, 0x70008cb3d0);
        }
        125 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008cb3d2); }
            let Some(t175) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008cb3da); };
            c.w64(4, t175);
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cb3df); };
            c.w64(2, t176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cb3e3); };
            c.w64(15, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cb3e7); };
            c.w64(7, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008cb3eb); };
            c.w64(11, t179);
            blk = 126; continue;
        }
        126 => {
            c.w64(12, c.r64(11));
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cb3f2); };
            c.w64(0, t180);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008cb3f6); };
            c.w64(11, t181);
            blk = 128; continue;
        }
        127 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008cb3ff); }
            let Some(t182) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008cb403); };
            c.w64(12, t182);
            blk = 128; continue;
        }
        128 => {
            c.w64(3, c.r64(15).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(12).wrapping_add(c.r64(3)), c.r32(0)) { return RegfileExit::fault(0x70008cb412); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 130; continue; }
            blk = 129; continue;
        }
        129 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r183 = c.r64(1) & c.r64(12);
            c.w64(1, r183);
            c.logic(r183, 64);
            let Some(t184) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cb424); };
            c.logic(t184 & 0x4u64, 8);
            if c.cc_ne() { blk = 357; continue; }
            blk = 130; continue;
        }
        130 => {
            let r185 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r185);
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008cb432); };
            c.w64(0, t186);
            c.cmp(c.r64(0), c.r64(15), 64);
            if c.cc_e() { blk = 204; continue; }
            blk = 131; continue;
        }
        131 => {
            let r187 = c.add(c.r64(2), 0x2u64, 64);
            c.w64(2, r187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cb443); };
            c.cmp(c.r64(2), t188, 64);
            if c.cc_l() { blk = 101; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008cb44d); };
            c.w64(3, t189);
            c.w64(0, c.r64(12));
            c.w64(12, c.r64(7));
            c.w64(7, c.r64(15));
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cb45a); };
            c.w64(15, t190);
            c.w64(11, c.r64(3));
            blk = 11; continue;
        }
        133 => {
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cb466); };
            c.w64(0, t191);
            let Some(t192) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cb46a); };
            c.w32(7, t192);
            let Some(t193) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008cb46d); };
            c.cmp(t193, 0x0u64, 32);
            if c.cc_ne() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cb473); };
            c.w64(9, t194);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008cb477); }
            blk = 138; continue;
        }
        135 => {
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008cb47f); };
            c.w64(11, t195);
            let r196 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r196);
            if c.cc_o() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cb48c); };
            c.w64(9, t197);
            blk = 141; continue;
        }
        137 => {
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cb492); };
            c.w64(9, t198);
            let r199 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r199);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cb4a2); };
            c.w64(9, t200);
            blk = 138; continue;
        }
        138 => {
            let Some(t201) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cb4a6); };
            c.w64(11, t201);
            let Some(t202) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008cb4ad); };
            c.w64(12, t202);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t203) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008cb4b5); };
            c.w64(2, t203);
            let Some(t204) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008cb4bc); };
            c.cmp(t204, c.r64(15), 64);
            if c.cc_be() { blk = 275; continue; }
            blk = 139; continue;
        }
        139 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008cb4c5); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 140; continue;
        }
        140 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008cb4df); };
            c.w64(12, t205);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008cb4e6); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008cb4ea); }
            blk = 141; continue;
        }
        141 => {
            c.w32(7, c.r32(7));
            let r206 = c.r32(7) & 0x20u64;
            c.w32(7, r206);
            c.logic(r206, 32);
            let r207 = c.shr(c.r32(7), 0x4u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r207);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r208 = c.r64(7) & 0xfffffffffffffffeu64;
            c.w64(7, r208);
            c.logic(r208, 64);
            let r209 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r209);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 143; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cb506); };
            c.w64(15, t210);
            blk = 202; continue;
        }
        143 => {
            c.w32(7, c.r32(11));
            c.logic(c.r32(11) & 0x80000001u64, 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cb51b); };
            c.w64(15, t211);
            blk = 202; continue;
        }
        145 => {
            c.w64(12, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cb527); };
            c.w64(15, t212);
            let Some(t213) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cb52b); };
            c.w32(2, t213);
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), c.r64(2), 64);
            if c.cc_ae() { blk = 202; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t214) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cb53b); };
            c.w32(2, t214);
            let r215 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r215);
            let Some(t216) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cb542); };
            c.w32(2, t216);
            c.logic(c.r8(2) & 0x7u64, 8);
            if c.cc_e() { blk = 163; continue; }
            blk = 147; continue;
        }
        147 => {
            c.w64(1, c.r64(15));
            c.w64(8, c.r64(2));
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(2));
            blk = 149; continue;
        }
        148 => {
            c.w64(8, c.r64(0));
            blk = 149; continue;
        }
        149 => {
            c.w64(0, c.r64(8));
            let r217 = c.r32(0) & 0x7u64;
            c.w32(0, r217);
            c.logic(r217, 32);
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_l() { blk = 154; continue; }
            blk = 150; continue;
        }
        150 => {
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_e() { blk = 162; continue; }
            blk = 151; continue;
        }
        151 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 159; continue; }
            blk = 152; continue;
        }
        152 => {
            c.cmp(c.r32(0), 0x5u64, 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 153; continue;
        }
        153 => {
            blk = 277; continue;
        }
        154 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 161; continue; }
            blk = 155; continue;
        }
        155 => {
            c.cmp(c.r32(0), 0x1u64, 32);
            if c.cc_e() { blk = 157; continue; }
            blk = 156; continue;
        }
        156 => {
            blk = 277; continue;
        }
        157 => {
            let Some(t218) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cb591); };
            c.w32(0, t218);
            let Some(t219) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cb594); };
            c.cmp(t219, c.r32(0), 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t220) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cb5a1); };
            c.w32(1, t220);
            let r221 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r221);
            let Some(t222) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cb5a7); };
            c.w32(0, t222);
            let r223 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r223);
            let Some(t224) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cb5ad); };
            c.w32(0, t224);
            blk = 148; continue;
        }
        159 => {
            let Some(t225) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cb5b3); };
            c.w32(0, t225);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r226 = c.add(c.r64(2), c.r64(0), 64);
            c.w64(2, r226);
            let Some(t227) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cb5bf); };
            c.w32(1, t227);
            let r228 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r228);
            let Some(t229) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cb5c5); };
            c.w32(0, t229);
            let r230 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r230);
            let Some(t231) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cb5cb); };
            c.w32(0, t231);
            blk = 148; continue;
        }
        160 => {
            let Some(t232) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cb5d1); };
            c.w32(1, t232);
            let r233 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r233);
            let Some(t234) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cb5d7); };
            c.w32(0, t234);
            let r235 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r235);
            let Some(t236) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cb5dd); };
            c.w32(0, t236);
            blk = 148; continue;
        }
        161 => {
            c.w64(0, c.r64(2));
            c.w64(2, c.r64(8));
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 164; continue;
        }
        162 => {
            c.w64(0, c.r64(2));
            c.w64(2, c.r64(8));
            c.w32(8, 0x1u64);
            blk = 164; continue;
        }
        163 => {
            c.w32(8, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(15));
            c.w64(6, c.r64(0));
            blk = 164; continue;
        }
        164 => {
            c.w64(3, (c.r32(9) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 279; continue; }
            blk = 165; continue;
        }
        165 => {
            let r237 = c.add(c.r64(1), 0xbu64, 64);
            c.w64(1, r237);
            blk = 166; continue;
        }
        166 => {
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r238 = c.r32(2) & 0x8u64;
            c.w32(2, r238);
            c.logic(r238, 32);
            let r239 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r239);
            c.cmp(c.r32(2), 0x8u64, 32);
            if c.cc_e() { blk = 168; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t240) = c.ld16(c.r64(1).wrapping_add(c.r64(0).wrapping_mul(2))) else { return RegfileExit::fault(0x70008cb629); };
            c.w32(0, t240);
            blk = 169; continue;
        }
        168 => {
            let Some(t241) = c.ld8(c.r64(0).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008cb62f); };
            c.w32(0, t241);
            blk = 169; continue;
        }
        169 => {
            let r242 = c.r32(0) & 0xfc00u64;
            c.w32(0, r242);
            c.logic(r242, 32);
            c.cmp(c.r32(0), 0xd800u64, 32);
            if c.cc_ne() { blk = 202; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t243) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cb643); };
            c.w32(0, t243);
            let r244 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r244);
            let Some(t245) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cb64a); };
            c.w32(0, t245);
            c.logic(c.r8(0) & 0x7u64, 8);
            if c.cc_e() { blk = 186; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w64(3, c.r64(15));
            c.w64(2, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(0));
            blk = 172; continue;
        }
        172 => {
            c.w64(1, c.r64(2));
            let r246 = c.r32(1) & 0x7u64;
            c.w32(1, r246);
            c.logic(r246, 32);
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_l() { blk = 177; continue; }
            blk = 173; continue;
        }
        173 => {
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_e() { blk = 185; continue; }
            blk = 174; continue;
        }
        174 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 175; continue;
        }
        175 => {
            c.cmp(c.r32(1), 0x5u64, 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 176; continue;
        }
        176 => {
            blk = 295; continue;
        }
        177 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 178; continue;
        }
        178 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 179; continue;
        }
        179 => {
            blk = 295; continue;
        }
        180 => {
            let Some(t247) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cb693); };
            c.w32(2, t247);
            let Some(t248) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cb696); };
            c.cmp(t248, c.r32(2), 32);
            if c.cc_ne() { blk = 296; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t249) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cb6a3); };
            c.w32(3, t249);
            let r250 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r250);
            let Some(t251) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cb6a9); };
            c.w32(2, t251);
            let r252 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r252);
            let Some(t253) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cb6af); };
            c.w32(2, t253);
            blk = 172; continue;
        }
        182 => {
            let Some(t254) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cb6b5); };
            c.w32(2, t254);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r255 = c.add(c.r64(0), c.r64(2), 64);
            c.w64(0, r255);
            let Some(t256) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cb6c1); };
            c.w32(3, t256);
            let r257 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r257);
            let Some(t258) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cb6c7); };
            c.w32(2, t258);
            let r259 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r259);
            let Some(t260) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cb6cd); };
            c.w32(2, t260);
            blk = 172; continue;
        }
        183 => {
            let Some(t261) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cb6d3); };
            c.w32(3, t261);
            let r262 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r262);
            let Some(t263) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cb6d9); };
            c.w32(2, t263);
            let r264 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r264);
            let Some(t265) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cb6df); };
            c.w32(2, t265);
            blk = 172; continue;
        }
        184 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(2));
            c.w64(2, c.r64(10));
            blk = 187; continue;
        }
        185 => {
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(2));
            c.w64(2, c.r64(10));
            c.w32(1, 0x1u64);
            blk = 187; continue;
        }
        186 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.w32(3, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(15));
            c.w64(6, c.r64(3));
            blk = 187; continue;
        }
        187 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 297; continue; }
            blk = 188; continue;
        }
        188 => {
            let r266 = c.add(c.r64(2), 0xbu64, 64);
            c.w64(2, r266);
            blk = 189; continue;
        }
        189 => {
            let r267 = c.add(c.r64(12), c.r64(3), 64);
            c.w64(12, r267);
            let r268 = c.r32(0) & 0x8u64;
            c.w32(0, r268);
            c.logic(r268, 32);
            c.cmp(c.r32(0), 0x8u64, 32);
            if c.cc_e() { blk = 191; continue; }
            blk = 190; continue;
        }
        190 => {
            let Some(t269) = c.ld16(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(2))) else { return RegfileExit::fault(0x70008cb739); };
            c.w32(12, t269);
            blk = 192; continue;
        }
        191 => {
            let Some(t270) = c.ld8(c.r64(12).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008cb740); };
            c.w32(12, t270);
            blk = 192; continue;
        }
        192 => {
            let r271 = c.r32(12) & 0xfc00u64;
            c.w32(12, r271);
            c.logic(r271, 32);
            c.cmp(c.r32(12), 0xdc00u64, 32);
            if c.cc_ne() { blk = 202; continue; }
            blk = 193; continue;
        }
        193 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 195; continue; }
            blk = 194; continue;
        }
        194 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008cb75b); }
            blk = 198; continue;
        }
        195 => {
            let r272 = c.add(c.r32(7), 0x2u64, 32);
            c.w32(7, r272);
            if c.cc_o() { blk = 197; continue; }
            blk = 196; continue;
        }
        196 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 201; continue;
        }
        197 => {
            let r273 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r273);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 198; continue;
        }
        198 => {
            let Some(t274) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cb779); };
            c.w64(7, t274);
            let Some(t275) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008cb780); };
            c.w64(11, t275);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t276) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008cb787); };
            c.w64(0, t276);
            let Some(t277) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008cb78e); };
            c.cmp(t277, c.r64(12), 64);
            if c.cc_be() { blk = 312; continue; }
            blk = 199; continue;
        }
        199 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x70008cb797); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 200; continue;
        }
        200 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t278) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008cb7b0); };
            c.w64(11, t278);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008cb7b7); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70008cb7bb); }
            blk = 201; continue;
        }
        201 => {
            c.w64(11, c.r64(7));
            blk = 202; continue;
        }
        202 => {
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cb7c3); };
            c.w64(3, t279);
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cb7c7); };
            c.w64(0, t280);
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cb7cb); };
            c.w64(7, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cb7cf); };
            c.w64(12, t282);
            blk = 11; continue;
        }
        203 => {
            return RegfileExit::fault(0x70008cb7d8);
        }
        204 => {
            c.w64(2, c.r64(15));
            blk = 232; continue;
        }
        205 => {
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cb7e1); };
            c.w64(3, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cb7e5); };
            c.w64(6, t284);
            blk = 206; continue;
        }
        206 => {
            c.w64(0, c.r64(3));
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cb7ec); };
            c.w64(3, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008cb7f0); };
            c.w64(1, t286);
            return c.call(0x700086fe54, 0x70008cb7fc, 0x70008cb7f7);
        }
        207 => {
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cb7fc); };
            c.w64(12, t287);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008cb800); }
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cb804); };
            c.cmp(c.r64(12), t288, 64);
            if c.cc_e() { blk = 209; continue; }
            blk = 208; continue;
        }
        208 => {
            c.w64(7, c.r64(12));
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cb80d); };
            c.w64(12, t289);
            blk = 229; continue;
        }
        209 => {
            c.w64(7, c.r64(12));
            if let Some(sr) = c.shr(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let r290 = c.add(c.r64(7), c.r64(12), 64);
            c.w64(7, r290);
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cb81f); };
            c.w64(11, t291);
            let Some(t292) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cb823); };
            c.w32(15, t292);
            let r293 = c.add(c.r64(7), 0x10u64, 64);
            c.w64(7, r293);
            if c.cc_e() { blk = 227; continue; }
            blk = 210; continue;
        }
        210 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            c.w8(3, (c.cc_g() as u64));
            c.w32(3, c.r8(3));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008cb83e); }
            let Some(t294) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008cb842); };
            c.cmp(t294, c.r32(15), 32);
            if c.cc_ne() { blk = 212; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t295) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008cb84b); };
            c.w64(15, t295);
            blk = 212; continue;
        }
        212 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 326; continue; }
            blk = 213; continue;
        }
        213 => {
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cb85b); };
            c.w64(3, t296);
            c.w64(2, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            c.w64(1, c.r64(12));
            if let Some(sr) = c.shr(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(1, c.r64(1).wrapping_add(c.r64(12)).wrapping_add(0x10));
            c.w32(6, c.r64(1).wrapping_add(c.r64(1)));
            let Some(t297) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cb875); };
            c.w64(8, t297);
            let Some(t298) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008cb87c); };
            c.w64(8, t298);
            c.w64(9, c.r64(1).wrapping_mul(4).wrapping_add(0x8));
            c.w64(7, c.r64(9).wrapping_add(c.r64(8)));
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(1), c.r64(2), 64);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t299) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008cb89c); };
            c.w64(0, t299);
            let Some(t300) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008cb8a3); };
            c.cmp(t300, c.r64(7), 64);
            if c.cc_be() { blk = 324; continue; }
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r64(9), 0x20000u64, 64);
            if c.cc_ae() { blk = 324; continue; }
            blk = 215; continue;
        }
        215 => {
            let Some(t301) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cb8b9); };
            c.w64(0, t301);
            if !c.st64(c.r64(0), c.r64(7)) { return RegfileExit::fault(0x70008cb8c0); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 216; continue;
        }
        216 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008cb8c7); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x70008cb8cb); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008cb8ce); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 219; continue; }
            blk = 217; continue;
        }
        217 => {
            c.w64(15, c.r64(12));
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w64(12, c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0x10));
            c.w64(12, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            let Some(t302) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008cb8ea); };
            c.w64(15, t302);
            blk = 218; continue;
        }
        218 => {
            let r303 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r303);
            if !c.st32(c.r64(12).wrapping_add(c.r64(7)), c.r32(15)) { return RegfileExit::fault(0x70008cb8f5); }
            c.cmp(c.r64(12), c.r64(2), 64);
            if c.cc_ne() { blk = 218; continue; }
            blk = 219; continue;
        }
        219 => {
            c.w64(12, c.r64(7));
            let r304 = c.r64(12) & 0xfffffffffffc0000u64;
            c.w64(12, r304);
            c.logic(r304, 64);
            let Some(t305) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cb908); };
            c.logic(t305 & 0x4u64, 8);
            if c.cc_e() { blk = 224; continue; }
            blk = 220; continue;
        }
        220 => {
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_e() { blk = 226; continue; }
            blk = 221; continue;
        }
        221 => {
            c.w32(12, 0x7u64);
            blk = 222; continue;
        }
        222 => {
            let Some(t306) = c.ld32(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008cb91c); };
            c.w32(15, t306);
            if !c.st32(c.r64(7).wrapping_add(c.r64(12)), c.r32(15)) { return RegfileExit::fault(0x70008cb920); }
            let r307 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r307);
            c.cmp(c.r64(2), c.r64(12), 64);
            if c.cc_ne() { blk = 222; continue; }
            blk = 223; continue;
        }
        223 => {
            blk = 226; continue;
        }
        224 => {
            c.w64(8, c.r64(3).wrapping_mul(4));
            c.w64(2, c.r64(11).wrapping_add(0x7));
            c.w64(1, c.r64(7).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r308 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r308);
            let r309 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r309);
            c.logic(r309, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008cb94a); }
            let Some(t310) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x70008cb94f); };
            c.w64(0, t310);
            c.w64(10, 0x70008cb95du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008cb95d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008cb961); }
            return c.call(c.r64(0), 0x70008cb967, 0x70008cb965);
        }
        225 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008cb967); }
            let Some(t311) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008cb96f); };
            c.w64(4, t311);
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cb974); };
            c.w64(3, t312);
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cb978); };
            c.w64(7, t313);
            blk = 226; continue;
        }
        226 => {
            c.w64(11, c.r64(7));
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cb97f); };
            c.w64(0, t314);
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cb983); };
            c.w64(7, t315);
            blk = 228; continue;
        }
        227 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008cb989); }
            let Some(t316) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008cb98d); };
            c.w64(11, t316);
            c.w64(12, c.r64(11));
            blk = 228; continue;
        }
        228 => {
            c.w64(12, c.r64(11));
            blk = 229; continue;
        }
        229 => {
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cb99a); };
            c.w64(11, t317);
            c.w64(15, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(12).wrapping_add(c.r64(15)), c.r32(0)) { return RegfileExit::fault(0x70008cb9a6); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 231; continue; }
            blk = 230; continue;
        }
        230 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r318 = c.r64(3) & c.r64(12);
            c.w64(3, r318);
            c.logic(r318, 64);
            let Some(t319) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cb9b8); };
            c.logic(t319 & 0x4u64, 8);
            if c.cc_ne() { blk = 354; continue; }
            blk = 231; continue;
        }
        231 => {
            c.w64(2, c.r64(11).wrapping_add(0x1));
            blk = 232; continue;
        }
        232 => {
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cb9c6); };
            c.w64(6, t320);
            let Some(t321) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cb9ca); };
            c.w32(11, t321);
            let r322 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r322);
            let Some(t323) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cb9d1); };
            c.w32(11, t323);
            let r324 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r324);
            let Some(t325) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cb9d8); };
            c.w32(15, t325);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70008cb9e2); }
            c.cmp(c.r64(15), 0x6bu64, 64);
            if c.cc_a() { blk = 234; continue; }
            blk = 233; continue;
        }
        233 => {
            return RegfileExit::fault(0x70008cb9ec);
        }
        234 => {
            let Some(t326) = c.ld32(c.r64(11).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008cb9ed); };
            c.w32(11, t326);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(11)) { return RegfileExit::fault(0x70008cb9f4); }
            c.cmp(c.r64(7), c.r64(2), 64);
            if c.cc_e() { blk = 251; continue; }
            blk = 235; continue;
        }
        235 => {
            let Some(t327) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cba01); };
            c.w32(7, t327);
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 249; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t328) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008cba10); };
            c.cmp(t328, c.r32(7), 32);
            if c.cc_ne() { blk = 238; continue; }
            blk = 237; continue;
        }
        237 => {
            let Some(t329) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008cba19); };
            c.w64(7, t329);
            blk = 238; continue;
        }
        238 => {
            c.cmp(c.r64(2), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 330; continue; }
            blk = 239; continue;
        }
        239 => {
            c.w32(15, c.r64(2).wrapping_add(c.r64(2)));
            c.w64(8, c.r64(2).wrapping_mul(4));
            let Some(t330) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cba39); };
            c.w64(0, t330);
            let Some(t331) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008cba40); };
            c.w64(0, t331);
            c.w64(3, c.r64(8).wrapping_add(0x8));
            c.w64(1, c.r64(3).wrapping_add(c.r64(0)));
            let Some(t332) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008cba4b); };
            c.w64(6, t332);
            let Some(t333) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008cba52); };
            c.cmp(t333, c.r64(1), 64);
            if c.cc_be() { blk = 328; continue; }
            blk = 240; continue;
        }
        240 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_ae() { blk = 328; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t334) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cba68); };
            c.w64(3, t334);
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x70008cba6f); }
            let r335 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r335);
            blk = 242; continue;
        }
        242 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008cba76); }
            c.w64(7, c.r64(0));
            let r336 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r336);
            c.logic(r336, 64);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008cba83); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008cba87); }
            let Some(t337) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cba8b); };
            c.logic(t337 & 0x4u64, 8);
            if c.cc_e() { blk = 247; continue; }
            blk = 243; continue;
        }
        243 => {
            c.w64(7, c.r64(8).wrapping_add(0x7));
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 250; continue; }
            blk = 244; continue;
        }
        244 => {
            c.w32(15, 0x7u64);
            blk = 245; continue;
        }
        245 => {
            let Some(t338) = c.ld32(c.r64(12).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x70008cbaa1); };
            c.w32(3, t338);
            if !c.st32(c.r64(0).wrapping_add(c.r64(15)), c.r32(3)) { return RegfileExit::fault(0x70008cbaa5); }
            let r339 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r339);
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_ne() { blk = 245; continue; }
            blk = 246; continue;
        }
        246 => {
            blk = 250; continue;
        }
        247 => {
            c.w64(7, c.r64(12).wrapping_add(0x7));
            c.w64(1, c.r64(0).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r340 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r340);
            let r341 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r341);
            c.logic(r341, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008cbac8); }
            c.w64(2, c.r64(7));
            let Some(t342) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x70008cbad0); };
            c.w64(0, t342);
            c.w64(10, 0x70008cbadeu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008cbade); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008cbae2); }
            return c.call(c.r64(0), 0x70008cbae8, 0x70008cbae6);
        }
        248 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008cbae8); }
            let Some(t343) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008cbaf0); };
            c.w64(4, t343);
            let Some(t344) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cbaf5); };
            c.w64(11, t344);
            let Some(t345) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cbaf9); };
            c.w64(2, t345);
            let Some(t346) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cbafd); };
            c.w64(0, t346);
            blk = 250; continue;
        }
        249 => {
            let Some(t347) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008cbb03); };
            c.w64(0, t347);
            c.w64(12, c.r64(0));
            blk = 250; continue;
        }
        250 => {
            c.w64(12, c.r64(0));
            blk = 251; continue;
        }
        251 => {
            c.w32(7, c.r64(2).wrapping_add(c.r64(2)));
            let Some(t348) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cbb13); };
            c.w64(15, t348);
            let Some(t349) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008cbb1a); };
            c.w64(0, t349);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            let Some(t350) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008cbb21); };
            c.w64(2, t350);
            let Some(t351) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008cbb28); };
            c.cmp(t351, c.r64(3), 64);
            if c.cc_be() { blk = 332; continue; }
            blk = 252; continue;
        }
        252 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x70008cbb31); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            blk = 253; continue;
        }
        253 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008cbb38); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008cbb3c); }
            let Some(t352) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008cbb40); };
            c.w64(7, t352);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008cbb47); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008cbb4b); }
            c.w64(0, c.r64(15));
            blk = 3; continue;
        }
        254 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cbb57); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008cbb58); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008cbb59); }
            let Some(t353) = c.ld64(c.r64(13).wrapping_add(0x2680)) else { return RegfileExit::fault(0x70008cbb5a); };
            c.w64(3, t353);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008cbb6b, 0x70008cbb66);
        }
        255 => {
            blk = 3; continue;
        }
        256 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x70008cbb70); }
            blk = 269; continue;
        }
        257 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x70008cbb79); }
            blk = 269; continue;
        }
        258 => {
            let Some(t354) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cbb82); };
            c.w64(1, t354);
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 269; continue; }
            blk = 259; continue;
        }
        259 => {
            let Some(t355) = c.ld64(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cbb8f); };
            c.w64(7, t355);
            blk = 44; continue;
        }
        260 => {
            c.w64(3, c.r64(7).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(7, c.r64(7).wrapping_add(c.r64(15).wrapping_mul(2)));
            let Some(t356) = c.ld32(c.r64(12).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008cbba0); };
            c.w32(15, t356);
            let r357 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r357);
            blk = 46; continue;
        }
        261 => {
            c.cmp(c.r64(7), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 315; continue; }
            blk = 262; continue;
        }
        262 => {
            c.cmp(c.r64(7), 0xfffffffffffffffdu64, 64);
            if c.cc_e() { blk = 264; continue; }
            blk = 263; continue;
        }
        263 => {
            let Some(t358) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cbbbd); };
            c.w64(0, t358);
            let Some(t359) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cbbc1); };
            c.w64(3, t359);
            let Some(t360) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cbbc5); };
            c.w64(6, t360);
            let Some(t361) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cbbc9); };
            c.w64(9, t361);
            blk = 269; continue;
        }
        264 => {
            let Some(t362) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cbbcf); };
            if !c.push64(t362) { return RegfileExit::fault(0x70008cbbcf); }
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cbbd2); };
            if !c.push64(t363) { return RegfileExit::fault(0x70008cbbd2); }
            let Some(t364) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cbbd5); };
            if !c.push64(t364) { return RegfileExit::fault(0x70008cbbd5); }
            let Some(t365) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008cbbd8); };
            if !c.push64(t365) { return RegfileExit::fault(0x70008cbbd8); }
            let Some(t366) = c.ld64(c.r64(13).wrapping_add(0x2660)) else { return RegfileExit::fault(0x70008cbbde); };
            c.w64(3, t366);
            c.w32(0, 0x4u64);
            let Some(t367) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cbbea); };
            c.w64(6, t367);
            return c.call(0x700086ed74, 0x70008cbbf3, 0x70008cbbee);
        }
        265 => {
            c.w64(12, c.r64(0));
            blk = 68; continue;
        }
        266 => {
            let Some(t368) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cbbfb); };
            c.w64(0, t368);
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cbbff); };
            c.w64(3, t369);
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cbc03); };
            c.w64(6, t370);
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cbc07); };
            c.w64(9, t371);
            blk = 269; continue;
        }
        267 => {
            let Some(t372) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cbc0d); };
            c.w64(3, t372);
            blk = 269; continue;
        }
        268 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x70008cbc13); }
            blk = 269; continue;
        }
        269 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cbc17); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008cbc18); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008cbc19); }
            let Some(t373) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008cbc1b); };
            if !c.push64(t373) { return RegfileExit::fault(0x70008cbc1b); }
            c.w32(0, 0x4u64);
            let Some(t374) = c.ld64(c.r64(13).wrapping_add(0x2650)) else { return RegfileExit::fault(0x70008cbc26); };
            c.w64(3, t374);
            c.w64(7, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008cbc38, 0x70008cbc33);
        }
        270 => {
            c.w64(12, c.r64(0));
            blk = 68; continue;
        }
        271 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x70008cbc40); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x70008cbc44); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(12)) { return RegfileExit::fault(0x70008cbc48); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(2)) { return RegfileExit::fault(0x70008cbc4c); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70008cbc58, 0x70008cbc53);
        }
        272 => {
            c.w64(7, c.r64(0));
            let Some(t375) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cbc5b); };
            c.w64(15, t375);
            let Some(t376) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cbc5f); };
            c.w64(11, t376);
            let Some(t377) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cbc63); };
            c.w64(0, t377);
            let Some(t378) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008cbc67); };
            c.w64(3, t378);
            let Some(t379) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cbc6b); };
            c.w64(9, t379);
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008cbc6f); };
            c.w64(12, t380);
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008cbc73); };
            c.w64(2, t381);
            blk = 82; continue;
        }
        273 => {
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(6)) { return RegfileExit::fault(0x70008cbc7c); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(1)) { return RegfileExit::fault(0x70008cbc80); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008cbc87); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febb4, 0x70008cbc93, 0x70008cbc8e);
        }
        274 => {
            c.w64(11, c.r64(0));
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cbc96); };
            c.w64(2, t382);
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cbc9a); };
            c.w64(15, t383);
            let Some(t384) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cbc9e); };
            c.w64(7, t384);
            let Some(t385) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cbca2); };
            c.w64(12, t385);
            let Some(t386) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008cbca6); };
            c.w64(6, t386);
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008cbcaa); };
            c.w64(1, t387);
            blk = 115; continue;
        }
        275 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008cbcbb); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x70008cbcbf); }
            return c.call(0x70007febf4, 0x70008cbcc9, 0x70008cbcc4);
        }
        276 => {
            c.w64(11, c.r64(0));
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cbccc); };
            c.w64(0, t388);
            let Some(t389) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cbcd0); };
            c.w64(3, t389);
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cbcd4); };
            c.w64(7, t390);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x70008cbcd8); }
            let Some(t391) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cbcdd); };
            c.w64(9, t391);
            blk = 140; continue;
        }
        277 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008cbce6); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008cbcea); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x70008cbcee); }
            blk = 282; continue;
        }
        278 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008cbcf4); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008cbcf8); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x70008cbcfc); }
            blk = 282; continue;
        }
        279 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008cbd02); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008cbd06); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x70008cbd0a); }
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 281; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t392) = c.ld64(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cbd13); };
            c.w64(1, t392);
            blk = 166; continue;
        }
        281 => {
            let Some(t393) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cbd1c); };
            c.w64(3, t393);
            blk = 282; continue;
        }
        282 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(0)) { return RegfileExit::fault(0x70008cbd26); }
            c.cmp(c.r64(0), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 284; continue; }
            blk = 283; continue;
        }
        283 => {
            let r394 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r394);
            blk = 293; continue;
        }
        284 => {
            let Some(t395) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cbd39); };
            c.w64(2, t395);
            let Some(t396) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008cbd40); };
            c.w64(1, t396);
            c.w64(8, c.r64(1).wrapping_add(0xc));
            let Some(t397) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008cbd47); };
            c.w64(9, t397);
            let Some(t398) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008cbd4e); };
            c.cmp(t398, c.r64(8), 64);
            if c.cc_a() { blk = 287; continue; }
            blk = 285; continue;
        }
        285 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008cbd5d, 0x70008cbd58);
        }
        286 => {
            c.w64(2, c.r64(0));
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cbd60); };
            c.w64(15, t399);
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cbd64); };
            c.w64(3, t400);
            let Some(t401) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008cbd68); };
            c.w64(11, t401);
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cbd6c); };
            c.w64(7, t402);
            let Some(t403) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008cbd70); };
            c.w64(12, t403);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cbd76); };
            c.w64(0, t404);
            blk = 288; continue;
        }
        287 => {
            if !c.st64(c.r64(2), c.r64(8)) { return RegfileExit::fault(0x70008cbd7c); }
            c.w64(2, c.r64(1).wrapping_add(0x1));
            blk = 288; continue;
        }
        288 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(0) as i64));
            c.logic(c.r64(0) & c.r64(0), 64);
            if c.cc_ns() { blk = 292; continue; }
            blk = 289; continue;
        }
        289 => {
            c.w64(10, c.r64(0));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 291; continue; }
            blk = 290; continue;
        }
        290 => {
            let r405 = c.r64(10) | 0x1u64;
            c.w64(10, r405);
            c.logic(r405, 64);
            blk = 291; continue;
        }
        291 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 292; continue;
        }
        292 => {
            let Some(t406) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008cbdaa); };
            c.w64(0, t406);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008cbdb1); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008cbdb4); }
            c.w64(0, c.r64(2));
            blk = 293; continue;
        }
        293 => {
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008cbdbc); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cbdbe); }
            c.w32(0, 0x2u64);
            let Some(t407) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008cbdc4); };
            c.w64(3, t407);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008cbdd3, 0x70008cbdce);
        }
        294 => {
            let r408 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r408);
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008cbdd5); };
            c.w64(12, t409);
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cbdd9); };
            c.w64(15, t410);
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008cbddd); };
            c.w64(11, t411);
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cbde1); };
            c.w64(7, t412);
            blk = 169; continue;
        }
        295 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008cbdea); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008cbdee); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x70008cbdf2); }
            blk = 299; continue;
        }
        296 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008cbdf8); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008cbdfc); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x70008cbe00); }
            blk = 299; continue;
        }
        297 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008cbe06); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008cbe0a); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x70008cbe0e); }
            c.logic(c.r8(0) & 0x10u64, 8);
            if c.cc_ne() { blk = 299; continue; }
            blk = 298; continue;
        }
        298 => {
            let Some(t413) = c.ld64(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cbe16); };
            c.w64(2, t413);
            blk = 189; continue;
        }
        299 => {
            c.cmp(c.r64(12), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 301; continue; }
            blk = 300; continue;
        }
        300 => {
            let r414 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r414);
            blk = 310; continue;
        }
        301 => {
            let Some(t415) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cbe30); };
            c.w64(0, t415);
            let Some(t416) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008cbe37); };
            c.w64(3, t416);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            let Some(t417) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008cbe3e); };
            c.w64(1, t417);
            let Some(t418) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008cbe45); };
            c.cmp(t418, c.r64(2), 64);
            if c.cc_a() { blk = 304; continue; }
            blk = 302; continue;
        }
        302 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008cbe54, 0x70008cbe4f);
        }
        303 => {
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008cbe54); };
            c.w64(12, t419);
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cbe58); };
            c.w64(15, t420);
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008cbe5c); };
            c.w64(11, t421);
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cbe60); };
            c.w64(7, t422);
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 305; continue;
        }
        304 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008cbe68); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 305; continue;
        }
        305 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(12) as i64));
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 309; continue; }
            blk = 306; continue;
        }
        306 => {
            c.w64(10, c.r64(12));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 308; continue; }
            blk = 307; continue;
        }
        307 => {
            let r423 = c.r64(10) | 0x1u64;
            c.w64(10, r423);
            c.logic(r423, 64);
            blk = 308; continue;
        }
        308 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 309; continue;
        }
        309 => {
            let Some(t424) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008cbe96); };
            c.w64(12, t424);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008cbe9d); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008cbea1); }
            c.w64(12, c.r64(0));
            blk = 310; continue;
        }
        310 => {
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008cbea9); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008cbeab); }
            let Some(t425) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008cbead); };
            c.w64(3, t425);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008cbebe, 0x70008cbeb9);
        }
        311 => {
            let r426 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r426);
            c.w64(12, c.r64(0));
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cbec3); };
            c.w64(15, t427);
            let Some(t428) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008cbec7); };
            c.w64(11, t428);
            let Some(t429) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cbecb); };
            c.w64(7, t429);
            blk = 192; continue;
        }
        312 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x70008cbed9); }
            return c.call(0x70007febf4, 0x70008cbee3, 0x70008cbede);
        }
        313 => {
            c.w64(7, c.r64(0));
            let Some(t430) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cbee6); };
            c.w64(15, t430);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x70008cbeea); }
            blk = 200; continue;
        }
        314 => {
            return RegfileExit::fault(0x70008cbef4);
        }
        315 => {
            let Some(t431) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70008cbef5); };
            c.w64(3, t431);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t432) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cbefe); };
            c.w64(6, t432);
            return c.call(0x700086ed74, 0x70008cbf07, 0x70008cbf02);
        }
        316 => {
            return RegfileExit::fault(0x70008cbf07);
        }
        317 => {
            let Some(t433) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008cbf08); };
            c.w64(3, t433);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008cbf18, 0x70008cbf13);
        }
        318 => {
            return RegfileExit::fault(0x70008cbf18);
        }
        319 => {
            return RegfileExit::fault(0x70008cbf19);
        }
        320 => {
            return RegfileExit::fault(0x70008cbf1a);
        }
        321 => {
            return RegfileExit::fault(0x70008cbf1b);
        }
        322 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t434) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008cbf20); };
            c.w64(3, t434);
            return c.call(0x700086ed74, 0x70008cbf2c, 0x70008cbf27);
        }
        323 => {
            return RegfileExit::fault(0x70008cbf2c);
        }
        324 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008cbf2d); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70008cbf31); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x70008cbf35); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(6)) { return RegfileExit::fault(0x70008cbf39); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febb4, 0x70008cbf45, 0x70008cbf40);
        }
        325 => {
            c.w64(7, c.r64(0));
            let Some(t435) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cbf48); };
            c.w64(3, t435);
            let Some(t436) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cbf4c); };
            c.w64(12, t436);
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cbf50); };
            c.w64(11, t437);
            let Some(t438) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cbf54); };
            c.w64(2, t438);
            let Some(t439) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cbf58); };
            c.w64(1, t439);
            let Some(t440) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008cbf5c); };
            c.w64(15, t440);
            let Some(t441) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cbf60); };
            c.w64(6, t441);
            blk = 216; continue;
        }
        326 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t442) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008cbf6d); };
            c.w64(3, t442);
            return c.call(0x700086ed74, 0x70008cbf79, 0x70008cbf74);
        }
        327 => {
            return RegfileExit::fault(0x70008cbf79);
        }
        328 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008cbf7a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008cbf7e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008cbf82); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008cbf86); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x70008cbf92, 0x70008cbf8d);
        }
        329 => {
            let Some(t443) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cbf92); };
            c.w64(15, t443);
            let Some(t444) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cbf96); };
            c.w64(11, t444);
            let Some(t445) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cbf9a); };
            c.w64(2, t445);
            let Some(t446) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cbf9e); };
            c.w64(12, t446);
            let Some(t447) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cbfa2); };
            c.w64(8, t447);
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cbfa6); };
            c.w64(7, t448);
            blk = 242; continue;
        }
        330 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t449) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008cbfb3); };
            c.w64(3, t449);
            return c.call(0x700086ed74, 0x70008cbfbf, 0x70008cbfba);
        }
        331 => {
            return RegfileExit::fault(0x70008cbfbf);
        }
        332 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008cbfc5); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008cbfc9); }
            return c.call(0x70007febf4, 0x70008cbfd2, 0x70008cbfcd);
        }
        333 => {
            c.w64(15, c.r64(0));
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cbfd5); };
            c.w64(12, t450);
            let Some(t451) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cbfd9); };
            c.w64(11, t451);
            let Some(t452) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cbfdd); };
            c.w64(7, t452);
            blk = 253; continue;
        }
        334 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008cbfe6); }
            c.cmp(c.r64(11), 0x9bu64, 64);
            if c.cc_a() { blk = 336; continue; }
            blk = 335; continue;
        }
        335 => {
            return RegfileExit::fault(0x70008cbff3);
        }
        336 => {
            let Some(t453) = c.ld32(c.r64(7).wrapping_add(0x273)) else { return RegfileExit::fault(0x70008cbff4); };
            c.w32(2, t453);
            let r454 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r454);
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x7000859b74, 0x70008cc004, 0x70008cbfff);
        }
        337 => {
            let Some(t455) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008cc004); };
            c.cmp(t455, c.r32(0), 32);
            if c.cc_ne() { blk = 346; continue; }
            blk = 338; continue;
        }
        338 => {
            let Some(t456) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cc011); };
            c.w64(7, t456);
            let Some(t457) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008cc018); };
            c.w64(11, t457);
            c.w64(12, c.r64(11).wrapping_add(0x1c));
            let Some(t458) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008cc01f); };
            c.w64(15, t458);
            let Some(t459) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008cc026); };
            c.cmp(t459, c.r64(12), 64);
            if c.cc_a() { blk = 341; continue; }
            blk = 339; continue;
        }
        339 => {
            c.w32(2, 0x1cu64);
            return c.call(0x70007febf4, 0x70008cc035, 0x70008cc030);
        }
        340 => {
            blk = 342; continue;
        }
        341 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x70008cc037); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 342; continue;
        }
        342 => {
            let Some(t460) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cc041); };
            c.w64(12, t460);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008cc045); }
            c.w32(7, 0x2u64);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008cc04e); }
            let Some(t461) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008cc051); };
            c.w64(11, t461);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008cc058); }
            c.w64(11, c.r64(0).wrapping_add(0x10));
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008cc060); }
            let Some(t462) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008cc064); };
            c.w64(12, t462);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008cc06b); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008cc06f); }
            let Some(t463) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008cc073); };
            c.w64(7, t463);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008cc07a); }
            let Some(t464) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cc07e); };
            c.w32(7, t464);
            let r465 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r465);
            let Some(t466) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cc084); };
            c.w64(15, t466);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008cc088); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 344; continue; }
            blk = 343; continue;
        }
        343 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r467 = c.r64(11) & c.r64(7);
            c.w64(11, r467);
            c.logic(r467, 64);
            let Some(t468) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cc09c); };
            c.logic(t468 & 0x4u64, 8);
            if c.cc_ne() { blk = 351; continue; }
            blk = 344; continue;
        }
        344 => {
            blk = 3; continue;
        }
        345 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008cc0a8); }
            blk = 346; continue;
        }
        346 => {
            let Some(t469) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cc0ac); };
            c.w64(7, t469);
            let Some(t470) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008cc0b3); };
            c.w64(11, t470);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t471) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008cc0ba); };
            c.w64(15, t471);
            let Some(t472) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008cc0c1); };
            c.cmp(t472, c.r64(12), 64);
            if c.cc_a() { blk = 349; continue; }
            blk = 347; continue;
        }
        347 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008cc0d0, 0x70008cc0cb);
        }
        348 => {
            blk = 350; continue;
        }
        349 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x70008cc0d2); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 350; continue;
        }
        350 => {
            let Some(t473) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cc0dc); };
            c.w64(7, t473);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008cc0e0); }
            c.w32(7, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008cc0e5); }
            let Some(t474) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008cc0e8); };
            c.w64(7, t474);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008cc0ef); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008cc0f2); }
            blk = 3; continue;
        }
        351 => {
            c.w32(15, c.r32(15));
            let r475 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r475);
            c.w64(11, 0xfffffffffffc0000u64);
            let r476 = c.r64(11) & c.r64(15);
            c.w64(11, r476);
            c.logic(r476, 64);
            let Some(t477) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cc10a); };
            c.logic(t477 & 0x2u64, 8);
            if c.cc_e() { blk = 344; continue; }
            blk = 352; continue;
        }
        352 => {
            c.w64(12, c.r64(7).wrapping_add(0x7));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008cc115); }
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008cc11f, 0x70008cc11a);
        }
        353 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008cc11f); }
            blk = 344; continue;
        }
        354 => {
            c.w32(0, c.r32(0));
            let r478 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r478);
            c.w64(3, 0xfffffffffffc0000u64);
            let r479 = c.r64(3) & c.r64(0);
            c.w64(3, r479);
            c.logic(r479, 64);
            let Some(t480) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cc132); };
            c.logic(t480 & 0x2u64, 8);
            if c.cc_e() { blk = 231; continue; }
            blk = 355; continue;
        }
        355 => {
            c.w64(2, c.r64(12).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cc140); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008cc141); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70008cc14e, 0x70008cc149);
        }
        356 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008cc14e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008cc150); }
            blk = 231; continue;
        }
        357 => {
            c.w32(0, c.r32(0));
            let r481 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r481);
            c.w64(1, 0xfffffffffffc0000u64);
            let r482 = c.r64(1) & c.r64(0);
            c.w64(1, r482);
            c.logic(r482, 64);
            let Some(t483) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cc165); };
            c.logic(t483 & 0x2u64, 8);
            if c.cc_e() { blk = 130; continue; }
            blk = 358; continue;
        }
        358 => {
            c.w64(6, c.r64(12).wrapping_add(c.r64(3)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cc173); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008cc174); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x70008cc181, 0x70008cc17c);
        }
        359 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008cc181); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008cc183); }
            blk = 130; continue;
        }
        360 => {
            c.w32(0, c.r32(0));
            let r484 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r484);
            c.w64(1, 0xfffffffffffc0000u64);
            let r485 = c.r64(1) & c.r64(0);
            c.w64(1, r485);
            c.logic(r485, 64);
            let Some(t486) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cc198); };
            c.logic(t486 & 0x2u64, 8);
            if c.cc_e() { blk = 97; continue; }
            blk = 361; continue;
        }
        361 => {
            c.w64(6, c.r64(12).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cc1a6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008cc1a7); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x70008cc1b4, 0x70008cc1af);
        }
        362 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008cc1b4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008cc1b6); }
            blk = 97; continue;
        }
        363 => {
            c.w32(0, c.r32(0));
            let r487 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r487);
            c.w64(15, 0xfffffffffffc0000u64);
            let r488 = c.r64(15) & c.r64(0);
            c.w64(15, r488);
            c.logic(r488, 64);
            let Some(t489) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cc1cb); };
            c.logic(t489 & 0x2u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 364; continue;
        }
        364 => {
            c.w64(3, c.r64(12).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cc1db); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008cc1dc); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008cc1e9, 0x70008cc1e4);
        }
        365 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008cc1e9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008cc1eb); }
            blk = 60; continue;
        }
        366 => {
            c.w32(0, c.r32(0));
            let r490 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r490);
            c.w64(15, 0xfffffffffffc0000u64);
            let r491 = c.r64(15) & c.r64(0);
            c.w64(15, r491);
            c.logic(r491, 64);
            let Some(t492) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cc200); };
            c.logic(t492 & 0x2u64, 8);
            if c.cc_e() { blk = 58; continue; }
            blk = 367; continue;
        }
        367 => {
            c.w64(3, c.r64(12).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cc210); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008cc211); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008cc21e, 0x70008cc219);
        }
        368 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008cc21e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008cc220); }
            blk = 58; continue;
        }
        369 => {
            return RegfileExit::jump(0x70008cc228);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cc234(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cc234); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cc238); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cc239); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cc23a); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cc23f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 24; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(7, c.r64(2).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cc257); };
            c.w64(8, t2);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc266); };
            c.w32(9, t3);
            let r4 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r4);
            let Some(t5) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cc26d); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 28; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008cc27d); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cc287); };
            c.w64(2, t6);
            blk = 6; continue;
        }
        5 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cc28d); };
            c.w64(2, t7);
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t8) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc29d); };
            c.w32(11, t8);
            let r9 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r9);
            let Some(t10) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cc2a4); };
            c.cmp(t10, 0x40u64, 16);
            if c.cc_ae() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t11) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cc2b6); };
            c.w64(7, t11);
            blk = 11; continue;
        }
        10 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cc2bc); };
            c.w64(7, t12);
            blk = 11; continue;
        }
        11 => {
            let Some(t13) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc2c3); };
            c.w32(11, t13);
            let r14 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r14);
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008cc2ca); };
            c.w64(12, t15);
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cc2d4); };
            c.cmp(t16, c.r32(15), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t17) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc2df); };
            c.w32(12, t17);
            let r18 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r18);
            let Some(t19) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cc2e6); };
            c.w32(12, t19);
            let r20 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r20);
            let Some(t21) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cc2ee); };
            c.w32(15, t21);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x9au64, 64);
            if c.cc_a() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return RegfileExit::fault(0x70008cc302);
        }
        14 => {
            let Some(t22) = c.ld32(c.r64(12).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008cc303); };
            c.w32(0, t22);
            let r23 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r23);
            let Some(t24) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008cc30e); };
            c.cmp(t24, c.r32(11), 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008cc314); };
            c.w32(0, t25);
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
            return RegfileExit::fault(0x70008cc328);
        }
        18 => {
            let Some(t26) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cc329); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            let Some(t28) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc330); };
            c.w32(11, t28);
            let r29 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r29);
            let Some(t30) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008cc337); };
            c.w32(15, t30);
            let r31 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r31);
            c.w64(0, c.r64(9).wrapping_add(0x1));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008cc342); }
            let Some(t32) = c.ld32(c.r64(12).wrapping_add(0x283)) else { return RegfileExit::fault(0x70008cc346); };
            c.cmp(t32, c.r32(11), 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008cc350); };
            c.w32(11, t33);
            c.w32(11, c.r32(11));
            let r34 = c.r32(11) & 0x8u64;
            c.w32(11, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(0, c.r64(8));
            c.w64(3, c.r64(2));
            c.w64(1, c.r64(7));
            return c.call(0x70008cab94, 0x70008cc36f, 0x70008cc36a);
        }
        21 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cc36f); };
            c.w64(2, t35);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cc376); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cc377); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cc37d); }
            return c.ret(0x0, 0x70008cc37f);
        }
        22 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008cc380); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cc382); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cc383); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2680)) else { return RegfileExit::fault(0x70008cc384); };
            c.w64(3, t36);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008cc395, 0x70008cc390);
        }
        23 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cc395); };
            c.w64(2, t37);
            let r38 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r38);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cc3a0); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cc3a1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cc3a7); }
            return c.ret(0x0, 0x70008cc3a9);
        }
        24 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008cc3aa); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008cc3ae); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cc3b2); };
            c.w64(3, t39);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008cc3c3, 0x70008cc3be);
        }
        25 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cc3c3); };
            c.w64(6, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cc3c7); };
            c.w64(0, t41);
            blk = 1; continue;
        }
        26 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008cc3d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008cc3d4); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008cc3d8); }
            c.w64(0, c.r64(2));
            return c.call(0x700089c794, 0x70008cc3e4, 0x70008cc3df);
        }
        27 => {
            c.w64(2, c.r64(0));
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cc3e7); };
            c.w64(8, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cc3eb); };
            c.w64(6, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cc3ef); };
            c.w64(9, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cc3f3); };
            c.w64(7, t45);
            blk = 8; continue;
        }
        28 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cc401); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cc402); };
            c.w64(2, t46);
            let Some(t47) = c.ld32(c.r64(2).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x70008cc409); };
            c.w32(2, t47);
            let r48 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r48);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cc412); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008cc413); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008cc415); };
            c.w64(3, t49);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008cc426, 0x70008cc421);
        }
        29 => {
            return RegfileExit::fault(0x70008cc426);
        }
        30 => {
            return RegfileExit::jump(0x70008cc428);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cc434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cc434); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cc438); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cc439); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cc43a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008cc43f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cc443); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 112; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cc44d); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 110; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc459); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cc460); };
            c.w32(11, t5);
            let r6 = c.sub(c.r32(11), 0xbau64, 32);
            c.w32(11, r6);
            c.cmp(c.r32(11), 0x78eu64, 32);
            if c.cc_a() { blk = 110; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cc479); };
            c.w64(11, t7);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 114; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc487); };
            c.w32(12, t8);
            let r9 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cc48e); };
            c.cmp(t10, 0x40u64, 16);
            if c.cc_ae() { blk = 114; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc49b); };
            c.w32(12, t11);
            let r12 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r12);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008cc4a2); }
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008cc4a9); };
            c.w64(3, t13);
            let Some(t14) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cc4b0); };
            c.cmp(t14, c.r32(15), 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t15) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc4ba); };
            c.w32(3, t15);
            let r16 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r16);
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cc4c0); };
            c.w32(3, t17);
            let r18 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r18);
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cc4c6); };
            c.w32(2, t19);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x9au64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x70008cc4d8);
        }
        8 => {
            let Some(t20) = c.ld32(c.r64(3).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008cc4d9); };
            c.w32(1, t20);
            let r21 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r21);
            let Some(t22) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008cc4e2); };
            c.cmp(t22, c.r32(12), 32);
            if c.cc_ne() { blk = 95; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t23) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008cc4ec); };
            c.w32(1, t23);
            c.logic(c.r32(1) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 95; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(2), 0x9fu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008cc504);
        }
        12 => {
            let Some(t24) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cc505); };
            c.w32(12, t24);
            let r25 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r25);
            let Some(t26) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc50d); };
            c.w32(2, t26);
            let r27 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r27);
            let Some(t28) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008cc515); };
            c.w32(1, t28);
            let r29 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r29);
            let Some(t30) = c.ld32(c.r64(3).wrapping_add(0x283)) else { return RegfileExit::fault(0x70008cc51b); };
            c.cmp(t30, c.r32(2), 32);
            if c.cc_ne() { blk = 116; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t31) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008cc527); };
            c.w32(7, t31);
            c.w32(7, c.r32(7));
            let r32 = c.r32(7) & 0x8u64;
            c.w32(7, r32);
            c.logic(r32, 32);
            c.cmp(c.r32(7), 0x8u64, 32);
            if c.cc_ne() { blk = 116; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t33) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cc538); };
            c.w32(12, t33);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t34) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008cc542); };
            c.w32(3, t34);
            let r35 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r35);
            c.logic(c.r8(12) & 0x9u64, 8);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008cc554); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(3, c.r64(15));
            c.w32(2, 0x1u64);
            blk = 29; continue;
        }
        16 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t36) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cc57d); };
            c.w32(1, t36);
            let r37 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r37);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008cc588); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008cc592); }
            blk = 23; continue;
        }
        20 => {
            c.w64(2, c.r64(3));
            let r38 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r38);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 23; continue;
        }
        21 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 22; continue;
        }
        22 => {
            let r39 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r39);
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008cc5af); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 23; continue;
        }
        23 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 27; continue; }
            blk = 24; continue;
        }
        24 => {
            blk = 26; continue;
        }
        25 => {
            c.cmp(c.r32(1), c.r32(3), 32);
            if c.cc_l() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        27 => {
            c.w32(1, 0x1u64);
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0x1u64);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 93; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t40) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc5df); };
            c.w32(1, t40);
            let r41 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r41);
            let Some(t42) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cc5e5); };
            c.w32(1, t42);
            let r43 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r43);
            let Some(t44) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cc5eb); };
            c.w32(7, t44);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008cc5f4); }
            c.cmp(c.r64(7), 0x9bu64, 64);
            if c.cc_a() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            return RegfileExit::fault(0x70008cc601);
        }
        31 => {
            let Some(t45) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc602); };
            c.w32(7, t45);
            let r46 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r46);
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(0x273)) else { return RegfileExit::fault(0x70008cc609); };
            c.w32(1, t47);
            let r48 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r48);
            let Some(t49) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cc612); };
            c.w32(7, t49);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008cc616); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 81; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(8, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t50) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cc626); };
            c.w32(9, t50);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_b() { blk = 81; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t51) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cc636); };
            c.w32(11, t51);
            let r52 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r52);
            let Some(t53) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cc63d); };
            c.w32(3, t53);
            let r54 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r54);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008cc643); }
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 36; continue;
        }
        36 => {
            blk = 140; continue;
        }
        37 => {
            blk = 40; continue;
        }
        38 => {
            c.w64(2, c.r64(1));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cc660); };
            c.w64(3, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cc664); };
            c.w64(1, t56);
            return c.call(0x7000859a54, 0x70008cc66d, 0x70008cc668);
        }
        39 => {
            c.w64(1, c.r64(0));
            blk = 82; continue;
        }
        40 => {
            let Some(t57) = c.ld32(c.r64(11).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008cc675); };
            c.w32(3, t57);
            c.cmp(c.r32(3), 0x7eu64, 32);
            if c.cc_a() { blk = 153; continue; }
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(7) & 0x7u64, 8);
            if c.cc_e() { blk = 58; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008cc68c); }
            c.w64(7, c.r64(15));
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cc693); };
            c.w64(12, t58);
            blk = 44; continue;
        }
        43 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70008cc699); }
            blk = 44; continue;
        }
        44 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cc69d); };
            c.w64(1, t59);
            let r60 = c.r32(1) & 0x7u64;
            c.w32(1, r60);
            c.logic(r60, 32);
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_l() { blk = 49; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(1), 0x5u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 153; continue;
        }
        49 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 153; continue;
        }
        52 => {
            let Some(t61) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cc6d0); };
            c.w32(1, t61);
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cc6d5); };
            c.cmp(t62, c.r32(1), 32);
            if c.cc_ne() { blk = 153; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t63) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cc6e2); };
            c.w32(12, t63);
            let r64 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r64);
            let Some(t65) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc6ea); };
            c.w32(1, t65);
            let r66 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r66);
            let Some(t67) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cc6f2); };
            c.w32(1, t67);
            blk = 43; continue;
        }
        54 => {
            let Some(t68) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cc6f8); };
            c.w32(1, t68);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r69 = c.add(c.r64(7), c.r64(1), 64);
            c.w64(7, r69);
            let Some(t70) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cc706); };
            c.w32(12, t70);
            let r71 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r71);
            let Some(t72) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc70e); };
            c.w32(1, t72);
            let r73 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r73);
            let Some(t74) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cc716); };
            c.w32(1, t74);
            blk = 43; continue;
        }
        55 => {
            let Some(t75) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cc71f); };
            c.w32(12, t75);
            let r76 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r76);
            let Some(t77) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc727); };
            c.w32(1, t77);
            let r78 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r78);
            let Some(t79) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cc72f); };
            c.w32(1, t79);
            blk = 43; continue;
        }
        56 => {
            c.w64(2, c.r64(12));
            c.w64(12, c.r64(7));
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 59; continue;
        }
        57 => {
            c.w64(1, c.r64(2));
            c.w64(2, c.r64(12));
            c.w64(12, c.r64(7));
            blk = 59; continue;
        }
        58 => {
            c.w64(12, c.r64(15));
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cc750); };
            c.w64(2, t80);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008cc754); }
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 142; continue; }
            blk = 60; continue;
        }
        60 => {
            let r81 = c.add(c.r64(2), 0xbu64, 64);
            c.w64(2, r81);
            blk = 61; continue;
        }
        61 => {
            c.w64(1, c.r64(12).wrapping_add(c.r64(9)));
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cc76b); };
            c.w64(7, t82);
            let r83 = c.r32(7) & 0x8u64;
            c.w32(7, r83);
            c.logic(r83, 32);
            let r84 = c.add(c.r64(12), c.r64(8), 64);
            c.w64(12, r84);
            c.cmp(c.r32(7), 0x8u64, 32);
            if c.cc_ne() { blk = 144; continue; }
            blk = 62; continue;
        }
        62 => {
            let r85 = c.add(c.r64(1), c.r64(2), 64);
            c.w64(1, r85);
            let r86 = c.add(c.r64(12), c.r64(2), 64);
            c.w64(12, r86);
            let Some(t87) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cc784); };
            c.w32(2, t87);
            let r88 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r88);
            blk = 63; continue;
        }
        63 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 153; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t89) = c.ld32(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008cc794); };
            c.w32(7, t89);
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2e70)) else { return RegfileExit::fault(0x70008cc797); };
            c.w64(9, t90);
            let Some(t91) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008cc79e); };
            c.w64(9, t91);
            c.cmp(c.r32(7), 0xffffffffffffffffu64, 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w32(2, c.r64(7).wrapping_mul(8));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(13));
            let Some(t92) = c.ld64(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008cc7b3); };
            c.w64(2, t92);
            blk = 67; continue;
        }
        66 => {
            let r93 = c.add(c.r64(2), 0x3fu64, 64);
            c.w64(2, r93);
            c.w64(7, c.r64(13));
            blk = 67; continue;
        }
        67 => {
            let r94 = c.add(c.r32(3), 0x2u64, 32);
            c.w32(3, r94);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w32(8, c.r32(8));
            c.w64(10, c.r64(4));
            let r95 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r95);
            let r96 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r96);
            c.logic(r96, 64);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x70008cc7db); }
            let r97 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r97);
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008cc7e2); };
            c.w64(11, t98);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(11)) { return RegfileExit::fault(0x70008cc7e9); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(3)) { return RegfileExit::fault(0x70008cc7ee); }
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(9)) { return RegfileExit::fault(0x70008cc7f3); }
            if !c.st64(c.r64(4).wrapping_add(0x38), 0x1u64) { return RegfileExit::fault(0x70008cc7f8); }
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(7)) { return RegfileExit::fault(0x70008cc801); }
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(0)) { return RegfileExit::fault(0x70008cc806); }
            c.w64(9, c.r64(1));
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cc80e); };
            c.w64(1, t99);
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(8));
            c.w64(8, c.r64(10));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(12));
            c.w64(12, c.r64(10));
            c.w64(10, 0x70008cc82bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008cc82b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008cc82f); }
            return c.call(c.r64(12), 0x70008cc836, 0x70008cc833);
        }
        68 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008cc836); }
            let Some(t100) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x70008cc83e); };
            c.w64(4, t100);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_e() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_ne() { blk = 145; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t101) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cc856); };
            c.w64(0, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cc85a); };
            c.w64(12, t102);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 81; continue;
        }
        71 => {
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cc866); };
            c.w64(1, t103);
            let Some(t104) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cc86a); };
            c.w32(11, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cc86e); };
            c.w64(12, t105);
            let Some(t106) = c.ld32(c.r64(12).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008cc872); };
            c.w32(12, t106);
            let r107 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r107);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r108 = c.sub(c.r32(11), 0x6u64, 32);
            c.w32(11, r108);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_l() { blk = 152; continue; }
            blk = 72; continue;
        }
        72 => {
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008cc88e); }
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cc892); };
            c.w64(11, t109);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008cc896); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r110 = c.r64(15) & c.r64(1);
            c.w64(15, r110);
            c.logic(r110, 64);
            let Some(t111) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cc8aa); };
            c.logic(t111 & 0x4u64, 8);
            if c.cc_ne() { blk = 160; continue; }
            blk = 74; continue;
        }
        74 => {
            if !c.st32(c.r64(1).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008cc8b5); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 76; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r112 = c.r64(15) & c.r64(1);
            c.w64(15, r112);
            c.logic(r112, 64);
            let Some(t113) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cc8c9); };
            c.logic(t113 & 0x4u64, 8);
            if c.cc_ne() { blk = 157; continue; }
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
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008cc8e5); };
            c.w64(0, t114);
            blk = 79; continue;
        }
        78 => {
            let r115 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r115);
            blk = 79; continue;
        }
        79 => {
            let Some(t116) = c.ld32(c.r64(0).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x70008cc8f2); };
            c.w32(3, t116);
            let r117 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r117);
            if !c.st32(c.r64(1).wrapping_add(c.r64(12)), c.r32(3)) { return RegfileExit::fault(0x70008cc8f8); }
            let r118 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r118);
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ne() { blk = 78; continue; }
            blk = 80; continue;
        }
        80 => {
            blk = 82; continue;
        }
        81 => {
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008cc907); };
            c.w64(1, t119);
            blk = 82; continue;
        }
        82 => {
            let Some(t120) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008cc90e); };
            c.cmp(t120, c.r32(1), 32);
            if c.cc_ne() { blk = 85; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t121) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cc917); };
            c.cmp(t121, 0x0u64, 32);
            if c.cc_e() { blk = 94; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t122) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cc91d); };
            c.w64(0, t122);
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008cc924); }
            blk = 94; continue;
        }
        85 => {
            let Some(t123) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cc92a); };
            c.cmp(t123, 0x0u64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t124) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cc930); };
            c.w32(11, t124);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4u64, 64);
            if c.cc_a() { blk = 88; continue; }
            blk = 87; continue;
        }
        87 => {
            return RegfileExit::fault(0x70008cc940);
        }
        88 => {
            let Some(t125) = c.ld32(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008cc941); };
            c.w32(11, t125);
            let Some(t126) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cc945); };
            c.w64(0, t126);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008cc949); }
            blk = 89; continue;
        }
        89 => {
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008cc94d); };
            c.w64(0, t127);
            blk = 90; continue;
        }
        90 => {
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cc954); };
            c.w64(1, t128);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cc95b); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            return c.ret(0x10, 0x70008cc962);
        }
        92 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cc965); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cc96c); }
            return c.ret(0x0, 0x70008cc96e);
        }
        93 => {
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70008cc96f); }
            blk = 94; continue;
        }
        94 => {
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cc973); };
            c.w64(0, t129);
            blk = 90; continue;
        }
        95 => {
            let Some(t130) = c.ld64(c.r64(13).wrapping_add(0x6e8)) else { return RegfileExit::fault(0x70008cc97c); };
            c.w64(3, t130);
            return c.call(0x7000870914, 0x70008cc988, 0x70008cc983);
        }
        96 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 103; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t131) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc98c); };
            c.w32(11, t131);
            let r132 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r132);
            let Some(t133) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008cc993); };
            c.logic(t133 & 0x2u64, 8);
            if c.cc_e() { blk = 103; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cc99a); };
            if !c.push64(t134) { return RegfileExit::fault(0x70008cc99a); }
            let Some(t135) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cc99d); };
            if !c.push64(t135) { return RegfileExit::fault(0x70008cc99d); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cc9a8); };
            c.w64(6, t136);
            return c.call(0x70007f2274, 0x70008cc9b1, 0x70008cc9ac);
        }
        99 => {
            let Some(t137) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008cc9b1); };
            c.cmp(t137, c.r32(0), 32);
            if c.cc_e() { blk = 105; continue; }
            blk = 100; continue;
        }
        100 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 155; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t138) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc9c2); };
            c.w32(11, t138);
            let r139 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r139);
            let Some(t140) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cc9c9); };
            c.cmp(t140, 0xbau64, 16);
            if c.cc_ae() { blk = 105; continue; }
            blk = 102; continue;
        }
        102 => {
            blk = 155; continue;
        }
        103 => {
            let Some(t141) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cc9d7); };
            c.w64(0, t141);
            let Some(t142) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cc9db); };
            c.w32(11, t142);
            let r143 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r143);
            let Some(t144) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cc9e2); };
            c.cmp(t144, 0x83eu64, 16);
            if c.cc_ne() { blk = 108; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cc9eb); };
            c.w64(6, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cc9ef); };
            c.w64(3, t146);
            return c.call(0x70008bf674, 0x70008cc9f8, 0x70008cc9f3);
        }
        105 => {
            let Some(t147) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008cc9f8); };
            c.cmp(t147, c.r32(0), 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t148) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cca01); };
            c.w64(0, t148);
            blk = 90; continue;
        }
        107 => {
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008cca0d); };
            c.w64(0, t149);
            blk = 90; continue;
        }
        108 => {
            c.w32(11, 0x80u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008cca1f); }
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cca21); };
            c.w64(11, t150);
            let Some(t151) = c.ld32(c.r64(11).wrapping_add(0x28b)) else { return RegfileExit::fault(0x70008cca28); };
            c.w32(11, t151);
            let r152 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r152);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008cca32); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cca34); }
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008cca35); };
            c.w64(3, t153);
            c.w32(0, 0x3u64);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cca41); };
            c.w64(6, t154);
            return c.call(0x700086ed74, 0x70008cca4a, 0x70008cca45);
        }
        109 => {
            return RegfileExit::fault(0x70008cca4a);
        }
        110 => {
            c.w32(11, 0x80u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008cca51); }
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cca53); };
            c.w64(11, t155);
            let Some(t156) = c.ld32(c.r64(11).wrapping_add(0x2a7)) else { return RegfileExit::fault(0x70008cca5a); };
            c.w32(11, t156);
            let r157 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r157);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008cca64); }
            let Some(t158) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008cca66); };
            c.w64(3, t158);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008cca77, 0x70008cca72);
        }
        111 => {
            return RegfileExit::fault(0x70008cca77);
        }
        112 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t159) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cca7a); };
            c.w64(3, t159);
            return c.call(0x700086ed74, 0x70008cca86, 0x70008cca81);
        }
        113 => {
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cca86); };
            c.w64(6, t160);
            blk = 1; continue;
        }
        114 => {
            c.w64(0, c.r64(11));
            return c.call(0x700089c794, 0x70008cca97, 0x70008cca92);
        }
        115 => {
            c.w64(11, c.r64(0));
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cca9a); };
            c.w64(6, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cca9e); };
            c.w64(0, t162);
            blk = 5; continue;
        }
        116 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008ccaa7); }
            let Some(t163) = c.ld16(c.r64(1).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008ccaab); };
            c.cmp(t163, 0x1u64, 16);
            if c.cc_le() { blk = 95; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t164) = c.ld8(c.r64(2).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008ccab6); };
            c.w32(7, t164);
            let r165 = c.r32(7) & 0xfcu64;
            c.w32(7, r165);
            c.logic(r165, 32);
            c.cmp(c.r32(7), 0x40u64, 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(7), 0x3cu64, 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let r166 = c.r32(8) | c.r32(7);
            c.w32(8, r166);
            c.logic(r166, 32);
            if c.cc_ne() { blk = 95; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t167) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ccadf); };
            c.cmp(t167, 0x410u64, 16);
            if c.cc_be() { blk = 95; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t168) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ccaeb); };
            c.logic(t168 & 0x200000u64, 32);
            if c.cc_ne() { blk = 95; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t169) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008ccaf8); };
            c.w32(7, t169);
            let Some(t170) = c.ld32(c.r64(13).wrapping_add(0x6e8)) else { return RegfileExit::fault(0x70008ccafb); };
            c.cmp(t170, c.r32(7), 32);
            if c.cc_ne() { blk = 95; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t171) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008ccb08); };
            c.w32(7, t171);
            c.w32(8, c.r32(7));
            c.logic(c.r8(7) & 0x4u64, 8);
            if c.cc_e() { blk = 123; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t172) = c.ld32(c.r64(1).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008ccb14); };
            c.w32(12, t172);
            blk = 136; continue;
        }
        123 => {
            let r173 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r173);
            if let Some(sr) = c.shr(c.r64(7), 0x13u64, 64) { c.w64(7, sr); }
            let Some(t174) = c.ld8(c.r64(2).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008ccb23); };
            c.w32(1, t174);
            let r175 = c.r64(7) & 0x3ffu64;
            c.w64(7, r175);
            c.logic(r175, 64);
            let r176 = c.r32(8) & 0x380u64;
            c.w32(8, r176);
            c.logic(r176, 32);
            let r177 = c.add(c.r64(1), c.r64(7), 64);
            c.w64(1, r177);
            c.cmp(c.r32(8), 0x100u64, 32);
            c.w8(7, (c.cc_ne() as u64));
            c.w32(7, c.r8(7));
            let Some(t178) = c.ld8(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ccb47); };
            c.w32(2, t178);
            c.cmp(c.r64(1), c.r64(2), 64);
            if c.cc_b() { blk = 128; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t179) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ccb50); };
            c.w32(12, t179);
            let r180 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r180);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 126; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008ccb5e); };
            c.w64(12, t181);
            blk = 126; continue;
        }
        126 => {
            let r182 = c.sub(c.r64(1), c.r64(2), 64);
            c.w64(1, r182);
            let Some(t183) = c.ld32(c.r64(12).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ccb68); };
            c.w32(12, t183);
            let r184 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r184);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 136; continue; }
            blk = 127; continue;
        }
        127 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ccb79); }
            blk = 130; continue;
        }
        128 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 135; continue; }
            blk = 129; continue;
        }
        129 => {
            let Some(t185) = c.ld32(c.r64(12).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ccb87); };
            c.w32(12, t185);
            let r186 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r186);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ccb8f); }
            blk = 130; continue;
        }
        130 => {
            let Some(t187) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ccb96); };
            c.w64(12, t187);
            let Some(t188) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008ccb9d); };
            c.w64(2, t188);
            c.w64(1, c.r64(2).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x70008ccba5); }
            let Some(t189) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ccbaa); };
            c.w64(7, t189);
            let Some(t190) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ccbb1); };
            c.cmp(t190, c.r64(1), 64);
            if c.cc_a() { blk = 133; continue; }
            blk = 131; continue;
        }
        131 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ccbc0, 0x70008ccbbb);
        }
        132 => {
            c.w64(12, c.r64(0));
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ccbc3); };
            c.w64(6, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ccbc7); };
            c.w64(0, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ccbcb); };
            c.w64(11, t193);
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ccbd2); };
            c.w64(3, t194);
            blk = 134; continue;
        }
        133 => {
            if !c.st64(c.r64(12), c.r64(1)) { return RegfileExit::fault(0x70008ccbd8); }
            c.w64(12, c.r64(2).wrapping_add(0x1));
            blk = 134; continue;
        }
        134 => {
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ccbe0); };
            c.w64(2, t195);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008ccbe7); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x70008ccbec); }
            if !c.movsd_mr(c.r64(12).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ccbf1); }
            blk = 136; continue;
        }
        135 => {
            let Some(t196) = c.ld32(c.r64(12).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ccbfa); };
            c.w32(12, t196);
            blk = 136; continue;
        }
        136 => {
            let Some(t197) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ccbff); };
            c.w32(2, t197);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x99u64, 64);
            if c.cc_a() { blk = 138; continue; }
            blk = 137; continue;
        }
        137 => {
            return RegfileExit::fault(0x70008ccc11);
        }
        138 => {
            let Some(t198) = c.ld32(c.r64(3).wrapping_add(0x26b)) else { return RegfileExit::fault(0x70008ccc12); };
            c.cmp(t198, c.r32(12), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 139; continue;
        }
        139 => {
            blk = 95; continue;
        }
        140 => {
            return RegfileExit::fault(0x70008ccc24);
        }
        141 => {
            blk = 153; continue;
        }
        142 => {
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ccc2a); };
            c.w64(7, t199);
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_ne() { blk = 153; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t200) = c.ld64(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ccc38); };
            c.w64(2, t200);
            blk = 61; continue;
        }
        144 => {
            c.w64(1, c.r64(2).wrapping_add(c.r64(1).wrapping_mul(2)));
            c.w64(12, c.r64(2).wrapping_add(c.r64(12).wrapping_mul(2)));
            let Some(t201) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008ccc49); };
            c.w32(2, t201);
            let r202 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r202);
            blk = 63; continue;
        }
        145 => {
            c.cmp(c.r64(11), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 150; continue; }
            blk = 146; continue;
        }
        146 => {
            c.cmp(c.r64(11), 0xfffffffffffffffdu64, 64);
            if c.cc_e() { blk = 148; continue; }
            blk = 147; continue;
        }
        147 => {
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ccc61); };
            c.w64(6, t203);
            let Some(t204) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ccc65); };
            c.w64(0, t204);
            blk = 153; continue;
        }
        148 => {
            let Some(t205) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ccc6b); };
            if !c.push64(t205) { return RegfileExit::fault(0x70008ccc6b); }
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ccc6e); };
            if !c.push64(t206) { return RegfileExit::fault(0x70008ccc6e); }
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ccc71); };
            if !c.push64(t207) { return RegfileExit::fault(0x70008ccc71); }
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ccc74); };
            if !c.push64(t208) { return RegfileExit::fault(0x70008ccc74); }
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x2658)) else { return RegfileExit::fault(0x70008ccc77); };
            c.w64(3, t209);
            c.w32(0, 0x4u64);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ccc83); };
            c.w64(6, t210);
            return c.call(0x700086ed74, 0x70008ccc8c, 0x70008ccc87);
        }
        149 => {
            c.w64(1, c.r64(0));
            blk = 82; continue;
        }
        150 => {
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70008ccc94); };
            c.w64(3, t211);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ccc9d); };
            c.w64(6, t212);
            return c.call(0x700086ed74, 0x70008ccca6, 0x70008ccca1);
        }
        151 => {
            return RegfileExit::fault(0x70008ccca6);
        }
        152 => {
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ccca7); };
            c.w64(6, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cccab); };
            c.w64(0, t214);
            blk = 153; continue;
        }
        153 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cccaf); }
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cccb0); };
            if !c.push64(t215) { return RegfileExit::fault(0x70008cccb0); }
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cccb3); };
            if !c.push64(t216) { return RegfileExit::fault(0x70008cccb3); }
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cccb6); };
            if !c.push64(t217) { return RegfileExit::fault(0x70008cccb6); }
            let Some(t218) = c.ld64(c.r64(13).wrapping_add(0x2648)) else { return RegfileExit::fault(0x70008cccb9); };
            c.w64(3, t218);
            c.w32(0, 0x4u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008ccccd, 0x70008cccc8);
        }
        154 => {
            c.w64(1, c.r64(0));
            blk = 82; continue;
        }
        155 => {
            c.w32(11, 0x88u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008cccdb); }
            let Some(t219) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cccdd); };
            c.w64(11, t219);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008ccce4); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ccce6); }
            let Some(t220) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008ccce7); };
            c.w64(3, t220);
            c.w32(0, 0x3u64);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cccf3); };
            c.w64(6, t221);
            return c.call(0x700086ed74, 0x70008cccfc, 0x70008cccf7);
        }
        156 => {
            return RegfileExit::fault(0x70008cccfc);
        }
        157 => {
            c.w32(11, c.r32(11));
            let r222 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r222);
            c.w64(15, 0xfffffffffffc0000u64);
            let r223 = c.r64(15) & c.r64(11);
            c.w64(15, r223);
            c.logic(r223, 64);
            let Some(t224) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ccd0d); };
            c.logic(t224 & 0x2u64, 8);
            if c.cc_e() { blk = 76; continue; }
            blk = 158; continue;
        }
        158 => {
            c.w64(0, c.r64(1).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ccd1c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ccd1d); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x70008ccd2a, 0x70008ccd25);
        }
        159 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ccd2a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ccd2c); }
            blk = 76; continue;
        }
        160 => {
            c.w32(11, c.r32(11));
            let r225 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r225);
            c.w64(15, 0xfffffffffffc0000u64);
            let r226 = c.r64(15) & c.r64(11);
            c.w64(15, r226);
            c.logic(r226, 64);
            let Some(t227) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ccd42); };
            c.logic(t227 & 0x2u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 161; continue;
        }
        161 => {
            c.w64(0, c.r64(1).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ccd51); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ccd52); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x70008ccd5f, 0x70008ccd5a);
        }
        162 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ccd5f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ccd61); }
            blk = 74; continue;
        }
        163 => {
            return RegfileExit::jump(0x70008ccd68);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ccd74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ccd74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ccd78); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ccd7e); };
            c.w32(7, t1);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008ccd87); };
            c.w32(11, t2);
            let r3 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r3);
            c.logic(c.r8(7) & 0x9u64, 8);
            c.w8(7, (c.cc_ne() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ccd9a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008ccd9e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008ccda2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008ccda6); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(12, 0);
            c.logic(0, 32);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 15; continue;
        }
        2 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ccdd0); };
            c.w32(15, t4);
            let r5 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r5);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ccddd); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ccde9); }
            blk = 9; continue;
        }
        6 => {
            c.w64(12, c.r64(11));
            let r6 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r6);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            blk = 9; continue;
        }
        7 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 8; continue;
        }
        8 => {
            let r7 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r7);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008cce0b); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
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
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_l() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 14; continue;
        }
        13 => {
            c.w32(15, 0x1u64);
            blk = 14; continue;
        }
        14 => {
            c.w32(12, 0);
            c.logic(0, 32);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 77; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t8) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cce3f); };
            c.w32(15, t8);
            let r9 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r9);
            let Some(t10) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cce46); };
            c.w32(15, t10);
            let r11 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r11);
            let Some(t12) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cce4d); };
            c.w32(2, t12);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008cce57); }
            c.cmp(c.r64(2), 0x9bu64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x70008cce64);
        }
        17 => {
            let Some(t13) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cce65); };
            c.w32(2, t13);
            let r14 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r14);
            let Some(t15) = c.ld32(c.r64(15).wrapping_add(0x273)) else { return RegfileExit::fault(0x70008cce6b); };
            c.w32(15, t15);
            let r16 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r16);
            let Some(t17) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cce75); };
            c.w32(2, t17);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x70008cce79); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 67; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(1, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t18) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cce8a); };
            c.w32(8, t18);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_b() { blk = 67; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cce9a); };
            c.w32(9, t19);
            let r20 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r20);
            let Some(t21) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ccea1); };
            c.w32(11, t21);
            let r22 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r22);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008ccea8); }
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 79; continue;
        }
        23 => {
            blk = 26; continue;
        }
        24 => {
            c.w64(2, c.r64(15));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ccec8); };
            c.w64(1, t23);
            return c.call(0x7000859a54, 0x70008cced1, 0x70008ccecc);
        }
        25 => {
            c.w64(15, c.r64(0));
            blk = 68; continue;
        }
        26 => {
            let Some(t24) = c.ld32(c.r64(9).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008cced9); };
            c.w32(11, t24);
            c.cmp(c.r32(11), 0x7eu64, 32);
            if c.cc_a() { blk = 92; continue; }
            blk = 27; continue;
        }
        27 => {
            c.logic(c.r8(2) & 0x7u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008ccef0); }
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(3));
            blk = 30; continue;
        }
        29 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008ccefb); }
            blk = 30; continue;
        }
        30 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cceff); };
            c.w64(15, t25);
            let r26 = c.r32(15) & 0x7u64;
            c.w32(15, r26);
            c.logic(r26, 32);
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_l() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 92; continue;
        }
        35 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            blk = 92; continue;
        }
        38 => {
            let Some(t27) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ccf39); };
            c.w32(15, t27);
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008ccf3d); };
            c.cmp(t28, c.r32(15), 32);
            if c.cc_ne() { blk = 92; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t29) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ccf4a); };
            c.w32(7, t29);
            let r30 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r30);
            let Some(t31) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ccf50); };
            c.w32(15, t31);
            let r32 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r32);
            let Some(t33) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ccf57); };
            c.w32(15, t33);
            blk = 29; continue;
        }
        40 => {
            let Some(t34) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ccf5e); };
            c.w32(15, t34);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r35 = c.add(c.r64(2), c.r64(15), 64);
            c.w64(2, r35);
            let Some(t36) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ccf6b); };
            c.w32(7, t36);
            let r37 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r37);
            let Some(t38) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ccf71); };
            c.w32(15, t38);
            let r39 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r39);
            let Some(t40) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ccf78); };
            c.w32(15, t40);
            blk = 29; continue;
        }
        41 => {
            let Some(t41) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ccf82); };
            c.w32(7, t41);
            let r42 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r42);
            let Some(t43) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ccf88); };
            c.w32(15, t43);
            let r44 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r44);
            let Some(t45) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ccf8f); };
            c.w32(15, t45);
            blk = 29; continue;
        }
        42 => {
            c.w64(15, c.r64(12));
            blk = 45; continue;
        }
        43 => {
            c.w32(15, 0x1u64);
            blk = 45; continue;
        }
        44 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008ccfa6); }
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(3));
            c.w64(15, c.r64(12));
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 81; continue; }
            blk = 46; continue;
        }
        46 => {
            let r46 = c.add(c.r64(7), 0xbu64, 64);
            c.w64(7, r46);
            blk = 47; continue;
        }
        47 => {
            c.w64(15, c.r64(2).wrapping_add(c.r64(8)));
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ccfc4); };
            c.w64(8, t47);
            let r48 = c.r32(8) & 0x8u64;
            c.w32(8, r48);
            c.logic(r48, 32);
            let r49 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r49);
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 48; continue;
        }
        48 => {
            let r50 = c.add(c.r64(15), c.r64(7), 64);
            c.w64(15, r50);
            let r51 = c.add(c.r64(7), c.r64(2), 64);
            c.w64(7, r51);
            let Some(t52) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ccfdf); };
            c.w32(2, t52);
            let r53 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r53);
            blk = 49; continue;
        }
        49 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008ccfe6); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 92; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t54) = c.ld32(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008ccff3); };
            c.w32(8, t54);
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2e70)) else { return RegfileExit::fault(0x70008ccff7); };
            c.w64(7, t55);
            let Some(t56) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ccffe); };
            c.w64(7, t56);
            c.cmp(c.r32(8), 0xffffffffffffffffu64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w32(2, c.r64(8).wrapping_mul(8));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(13));
            let Some(t57) = c.ld64(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008cd015); };
            c.w64(2, t57);
            blk = 53; continue;
        }
        52 => {
            let r58 = c.add(c.r64(2), 0x3fu64, 64);
            c.w64(2, r58);
            c.w64(8, c.r64(13));
            blk = 53; continue;
        }
        53 => {
            let r59 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r59);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w32(1, c.r32(1));
            c.w64(10, c.r64(4));
            let r60 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r60);
            let r61 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r61);
            c.logic(r61, 64);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x70008cd03d); }
            let r62 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r62);
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008cd045); };
            c.w64(12, t63);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(12)) { return RegfileExit::fault(0x70008cd04c); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(11)) { return RegfileExit::fault(0x70008cd051); }
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(7)) { return RegfileExit::fault(0x70008cd056); }
            if !c.st64(c.r64(4).wrapping_add(0x38), 0x1u64) { return RegfileExit::fault(0x70008cd05b); }
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(8)) { return RegfileExit::fault(0x70008cd064); }
            c.w64(7, c.r64(0));
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(7)) { return RegfileExit::fault(0x70008cd06c); }
            c.w64(7, c.r64(2));
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(3));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cd07a); };
            c.w64(8, t64);
            c.w64(9, c.r64(15));
            c.w64(10, 0x70008cd088u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008cd088); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008cd08c); }
            return c.call(c.r64(7), 0x70008cd092, 0x70008cd090);
        }
        54 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008cd092); }
            let Some(t65) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x70008cd09a); };
            c.w64(4, t65);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_e() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_ne() { blk = 84; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cd0b2); };
            c.w64(0, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cd0b6); };
            c.w64(7, t67);
            blk = 67; continue;
        }
        57 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cd0bf); };
            c.w64(15, t68);
            let Some(t69) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cd0c3); };
            c.w32(7, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cd0c7); };
            c.w64(11, t70);
            let Some(t71) = c.ld32(c.r64(11).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008cd0cb); };
            c.w32(11, t71);
            let r72 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r72);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r73 = c.sub(c.r32(7), 0x6u64, 32);
            c.w32(7, r73);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_l() { blk = 91; continue; }
            blk = 58; continue;
        }
        58 => {
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008cd0e5); }
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cd0e9); };
            c.w64(7, t74);
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008cd0ed); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r75 = c.r64(12) & c.r64(15);
            c.w64(12, r75);
            c.logic(r75, 64);
            let Some(t76) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cd101); };
            c.logic(t76 & 0x4u64, 8);
            if c.cc_ne() { blk = 97; continue; }
            blk = 60; continue;
        }
        60 => {
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008cd10d); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r77 = c.r64(12) & c.r64(15);
            c.w64(12, r77);
            c.logic(r77, 64);
            let Some(t78) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cd121); };
            c.logic(t78 & 0x4u64, 8);
            if c.cc_ne() { blk = 94; continue; }
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
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008cd13e); };
            c.w64(0, t79);
            blk = 65; continue;
        }
        64 => {
            let r80 = c.add(c.r64(11), 0x4u64, 64);
            c.w64(11, r80);
            blk = 65; continue;
        }
        65 => {
            let Some(t81) = c.ld32(c.r64(0).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008cd14b); };
            c.w32(3, t81);
            let r82 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r82);
            if !c.st32(c.r64(15).wrapping_add(c.r64(11)), c.r32(3)) { return RegfileExit::fault(0x70008cd151); }
            let r83 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r83);
            c.cmp(c.r64(7), c.r64(12), 64);
            if c.cc_ne() { blk = 64; continue; }
            blk = 66; continue;
        }
        66 => {
            blk = 68; continue;
        }
        67 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008cd160); };
            c.w64(15, t84);
            blk = 68; continue;
        }
        68 => {
            let Some(t85) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008cd167); };
            c.cmp(t85, c.r32(15), 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t86) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cd170); };
            c.cmp(t86, 0x0u64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cd176); };
            c.w64(7, t87);
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008cd17d); }
            blk = 78; continue;
        }
        71 => {
            let Some(t88) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cd183); };
            c.cmp(t88, 0x0u64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t89) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cd189); };
            c.w32(7, t89);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x4u64, 64);
            if c.cc_a() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            return RegfileExit::fault(0x70008cd199);
        }
        74 => {
            let Some(t90) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008cd19a); };
            c.w32(11, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cd19e); };
            c.w64(7, t91);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008cd1a2); }
            blk = 75; continue;
        }
        75 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008cd1a6); };
            c.w64(0, t92);
            blk = 76; continue;
        }
        76 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cd1b0); }
            return c.ret(0x0, 0x70008cd1b1);
        }
        77 => {
            c.w32(7, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(7)) { return RegfileExit::fault(0x70008cd1b4); }
            blk = 78; continue;
        }
        78 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cd1b7); };
            c.w64(0, t93);
            blk = 76; continue;
        }
        79 => {
            return RegfileExit::fault(0x70008cd1c0);
        }
        80 => {
            blk = 92; continue;
        }
        81 => {
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cd1c6); };
            c.w64(15, t94);
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 92; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t95) = c.ld64(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cd1d4); };
            c.w64(7, t95);
            blk = 47; continue;
        }
        83 => {
            c.w64(15, c.r64(7).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.w64(7, c.r64(7).wrapping_add(c.r64(2).wrapping_mul(2)));
            let Some(t96) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008cd1e5); };
            c.w32(2, t96);
            let r97 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r97);
            blk = 49; continue;
        }
        84 => {
            c.cmp(c.r64(7), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 89; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r64(7), 0xfffffffffffffffdu64, 64);
            if c.cc_e() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cd1fd); };
            c.w64(0, t98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cd201); };
            c.w64(3, t99);
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cd205); };
            c.w64(6, t100);
            blk = 92; continue;
        }
        87 => {
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cd20b); };
            if !c.push64(t101) { return RegfileExit::fault(0x70008cd20b); }
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cd20e); };
            if !c.push64(t102) { return RegfileExit::fault(0x70008cd20e); }
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cd211); };
            if !c.push64(t103) { return RegfileExit::fault(0x70008cd211); }
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cd214); };
            if !c.push64(t104) { return RegfileExit::fault(0x70008cd214); }
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x2658)) else { return RegfileExit::fault(0x70008cd217); };
            c.w64(3, t105);
            c.w32(0, 0x4u64);
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cd223); };
            c.w64(6, t106);
            return c.call(0x700086ed74, 0x70008cd22c, 0x70008cd227);
        }
        88 => {
            c.w64(15, c.r64(0));
            blk = 68; continue;
        }
        89 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70008cd234); };
            c.w64(3, t107);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cd23d); };
            c.w64(6, t108);
            return c.call(0x700086ed74, 0x70008cd246, 0x70008cd241);
        }
        90 => {
            return RegfileExit::fault(0x70008cd246);
        }
        91 => {
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cd247); };
            c.w64(0, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cd24b); };
            c.w64(3, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cd24f); };
            c.w64(6, t111);
            blk = 92; continue;
        }
        92 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cd253); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008cd254); }
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cd255); };
            if !c.push64(t112) { return RegfileExit::fault(0x70008cd255); }
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cd258); };
            if !c.push64(t113) { return RegfileExit::fault(0x70008cd258); }
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x2648)) else { return RegfileExit::fault(0x70008cd25b); };
            c.w64(3, t114);
            c.w32(0, 0x4u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008cd26f, 0x70008cd26a);
        }
        93 => {
            c.w64(15, c.r64(0));
            blk = 68; continue;
        }
        94 => {
            c.w32(7, c.r32(7));
            let r115 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r115);
            c.w64(12, 0xfffffffffffc0000u64);
            let r116 = c.r64(12) & c.r64(7);
            c.w64(12, r116);
            c.logic(r116, 64);
            let Some(t117) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cd286); };
            c.logic(t117 & 0x2u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(0, c.r64(15).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cd296); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008cd297); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x70008cd2a4, 0x70008cd29f);
        }
        96 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008cd2a4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008cd2a6); }
            blk = 62; continue;
        }
        97 => {
            c.w32(7, c.r32(7));
            let r118 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r118);
            c.w64(12, 0xfffffffffffc0000u64);
            let r119 = c.r64(12) & c.r64(7);
            c.w64(12, r119);
            c.logic(r119, 64);
            let Some(t120) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cd2bb); };
            c.logic(t120 & 0x2u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 98; continue;
        }
        98 => {
            c.w64(0, c.r64(15).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cd2cb); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008cd2cc); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x70008cd2d9, 0x70008cd2d4);
        }
        99 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008cd2d9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008cd2db); }
            blk = 60; continue;
        }
        100 => {
            return RegfileExit::jump(0x70008cd2e4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cd2f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cd2f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cd2f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cd2f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cd2fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cd2ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cd309); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cd312); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cd318); };
            c.cmp(t5, 0x83eu64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cd320); };
            c.w32(2, t6);
            c.w32(2, c.r32(2));
            let r7 = c.r32(2) & 0x2u64;
            c.w32(2, r7);
            c.logic(r7, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r8 = c.r64(2) & 0xfffffffffffffffeu64;
            c.w64(2, r8);
            c.logic(r8, 64);
            let r9 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r9);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cd336); };
            c.w64(0, t10);
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cd33d); };
            c.w64(1, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cd344); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x70008cd34b);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cd34e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cd355); }
            return c.ret(0x0, 0x70008cd357);
        }
        8 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008cd358); };
            c.w64(0, t12);
            blk = 5; continue;
        }
        9 => {
            let Some(t13) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cd361); };
            c.w32(1, t13);
            let r14 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r14);
            let Some(t15) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cd367); };
            c.w32(1, t15);
            let r16 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r16);
            let Some(t17) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cd36d); };
            c.w32(7, t17);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x9au64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x70008cd37f);
        }
        11 => {
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008cd380); };
            c.w32(1, t18);
            let r19 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r19);
            let Some(t20) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008cd389); };
            c.w32(1, t20);
            let r21 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r21);
            let Some(t22) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cd38f); };
            c.cmp(t22, c.r32(2), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cd394); };
            c.w64(0, t23);
            blk = 5; continue;
        }
        13 => {
            c.w32(2, 0x146u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cd3a2); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cd3a3); };
            c.w64(2, t24);
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x70008cd3aa); };
            c.w32(2, t25);
            let r26 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r26);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cd3b3); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008cd3b4); };
            c.w64(3, t27);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008cd3c5, 0x70008cd3c0);
        }
        14 => {
            return RegfileExit::fault(0x70008cd3c5);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008cd3c6); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cd3ca); };
            c.w64(3, t28);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008cd3d8, 0x70008cd3d3);
        }
        16 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cd3d8); };
            c.w64(6, t29);
            blk = 1; continue;
        }
        17 => {
            return RegfileExit::jump(0x70008cd3e4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cd3f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cd3f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cd3f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cd3f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cd3fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cd3ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cd409); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cd412); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cd418); };
            c.cmp(t5, 0x83eu64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cd420); };
            c.w32(2, t6);
            c.w32(2, c.r32(2));
            let r7 = c.r32(2) & 0x4u64;
            c.w32(2, r7);
            c.logic(r7, 32);
            let r8 = c.shr(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r8);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r9 = c.r64(2) & 0xfffffffffffffffeu64;
            c.w64(2, r9);
            c.logic(r9, 64);
            let r10 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r10);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cd438); };
            c.w64(0, t11);
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cd43f); };
            c.w64(1, t12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cd446); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x70008cd44d);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cd450); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cd457); }
            return c.ret(0x0, 0x70008cd459);
        }
        8 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008cd45a); };
            c.w64(0, t13);
            blk = 5; continue;
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cd463); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cd469); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cd46f); };
            c.w32(7, t18);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x9au64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x70008cd481);
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(1).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008cd482); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008cd48b); };
            c.w32(1, t21);
            let r22 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r22);
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cd491); };
            c.cmp(t23, c.r32(2), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cd496); };
            c.w64(0, t24);
            blk = 5; continue;
        }
        13 => {
            c.w32(2, 0x146u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cd4a4); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cd4a5); };
            c.w64(2, t25);
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(0x2af)) else { return RegfileExit::fault(0x70008cd4ac); };
            c.w32(2, t26);
            let r27 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r27);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cd4b5); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008cd4b6); };
            c.w64(3, t28);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008cd4c7, 0x70008cd4c2);
        }
        14 => {
            return RegfileExit::fault(0x70008cd4c7);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008cd4c8); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cd4cc); };
            c.w64(3, t29);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008cd4da, 0x70008cd4d5);
        }
        16 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cd4da); };
            c.w64(6, t30);
            blk = 1; continue;
        }
        17 => {
            return RegfileExit::jump(0x70008cd4e4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cd4f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cd4f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cd4f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cd4f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cd4fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cd4ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cd509); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cd512); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cd518); };
            c.cmp(t5, 0x83eu64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cd520); };
            c.w32(2, t6);
            c.w32(2, c.r32(2));
            let r7 = c.r32(2) & 0x8u64;
            c.w32(2, r7);
            c.logic(r7, 32);
            let r8 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r8);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r9 = c.r64(2) & 0xfffffffffffffffeu64;
            c.w64(2, r9);
            c.logic(r9, 64);
            let r10 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r10);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cd539); };
            c.w64(0, t11);
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cd540); };
            c.w64(1, t12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cd547); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x70008cd54e);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cd551); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cd558); }
            return c.ret(0x0, 0x70008cd55a);
        }
        8 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008cd55b); };
            c.w64(0, t13);
            blk = 5; continue;
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cd564); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cd56a); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cd570); };
            c.w32(7, t18);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x9au64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x70008cd582);
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(1).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008cd583); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008cd58c); };
            c.w32(1, t21);
            let r22 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r22);
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cd592); };
            c.cmp(t23, c.r32(2), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cd597); };
            c.w64(0, t24);
            blk = 5; continue;
        }
        13 => {
            c.w32(2, 0x146u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cd5a5); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cd5a6); };
            c.w64(2, t25);
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(0x2b3)) else { return RegfileExit::fault(0x70008cd5ad); };
            c.w32(2, t26);
            let r27 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r27);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cd5b6); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008cd5b7); };
            c.w64(3, t28);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008cd5c8, 0x70008cd5c3);
        }
        14 => {
            return RegfileExit::fault(0x70008cd5c8);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008cd5c9); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cd5cd); };
            c.w64(3, t29);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008cd5db, 0x70008cd5d6);
        }
        16 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cd5db); };
            c.w64(6, t30);
            blk = 1; continue;
        }
        17 => {
            return RegfileExit::jump(0x70008cd5e8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cd5f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cd5f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cd5f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cd5f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cd5fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cd5ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cd609); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cd612); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cd618); };
            c.cmp(t5, 0x83eu64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cd620); };
            c.w32(2, t6);
            c.w32(2, c.r32(2));
            let r7 = c.r32(2) & 0x100u64;
            c.w32(2, r7);
            c.logic(r7, 32);
            let r8 = c.shr(c.r32(2), 0x7u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r8);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r9 = c.r64(2) & 0xfffffffffffffffeu64;
            c.w64(2, r9);
            c.logic(r9, 64);
            let r10 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r10);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cd63c); };
            c.w64(0, t11);
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cd643); };
            c.w64(1, t12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cd64a); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x70008cd651);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cd654); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cd65b); }
            return c.ret(0x0, 0x70008cd65d);
        }
        8 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008cd65e); };
            c.w64(0, t13);
            blk = 5; continue;
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cd667); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cd66d); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cd673); };
            c.w32(7, t18);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x9au64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x70008cd685);
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(1).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008cd686); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008cd68f); };
            c.w32(1, t21);
            let r22 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r22);
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cd695); };
            c.cmp(t23, c.r32(2), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cd69a); };
            c.w64(0, t24);
            blk = 5; continue;
        }
        13 => {
            c.w32(2, 0x146u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cd6a8); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cd6a9); };
            c.w64(2, t25);
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(0x2b7)) else { return RegfileExit::fault(0x70008cd6b0); };
            c.w32(2, t26);
            let r27 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r27);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cd6b9); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008cd6ba); };
            c.w64(3, t28);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008cd6cb, 0x70008cd6c6);
        }
        14 => {
            return RegfileExit::fault(0x70008cd6cb);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008cd6cc); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cd6d0); };
            c.w64(3, t29);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008cd6de, 0x70008cd6d9);
        }
        16 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cd6de); };
            c.w64(6, t30);
            blk = 1; continue;
        }
        17 => {
            return RegfileExit::jump(0x70008cd6e8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cd6f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cd6f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cd6f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cd6f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cd6fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cd6ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cd709); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cd712); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cd718); };
            c.cmp(t5, 0x83eu64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cd720); };
            c.w32(2, t6);
            c.w32(2, c.r32(2));
            let r7 = c.r32(2) & 0x80u64;
            c.w32(2, r7);
            c.logic(r7, 32);
            let r8 = c.shr(c.r32(2), 0x6u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r8);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r9 = c.r64(2) & 0xfffffffffffffffeu64;
            c.w64(2, r9);
            c.logic(r9, 64);
            let r10 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r10);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cd73c); };
            c.w64(0, t11);
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cd743); };
            c.w64(1, t12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cd74a); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x70008cd751);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cd754); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cd75b); }
            return c.ret(0x0, 0x70008cd75d);
        }
        8 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008cd75e); };
            c.w64(0, t13);
            blk = 5; continue;
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cd767); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cd76d); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cd773); };
            c.w32(7, t18);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x9au64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x70008cd785);
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(1).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008cd786); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008cd78f); };
            c.w32(1, t21);
            let r22 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r22);
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cd795); };
            c.cmp(t23, c.r32(2), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cd79a); };
            c.w64(0, t24);
            blk = 5; continue;
        }
        13 => {
            c.w32(2, 0x146u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cd7a8); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cd7a9); };
            c.w64(2, t25);
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(0x2bb)) else { return RegfileExit::fault(0x70008cd7b0); };
            c.w32(2, t26);
            let r27 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r27);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cd7b9); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008cd7ba); };
            c.w64(3, t28);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008cd7cb, 0x70008cd7c6);
        }
        14 => {
            return RegfileExit::fault(0x70008cd7cb);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008cd7cc); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cd7d0); };
            c.w64(3, t29);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008cd7de, 0x70008cd7d9);
        }
        16 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cd7de); };
            c.w64(6, t30);
            blk = 1; continue;
        }
        17 => {
            return RegfileExit::jump(0x70008cd7e8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cd7f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cd7f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cd7f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cd7f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cd7fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cd7ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cd809); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cd812); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cd818); };
            c.cmp(t5, 0x83eu64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cd820); };
            c.w32(2, t6);
            c.w32(2, c.r32(2));
            let r7 = c.r32(2) & 0x40u64;
            c.w32(2, r7);
            c.logic(r7, 32);
            let r8 = c.shr(c.r32(2), 0x5u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r8);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r9 = c.r64(2) & 0xfffffffffffffffeu64;
            c.w64(2, r9);
            c.logic(r9, 64);
            let r10 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r10);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cd839); };
            c.w64(0, t11);
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cd840); };
            c.w64(1, t12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cd847); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x70008cd84e);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cd851); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cd858); }
            return c.ret(0x0, 0x70008cd85a);
        }
        8 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008cd85b); };
            c.w64(0, t13);
            blk = 5; continue;
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cd864); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cd86a); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cd870); };
            c.w32(7, t18);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x9au64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x70008cd882);
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(1).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008cd883); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008cd88c); };
            c.w32(1, t21);
            let r22 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r22);
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cd892); };
            c.cmp(t23, c.r32(2), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cd897); };
            c.w64(0, t24);
            blk = 5; continue;
        }
        13 => {
            c.w32(2, 0x146u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cd8a5); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cd8a6); };
            c.w64(2, t25);
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x70008cd8ad); };
            c.w32(2, t26);
            let r27 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r27);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cd8b6); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008cd8b7); };
            c.w64(3, t28);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008cd8c8, 0x70008cd8c3);
        }
        14 => {
            return RegfileExit::fault(0x70008cd8c8);
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008cd8c9); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cd8cd); };
            c.w64(3, t29);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008cd8db, 0x70008cd8d6);
        }
        16 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cd8db); };
            c.w64(6, t30);
            blk = 1; continue;
        }
        17 => {
            return RegfileExit::jump(0x70008cd8e8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cd8f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cd8f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cd8f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cd8f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cd8fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cd8ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 16; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cd909); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cd912); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cd918); };
            c.cmp(t5, 0x83eu64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cd920); };
            c.w32(2, t6);
            c.w32(2, c.r32(2));
            let r7 = c.r32(2) & 0x10u64;
            c.w32(2, r7);
            c.logic(r7, 32);
            let r8 = c.shr(c.r32(2), 0x3u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r8);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r9 = c.r64(2) & 0xfffffffffffffffeu64;
            c.w64(2, r9);
            c.logic(r9, 64);
            let r10 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r10);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cd939); };
            c.w64(0, t11);
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cd940); };
            c.w64(1, t12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cd947); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x70008cd94e);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cd951); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cd958); }
            return c.ret(0x0, 0x70008cd95a);
        }
        8 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008cd95b); };
            c.w64(0, t13);
            blk = 5; continue;
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cd964); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cd96a); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cd970); };
            c.w32(7, t18);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x9au64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x70008cd982);
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(1).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008cd983); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008cd98c); };
            c.w32(1, t21);
            let r22 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r22);
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cd992); };
            c.cmp(t23, c.r32(2), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(2, 0x16u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cd99c); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x70008cd99d); };
            c.w64(3, t24);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008cd9ae, 0x70008cd9a9);
        }
        13 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cd9ae); };
            c.w64(0, t25);
            blk = 5; continue;
        }
        14 => {
            c.w32(2, 0x146u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cd9bc); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cd9bd); };
            c.w64(2, t26);
            let Some(t27) = c.ld32(c.r64(2).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x70008cd9c4); };
            c.w32(2, t27);
            let r28 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r28);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cd9cd); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008cd9ce); };
            c.w64(3, t29);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008cd9df, 0x70008cd9da);
        }
        15 => {
            return RegfileExit::fault(0x70008cd9df);
        }
        16 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008cd9e0); }
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cd9e4); };
            c.w64(3, t30);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008cd9f2, 0x70008cd9ed);
        }
        17 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cd9f2); };
            c.w64(6, t31);
            blk = 1; continue;
        }
        18 => {
            return RegfileExit::jump(0x70008cd9fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cda14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cda14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cda18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cda19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cda1a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cda1f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 16; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cda29); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cda32); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cda38); };
            c.cmp(t5, 0x83eu64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cda40); };
            c.w32(2, t6);
            c.w32(2, c.r32(2));
            let r7 = c.r32(2) & 0x20u64;
            c.w32(2, r7);
            c.logic(r7, 32);
            let r8 = c.shr(c.r32(2), 0x4u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r8);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r9 = c.r64(2) & 0xfffffffffffffffeu64;
            c.w64(2, r9);
            c.logic(r9, 64);
            let r10 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r10);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cda59); };
            c.w64(0, t11);
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cda60); };
            c.w64(1, t12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cda67); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x70008cda6e);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cda71); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cda78); }
            return c.ret(0x0, 0x70008cda7a);
        }
        8 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008cda7b); };
            c.w64(0, t13);
            blk = 5; continue;
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cda84); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cda8a); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cda90); };
            c.w32(7, t18);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x9au64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x70008cdaa2);
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(1).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008cdaa3); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008cdaac); };
            c.w32(1, t21);
            let r22 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r22);
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cdab2); };
            c.cmp(t23, c.r32(2), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(2, 0x1au64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cdabc); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x70008cdabd); };
            c.w64(3, t24);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008cdace, 0x70008cdac9);
        }
        13 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cdace); };
            c.w64(0, t25);
            blk = 5; continue;
        }
        14 => {
            c.w32(2, 0x146u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cdadc); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cdadd); };
            c.w64(2, t26);
            let Some(t27) = c.ld32(c.r64(2).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x70008cdae4); };
            c.w32(2, t27);
            let r28 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r28);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cdaed); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008cdaee); };
            c.w64(3, t29);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008cdaff, 0x70008cdafa);
        }
        15 => {
            return RegfileExit::fault(0x70008cdaff);
        }
        16 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008cdb00); }
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cdb04); };
            c.w64(3, t30);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008cdb12, 0x70008cdb0d);
        }
        17 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cdb12); };
            c.w64(6, t31);
            blk = 1; continue;
        }
        18 => {
            return RegfileExit::jump(0x70008cdb1c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cdb34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cdb34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cdb38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cdb39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cdb3a); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008cdb3f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cdb43); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 172; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cdb4d); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 210; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cdb59); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cdb5f); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 210; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(7, 0);
            c.logic(0, 32);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008cdb6d); };
            c.w64(8, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cdb74); };
            c.cmp(t7, c.r32(7), 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cdb7e); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cdb85); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            let Some(t12) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cdb8c); };
            c.w32(9, t12);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x9au64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70008cdb9f);
        }
        6 => {
            let Some(t13) = c.ld32(c.r64(8).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008cdba0); };
            c.w32(11, t13);
            let r14 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r14);
            let Some(t15) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008cdbaa); };
            c.cmp(t15, c.r32(1), 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008cdbb4); };
            c.w32(11, t16);
            c.logic(c.r32(11) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(9), 0x9fu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x70008cdbce);
        }
        10 => {
            let Some(t17) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cdbcf); };
            c.w32(1, t17);
            let r18 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r18);
            let Some(t19) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cdbd5); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008cdbdb); };
            c.w32(9, t21);
            let r22 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r22);
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0x283)) else { return RegfileExit::fault(0x70008cdbe2); };
            c.cmp(t23, c.r32(1), 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t24) = c.ld32(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008cdbef); };
            c.w32(1, t24);
            c.w32(1, c.r32(1));
            let r25 = c.r32(1) & 0x8u64;
            c.w32(1, r25);
            c.logic(r25, 32);
            c.cmp(c.r32(1), 0x8u64, 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cdc01); };
            c.w32(1, t26);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.logic(c.r8(1) & 0xffffffffffffff80u64, 8);
            c.w8(7, (c.cc_ne() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 15; continue;
        }
        14 => {
            c.w32(0, 0x1u64);
            c.w64(8, c.r64(0));
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(8, (c.cc_ne() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            let r27 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r27);
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(1) & 0x2u64, 8);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let r28 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r28);
            blk = 19; continue;
        }
        19 => {
            c.logic(c.r8(1) & 0x40u64, 8);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let r29 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r29);
            blk = 21; continue;
        }
        21 => {
            c.logic(c.r8(1) & 0x4u64, 8);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            let r30 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r30);
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(1) & 0x20u64, 8);
            c.w8(15, (c.cc_ne() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            let r31 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r31);
            blk = 25; continue;
        }
        25 => {
            c.logic(c.r8(1) & 0x10u64, 8);
            c.w8(3, (c.cc_ne() as u64));
            c.w32(3, c.r8(3));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008cdc93); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            let r32 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r32);
            blk = 27; continue;
        }
        27 => {
            c.logic(c.r8(1) & 0x8u64, 8);
            c.w8(1, (c.cc_ne() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008cdca8); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            let r33 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r33);
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w32(6, 0xcu64);
            let r34 = c.add(c.r64(2), c.r64(6), 64);
            c.w64(2, r34);
            if c.cc_o() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cdcc6); };
            c.w64(6, t35);
            let Some(t36) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008cdccd); };
            c.w64(6, t36);
            let r37 = c.add(c.r64(2), 0x3u64, 64);
            c.w64(2, r37);
            let r38 = c.r64(2) & 0xfffffffffffffffcu64;
            c.w64(2, r38);
            c.logic(r38, 64);
            c.w64(1, c.r64(6).wrapping_add(c.r64(2)));
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008cdcdc); };
            c.w64(3, t39);
            let Some(t40) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008cdce3); };
            c.cmp(t40, c.r64(1), 64);
            if c.cc_be() { blk = 174; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 174; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008cdcf9); };
            c.w64(3, t41);
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x70008cdd00); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 34; continue;
        }
        34 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008cdd07); };
            c.w64(3, t42);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008cdd0e); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008cdd11); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008cdd18); }
            c.w64(0, c.r64(1));
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cdd1e); };
            c.w64(1, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cdd22); };
            c.w64(3, t44);
            blk = 37; continue;
        }
        35 => {
            return RegfileExit::fault(0x70008cdd28);
        }
        36 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cdd29); };
            c.w64(0, t45);
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(2, 0xbu64);
            blk = 40; continue;
        }
        39 => {
            if !c.st8(c.r64(0).wrapping_add(0xb), 0x64u64) { return RegfileExit::fault(0x70008cdd3c); }
            c.w32(2, 0xcu64);
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), 0x67u64) { return RegfileExit::fault(0x70008cdd4b); }
            let r46 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r46);
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), 0x69u64) { return RegfileExit::fault(0x70008cdd59); }
            let r47 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r47);
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), 0x6cu64) { return RegfileExit::fault(0x70008cdd67); }
            let r48 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r48);
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), 0x6du64) { return RegfileExit::fault(0x70008cdd75); }
            let r49 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r49);
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), 0x73u64) { return RegfileExit::fault(0x70008cdd83); }
            let r50 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r50);
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(7, c.r64(2).wrapping_add(0x1));
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), 0x75u64) { return RegfileExit::fault(0x70008cdd94); }
            c.w64(2, c.r64(7));
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), 0x79u64) { return RegfileExit::fault(0x70008cdda0); }
            blk = 54; continue;
        }
        54 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cdda4); };
            c.w64(1, t51);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cddab); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            return c.ret(0x8, 0x70008cddb2);
        }
        56 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cddb5); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cddbc); }
            return c.ret(0x0, 0x70008cddbe);
        }
        57 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x768)) else { return RegfileExit::fault(0x70008cddbf); };
            c.w64(3, t52);
            return c.call(0x7000870914, 0x70008cddcb, 0x70008cddc6);
        }
        58 => {
            let Some(t53) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cddcb); };
            c.cmp(t53, c.r32(0), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 67; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t54) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cddda); };
            c.cmp(t54, c.r32(0), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t55) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cdde3); };
            c.w32(1, t55);
            let r56 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r56);
            let Some(t57) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008cdde9); };
            c.logic(t57 & 0x10u64, 8);
            if c.cc_ne() { blk = 65; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t58) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008cddef); };
            c.cmp(t58, c.r32(1), 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t59) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cddfc); };
            c.cmp(t59, 0x41u64, 16);
            if c.cc_e() { blk = 176; continue; }
            blk = 64; continue;
        }
        64 => {
            blk = 68; continue;
        }
        65 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 71; continue;
        }
        66 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 71; continue;
        }
        67 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w32(7, 0x1u64);
            c.w32(0, 0x1u64);
            blk = 71; continue;
        }
        69 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 71; continue;
        }
        70 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 71; continue;
        }
        71 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008cde30); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x70008cde34); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x798)) else { return RegfileExit::fault(0x70008cde38); };
            c.w64(3, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cde3f); };
            c.w64(0, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cde43); };
            c.w64(6, t62);
            return c.call(0x7000870914, 0x70008cde4c, 0x70008cde47);
        }
        72 => {
            let Some(t63) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cde4c); };
            c.cmp(t63, c.r32(0), 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 79; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t64) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cde5b); };
            c.cmp(t64, c.r32(0), 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t65) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cde64); };
            c.w32(1, t65);
            let r66 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r66);
            let Some(t67) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008cde6a); };
            c.logic(t67 & 0x10u64, 8);
            if c.cc_ne() { blk = 81; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t68) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008cde70); };
            c.cmp(t68, c.r32(1), 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t69) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cde7d); };
            c.cmp(t69, 0x41u64, 16);
            if c.cc_e() { blk = 180; continue; }
            blk = 78; continue;
        }
        78 => {
            blk = 80; continue;
        }
        79 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cde8f); };
            c.w64(7, t70);
            let r71 = c.r64(7) | 0x2u64;
            c.w64(7, r71);
            c.logic(r71, 64);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cde97); };
            c.w64(0, t72);
            let r73 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r73);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008cde9e); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x70008cdea2); }
            blk = 81; continue;
        }
        81 => {
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x888)) else { return RegfileExit::fault(0x70008cdea6); };
            c.w64(3, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cdead); };
            c.w64(0, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cdeb1); };
            c.w64(6, t76);
            return c.call(0x7000870914, 0x70008cdeba, 0x70008cdeb5);
        }
        82 => {
            let Some(t77) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cdeba); };
            c.cmp(t77, c.r32(0), 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 83; continue;
        }
        83 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 89; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t78) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cdec9); };
            c.cmp(t78, c.r32(0), 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t79) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cded2); };
            c.w32(1, t79);
            let r80 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r80);
            let Some(t81) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008cded8); };
            c.logic(t81 & 0x10u64, 8);
            if c.cc_ne() { blk = 91; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t82) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008cdede); };
            c.cmp(t82, c.r32(1), 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t83) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cdeeb); };
            c.cmp(t83, 0x41u64, 16);
            if c.cc_e() { blk = 184; continue; }
            blk = 88; continue;
        }
        88 => {
            blk = 90; continue;
        }
        89 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cdefd); };
            c.w64(7, t84);
            let r85 = c.r64(7) | 0x4u64;
            c.w64(7, r85);
            c.logic(r85, 64);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cdf05); };
            c.w64(0, t86);
            let r87 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r87);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008cdf0c); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x70008cdf10); }
            blk = 91; continue;
        }
        91 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x6a8)) else { return RegfileExit::fault(0x70008cdf14); };
            c.w64(3, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cdf1b); };
            c.w64(0, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cdf1f); };
            c.w64(6, t90);
            return c.call(0x7000870914, 0x70008cdf28, 0x70008cdf23);
        }
        92 => {
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cdf28); };
            c.cmp(t91, c.r32(0), 32);
            if c.cc_e() { blk = 101; continue; }
            blk = 93; continue;
        }
        93 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 99; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t92) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cdf37); };
            c.cmp(t92, c.r32(0), 32);
            if c.cc_e() { blk = 101; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t93) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cdf40); };
            c.w32(1, t93);
            let r94 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r94);
            let Some(t95) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008cdf46); };
            c.logic(t95 & 0x10u64, 8);
            if c.cc_ne() { blk = 101; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t96) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008cdf4c); };
            c.cmp(t96, c.r32(1), 32);
            if c.cc_e() { blk = 190; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t97) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cdf59); };
            c.cmp(t97, 0x41u64, 16);
            if c.cc_e() { blk = 188; continue; }
            blk = 98; continue;
        }
        98 => {
            blk = 100; continue;
        }
        99 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 101; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cdf6b); };
            c.w64(7, t98);
            let r99 = c.r64(7) | 0x20u64;
            c.w64(7, r99);
            c.logic(r99, 64);
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cdf73); };
            c.w64(0, t100);
            let r101 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r101);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008cdf7a); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x70008cdf7e); }
            blk = 101; continue;
        }
        101 => {
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0xb40)) else { return RegfileExit::fault(0x70008cdf82); };
            c.w64(3, t102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cdf89); };
            c.w64(0, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cdf8d); };
            c.w64(6, t104);
            return c.call(0x7000870914, 0x70008cdf96, 0x70008cdf91);
        }
        102 => {
            let Some(t105) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cdf96); };
            c.cmp(t105, c.r32(0), 32);
            if c.cc_e() { blk = 111; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 109; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t106) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cdfa5); };
            c.cmp(t106, c.r32(0), 32);
            if c.cc_e() { blk = 111; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t107) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cdfae); };
            c.w32(1, t107);
            let r108 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r108);
            let Some(t109) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008cdfb4); };
            c.logic(t109 & 0x10u64, 8);
            if c.cc_ne() { blk = 111; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t110) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008cdfba); };
            c.cmp(t110, c.r32(1), 32);
            if c.cc_e() { blk = 194; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t111) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cdfc7); };
            c.cmp(t111, 0x41u64, 16);
            if c.cc_e() { blk = 192; continue; }
            blk = 108; continue;
        }
        108 => {
            blk = 110; continue;
        }
        109 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 111; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cdfd9); };
            c.w64(7, t112);
            let r113 = c.r64(7) | 0x10u64;
            c.w64(7, r113);
            c.logic(r113, 64);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cdfe1); };
            c.w64(0, t114);
            let r115 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r115);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008cdfe8); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x70008cdfec); }
            blk = 111; continue;
        }
        111 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0xa78)) else { return RegfileExit::fault(0x70008cdff0); };
            c.w64(3, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008cdff7); };
            c.w64(0, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cdffb); };
            c.w64(6, t118);
            return c.call(0x7000870914, 0x70008ce004, 0x70008cdfff);
        }
        112 => {
            let Some(t119) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ce004); };
            c.cmp(t119, c.r32(0), 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 119; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t120) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008ce013); };
            c.cmp(t120, c.r32(0), 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 115; continue;
        }
        115 => {
            let Some(t121) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ce01c); };
            c.w32(1, t121);
            let r122 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r122);
            let Some(t123) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008ce022); };
            c.logic(t123 & 0x10u64, 8);
            if c.cc_ne() { blk = 121; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t124) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ce028); };
            c.cmp(t124, c.r32(1), 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t125) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ce035); };
            c.cmp(t125, 0x41u64, 16);
            if c.cc_e() { blk = 196; continue; }
            blk = 118; continue;
        }
        118 => {
            blk = 120; continue;
        }
        119 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ce047); };
            c.w64(7, t126);
            let r127 = c.r64(7) | 0x8u64;
            c.w64(7, r127);
            c.logic(r127, 64);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ce04f); };
            c.w64(0, t128);
            let r129 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r129);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008ce056); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x70008ce05a); }
            blk = 121; continue;
        }
        121 => {
            let Some(t130) = c.ld64(c.r64(13).wrapping_add(0x790)) else { return RegfileExit::fault(0x70008ce05e); };
            c.w64(3, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ce065); };
            c.w64(0, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ce069); };
            c.w64(6, t132);
            return c.call(0x7000870914, 0x70008ce072, 0x70008ce06d);
        }
        122 => {
            let Some(t133) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ce072); };
            c.cmp(t133, c.r32(0), 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 129; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t134) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008ce081); };
            c.cmp(t134, c.r32(0), 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t135) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ce08a); };
            c.w32(1, t135);
            let r136 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r136);
            let Some(t137) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008ce090); };
            c.logic(t137 & 0x10u64, 8);
            if c.cc_ne() { blk = 131; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t138) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ce096); };
            c.cmp(t138, c.r32(1), 32);
            if c.cc_e() { blk = 202; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t139) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ce0a3); };
            c.cmp(t139, 0x41u64, 16);
            if c.cc_e() { blk = 200; continue; }
            blk = 128; continue;
        }
        128 => {
            blk = 130; continue;
        }
        129 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ce0b5); };
            c.w64(7, t140);
            let r141 = c.r64(7) | 0x80u64;
            c.w64(7, r141);
            c.logic(r141, 64);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ce0c0); };
            c.w64(0, t142);
            let r143 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r143);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008ce0c7); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x70008ce0cb); }
            blk = 131; continue;
        }
        131 => {
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0x11d8)) else { return RegfileExit::fault(0x70008ce0cf); };
            c.w64(1, t144);
            let Some(t145) = c.ld8(c.r64(1)) else { return RegfileExit::fault(0x70008ce0d6); };
            c.logic(t145 & 0xffffffffffffffffu64, 8);
            if c.cc_e() { blk = 146; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x830)) else { return RegfileExit::fault(0x70008ce0df); };
            c.w64(3, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ce0e6); };
            c.w64(0, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ce0ea); };
            c.w64(6, t148);
            return c.call(0x7000870914, 0x70008ce0f3, 0x70008ce0ee);
        }
        133 => {
            let Some(t149) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ce0f3); };
            c.cmp(t149, c.r32(0), 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 134; continue;
        }
        134 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 142; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t150) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008ce106); };
            c.cmp(t150, c.r32(0), 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t151) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ce10f); };
            c.w32(1, t151);
            let r152 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r152);
            let Some(t153) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008ce115); };
            c.logic(t153 & 0x10u64, 8);
            if c.cc_ne() { blk = 140; continue; }
            blk = 137; continue;
        }
        137 => {
            let Some(t154) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ce11b); };
            c.cmp(t154, c.r32(1), 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t155) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ce128); };
            c.cmp(t155, 0x41u64, 16);
            if c.cc_e() { blk = 204; continue; }
            blk = 139; continue;
        }
        139 => {
            blk = 143; continue;
        }
        140 => {
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ce135); };
            c.w64(0, t156);
            blk = 147; continue;
        }
        141 => {
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ce13b); };
            c.w64(0, t157);
            blk = 147; continue;
        }
        142 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ce146); };
            c.w64(7, t158);
            let r159 = c.r64(7) | 0x40u64;
            c.w64(7, r159);
            c.logic(r159, 64);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ce14e); };
            c.w64(0, t160);
            let r161 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r161);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008ce155); }
            blk = 147; continue;
        }
        144 => {
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ce15b); };
            c.w64(0, t162);
            blk = 147; continue;
        }
        145 => {
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ce161); };
            c.w64(0, t163);
            blk = 147; continue;
        }
        146 => {
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ce167); };
            c.w64(0, t164);
            blk = 147; continue;
        }
        147 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 154; continue; }
            blk = 148; continue;
        }
        148 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w32(7, 0xcu64);
            let r165 = c.add(c.r64(1), c.r64(7), 64);
            c.w64(1, r165);
            if c.cc_o() { blk = 153; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t166) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ce17d); };
            c.w64(7, t166);
            let Some(t167) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ce184); };
            c.w64(7, t167);
            let r168 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r168);
            let r169 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r169);
            c.logic(r169, 64);
            c.w64(8, c.r64(7).wrapping_add(c.r64(1)));
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ce193); };
            c.w64(9, t170);
            let Some(t171) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ce19a); };
            c.cmp(t171, c.r64(8), 64);
            if c.cc_be() { blk = 208; continue; }
            blk = 150; continue;
        }
        150 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 208; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ce1b0); };
            c.w64(1, t172);
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008ce1b7); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 152; continue;
        }
        152 => {
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008ce1be); };
            c.w64(7, t173);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008ce1c5); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008ce1c8); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008ce1cf); }
            c.w64(0, c.r64(1));
            blk = 155; continue;
        }
        153 => {
            return RegfileExit::fault(0x70008ce1d7);
        }
        154 => {
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008ce1d8); };
            c.w64(0, t174);
            blk = 155; continue;
        }
        155 => {
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ce1df); };
            c.w64(7, t175);
            c.logic(c.r8(7) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 157; continue; }
            blk = 156; continue;
        }
        156 => {
            c.w32(2, 0xbu64);
            blk = 158; continue;
        }
        157 => {
            if !c.st8(c.r64(0).wrapping_add(0xb), 0x64u64) { return RegfileExit::fault(0x70008ce1f0); }
            c.w32(2, 0xcu64);
            blk = 158; continue;
        }
        158 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), 0x67u64) { return RegfileExit::fault(0x70008ce1ff); }
            let r176 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r176);
            blk = 160; continue;
        }
        160 => {
            c.logic(c.r8(7) & 0x2u64, 8);
            if c.cc_e() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), 0x69u64) { return RegfileExit::fault(0x70008ce20d); }
            let r177 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r177);
            blk = 162; continue;
        }
        162 => {
            c.logic(c.r8(7) & 0x40u64, 8);
            if c.cc_e() { blk = 164; continue; }
            blk = 163; continue;
        }
        163 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), 0x6cu64) { return RegfileExit::fault(0x70008ce21b); }
            let r178 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r178);
            blk = 164; continue;
        }
        164 => {
            c.logic(c.r8(7) & 0x4u64, 8);
            if c.cc_e() { blk = 166; continue; }
            blk = 165; continue;
        }
        165 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), 0x6du64) { return RegfileExit::fault(0x70008ce229); }
            let r179 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r179);
            blk = 166; continue;
        }
        166 => {
            c.logic(c.r8(7) & 0x20u64, 8);
            if c.cc_e() { blk = 168; continue; }
            blk = 167; continue;
        }
        167 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), 0x73u64) { return RegfileExit::fault(0x70008ce237); }
            let r180 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r180);
            blk = 168; continue;
        }
        168 => {
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_e() { blk = 170; continue; }
            blk = 169; continue;
        }
        169 => {
            c.w64(1, c.r64(2).wrapping_add(0x1));
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), 0x75u64) { return RegfileExit::fault(0x70008ce249); }
            c.w64(2, c.r64(1));
            blk = 170; continue;
        }
        170 => {
            c.logic(c.r8(7) & 0x8u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 171; continue;
        }
        171 => {
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), 0x79u64) { return RegfileExit::fault(0x70008ce25a); }
            blk = 54; continue;
        }
        172 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ce265); };
            c.w64(3, t181);
            return c.call(0x700086ed74, 0x70008ce271, 0x70008ce26c);
        }
        173 => {
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ce271); };
            c.w64(6, t182);
            blk = 1; continue;
        }
        174 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008ce27a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008ce27e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008ce282); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008ce286); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008ce28a); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x70008ce28e); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x70008ce292); }
            return c.call(0x70007febb4, 0x70008ce29b, 0x70008ce296);
        }
        175 => {
            c.w64(1, c.r64(0));
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ce29e); };
            c.w64(15, t183);
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ce2a2); };
            c.w64(12, t184);
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ce2a6); };
            c.w64(11, t185);
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ce2aa); };
            c.w64(9, t186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ce2ae); };
            c.w64(8, t187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008ce2b2); };
            c.w64(7, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ce2b6); };
            c.w64(0, t189);
            blk = 34; continue;
        }
        176 => {
            let Some(t190) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ce2bf); };
            c.logic(t190 & 0x7ffffffeu64, 32);
            if c.cc_ne() { blk = 68; continue; }
            blk = 177; continue;
        }
        177 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 71; continue;
        }
        178 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ce2d5); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 68; continue; }
            blk = 179; continue;
        }
        179 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 71; continue;
        }
        180 => {
            let Some(t191) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ce2f1); };
            c.logic(t191 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 181; continue;
        }
        181 => {
            blk = 80; continue;
        }
        182 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ce303); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 80; continue; }
            blk = 183; continue;
        }
        183 => {
            blk = 81; continue;
        }
        184 => {
            let Some(t192) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ce31b); };
            c.logic(t192 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 185; continue;
        }
        185 => {
            blk = 90; continue;
        }
        186 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ce32d); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 90; continue; }
            blk = 187; continue;
        }
        187 => {
            blk = 91; continue;
        }
        188 => {
            let Some(t193) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ce345); };
            c.logic(t193 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 101; continue; }
            blk = 189; continue;
        }
        189 => {
            blk = 100; continue;
        }
        190 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ce357); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 100; continue; }
            blk = 191; continue;
        }
        191 => {
            blk = 101; continue;
        }
        192 => {
            let Some(t194) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ce36f); };
            c.logic(t194 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 111; continue; }
            blk = 193; continue;
        }
        193 => {
            blk = 110; continue;
        }
        194 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ce381); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 110; continue; }
            blk = 195; continue;
        }
        195 => {
            blk = 111; continue;
        }
        196 => {
            let Some(t195) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ce399); };
            c.logic(t195 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 197; continue;
        }
        197 => {
            blk = 120; continue;
        }
        198 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ce3ab); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 120; continue; }
            blk = 199; continue;
        }
        199 => {
            blk = 121; continue;
        }
        200 => {
            let Some(t196) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ce3c3); };
            c.logic(t196 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 201; continue;
        }
        201 => {
            blk = 130; continue;
        }
        202 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ce3d5); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 130; continue; }
            blk = 203; continue;
        }
        203 => {
            blk = 131; continue;
        }
        204 => {
            let Some(t197) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ce3ed); };
            c.logic(t197 & 0x7ffffffeu64, 32);
            if c.cc_ne() { blk = 143; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ce3fa); };
            c.w64(0, t198);
            blk = 147; continue;
        }
        206 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ce403); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 143; continue; }
            blk = 207; continue;
        }
        207 => {
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ce416); };
            c.w64(0, t199);
            blk = 147; continue;
        }
        208 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x70008ce41f); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70008ce42b, 0x70008ce426);
        }
        209 => {
            c.w64(1, c.r64(0));
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ce42e); };
            c.w64(0, t200);
            blk = 152; continue;
        }
        210 => {
            c.w32(1, 0x144u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008ce43c); }
            let Some(t201) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008ce43d); };
            c.w64(1, t201);
            let Some(t202) = c.ld32(c.r64(1).wrapping_add(0x2cb)) else { return RegfileExit::fault(0x70008ce444); };
            c.w32(1, t202);
            let r203 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r203);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008ce44d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ce44e); }
            let Some(t204) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008ce44f); };
            c.w64(3, t204);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008ce460, 0x70008ce45b);
        }
        211 => {
            return RegfileExit::fault(0x70008ce460);
        }
        212 => {
            return RegfileExit::jump(0x70008ce464);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ce474(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ce474); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ce478); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ce479); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ce47a); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ce47f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 63; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ce489); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 70; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ce495); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ce49b); };
            c.cmp(t5, 0x40u64, 16);
            if c.cc_ae() { blk = 65; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ce4a6); };
            c.w32(1, t6);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008ce4a9); }
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ce4ad); };
            c.w64(7, t7);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008ce4b1); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(0, c.r64(7));
            return c.call(0x7000899a14, 0x70008ce4c3, 0x70008ce4be);
        }
        5 => {
            c.w64(7, c.r64(0));
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ce4c6); };
            c.w64(0, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ce4ca); };
            c.w64(1, t9);
            blk = 6; continue;
        }
        6 => {
            c.w32(8, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ce4e3); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 14; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 10; continue;
        }
        9 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ge() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(11, c.r64(1).wrapping_add(c.r64(1)));
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(15, c.r32(11));
            let r10 = c.add(c.r32(8), c.r32(15), 32);
            c.w32(8, r10);
            if c.cc_o() { blk = 72; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ce53e); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 62; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 18; continue;
        }
        17 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 62; continue; }
            blk = 18; continue;
        }
        18 => {
            let r11 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r11);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 19; continue;
        }
        19 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ce563); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ce56e); }
            blk = 24; continue;
        }
        21 => {
            c.w64(1, c.r64(7));
            let r12 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r12);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            blk = 24; continue;
        }
        22 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            let r13 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r13);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ce58c); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 24; continue;
        }
        24 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 28; continue; }
            blk = 25; continue;
        }
        25 => {
            blk = 27; continue;
        }
        26 => {
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_le() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 29; continue;
        }
        28 => {
            c.w32(9, 0x1u64);
            blk = 29; continue;
        }
        29 => {
            c.w32(1, 0x1u64);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 62; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            blk = 35; continue;
        }
        32 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ce5cd); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(7, cv);
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_s() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(10, 0x8000000000000000u64);
            let r14 = c.r64(7) | c.r64(10);
            c.w64(7, r14);
            c.logic(r14, 64);
            blk = 35; continue;
        }
        35 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ce607); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ce60e); };
            c.w32(8, t17);
            c.logic(c.r8(8) & 0x7u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(0));
            c.w64(11, c.r64(8));
            c.w64(8, c.r64(6));
            blk = 37; continue;
        }
        37 => {
            c.w64(12, c.r64(11));
            let r18 = c.r32(12) & 0x7u64;
            c.w32(12, r18);
            c.logic(r18, 32);
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_l() { blk = 42; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 41; continue;
        }
        41 => {
            blk = 89; continue;
        }
        42 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            blk = 89; continue;
        }
        45 => {
            let Some(t19) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ce661); };
            c.w32(11, t19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008ce665); };
            c.cmp(t20, c.r32(11), 32);
            if c.cc_ne() { blk = 90; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t21) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ce672); };
            c.w32(9, t21);
            let r22 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r22);
            let Some(t23) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ce679); };
            c.w32(11, t23);
            let r24 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r24);
            let Some(t25) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ce680); };
            c.w32(11, t25);
            blk = 37; continue;
        }
        47 => {
            let Some(t26) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ce687); };
            c.w32(11, t26);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r27 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ce694); };
            c.w32(9, t28);
            let r29 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r29);
            let Some(t30) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ce69b); };
            c.w32(11, t30);
            let r31 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r31);
            let Some(t32) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ce6a2); };
            c.w32(11, t32);
            blk = 37; continue;
        }
        48 => {
            let Some(t33) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ce6ac); };
            c.w32(9, t33);
            let r34 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r34);
            let Some(t35) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ce6b3); };
            c.w32(11, t35);
            let r36 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r36);
            let Some(t37) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ce6ba); };
            c.w32(11, t37);
            blk = 37; continue;
        }
        49 => {
            c.w64(1, c.r64(8));
            c.w64(8, c.r64(11));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 52; continue;
        }
        50 => {
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(8));
            c.w64(8, c.r64(10));
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(11));
            c.w64(11, c.r64(10));
            blk = 52; continue;
        }
        51 => {
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(0));
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 53; continue;
        }
        53 => {
            let r38 = c.add(c.r64(9), 0xbu64, 64);
            c.w64(9, r38);
            blk = 54; continue;
        }
        54 => {
            let r39 = c.add(c.r64(1), c.r64(7), 64);
            c.w64(1, r39);
            let r40 = c.r32(8) & 0x8u64;
            c.w32(8, r40);
            c.logic(r40, 32);
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t41) = c.ld16(c.r64(9).wrapping_add(c.r64(1).wrapping_mul(2))) else { return RegfileExit::fault(0x70008ce709); };
            c.w32(1, t41);
            blk = 57; continue;
        }
        56 => {
            let Some(t42) = c.ld8(c.r64(1).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70008ce710); };
            c.w32(1, t42);
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(1), 0xffu64, 32);
            if c.cc_g() { blk = 106; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(7, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008ce729); };
            c.w64(8, t43);
            let Some(t44) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ce730); };
            c.w32(0, t44);
            let r45 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r45);
            let Some(t46) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ce738); };
            c.cmp(t46, c.r32(0), 32);
            if c.cc_e() { blk = 111; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ce745); };
            c.w64(1, t47);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ce74c); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            return c.ret(0x10, 0x70008ce753);
        }
        61 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ce756); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ce75d); }
            return c.ret(0x0, 0x70008ce75f);
        }
        62 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ce760); };
            c.w64(0, t48);
            blk = 59; continue;
        }
        63 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008ce769); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ce76f); };
            c.w64(3, t49);
            return c.call(0x700086ed74, 0x70008ce77b, 0x70008ce776);
        }
        64 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ce77b); };
            c.w64(6, t50);
            blk = 1; continue;
        }
        65 => {
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008ce784); };
            c.cmp(t51, c.r32(0), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t52) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ce791); };
            c.cmp(t52, c.r32(0), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008ce7a0); }
            let r53 = c.r32(1) | c.r32(7);
            c.w32(1, r53);
            c.logic(r53, 32);
            if c.cc_ne() { blk = 68; continue; }
            blk = 66; continue;
        }
        66 => {
            return c.call(0x700089c794, 0x70008ce7ad, 0x70008ce7a8);
        }
        67 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ce7ad); };
            c.w64(6, t54);
            blk = 3; continue;
        }
        68 => {
            c.w32(1, 0x36u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008ce7bb); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008ce7bc); };
            c.w64(1, t55);
            let Some(t56) = c.ld32(c.r64(1).wrapping_add(0x2cf)) else { return RegfileExit::fault(0x70008ce7c3); };
            c.w32(1, t56);
            let r57 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r57);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008ce7cc); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008ce7cd); };
            c.w64(3, t58);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008ce7de, 0x70008ce7d9);
        }
        69 => {
            return RegfileExit::fault(0x70008ce7de);
        }
        70 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008ce7df); }
            return c.call(0x70008a1214, 0x70008ce7e8, 0x70008ce7e3);
        }
        71 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ce7e8); };
            c.w64(6, t59);
            blk = 3; continue;
        }
        72 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ce7f7); }
            blk = 75; continue;
        }
        74 => {
            let r60 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r60);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 75; continue;
        }
        75 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ce80e); }
            blk = 78; continue;
        }
        77 => {
            let r61 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r61);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 78; continue;
        }
        78 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x70008ce832); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 84; continue; }
            blk = 79; continue;
        }
        79 => {
            if c.cc_ne() { blk = 84; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            c.w64(8, c.r64(7));
            let r62 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r62);
            if c.cc_o() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 14; continue;
        }
        84 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ce863); };
            c.w64(7, t63);
            let Some(t64) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ce86a); };
            c.w64(8, t64);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ce871); };
            c.w64(11, t65);
            let Some(t66) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008ce878); };
            c.cmp(t66, c.r64(9), 64);
            if c.cc_a() { blk = 87; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ce887, 0x70008ce882);
        }
        86 => {
            blk = 88; continue;
        }
        87 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008ce889); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 88; continue;
        }
        88 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ce893); };
            c.w64(1, t67);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ce89a); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x70008ce89d); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ce8a2); }
            c.w64(7, c.r64(0));
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ce8aa); };
            c.w64(0, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ce8ae); };
            c.w64(1, t69);
            blk = 14; continue;
        }
        89 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008ce8b7); }
            blk = 93; continue;
        }
        90 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008ce8bd); }
            blk = 93; continue;
        }
        91 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008ce8c3); }
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t70) = c.ld64(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ce8cd); };
            c.w64(9, t70);
            blk = 54; continue;
        }
        93 => {
            c.cmp(c.r64(7), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w32(1, c.r64(7).wrapping_add(c.r64(7)));
            blk = 104; continue;
        }
        95 => {
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ce8e4); };
            c.w64(1, t71);
            let Some(t72) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ce8eb); };
            c.w64(8, t72);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ce8f2); };
            c.w64(11, t73);
            let Some(t74) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008ce8f9); };
            c.cmp(t74, c.r64(9), 64);
            if c.cc_a() { blk = 98; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ce908, 0x70008ce903);
        }
        97 => {
            c.w64(1, c.r64(0));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ce90b); };
            c.w64(0, t75);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ce911); };
            c.w64(7, t76);
            blk = 99; continue;
        }
        98 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008ce917); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 99; continue;
        }
        99 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ce91e); };
            c.w64(8, t77);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008ce925); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(7) as i64));
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 103; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w64(10, c.r64(7));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 102; continue; }
            blk = 101; continue;
        }
        101 => {
            let r78 = c.r64(10) | 0x1u64;
            c.w64(10, r78);
            c.logic(r78, 64);
            blk = 102; continue;
        }
        102 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 103; continue;
        }
        103 => {
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ce950); }
            blk = 104; continue;
        }
        104 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ce955); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008ce956); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008ce957); };
            c.w64(3, t79);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008ce968, 0x70008ce963);
        }
        105 => {
            let r80 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r80);
            c.w64(1, c.r64(0));
            blk = 57; continue;
        }
        106 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ce972); };
            c.w64(7, t81);
            let Some(t82) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ce979); };
            c.w64(8, t82);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008ce980); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ce984); };
            c.w64(11, t83);
            let Some(t84) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008ce98b); };
            c.cmp(t84, c.r64(9), 64);
            if c.cc_a() { blk = 109; continue; }
            blk = 107; continue;
        }
        107 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008ce99a, 0x70008ce995);
        }
        108 => {
            blk = 110; continue;
        }
        109 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008ce99c); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 110; continue;
        }
        110 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008ce9a6); };
            c.w64(1, t85);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ce9ad); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008ce9b0); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008ce9b7); }
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ce9be); };
            c.w64(1, t86);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(1)) { return RegfileExit::fault(0x70008ce9c2); }
            blk = 59; continue;
        }
        111 => {
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ce9cb); };
            c.w64(9, t87);
            let Some(t88) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ce9d2); };
            c.w64(11, t88);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008ce9d9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008ce9dd); }
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ce9e1); };
            c.w64(15, t89);
            let Some(t90) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008ce9e8); };
            c.cmp(t90, c.r64(12), 64);
            if c.cc_a() { blk = 114; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008ce9f7, 0x70008ce9f2);
        }
        113 => {
            blk = 115; continue;
        }
        114 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008ce9f9); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 115; continue;
        }
        115 => {
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008cea03); };
            c.w64(1, t91);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008cea0a); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008cea0d); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008cea14); }
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cea1b); };
            c.w64(1, t92);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(1)) { return RegfileExit::fault(0x70008cea1f); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008cea22); };
            c.w64(8, t93);
            let Some(t94) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cea29); };
            c.w32(7, t94);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            return RegfileExit::fault(0x70008cea38);
        }
        117 => {
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cea39); };
            c.w64(7, t95);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x70008cea3d); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r96 = c.r64(1) & c.r64(8);
            c.w64(1, r96);
            c.logic(r96, 64);
            let Some(t97) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cea4f); };
            c.logic(t97 & 0x4u64, 8);
            if c.cc_ne() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            blk = 59; continue;
        }
        120 => {
            c.w32(0, c.r32(0));
            let r98 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r98);
            c.w64(1, 0xfffffffffffc0000u64);
            let r99 = c.r64(1) & c.r64(0);
            c.w64(1, r99);
            c.logic(r99, 64);
            let Some(t100) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008cea69); };
            c.logic(t100 & 0x2u64, 8);
            if c.cc_e() { blk = 119; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cea73); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008cea74); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008cea81, 0x70008cea7c);
        }
        122 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008cea81); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008cea83); }
            blk = 119; continue;
        }
        123 => {
            return RegfileExit::jump(0x70008cea88);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cea94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cea94); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cea98); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cea99); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cea9a); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008cea9f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ceaa3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 234; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(7, c.r64(2).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ceabb); };
            c.w64(8, t2);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008ceac3); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ceacd); };
            c.w64(2, t3);
            c.w64(11, c.r64(2));
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cead6); };
            c.w64(11, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ceadd); };
            c.w64(2, t5);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(11)) { return RegfileExit::fault(0x70008ceae4); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ceaee); };
            c.w64(2, t6);
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008ceaf2); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 241; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ceb00); };
            c.w32(7, t7);
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            let Some(t9) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ceb07); };
            c.cmp(t9, 0x40u64, 16);
            if c.cc_ae() { blk = 236; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x70008ceb1e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008ceb22); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ceb2f); };
            c.w32(12, t10);
            let r11 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ceb36); };
            c.w32(12, t12);
            let r13 = c.sub(c.r32(12), 0xbau64, 32);
            c.w32(12, r13);
            c.cmp(c.r32(12), 0x78eu64, 32);
            if c.cc_a() { blk = 35; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xcd0)) else { return RegfileExit::fault(0x70008ceb50); };
            c.w64(3, t14);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x70008ceb5f, 0x70008ceb5a);
        }
        11 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ceb5f); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ceb6c); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008ceb7f); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ceb8c); };
            c.w32(2, t18);
            let r19 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r19);
            let Some(t20) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008ceb92); };
            c.logic(t20 & 0x10u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ceb98); };
            c.cmp(t21, c.r32(2), 32);
            if c.cc_e() { blk = 245; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t22) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ceba5); };
            c.cmp(t22, 0x41u64, 16);
            if c.cc_e() { blk = 243; continue; }
            blk = 18; continue;
        }
        18 => {
            blk = 20; continue;
        }
        19 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cebb7); };
            c.w64(2, t23);
            let Some(t24) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cebbb); };
            c.w32(2, t24);
            let r25 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r25);
            let Some(t26) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cebc1); };
            c.cmp(t26, 0x83eu64, 16);
            if c.cc_ne() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0x1u64);
            blk = 23; continue;
        }
        22 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 23; continue;
        }
        23 => {
            c.w32(0, 0x1u64);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(2, 0x90u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cebe5); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x70008cebe6); };
            c.w64(3, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cebed); };
            c.w64(6, t28);
            return c.call(0x700086ed74, 0x70008cebf6, 0x70008cebf1);
        }
        25 => {
            blk = 33; continue;
        }
        26 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cebf8); };
            c.w64(2, t29);
            let Some(t30) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cebfc); };
            c.w32(7, t30);
            let r31 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r31);
            let Some(t32) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cec02); };
            c.cmp(t32, 0x83eu64, 16);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(7, 0x92u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cec0f); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x70008cec10); };
            c.w64(3, t33);
            c.w32(0, 0x1u64);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cec1c); };
            c.w64(6, t34);
            return c.call(0x700086ed74, 0x70008cec25, 0x70008cec20);
        }
        28 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cec25); };
            c.w64(0, t35);
            blk = 36; continue;
        }
        29 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cec2b); };
            c.w64(0, t36);
            let Some(t37) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cec2f); };
            c.w32(2, t37);
            let r38 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r38);
            let Some(t39) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cec35); };
            c.cmp(t39, 0x83eu64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(2, 0x1u64);
            blk = 32; continue;
        }
        31 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(2, 0x6cu64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cec50); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cec51); };
            c.w64(2, t40);
            let Some(t41) = c.ld32(c.r64(2).wrapping_add(0x2d3)) else { return RegfileExit::fault(0x70008cec58); };
            c.w32(2, t41);
            let r42 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r42);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cec61); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008cec62); };
            c.w64(3, t43);
            c.w32(0, 0x2u64);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cec6e); };
            c.w64(6, t44);
            return c.call(0x700086ed74, 0x70008cec77, 0x70008cec72);
        }
        34 => {
            return RegfileExit::fault(0x70008cec77);
        }
        35 => {
            c.w64(0, c.r64(11));
            blk = 36; continue;
        }
        36 => {
            let Some(t45) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cec7b); };
            c.cmp(t45, 0x0u64, 32);
            if c.cc_ne() { blk = 247; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t46) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cec85); };
            c.w32(2, t46);
            let r47 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r47);
            let Some(t48) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cec8b); };
            c.cmp(t48, 0x40u64, 16);
            if c.cc_ae() { blk = 247; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008cec96); };
            c.w64(8, t49);
            let Some(t50) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cec9a); };
            c.w32(2, t50);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(0)) { return RegfileExit::fault(0x70008cec9e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008ceca2); }
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ceca6); };
            c.w64(7, t51);
            let Some(t52) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cecaa); };
            c.cmp(t52, c.r32(7), 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(9, c.r64(2));
            blk = 61; continue;
        }
        40 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(0, c.r64(7));
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cecc4); };
            c.w64(6, t53);
            return c.call(0x7000899a14, 0x70008ceccd, 0x70008cecc8);
        }
        42 => {
            c.w64(7, c.r64(0));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cecd0); };
            c.w64(0, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008cecd4); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cecd8); };
            c.w64(2, t56);
            blk = 43; continue;
        }
        43 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 48; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_be() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(9, c.r64(7));
            blk = 61; continue;
        }
        47 => {
            c.w64(9, c.r64(2));
            blk = 61; continue;
        }
        48 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 61; continue;
        }
        49 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ced0a); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 60; continue; }
            blk = 50; continue;
        }
        50 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(2) as i64));
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(10, c.r64(2));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            let r57 = c.r64(10) | 0x1u64;
            c.w64(10, r57);
            c.logic(r57, 64);
            blk = 53; continue;
        }
        53 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(10) as i64));
            c.arith_sd(0, 1, c.xmm_u64(1));
            blk = 54; continue;
        }
        54 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 59; continue; }
            blk = 55; continue;
        }
        55 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 58; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_s() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(10, 0x8000000000000000u64);
            let r58 = c.r64(2) | c.r64(10);
            c.w64(2, r58);
            c.logic(r58, 64);
            blk = 58; continue;
        }
        58 => {
            c.w64(9, c.r64(2));
            blk = 61; continue;
        }
        59 => {
            c.w64(9, c.r64(2));
            blk = 61; continue;
        }
        60 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 61; continue;
        }
        61 => {
            c.w64(2, c.r64(0));
            let Some(t59) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ced8b); };
            c.w32(7, t59);
            let r60 = c.sub(c.r64(9), c.r64(7), 64);
            c.w64(9, r60);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008ced91); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_l() { blk = 233; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t61) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ced9f); };
            c.w32(7, t61);
            c.w64(11, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008ceda6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008cedaa); }
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 64; continue;
        }
        63 => {
            c.w64(8, c.r64(0));
            blk = 64; continue;
        }
        64 => {
            let Some(t62) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cedb6); };
            c.w32(15, t62);
            let r63 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r63);
            let Some(t64) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cedbd); };
            c.w32(15, t64);
            c.w64(0, c.r64(15));
            let r65 = c.r32(0) & 0xfu64;
            c.w32(0, r65);
            c.logic(r65, 32);
            c.cmp(c.r32(0), 0x8u64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(3, c.r64(15));
            let r66 = c.r32(3) & 0x7u64;
            c.w32(3, r66);
            c.logic(r66, 32);
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_ne() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cede5); };
            c.w32(8, t67);
            let r68 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r68);
            c.w64(0, c.r64(8));
            blk = 63; continue;
        }
        68 => {
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t69) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cedf6); };
            c.w32(15, t69);
            let r70 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r70);
            let Some(t71) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cedfd); };
            c.cmp(t71, 0x0u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 70; continue;
        }
        70 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008cee04); }
            c.w64(0, c.r64(8));
            return c.call(0x70008e88b4, 0x70008cee10, 0x70008cee0b);
        }
        71 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cee10); };
            c.w64(12, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cee14); };
            c.w64(2, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cee18); };
            c.w64(9, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cee1c); };
            c.w64(7, t75);
            blk = 63; continue;
        }
        72 => {
            let Some(t76) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cee22); };
            c.w32(8, t76);
            let r77 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r77);
            c.w64(0, c.r64(8));
            blk = 63; continue;
        }
        73 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t78) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cee33); };
            c.w32(11, t78);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r79 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r79);
            let Some(t80) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cee40); };
            c.w32(8, t80);
            let r81 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r81);
            c.w64(0, c.r64(8));
            blk = 63; continue;
        }
        75 => {
            let Some(t82) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cee4f); };
            c.w32(11, t82);
            c.cmp(c.r32(11), c.r32(7), 32);
            if c.cc_b() { blk = 78; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cee5b); };
            let r84 = c.sub(c.r64(11), t83, 64);
            c.w64(11, r84);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_b() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(11, c.r64(12).wrapping_add(0xc));
            blk = 91; continue;
        }
        78 => {
            return RegfileExit::fault(0x70008cee6e);
        }
        79 => {
            let Some(t85) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cee6f); };
            c.w32(11, t85);
            c.cmp(c.r32(11), c.r32(7), 32);
            if c.cc_b() { blk = 82; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cee7b); };
            let r87 = c.sub(c.r64(11), t86, 64);
            c.w64(11, r87);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_b() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w64(11, c.r64(12).wrapping_mul(2).wrapping_add(0xc));
            blk = 165; continue;
        }
        82 => {
            return RegfileExit::fault(0x70008cee91);
        }
        83 => {
            c.cmp(c.r32(0), 0xau64, 32);
            if c.cc_ne() { blk = 157; continue; }
            blk = 84; continue;
        }
        84 => {
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t88) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ceea1); };
            c.w64(11, t88);
            c.w64(0, c.r64(11));
            blk = 88; continue;
        }
        86 => {
            c.w64(10, c.r64(4));
            let r89 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r89);
            let r90 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r90);
            c.logic(r90, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008ceeb5); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x70008ceeba); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008ceebe); }
            c.w64(1, c.r64(8));
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008ceec5); };
            c.w64(0, t91);
            c.w64(10, 0x70008ceed3u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008ceed3); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008ceed7); }
            return c.call(c.r64(0), 0x70008ceedd, 0x70008ceedb);
        }
        87 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008ceedd); }
            let Some(t92) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ceee5); };
            c.w64(4, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ceeea); };
            c.w64(12, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008ceeee); };
            c.w64(2, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ceef2); };
            c.w64(9, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ceef6); };
            c.w64(7, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008ceefa); };
            c.w64(8, t97);
            blk = 88; continue;
        }
        88 => {
            c.w64(1, c.r64(8));
            let Some(t98) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cef01); };
            c.w32(8, t98);
            c.cmp(c.r32(8), c.r32(7), 32);
            if c.cc_b() { blk = 156; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cef11); };
            let r100 = c.sub(c.r64(8), t99, 64);
            c.w64(8, r100);
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_b() { blk = 156; continue; }
            blk = 90; continue;
        }
        90 => {
            c.w64(11, c.r64(12).wrapping_add(c.r64(0)).wrapping_add(0x1));
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 91; continue;
        }
        91 => {
            let Some(t101) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cef26); };
            c.w32(12, t101);
            c.w64(15, (c.r32(12) as u32 as i32 as i64 as u64));
            let r102 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cef30); };
            c.w64(0, t103);
            let r104 = c.sub(c.r64(0), c.r64(15), 64);
            c.w64(0, r104);
            c.cmp(c.r32(7), c.r32(12), 32);
            c.w8(7, (c.cc_b() as u64));
            c.w32(7, c.r8(7));
            c.w64(3, c.r64(11).wrapping_add(c.r64(15)));
            c.cmp(c.r64(0), c.r64(9), 64);
            c.w8(9, (c.cc_b() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x70008cef51); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008cef55); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008cef59); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008cef5d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008cef61); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008cef65); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008cef69); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 92; continue;
        }
        92 => {
            let Some(t105) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cef6f); };
            c.w32(1, t105);
            let r106 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r106);
            let Some(t107) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cef75); };
            c.w32(1, t107);
            c.w64(6, c.r64(1));
            let r108 = c.r32(6) & 0xfu64;
            c.w32(6, r108);
            c.logic(r108, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x70008cef7f); }
            c.cmp(c.r32(6), 0x8u64, 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 93; continue;
        }
        93 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 94; continue;
        }
        94 => {
            let r109 = c.r32(1) & 0x7u64;
            c.w32(1, r109);
            c.logic(r109, 32);
            c.cmp(c.r32(1), 0x5u64, 32);
            if c.cc_ne() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t110) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cef99); };
            c.w32(2, t110);
            let r111 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r111);
            blk = 92; continue;
        }
        96 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_ne() { blk = 101; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t112) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cefa6); };
            c.w32(1, t112);
            let r113 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r113);
            let Some(t114) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cefac); };
            c.cmp(t114, 0x0u64, 32);
            if c.cc_e() { blk = 100; continue; }
            blk = 98; continue;
        }
        98 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008cefb2); }
            c.w64(0, c.r64(2));
            return c.call(0x70008e88b4, 0x70008cefbe, 0x70008cefb9);
        }
        99 => {
            c.w64(2, c.r64(0));
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008cefc1); };
            c.w64(8, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cefc5); };
            c.w64(3, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cefc9); };
            c.w64(15, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cefcd); };
            c.w64(0, t118);
            blk = 92; continue;
        }
        100 => {
            let Some(t119) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cefd3); };
            c.w32(2, t119);
            let r120 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r120);
            blk = 92; continue;
        }
        101 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t121) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cefe0); };
            c.w32(7, t121);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let r122 = c.add(c.r64(0), c.r64(7), 64);
            c.w64(0, r122);
            let Some(t123) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cefec); };
            c.w32(2, t123);
            let r124 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r124);
            blk = 92; continue;
        }
        103 => {
            let Some(t125) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ceff7); };
            c.w32(7, t125);
            let Some(t126) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008ceffa); };
            c.cmp(c.r32(7), t126, 32);
            if c.cc_b() { blk = 106; continue; }
            blk = 104; continue;
        }
        104 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r127 = c.sub(c.r64(7), c.r64(15), 64);
            c.w64(7, r127);
            c.cmp(c.r64(7), c.r64(0), 64);
            if c.cc_b() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            c.w64(7, c.r64(0).wrapping_add(0xc));
            blk = 119; continue;
        }
        106 => {
            return RegfileExit::fault(0x70008cf013);
        }
        107 => {
            let Some(t128) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cf014); };
            c.w32(7, t128);
            let Some(t129) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008cf017); };
            c.cmp(c.r32(7), t129, 32);
            if c.cc_b() { blk = 110; continue; }
            blk = 108; continue;
        }
        108 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r130 = c.sub(c.r64(7), c.r64(15), 64);
            c.w64(7, r130);
            c.cmp(c.r64(7), c.r64(0), 64);
            if c.cc_b() { blk = 110; continue; }
            blk = 109; continue;
        }
        109 => {
            c.w64(7, c.r64(0).wrapping_mul(2).wrapping_add(0xc));
            blk = 141; continue;
        }
        110 => {
            return RegfileExit::fault(0x70008cf034);
        }
        111 => {
            c.cmp(c.r32(6), 0xau64, 32);
            if c.cc_ne() { blk = 133; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t131) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cf03e); };
            c.logic(t131 & 0x10u64, 8);
            if c.cc_ne() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t132) = c.ld64(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cf044); };
            c.w64(7, t132);
            blk = 116; continue;
        }
        114 => {
            c.w64(10, c.r64(4));
            let r133 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r133);
            let r134 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r134);
            c.logic(r134, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008cf055); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(2)) { return RegfileExit::fault(0x70008cf05a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008cf05e); }
            c.w64(1, c.r64(2));
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008cf065); };
            c.w64(0, t135);
            c.w64(10, 0x70008cf073u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008cf073); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008cf077); }
            return c.call(c.r64(0), 0x70008cf07d, 0x70008cf07b);
        }
        115 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008cf07d); }
            let Some(t136) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008cf085); };
            c.w64(4, t136);
            c.w64(7, c.r64(0));
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008cf08d); };
            c.w64(8, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cf091); };
            c.w64(3, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cf095); };
            c.w64(15, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cf099); };
            c.w64(2, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cf09d); };
            c.w64(0, t141);
            blk = 116; continue;
        }
        116 => {
            c.w64(1, c.r64(2));
            let Some(t142) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cf0a4); };
            c.w32(2, t142);
            let Some(t143) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008cf0a7); };
            c.cmp(c.r32(2), t143, 32);
            if c.cc_b() { blk = 132; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r144 = c.sub(c.r64(2), c.r64(15), 64);
            c.w64(2, r144);
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_b() { blk = 132; continue; }
            blk = 118; continue;
        }
        118 => {
            c.w64(7, c.r64(0).wrapping_add(c.r64(7)).wrapping_add(0x1));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 119; continue;
        }
        119 => {
            let Some(t145) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cf0c2); };
            c.cmp(t145, 0x0u64, 32);
            if c.cc_ne() { blk = 129; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t146) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cf0c8); };
            c.cmp(t146, 0x0u64, 32);
            if c.cc_ne() { blk = 129; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w64(9, c.r64(15));
            let r147 = c.add(c.r64(9), c.r64(7), 64);
            c.w64(9, r147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cf0d4); };
            c.w64(11, t148);
            blk = 123; continue;
        }
        122 => {
            let r149 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r149);
            let r150 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r150);
            blk = 123; continue;
        }
        123 => {
            c.cmp(c.r64(9), c.r64(7), 64);
            if c.cc_e() { blk = 128; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t151) = c.ld8(c.r64(2).wrapping_add(c.r64(7)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cf0e7); };
            c.w32(12, t151);
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_e() { blk = 127; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t152) = c.ld8(c.r64(8).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cf0f2); };
            c.cmp(t152, c.r8(12), 8);
            if c.cc_e() { blk = 122; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 130; continue;
        }
        127 => {
            return RegfileExit::fault(0x70008cf0fd);
        }
        128 => {
            c.w32(2, 0x1u64);
            blk = 130; continue;
        }
        129 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 228; continue; }
            blk = 131; continue;
        }
        131 => {
            blk = 227; continue;
        }
        132 => {
            return RegfileExit::fault(0x70008cf115);
        }
        133 => {
            c.cmp(c.r32(6), 0x2u64, 32);
            if c.cc_ne() { blk = 155; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t153) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cf11f); };
            c.logic(t153 & 0x10u64, 8);
            if c.cc_ne() { blk = 136; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t154) = c.ld64(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cf125); };
            c.w64(7, t154);
            blk = 138; continue;
        }
        136 => {
            c.w64(10, c.r64(4));
            let r155 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r155);
            let r156 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r156);
            c.logic(r156, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008cf136); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(2)) { return RegfileExit::fault(0x70008cf13b); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008cf13f); }
            c.w64(1, c.r64(2));
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008cf146); };
            c.w64(0, t157);
            c.w64(10, 0x70008cf154u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008cf154); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008cf158); }
            return c.call(c.r64(0), 0x70008cf15e, 0x70008cf15c);
        }
        137 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008cf15e); }
            let Some(t158) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008cf166); };
            c.w64(4, t158);
            c.w64(7, c.r64(0));
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008cf16e); };
            c.w64(8, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cf172); };
            c.w64(3, t160);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cf176); };
            c.w64(15, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cf17a); };
            c.w64(2, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cf17e); };
            c.w64(0, t163);
            blk = 138; continue;
        }
        138 => {
            let Some(t164) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cf182); };
            c.w32(2, t164);
            let Some(t165) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008cf185); };
            c.cmp(c.r32(2), t165, 32);
            if c.cc_b() { blk = 154; continue; }
            blk = 139; continue;
        }
        139 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r166 = c.sub(c.r64(2), c.r64(15), 64);
            c.w64(2, r166);
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_b() { blk = 154; continue; }
            blk = 140; continue;
        }
        140 => {
            c.w64(7, c.r64(7).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x1));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 141; continue;
        }
        141 => {
            let Some(t167) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cf1a0); };
            c.cmp(t167, 0x0u64, 32);
            if c.cc_ne() { blk = 151; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t168) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cf1a6); };
            c.cmp(t168, 0x0u64, 32);
            if c.cc_ne() { blk = 151; continue; }
            blk = 143; continue;
        }
        143 => {
            c.w64(9, c.r64(15));
            c.w64(9, c.r64(7).wrapping_add(c.r64(9).wrapping_mul(2)));
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cf1b3); };
            c.w64(11, t169);
            c.w64(12, c.r64(8));
            c.w64(8, c.r64(3));
            blk = 145; continue;
        }
        144 => {
            let r170 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r170);
            let r171 = c.add(c.r64(7), 0x2u64, 64);
            c.w64(7, r171);
            blk = 145; continue;
        }
        145 => {
            c.cmp(c.r64(9), c.r64(7), 64);
            if c.cc_e() { blk = 150; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t172) = c.ld16(c.r64(2).wrapping_add(c.r64(7)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cf1cc); };
            c.w32(15, t172);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_e() { blk = 149; continue; }
            blk = 147; continue;
        }
        147 => {
            let Some(t173) = c.ld8(c.r64(12).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cf1d7); };
            c.w32(0, t173);
            c.cmp(c.r32(0), c.r32(15), 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 148; continue;
        }
        148 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 152; continue;
        }
        149 => {
            return RegfileExit::fault(0x70008cf1e6);
        }
        150 => {
            c.w32(2, 0x1u64);
            blk = 152; continue;
        }
        151 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 152; continue;
        }
        152 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 228; continue; }
            blk = 153; continue;
        }
        153 => {
            blk = 227; continue;
        }
        154 => {
            return RegfileExit::fault(0x70008cf1fe);
        }
        155 => {
            return RegfileExit::fault(0x70008cf1ff);
        }
        156 => {
            return RegfileExit::fault(0x70008cf200);
        }
        157 => {
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_ne() { blk = 232; continue; }
            blk = 158; continue;
        }
        158 => {
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t174) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cf210); };
            c.w64(11, t174);
            c.w64(0, c.r64(11));
            blk = 162; continue;
        }
        160 => {
            c.w64(10, c.r64(4));
            let r175 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r175);
            let r176 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r176);
            c.logic(r176, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008cf224); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x70008cf229); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008cf22d); }
            c.w64(1, c.r64(8));
            let Some(t177) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008cf234); };
            c.w64(0, t177);
            c.w64(10, 0x70008cf242u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008cf242); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008cf246); }
            return c.call(c.r64(0), 0x70008cf24c, 0x70008cf24a);
        }
        161 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008cf24c); }
            let Some(t178) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008cf254); };
            c.w64(4, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cf259); };
            c.w64(12, t179);
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cf25d); };
            c.w64(2, t180);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cf261); };
            c.w64(9, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cf265); };
            c.w64(7, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008cf269); };
            c.w64(8, t183);
            blk = 162; continue;
        }
        162 => {
            let Some(t184) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cf26d); };
            c.w32(8, t184);
            c.cmp(c.r32(8), c.r32(7), 32);
            if c.cc_b() { blk = 231; continue; }
            blk = 163; continue;
        }
        163 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cf27d); };
            let r186 = c.sub(c.r64(8), t185, 64);
            c.w64(8, r186);
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_b() { blk = 231; continue; }
            blk = 164; continue;
        }
        164 => {
            c.w64(11, c.r64(0).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x1));
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 165; continue;
        }
        165 => {
            let Some(t187) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cf292); };
            c.w32(12, t187);
            c.w64(15, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(11).wrapping_add(c.r64(9).wrapping_mul(2)));
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cf29d); };
            c.w64(0, t188);
            let r189 = c.sub(c.r64(0), c.r64(15), 64);
            c.w64(0, r189);
            c.cmp(c.r32(7), c.r32(12), 32);
            c.w8(7, (c.cc_b() as u64));
            c.w32(7, c.r8(7));
            c.w64(3, c.r64(11).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.cmp(c.r64(0), c.r64(9), 64);
            c.w8(9, (c.cc_b() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x70008cf2be); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008cf2c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008cf2c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008cf2ca); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008cf2ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008cf2d2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008cf2d6); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 166; continue;
        }
        166 => {
            let Some(t190) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cf2dc); };
            c.w32(1, t190);
            let r191 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r191);
            let Some(t192) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cf2e2); };
            c.w32(1, t192);
            c.w64(6, c.r64(1));
            let r193 = c.r32(6) & 0xfu64;
            c.w32(6, r193);
            c.logic(r193, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x70008cf2ec); }
            c.cmp(c.r32(6), 0x8u64, 32);
            if c.cc_e() { blk = 177; continue; }
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 181; continue; }
            blk = 168; continue;
        }
        168 => {
            let r194 = c.r32(1) & 0x7u64;
            c.w32(1, r194);
            c.logic(r194, 32);
            c.cmp(c.r32(1), 0x5u64, 32);
            if c.cc_ne() { blk = 170; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t195) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cf306); };
            c.w32(2, t195);
            let r196 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r196);
            blk = 166; continue;
        }
        170 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_ne() { blk = 175; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t197) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cf313); };
            c.w32(1, t197);
            let r198 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r198);
            let Some(t199) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cf319); };
            c.cmp(t199, 0x0u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 172; continue;
        }
        172 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008cf31f); }
            c.w64(0, c.r64(2));
            return c.call(0x70008e88b4, 0x70008cf32b, 0x70008cf326);
        }
        173 => {
            c.w64(2, c.r64(0));
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008cf32e); };
            c.w64(8, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cf332); };
            c.w64(3, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cf336); };
            c.w64(15, t202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cf33a); };
            c.w64(0, t203);
            blk = 166; continue;
        }
        174 => {
            let Some(t204) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cf340); };
            c.w32(2, t204);
            let r205 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r205);
            blk = 166; continue;
        }
        175 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_ne() { blk = 185; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t206) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008cf34d); };
            c.w32(7, t206);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let r207 = c.add(c.r64(0), c.r64(7), 64);
            c.w64(0, r207);
            let Some(t208) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008cf359); };
            c.w32(2, t208);
            let r209 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r209);
            blk = 166; continue;
        }
        177 => {
            let Some(t210) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cf364); };
            c.w32(7, t210);
            let Some(t211) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008cf367); };
            c.cmp(c.r32(7), t211, 32);
            if c.cc_b() { blk = 180; continue; }
            blk = 178; continue;
        }
        178 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r212 = c.sub(c.r64(7), c.r64(15), 64);
            c.w64(7, r212);
            c.cmp(c.r64(7), c.r64(0), 64);
            if c.cc_b() { blk = 180; continue; }
            blk = 179; continue;
        }
        179 => {
            c.w64(7, c.r64(0).wrapping_add(0xc));
            blk = 193; continue;
        }
        180 => {
            return RegfileExit::fault(0x70008cf380);
        }
        181 => {
            let Some(t213) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cf381); };
            c.w32(7, t213);
            let Some(t214) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008cf384); };
            c.cmp(c.r32(7), t214, 32);
            if c.cc_b() { blk = 184; continue; }
            blk = 182; continue;
        }
        182 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r215 = c.sub(c.r64(7), c.r64(15), 64);
            c.w64(7, r215);
            c.cmp(c.r64(7), c.r64(0), 64);
            if c.cc_b() { blk = 184; continue; }
            blk = 183; continue;
        }
        183 => {
            c.w64(7, c.r64(0).wrapping_mul(2).wrapping_add(0xc));
            blk = 215; continue;
        }
        184 => {
            return RegfileExit::fault(0x70008cf3a1);
        }
        185 => {
            c.cmp(c.r32(6), 0xau64, 32);
            if c.cc_ne() { blk = 207; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t216) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cf3ab); };
            c.logic(t216 & 0x10u64, 8);
            if c.cc_ne() { blk = 188; continue; }
            blk = 187; continue;
        }
        187 => {
            let Some(t217) = c.ld64(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cf3b1); };
            c.w64(7, t217);
            blk = 190; continue;
        }
        188 => {
            c.w64(10, c.r64(4));
            let r218 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r218);
            let r219 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r219);
            c.logic(r219, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008cf3c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(2)) { return RegfileExit::fault(0x70008cf3c7); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008cf3cb); }
            c.w64(1, c.r64(2));
            let Some(t220) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008cf3d2); };
            c.w64(0, t220);
            c.w64(10, 0x70008cf3e0u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008cf3e0); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008cf3e4); }
            return c.call(c.r64(0), 0x70008cf3ea, 0x70008cf3e8);
        }
        189 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008cf3ea); }
            let Some(t221) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008cf3f2); };
            c.w64(4, t221);
            c.w64(7, c.r64(0));
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008cf3fa); };
            c.w64(8, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cf3fe); };
            c.w64(3, t223);
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cf402); };
            c.w64(15, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cf406); };
            c.w64(2, t225);
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cf40a); };
            c.w64(0, t226);
            blk = 190; continue;
        }
        190 => {
            c.w64(1, c.r64(2));
            let Some(t227) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cf411); };
            c.w32(2, t227);
            let Some(t228) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008cf414); };
            c.cmp(c.r32(2), t228, 32);
            if c.cc_b() { blk = 206; continue; }
            blk = 191; continue;
        }
        191 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r229 = c.sub(c.r64(2), c.r64(15), 64);
            c.w64(2, r229);
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_b() { blk = 206; continue; }
            blk = 192; continue;
        }
        192 => {
            c.w64(7, c.r64(0).wrapping_add(c.r64(7)).wrapping_add(0x1));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 193; continue;
        }
        193 => {
            let Some(t230) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cf42f); };
            c.cmp(t230, 0x0u64, 32);
            if c.cc_ne() { blk = 203; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t231) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cf435); };
            c.cmp(t231, 0x0u64, 32);
            if c.cc_ne() { blk = 203; continue; }
            blk = 195; continue;
        }
        195 => {
            c.w64(9, c.r64(15));
            let r232 = c.add(c.r64(9), c.r64(7), 64);
            c.w64(9, r232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cf441); };
            c.w64(11, t233);
            blk = 197; continue;
        }
        196 => {
            let r234 = c.add(c.r64(11), 0x2u64, 64);
            c.w64(11, r234);
            let r235 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r235);
            blk = 197; continue;
        }
        197 => {
            c.cmp(c.r64(9), c.r64(7), 64);
            if c.cc_e() { blk = 202; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t236) = c.ld8(c.r64(2).wrapping_add(c.r64(7)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cf454); };
            c.w32(12, t236);
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_e() { blk = 201; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t237) = c.ld16(c.r64(8).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cf45f); };
            c.w32(15, t237);
            c.cmp(c.r32(15), c.r32(12), 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 200; continue;
        }
        200 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 204; continue;
        }
        201 => {
            return RegfileExit::fault(0x70008cf46e);
        }
        202 => {
            c.w32(2, 0x1u64);
            blk = 204; continue;
        }
        203 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 204; continue;
        }
        204 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 228; continue; }
            blk = 205; continue;
        }
        205 => {
            blk = 227; continue;
        }
        206 => {
            return RegfileExit::fault(0x70008cf486);
        }
        207 => {
            c.cmp(c.r32(6), 0x2u64, 32);
            if c.cc_ne() { blk = 230; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t238) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008cf490); };
            c.logic(t238 & 0x10u64, 8);
            if c.cc_ne() { blk = 210; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t239) = c.ld64(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008cf496); };
            c.w64(7, t239);
            blk = 212; continue;
        }
        210 => {
            c.w64(10, c.r64(4));
            let r240 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r240);
            let r241 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r241);
            c.logic(r241, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008cf4a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(2)) { return RegfileExit::fault(0x70008cf4ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008cf4b0); }
            c.w64(1, c.r64(2));
            let Some(t242) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008cf4b7); };
            c.w64(0, t242);
            c.w64(10, 0x70008cf4c5u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008cf4c5); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008cf4c9); }
            return c.call(c.r64(0), 0x70008cf4cf, 0x70008cf4cd);
        }
        211 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008cf4cf); }
            let Some(t243) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008cf4d7); };
            c.w64(4, t243);
            c.w64(7, c.r64(0));
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008cf4df); };
            c.w64(8, t244);
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cf4e3); };
            c.w64(3, t245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cf4e7); };
            c.w64(15, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cf4eb); };
            c.w64(2, t247);
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008cf4ef); };
            c.w64(0, t248);
            blk = 212; continue;
        }
        212 => {
            let Some(t249) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cf4f3); };
            c.w32(2, t249);
            let Some(t250) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008cf4f6); };
            c.cmp(c.r32(2), t250, 32);
            if c.cc_b() { blk = 229; continue; }
            blk = 213; continue;
        }
        213 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r251 = c.sub(c.r64(2), c.r64(15), 64);
            c.w64(2, r251);
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_b() { blk = 229; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w64(7, c.r64(7).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x1));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 215; continue;
        }
        215 => {
            let Some(t252) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008cf515); };
            c.cmp(t252, 0x0u64, 32);
            if c.cc_ne() { blk = 225; continue; }
            blk = 216; continue;
        }
        216 => {
            let Some(t253) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008cf51b); };
            c.cmp(t253, 0x0u64, 32);
            if c.cc_ne() { blk = 225; continue; }
            blk = 217; continue;
        }
        217 => {
            c.w64(9, c.r64(15));
            c.w64(9, c.r64(7).wrapping_add(c.r64(9).wrapping_mul(2)));
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cf528); };
            c.w64(11, t254);
            c.w64(12, c.r64(8));
            c.w64(8, c.r64(3));
            blk = 219; continue;
        }
        218 => {
            let r255 = c.add(c.r64(11), 0x2u64, 64);
            c.w64(11, r255);
            let r256 = c.add(c.r64(7), 0x2u64, 64);
            c.w64(7, r256);
            blk = 219; continue;
        }
        219 => {
            c.cmp(c.r64(9), c.r64(7), 64);
            if c.cc_e() { blk = 224; continue; }
            blk = 220; continue;
        }
        220 => {
            let Some(t257) = c.ld16(c.r64(2).wrapping_add(c.r64(7)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cf541); };
            c.w32(15, t257);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_e() { blk = 223; continue; }
            blk = 221; continue;
        }
        221 => {
            let Some(t258) = c.ld16(c.r64(12).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cf54c); };
            c.cmp(t258, c.r16(15), 16);
            if c.cc_e() { blk = 218; continue; }
            blk = 222; continue;
        }
        222 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 226; continue;
        }
        223 => {
            return RegfileExit::fault(0x70008cf558);
        }
        224 => {
            c.w32(2, 0x1u64);
            blk = 226; continue;
        }
        225 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 226; continue;
        }
        226 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 228; continue; }
            blk = 227; continue;
        }
        227 => {
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cf567); };
            c.w64(2, t259);
            let r260 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r260);
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cf56f); };
            c.w64(0, t261);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cf579); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cf57a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cf580); }
            return c.ret(0x0, 0x70008cf582);
        }
        228 => {
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cf583); };
            c.w64(2, t262);
            let r263 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r263);
            let Some(t264) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008cf58b); };
            c.w64(0, t264);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cf595); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cf596); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cf59c); }
            return c.ret(0x0, 0x70008cf59e);
        }
        229 => {
            return RegfileExit::fault(0x70008cf59f);
        }
        230 => {
            return RegfileExit::fault(0x70008cf5a0);
        }
        231 => {
            return RegfileExit::fault(0x70008cf5a1);
        }
        232 => {
            return RegfileExit::fault(0x70008cf5a2);
        }
        233 => {
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cf5a3); };
            c.w64(2, t265);
            let r266 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r266);
            let Some(t267) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008cf5ab); };
            c.w64(0, t267);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cf5b5); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cf5b6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cf5bc); }
            return c.ret(0x0, 0x70008cf5be);
        }
        234 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008cf5bf); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t268) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cf5c5); };
            c.w64(3, t268);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008cf5d4, 0x70008cf5cf);
        }
        235 => {
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cf5d4); };
            c.w64(6, t269);
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cf5d8); };
            c.w64(0, t270);
            blk = 1; continue;
        }
        236 => {
            let Some(t271) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008cf5e1); };
            c.cmp(t271, c.r32(8), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let Some(t272) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cf5f0); };
            c.cmp(t272, c.r32(8), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let r273 = c.r32(7) | c.r32(12);
            c.w32(7, r273);
            c.logic(r273, 32);
            if c.cc_ne() { blk = 239; continue; }
            blk = 237; continue;
        }
        237 => {
            c.w64(0, c.r64(8));
            return c.call(0x700089c794, 0x70008cf60c, 0x70008cf607);
        }
        238 => {
            c.w64(8, c.r64(0));
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cf60f); };
            c.w64(9, t274);
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cf613); };
            c.w64(2, t275);
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cf617); };
            c.w64(6, t276);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cf61b); };
            c.w64(11, t277);
            blk = 8; continue;
        }
        239 => {
            c.w32(2, 0x36u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cf629); }
            let Some(t278) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cf62a); };
            c.w64(2, t278);
            let Some(t279) = c.ld32(c.r64(2).wrapping_add(0x2d3)) else { return RegfileExit::fault(0x70008cf631); };
            c.w32(2, t279);
            let r280 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r280);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cf63a); }
            let Some(t281) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008cf63b); };
            c.w64(3, t281);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008cf64c, 0x70008cf647);
        }
        240 => {
            return RegfileExit::fault(0x70008cf64c);
        }
        241 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a1214, 0x70008cf655, 0x70008cf650);
        }
        242 => {
            c.w64(8, c.r64(0));
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cf658); };
            c.w64(9, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cf65c); };
            c.w64(2, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cf660); };
            c.w64(6, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008cf664); };
            c.w64(11, t285);
            blk = 8; continue;
        }
        243 => {
            let Some(t286) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008cf66d); };
            c.logic(t286 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 244; continue;
        }
        244 => {
            blk = 20; continue;
        }
        245 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008cf67f); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 20; continue; }
            blk = 246; continue;
        }
        246 => {
            blk = 26; continue;
        }
        247 => {
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cf697); };
            c.w64(6, t287);
            return c.call(0x700089c794, 0x70008cf6a0, 0x70008cf69b);
        }
        248 => {
            blk = 38; continue;
        }
        249 => {
            return RegfileExit::jump(0x70008cf6a8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cf6b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cf6b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008cf6b8); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008cf6be); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008cf6c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008cf6c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008cf6ca); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cf6d6); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cf6dd); };
            c.cmp(t3, 0x40u64, 16);
            if c.cc_ae() { blk = 18; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008cf6e9); }
            c.w64(3, c.r64(1));
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cf6f0); };
            c.w64(0, t4);
            let Some(t5) = c.ld32(c.r64(0).wrapping_add(0x2d7)) else { return RegfileExit::fault(0x70008cf6f7); };
            c.w32(0, t5);
            let r6 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r6);
            c.w64(8, c.r64(0));
            return c.call(0x700086f9d4, 0x70008cf708, 0x70008cf703);
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cf708); };
            c.w64(3, t7);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008cf70c); }
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cf710); };
            c.cmp(t8, c.r32(3), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld8(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cf71d); };
            c.logic(t9 & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cf727); };
            c.w64(2, t10);
            let Some(t11) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008cf72b); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            let Some(t13) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008cf731); };
            c.cmp(t13, 0x40u64, 16);
            if c.cc_ae() { blk = 26; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cf73c); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2780)) else { return RegfileExit::fault(0x70008cf73d); };
            c.w64(2, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cf744); };
            c.w64(6, t15);
            c.w32(0, 0x1u64);
            c.w64(3, c.r64(2));
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008cf758, 0x70008cf753);
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008cf758); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cf75c); };
            c.w64(3, t16);
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(0x2db)) else { return RegfileExit::fault(0x70008cf763); };
            c.w32(3, t17);
            let r18 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cf76c); };
            c.w64(0, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cf770); };
            c.w64(6, t20);
            return c.call(0x700086f9d4, 0x70008cf779, 0x70008cf774);
        }
        8 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cf779); };
            c.w64(6, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008cf77d); };
            c.w64(3, t22);
            return c.call(0x700086f9d4, 0x70008cf786, 0x70008cf781);
        }
        9 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cf786); };
            c.w64(3, t23);
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(0x2df)) else { return RegfileExit::fault(0x70008cf78d); };
            c.w32(3, t24);
            let r25 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cf796); };
            c.w64(6, t26);
            return c.call(0x700086f9d4, 0x70008cf79f, 0x70008cf79a);
        }
        10 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cf79f); };
            c.w64(6, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cf7a3); };
            c.w64(3, t28);
            return c.call(0x700086f9d4, 0x70008cf7ac, 0x70008cf7a7);
        }
        11 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cf7ac); };
            c.w64(3, t29);
            let Some(t30) = c.ld32(c.r64(3).wrapping_add(0x2e3)) else { return RegfileExit::fault(0x70008cf7b3); };
            c.w32(3, t30);
            let r31 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cf7bc); };
            c.w64(6, t32);
            return c.call(0x700086f9d4, 0x70008cf7c5, 0x70008cf7c0);
        }
        12 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cf7c5); };
            c.w64(3, t33);
            let Some(t34) = c.ld32(c.r64(3).wrapping_add(0x2e7)) else { return RegfileExit::fault(0x70008cf7cc); };
            c.w32(3, t34);
            let r35 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cf7d5); };
            c.w64(6, t36);
            return c.call(0x700086f9d4, 0x70008cf7de, 0x70008cf7d9);
        }
        13 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cf7de); };
            c.w64(6, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cf7e2); };
            c.w64(3, t38);
            return c.call(0x700086f9d4, 0x70008cf7eb, 0x70008cf7e6);
        }
        14 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cf7eb); };
            c.w64(3, t39);
            let Some(t40) = c.ld32(c.r64(3).wrapping_add(0x2eb)) else { return RegfileExit::fault(0x70008cf7f2); };
            c.w32(3, t40);
            let r41 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cf7fb); };
            c.w64(6, t42);
            return c.call(0x700086f9d4, 0x70008cf804, 0x70008cf7ff);
        }
        15 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cf804); };
            c.w64(6, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cf808); };
            c.w64(3, t44);
            return c.call(0x700086f9d4, 0x70008cf811, 0x70008cf80c);
        }
        16 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cf811); };
            c.w64(6, t45);
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cf815); };
            c.w64(3, t46);
            let Some(t47) = c.ld32(c.r64(3).wrapping_add(0x2e7)) else { return RegfileExit::fault(0x70008cf81c); };
            c.w32(3, t47);
            let r48 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r48);
            return c.call(0x700086f9d4, 0x70008cf82a, 0x70008cf825);
        }
        17 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cf82d); }
            return c.ret(0x0, 0x70008cf82e);
        }
        18 => {
            let Some(t49) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008cf82f); };
            c.cmp(t49, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cf83e); };
            c.cmp(t50, c.r32(0), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let r51 = c.r32(8) | c.r32(9);
            c.w32(8, r51);
            c.logic(r51, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            return c.call(0x700089c794, 0x70008cf857, 0x70008cf852);
        }
        20 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cf857); };
            c.w64(1, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cf85b); };
            c.w64(6, t53);
            blk = 2; continue;
        }
        21 => {
            c.w32(2, 0x36u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008cf869); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008cf86a); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008cf86b); };
            c.w64(3, t54);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008cf87c, 0x70008cf877);
        }
        22 => {
            return RegfileExit::fault(0x70008cf87c);
        }
        23 => {
            return c.call(0x70008a1214, 0x70008cf882, 0x70008cf87d);
        }
        24 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008cf882); };
            c.w64(1, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cf886); };
            c.w64(6, t56);
            blk = 2; continue;
        }
        25 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008cf88f); };
            c.w64(2, t57);
            blk = 26; continue;
        }
        26 => {
            c.w64(0, c.r64(2));
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008cf896); };
            c.w64(6, t58);
            return c.call(0x700089c794, 0x70008cf89f, 0x70008cf89a);
        }
        27 => {
            c.w64(2, c.r64(0));
            blk = 6; continue;
        }
        28 => {
            return RegfileExit::jump(0x70008cf8a8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cf8b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cf8b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cf8b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cf8b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cf8ba); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cf8bf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cf8cf); };
            c.w64(9, t2);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008cf8d7); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cf8e1); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cf8e7); };
            c.w64(8, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cf8ee); };
            c.w64(3, t5);
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0x2ef)) else { return RegfileExit::fault(0x70008cf8f5); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cf8fe); };
            c.w64(1, t8);
            let Some(t9) = c.ld32(c.r64(1).wrapping_add(0x2f3)) else { return RegfileExit::fault(0x70008cf905); };
            c.w32(1, t9);
            let r10 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r10);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x890)) else { return RegfileExit::fault(0x70008cf90e); };
            c.w64(2, t11);
            c.w64(0, c.r64(9));
            c.w64(7, c.r64(8));
            return c.call(0x70008cf6b4, 0x70008cf920, 0x70008cf91b);
        }
        5 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cf920); };
            c.w64(8, t12);
            let r13 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r13);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cf92b); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cf92c); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cf932); }
            return c.ret(0x0, 0x70008cf934);
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008cf935); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008cf939); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cf93d); };
            c.w64(3, t14);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008cf94e, 0x70008cf949);
        }
        7 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cf94e); };
            c.w64(6, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cf952); };
            c.w64(0, t16);
            blk = 1; continue;
        }
        8 => {
            return RegfileExit::jump(0x70008cf95c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cf974(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cf974); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cf978); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cf979); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cf97a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008cf97f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cf983); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cf993); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cf998); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x2f7)) else { return RegfileExit::fault(0x70008cf99f); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cf9a8); };
            c.w64(1, t6);
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0x2fb)) else { return RegfileExit::fault(0x70008cf9af); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cf9b8); };
            c.w64(2, t9);
            c.w64(0, c.r64(8));
            c.w64(7, c.r64(2));
            return c.call(0x70008cf6b4, 0x70008cf9ca, 0x70008cf9c5);
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cf9ca); };
            c.w64(8, (t10 as u32 as i32 as i64 as u64));
            let r11 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cf9d5); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cf9d6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cf9dc); }
            return c.ret(0x0, 0x70008cf9de);
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008cf9df); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cf9e3); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008cf9f4, 0x70008cf9ef);
        }
        4 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cf9f4); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        5 => {
            return RegfileExit::jump(0x70008cf9fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cfa14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cfa14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cfa18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cfa19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cfa1a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008cfa1f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cfa23); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cfa33); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfa38); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x70008cfa3f); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfa48); };
            c.w64(1, t6);
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0x303)) else { return RegfileExit::fault(0x70008cfa4f); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cfa58); };
            c.w64(2, t9);
            c.w64(0, c.r64(8));
            c.w64(7, c.r64(2));
            return c.call(0x70008cf6b4, 0x70008cfa6a, 0x70008cfa65);
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cfa6a); };
            c.w64(8, (t10 as u32 as i32 as i64 as u64));
            let r11 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cfa75); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cfa76); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cfa7c); }
            return c.ret(0x0, 0x70008cfa7e);
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008cfa7f); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cfa83); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008cfa94, 0x70008cfa8f);
        }
        4 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cfa94); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        5 => {
            return RegfileExit::jump(0x70008cfa9c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cfab4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cfab4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cfab8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cfab9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cfaba); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008cfabf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cfac3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cfad3); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfad8); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x307)) else { return RegfileExit::fault(0x70008cfadf); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfae8); };
            c.w64(1, t6);
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0x30b)) else { return RegfileExit::fault(0x70008cfaef); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cfaf8); };
            c.w64(2, t9);
            c.w64(0, c.r64(8));
            c.w64(7, c.r64(2));
            return c.call(0x70008cf6b4, 0x70008cfb0a, 0x70008cfb05);
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cfb0a); };
            c.w64(8, (t10 as u32 as i32 as i64 as u64));
            let r11 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cfb15); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cfb16); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cfb1c); }
            return c.ret(0x0, 0x70008cfb1e);
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008cfb1f); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cfb23); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008cfb34, 0x70008cfb2f);
        }
        4 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cfb34); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        5 => {
            return RegfileExit::jump(0x70008cfb3c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cfb54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cfb54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cfb58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cfb59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cfb5a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cfb5f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cfb73); };
            c.w64(9, t2);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008cfb7b); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cfb85); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cfb8b); };
            c.w64(8, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfb92); };
            c.w64(3, t5);
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0x30f)) else { return RegfileExit::fault(0x70008cfb99); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfba2); };
            c.w64(1, t8);
            let Some(t9) = c.ld32(c.r64(1).wrapping_add(0x313)) else { return RegfileExit::fault(0x70008cfba9); };
            c.w32(1, t9);
            let r10 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r10);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfbb2); };
            c.w64(2, t11);
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x317)) else { return RegfileExit::fault(0x70008cfbb9); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            c.w64(0, c.r64(9));
            c.w64(7, c.r64(8));
            return c.call(0x70008cf6b4, 0x70008cfbcd, 0x70008cfbc8);
        }
        5 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cfbcd); };
            c.w64(8, t14);
            let r15 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r15);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cfbd8); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cfbd9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cfbdf); }
            return c.ret(0x0, 0x70008cfbe1);
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008cfbe2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008cfbe6); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cfbea); };
            c.w64(3, t16);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008cfbfb, 0x70008cfbf6);
        }
        7 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cfbfb); };
            c.w64(6, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cfbff); };
            c.w64(0, t18);
            blk = 1; continue;
        }
        8 => {
            return RegfileExit::jump(0x70008cfc0c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cfc14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cfc14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cfc18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cfc19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cfc1a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cfc1f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cfc33); };
            c.w64(9, t2);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008cfc3b); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cfc45); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cfc4b); };
            c.w64(8, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfc52); };
            c.w64(3, t5);
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0x31b)) else { return RegfileExit::fault(0x70008cfc59); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfc62); };
            c.w64(1, t8);
            let Some(t9) = c.ld32(c.r64(1).wrapping_add(0x313)) else { return RegfileExit::fault(0x70008cfc69); };
            c.w32(1, t9);
            let r10 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r10);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfc72); };
            c.w64(2, t11);
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x31f)) else { return RegfileExit::fault(0x70008cfc79); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            c.w64(0, c.r64(9));
            c.w64(7, c.r64(8));
            return c.call(0x70008cf6b4, 0x70008cfc8d, 0x70008cfc88);
        }
        5 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cfc8d); };
            c.w64(8, t14);
            let r15 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r15);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cfc98); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cfc99); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cfc9f); }
            return c.ret(0x0, 0x70008cfca1);
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008cfca2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008cfca6); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cfcaa); };
            c.w64(3, t16);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008cfcbb, 0x70008cfcb6);
        }
        7 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cfcbb); };
            c.w64(6, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cfcbf); };
            c.w64(0, t18);
            blk = 1; continue;
        }
        8 => {
            return RegfileExit::jump(0x70008cfccc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cfcd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cfcd4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cfcd8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cfcd9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cfcda); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008cfcdf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cfce3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cfcf3); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfcf8); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x323)) else { return RegfileExit::fault(0x70008cfcff); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfd08); };
            c.w64(1, t6);
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0x327)) else { return RegfileExit::fault(0x70008cfd0f); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cfd18); };
            c.w64(2, t9);
            c.w64(0, c.r64(8));
            c.w64(7, c.r64(2));
            return c.call(0x70008cf6b4, 0x70008cfd2a, 0x70008cfd25);
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cfd2a); };
            c.w64(8, (t10 as u32 as i32 as i64 as u64));
            let r11 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cfd35); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cfd36); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cfd3c); }
            return c.ret(0x0, 0x70008cfd3e);
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008cfd3f); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cfd43); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008cfd54, 0x70008cfd4f);
        }
        4 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cfd54); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        5 => {
            return RegfileExit::jump(0x70008cfd5c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cfd74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cfd74); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cfd78); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cfd79); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cfd7a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008cfd7f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cfd83); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cfd93); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfd98); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x32b)) else { return RegfileExit::fault(0x70008cfd9f); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfda8); };
            c.w64(1, t6);
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0x32f)) else { return RegfileExit::fault(0x70008cfdaf); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cfdb8); };
            c.w64(2, t9);
            c.w64(0, c.r64(8));
            c.w64(7, c.r64(2));
            return c.call(0x70008cf6b4, 0x70008cfdca, 0x70008cfdc5);
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cfdca); };
            c.w64(8, (t10 as u32 as i32 as i64 as u64));
            let r11 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cfdd5); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cfdd6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cfddc); }
            return c.ret(0x0, 0x70008cfdde);
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008cfddf); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cfde3); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008cfdf4, 0x70008cfdef);
        }
        4 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cfdf4); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        5 => {
            return RegfileExit::jump(0x70008cfdfc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cfe14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cfe14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cfe18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cfe19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cfe1a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cfe1f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cfe33); };
            c.w64(9, t2);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008cfe3b); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cfe45); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008cfe4b); };
            c.w64(8, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfe52); };
            c.w64(3, t5);
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0x333)) else { return RegfileExit::fault(0x70008cfe59); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfe62); };
            c.w64(1, t8);
            let Some(t9) = c.ld32(c.r64(1).wrapping_add(0x2f3)) else { return RegfileExit::fault(0x70008cfe69); };
            c.w32(1, t9);
            let r10 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r10);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfe72); };
            c.w64(2, t11);
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x337)) else { return RegfileExit::fault(0x70008cfe79); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            c.w64(0, c.r64(9));
            c.w64(7, c.r64(8));
            return c.call(0x70008cf6b4, 0x70008cfe8d, 0x70008cfe88);
        }
        5 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cfe8d); };
            c.w64(8, t14);
            let r15 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r15);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cfe98); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cfe99); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cfe9f); }
            return c.ret(0x0, 0x70008cfea1);
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008cfea2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008cfea6); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cfeaa); };
            c.w64(3, t16);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008cfebb, 0x70008cfeb6);
        }
        7 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cfebb); };
            c.w64(6, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cfebf); };
            c.w64(0, t18);
            blk = 1; continue;
        }
        8 => {
            return RegfileExit::jump(0x70008cfecc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cfed4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cfed4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cfed8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cfed9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cfeda); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008cfedf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cfee3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cfef3); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cfef8); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x33b)) else { return RegfileExit::fault(0x70008cfeff); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cff08); };
            c.w64(1, t6);
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0x33f)) else { return RegfileExit::fault(0x70008cff0f); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cff18); };
            c.w64(2, t9);
            c.w64(0, c.r64(8));
            c.w64(7, c.r64(2));
            return c.call(0x70008cf6b4, 0x70008cff2a, 0x70008cff25);
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cff2a); };
            c.w64(8, (t10 as u32 as i32 as i64 as u64));
            let r11 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cff35); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cff36); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cff3c); }
            return c.ret(0x0, 0x70008cff3e);
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008cff3f); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cff43); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008cff54, 0x70008cff4f);
        }
        4 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cff54); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        5 => {
            return RegfileExit::jump(0x70008cff5c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008cff74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008cff74); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008cff78); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008cff79); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008cff7a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008cff7f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008cff83); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008cff93); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cff98); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x343)) else { return RegfileExit::fault(0x70008cff9f); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008cffa8); };
            c.w64(1, t6);
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0x347)) else { return RegfileExit::fault(0x70008cffaf); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008cffb8); };
            c.w64(2, t9);
            c.w64(0, c.r64(8));
            c.w64(7, c.r64(2));
            return c.call(0x70008cf6b4, 0x70008cffca, 0x70008cffc5);
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008cffca); };
            c.w64(8, (t10 as u32 as i32 as i64 as u64));
            let r11 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008cffd5); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008cffd6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008cffdc); }
            return c.ret(0x0, 0x70008cffde);
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008cffdf); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008cffe3); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008cfff4, 0x70008cffef);
        }
        4 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008cfff4); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        5 => {
            return RegfileExit::jump(0x70008cfffc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d0014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d0014); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d0018); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d0019); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d001a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d001f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d0023); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d0033); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d0038); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x34b)) else { return RegfileExit::fault(0x70008d003f); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d0048); };
            c.w64(1, t6);
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0x34f)) else { return RegfileExit::fault(0x70008d004f); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d0058); };
            c.w64(2, t9);
            c.w64(0, c.r64(8));
            c.w64(7, c.r64(2));
            return c.call(0x70008cf6b4, 0x70008d006a, 0x70008d0065);
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d006a); };
            c.w64(8, (t10 as u32 as i32 as i64 as u64));
            let r11 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d0075); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d0076); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d007c); }
            return c.ret(0x0, 0x70008d007e);
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008d007f); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d0083); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008d0094, 0x70008d008f);
        }
        4 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d0094); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        5 => {
            return RegfileExit::jump(0x70008d009c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d00b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d00b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d00b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d00b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d00ba); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d00bf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d00c3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d00d3); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d00d8); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x353)) else { return RegfileExit::fault(0x70008d00df); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d00e8); };
            c.w64(1, t6);
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0x357)) else { return RegfileExit::fault(0x70008d00ef); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d00f8); };
            c.w64(2, t9);
            c.w64(0, c.r64(8));
            c.w64(7, c.r64(2));
            return c.call(0x70008cf6b4, 0x70008d010a, 0x70008d0105);
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d010a); };
            c.w64(8, (t10 as u32 as i32 as i64 as u64));
            let r11 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d0115); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d0116); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d011c); }
            return c.ret(0x0, 0x70008d011e);
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008d011f); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d0123); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008d0134, 0x70008d012f);
        }
        4 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d0134); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        5 => {
            return RegfileExit::jump(0x70008d013c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d0154(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d0154); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d0158); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d0159); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d015a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008d015f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d0163); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 64; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(7, c.r64(2).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d017b); };
            c.w64(8, t2);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008d0183); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d018d); };
            c.w64(2, t3);
            c.w64(11, c.r64(2));
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d0196); };
            c.w64(11, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d019d); };
            c.w64(2, t5);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008d01a4); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d01ae); };
            c.w64(2, t6);
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008d01b2); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 71; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d01c0); };
            c.w32(7, t7);
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            let Some(t9) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d01c7); };
            c.cmp(t9, 0x40u64, 16);
            if c.cc_ae() { blk = 66; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008d01de); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008d01e2); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d01ef); };
            c.w32(12, t10);
            let r11 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d01f6); };
            c.w32(12, t12);
            let r13 = c.sub(c.r32(12), 0xbau64, 32);
            c.w32(12, r13);
            c.cmp(c.r32(12), 0x78eu64, 32);
            if c.cc_a() { blk = 35; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xcd0)) else { return RegfileExit::fault(0x70008d0210); };
            c.w64(3, t14);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x70008d021f, 0x70008d021a);
        }
        11 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d021f); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008d022c); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d023f); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d024c); };
            c.w32(2, t18);
            let r19 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r19);
            let Some(t20) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008d0252); };
            c.logic(t20 & 0x10u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008d0258); };
            c.cmp(t21, c.r32(2), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t22) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d0265); };
            c.cmp(t22, 0x41u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 18; continue;
        }
        18 => {
            blk = 20; continue;
        }
        19 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d0277); };
            c.w64(2, t23);
            let Some(t24) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d027b); };
            c.w32(2, t24);
            let r25 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r25);
            let Some(t26) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d0281); };
            c.cmp(t26, 0x83eu64, 16);
            if c.cc_ne() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(2, 0x1u64);
            blk = 23; continue;
        }
        22 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 23; continue;
        }
        23 => {
            c.w32(0, 0x1u64);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(2, 0x90u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d02a5); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x70008d02a6); };
            c.w64(3, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d02ad); };
            c.w64(6, t28);
            return c.call(0x700086ed74, 0x70008d02b6, 0x70008d02b1);
        }
        25 => {
            blk = 33; continue;
        }
        26 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d02b8); };
            c.w64(2, t29);
            let Some(t30) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d02bc); };
            c.w32(7, t30);
            let r31 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r31);
            let Some(t32) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d02c2); };
            c.cmp(t32, 0x83eu64, 16);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(7, 0x92u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d02cf); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x70008d02d0); };
            c.w64(3, t33);
            c.w32(0, 0x1u64);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d02dc); };
            c.w64(6, t34);
            return c.call(0x700086ed74, 0x70008d02e5, 0x70008d02e0);
        }
        28 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d02e5); };
            c.w64(0, t35);
            blk = 36; continue;
        }
        29 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d02eb); };
            c.w64(0, t36);
            let Some(t37) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d02ef); };
            c.w32(2, t37);
            let r38 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r38);
            let Some(t39) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d02f5); };
            c.cmp(t39, 0x83eu64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(2, 0x1u64);
            blk = 32; continue;
        }
        31 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(2, 0x6cu64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d0310); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d0311); };
            c.w64(2, t40);
            let Some(t41) = c.ld32(c.r64(2).wrapping_add(0x35b)) else { return RegfileExit::fault(0x70008d0318); };
            c.w32(2, t41);
            let r42 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r42);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d0321); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d0322); };
            c.w64(3, t43);
            c.w32(0, 0x2u64);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d032e); };
            c.w64(6, t44);
            return c.call(0x700086ed74, 0x70008d0337, 0x70008d0332);
        }
        34 => {
            return RegfileExit::fault(0x70008d0337);
        }
        35 => {
            c.w64(0, c.r64(11));
            blk = 36; continue;
        }
        36 => {
            let Some(t45) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d033b); };
            c.cmp(t45, 0x0u64, 32);
            if c.cc_ne() { blk = 77; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t46) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d0345); };
            c.w32(2, t46);
            let r47 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r47);
            let Some(t48) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d034b); };
            c.cmp(t48, 0x40u64, 16);
            if c.cc_ae() { blk = 77; continue; }
            blk = 38; continue;
        }
        38 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008d0356); }
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d035a); };
            c.w64(2, t49);
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d035e); };
            c.cmp(t50, c.r32(2), 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(7, 0);
            c.logic(0, 32);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d0369); };
            c.w64(8, t51);
            blk = 60; continue;
        }
        40 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d0372); };
            c.w64(8, t52);
            let Some(t53) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d0376); };
            c.w32(7, t53);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008d037a); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(0, c.r64(2));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d0386); };
            c.w64(6, t54);
            return c.call(0x7000899a14, 0x70008d038f, 0x70008d038a);
        }
        42 => {
            c.w64(2, c.r64(0));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d0392); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d0396); };
            c.w64(0, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d039a); };
            c.w64(7, t57);
            blk = 43; continue;
        }
        43 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_l() { blk = 47; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(7), c.r64(2), 64);
            if c.cc_be() { blk = 59; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(7, c.r64(2));
            blk = 59; continue;
        }
        47 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 59; continue;
        }
        48 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008d03c6); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 58; continue; }
            blk = 49; continue;
        }
        49 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(7) as i64));
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(10, c.r64(7));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            let r58 = c.r64(10) | 0x1u64;
            c.w64(10, r58);
            c.logic(r58, 64);
            blk = 52; continue;
        }
        52 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(10) as i64));
            c.arith_sd(0, 1, c.xmm_u64(1));
            blk = 53; continue;
        }
        53 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 59; continue; }
            blk = 54; continue;
        }
        54 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(7, cv);
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_s() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(10, 0x8000000000000000u64);
            let r59 = c.r64(7) | c.r64(10);
            c.w64(7, r59);
            c.logic(r59, 64);
            blk = 57; continue;
        }
        57 => {
            blk = 59; continue;
        }
        58 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 59; continue;
        }
        59 => {
            let r60 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r60);
            blk = 60; continue;
        }
        60 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(8));
            c.w64(1, c.r64(7));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d0446); };
            c.w64(6, t61);
            return c.call(0x70008e8b74, 0x70008d044f, 0x70008d044a);
        }
        61 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d044f); };
            c.w64(2, t62);
            let r63 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r63);
            c.cmp(c.r32(0), 0xfffffffffffffffeu64, 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008d045c); };
            c.w64(0, t64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d0466); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d0467); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d046d); }
            return c.ret(0x0, 0x70008d046f);
        }
        63 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008d0470); };
            c.w64(0, t65);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d047a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d047b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d0481); }
            return c.ret(0x0, 0x70008d0483);
        }
        64 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d0484); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d048a); };
            c.w64(3, t66);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008d0499, 0x70008d0494);
        }
        65 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d0499); };
            c.w64(6, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d049d); };
            c.w64(0, t68);
            blk = 1; continue;
        }
        66 => {
            let Some(t69) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d04a6); };
            c.cmp(t69, c.r32(8), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let Some(t70) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d04b5); };
            c.cmp(t70, c.r32(8), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let r71 = c.r32(7) | c.r32(12);
            c.w32(7, r71);
            c.logic(r71, 32);
            if c.cc_ne() { blk = 69; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(0, c.r64(8));
            return c.call(0x700089c794, 0x70008d04d1, 0x70008d04cc);
        }
        68 => {
            c.w64(8, c.r64(0));
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d04d4); };
            c.w64(9, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d04d8); };
            c.w64(6, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d04dc); };
            c.w64(2, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d04e0); };
            c.w64(11, t75);
            blk = 8; continue;
        }
        69 => {
            c.w32(2, 0x36u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d04ee); }
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d04ef); };
            c.w64(2, t76);
            let Some(t77) = c.ld32(c.r64(2).wrapping_add(0x35b)) else { return RegfileExit::fault(0x70008d04f6); };
            c.w32(2, t77);
            let r78 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r78);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d04ff); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d0500); };
            c.w64(3, t79);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008d0511, 0x70008d050c);
        }
        70 => {
            return RegfileExit::fault(0x70008d0511);
        }
        71 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a1214, 0x70008d051a, 0x70008d0515);
        }
        72 => {
            c.w64(8, c.r64(0));
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d051d); };
            c.w64(9, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d0521); };
            c.w64(6, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d0525); };
            c.w64(2, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d0529); };
            c.w64(11, t83);
            blk = 8; continue;
        }
        73 => {
            let Some(t84) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d0532); };
            c.logic(t84 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 74; continue;
        }
        74 => {
            blk = 20; continue;
        }
        75 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008d0544); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 20; continue; }
            blk = 76; continue;
        }
        76 => {
            blk = 26; continue;
        }
        77 => {
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d055c); };
            c.w64(6, t85);
            return c.call(0x700089c794, 0x70008d0565, 0x70008d0560);
        }
        78 => {
            blk = 38; continue;
        }
        79 => {
            return RegfileExit::jump(0x70008d056c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d0574(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d0574); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d0578); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d0579); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d057a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008d057f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d0583); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 35; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(7, c.r64(2).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d059b); };
            c.w64(8, t2);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008d05a3); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d05ad); };
            c.w64(2, t3);
            c.w64(11, c.r64(2));
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d05b6); };
            c.w64(11, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d05bd); };
            c.w64(2, t5);
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d05ca); };
            c.w64(2, t6);
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 42; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d05d8); };
            c.w32(7, t7);
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            let Some(t9) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d05df); };
            c.cmp(t9, 0x40u64, 16);
            if c.cc_ae() { blk = 37; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008d05ea); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d05f8); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d05ff); };
            c.cmp(t12, 0x40u64, 16);
            if c.cc_ae() { blk = 44; continue; }
            blk = 10; continue;
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008d060a); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d060e); };
            c.cmp(t13, c.r32(2), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 33; continue;
        }
        12 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d061e); };
            c.w32(7, t14);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008d0622); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(2));
            blk = 16; continue;
        }
        14 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x70008d0638, 0x70008d0633);
        }
        15 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d0638); };
            c.w64(7, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d063c); };
            c.w64(9, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d0640); };
            c.w64(8, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d0644); };
            c.w64(11, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d0648); };
            c.w64(6, t19);
            blk = 16; continue;
        }
        16 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(7), c.r64(2), 64);
            if c.cc_be() { blk = 32; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(7, c.r64(2));
            blk = 32; continue;
        }
        20 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 32; continue;
        }
        21 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008d0673); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 31; continue; }
            blk = 22; continue;
        }
        22 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(7) as i64));
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(10, c.r64(7));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            let r20 = c.r64(10) | 0x1u64;
            c.w64(10, r20);
            c.logic(r20, 64);
            blk = 25; continue;
        }
        25 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(10) as i64));
            c.arith_sd(0, 1, c.xmm_u64(1));
            blk = 26; continue;
        }
        26 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(7, cv);
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_s() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(10, 0x8000000000000000u64);
            let r21 = c.r64(7) | c.r64(10);
            c.w64(7, r21);
            c.logic(r21, 64);
            blk = 30; continue;
        }
        30 => {
            blk = 32; continue;
        }
        31 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 32; continue;
        }
        32 => {
            let r22 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r22);
            blk = 33; continue;
        }
        33 => {
            c.w64(0, c.r64(8));
            c.w64(3, c.r64(11));
            c.w64(1, c.r64(7));
            return c.call(0x70008e8b74, 0x70008d06f8, 0x70008d06f3);
        }
        34 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d06f8); };
            c.w64(2, t23);
            let r24 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r24);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d0703); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d0704); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d070a); }
            return c.ret(0x0, 0x70008d070c);
        }
        35 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d070d); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d0711); };
            c.w64(3, t25);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008d0722, 0x70008d071d);
        }
        36 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d0722); };
            c.w64(6, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d0726); };
            c.w64(0, t27);
            blk = 1; continue;
        }
        37 => {
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d072f); };
            c.cmp(t28, c.r32(8), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let Some(t29) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d073e); };
            c.cmp(t29, c.r32(8), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008d074d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008d0751); }
            let r30 = c.r32(7) | c.r32(12);
            c.w32(7, r30);
            c.logic(r30, 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, c.r64(8));
            return c.call(0x700089c794, 0x70008d0762, 0x70008d075d);
        }
        39 => {
            c.w64(8, c.r64(0));
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d0765); };
            c.w64(9, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d0769); };
            c.w64(6, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d076d); };
            c.w64(2, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d0771); };
            c.w64(11, t34);
            blk = 8; continue;
        }
        40 => {
            c.w32(2, 0x36u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d077f); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d0780); };
            c.w64(2, t35);
            let Some(t36) = c.ld32(c.r64(2).wrapping_add(0x35f)) else { return RegfileExit::fault(0x70008d0787); };
            c.w32(2, t36);
            let r37 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r37);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d0790); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d0791); };
            c.w64(3, t38);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008d07a2, 0x70008d079d);
        }
        41 => {
            return RegfileExit::fault(0x70008d07a2);
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008d07a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008d07a7); }
            c.w64(0, c.r64(8));
            return c.call(0x70008a1214, 0x70008d07b3, 0x70008d07ae);
        }
        43 => {
            c.w64(8, c.r64(0));
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d07b6); };
            c.w64(9, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d07ba); };
            c.w64(6, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d07be); };
            c.w64(2, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d07c2); };
            c.w64(11, t42);
            blk = 8; continue;
        }
        44 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008d07cb); }
            c.w64(0, c.r64(11));
            return c.call(0x700089c794, 0x70008d07d7, 0x70008d07d2);
        }
        45 => {
            c.w64(11, c.r64(0));
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d07da); };
            c.w64(9, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d07de); };
            c.w64(8, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d07e2); };
            c.w64(6, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d07e6); };
            c.w64(2, t46);
            blk = 10; continue;
        }
        46 => {
            return RegfileExit::jump(0x70008d07f0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d07f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d07f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d07f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d07f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d07fa); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d07ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d0809); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d0815); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d081b); };
            c.cmp(t5, 0x40u64, 16);
            if c.cc_ae() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d0826); };
            c.w32(1, t6);
            let r7 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r7);
            let Some(t8) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d082c); };
            c.w32(1, t8);
            let r9 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r9);
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d0832); };
            c.w32(7, t10);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x58u64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008d0841);
        }
        5 => {
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0x167)) else { return RegfileExit::fault(0x70008d0842); };
            c.w32(1, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d0848); };
            c.w64(7, t12);
            let Some(t13) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008d084f); };
            c.w64(8, t13);
            c.w64(9, c.r64(8).wrapping_add(0x14));
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d0856); };
            c.w64(11, t14);
            let Some(t15) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008d085d); };
            c.cmp(t15, c.r64(9), 64);
            if c.cc_be() { blk = 19; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008d0866); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008d086d); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008d0870); };
            c.w64(1, t16);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008d0877); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008d087a); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008d087d); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008d0882); }
            c.w64(0, c.r64(7));
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008d0888); };
            c.w64(1, t17);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d088f); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x8, 0x70008d0896);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d0899); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d08a0); }
            return c.ret(0x0, 0x70008d08a2);
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008d08a3); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d08a7); };
            c.w64(3, t18);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008d08b5, 0x70008d08b0);
        }
        11 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d08b5); };
            c.w64(6, t19);
            blk = 1; continue;
        }
        12 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d08be); };
            c.cmp(t20, c.r32(0), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d08cb); };
            c.cmp(t21, c.r32(0), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008d08da); }
            let r22 = c.r32(1) | c.r32(7);
            c.w32(1, r22);
            c.logic(r22, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            return c.call(0x700089c794, 0x70008d08e7, 0x70008d08e2);
        }
        14 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d08e7); };
            c.w64(6, t23);
            blk = 3; continue;
        }
        15 => {
            c.w32(1, 0x36u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d08f5); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d08f6); };
            c.w64(1, t24);
            let Some(t25) = c.ld32(c.r64(1).wrapping_add(0x363)) else { return RegfileExit::fault(0x70008d08fd); };
            c.w32(1, t25);
            let r26 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r26);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d0906); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d0907); };
            c.w64(3, t27);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008d0918, 0x70008d0913);
        }
        16 => {
            return RegfileExit::fault(0x70008d0918);
        }
        17 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008d0919); }
            return c.call(0x70008a1214, 0x70008d0922, 0x70008d091d);
        }
        18 => {
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d0922); };
            c.w64(6, t28);
            blk = 3; continue;
        }
        19 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d0930); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008d0934); }
            return c.call(0x70007febb4, 0x70008d093d, 0x70008d0938);
        }
        20 => {
            c.w64(7, c.r64(0));
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d0940); };
            c.w64(0, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d0944); };
            c.w64(1, t30);
            blk = 7; continue;
        }
        21 => {
            return RegfileExit::jump(0x70008d0950);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d0954(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d0954); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d0958); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d0959); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d095a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d095f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 73; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d0969); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 71; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d0976); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d097c); };
            c.cmp(t5, 0x840u64, 16);
            if c.cc_ne() { blk = 71; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d0988); };
            c.w32(7, t6);
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d098b); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d0995); };
            c.w32(9, t9);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(9), c.r64(7), 64);
            if c.cc_le() { blk = 66; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d09a5); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d09ac); };
            c.w32(11, t12);
            c.logic(c.r8(11) & 0x7u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(15, c.r64(8));
            c.w64(0, c.r64(11));
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(11));
            blk = 6; continue;
        }
        6 => {
            c.w64(3, c.r64(0));
            let r13 = c.r32(3) & 0x7u64;
            c.w32(3, r13);
            c.logic(r13, 32);
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_l() { blk = 11; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 75; continue;
        }
        11 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 75; continue;
        }
        14 => {
            let Some(t14) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d09f9); };
            c.w32(0, t14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d09fd); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_ne() { blk = 76; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t16) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d0a0a); };
            c.w32(15, t16);
            let r17 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r17);
            let Some(t18) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d0a11); };
            c.w32(0, t18);
            let r19 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r19);
            let Some(t20) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d0a18); };
            c.w32(0, t20);
            blk = 6; continue;
        }
        16 => {
            let Some(t21) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d0a1e); };
            c.w32(0, t21);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r22 = c.add(c.r64(11), c.r64(0), 64);
            c.w64(11, r22);
            let Some(t23) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d0a2b); };
            c.w32(15, t23);
            let r24 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r24);
            let Some(t25) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d0a32); };
            c.w32(0, t25);
            let r26 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r26);
            let Some(t27) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d0a39); };
            c.w32(0, t27);
            blk = 6; continue;
        }
        17 => {
            let Some(t28) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d0a3f); };
            c.w32(15, t28);
            let r29 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r29);
            let Some(t30) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d0a46); };
            c.w32(0, t30);
            let r31 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r31);
            let Some(t32) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d0a4d); };
            c.w32(0, t32);
            blk = 6; continue;
        }
        18 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(11, c.r64(10));
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(10));
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 21; continue;
        }
        19 => {
            c.w32(3, 0x1u64);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(11, c.r64(10));
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 21; continue;
        }
        20 => {
            c.w32(12, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(8));
            c.w32(3, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(12));
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 77; continue; }
            blk = 22; continue;
        }
        22 => {
            let r33 = c.add(c.r64(0), 0xbu64, 64);
            c.w64(0, r33);
            blk = 23; continue;
        }
        23 => {
            let r34 = c.add(c.r64(15), c.r64(7), 64);
            c.w64(15, r34);
            let r35 = c.r32(11) & 0x8u64;
            c.w32(11, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t36) = c.ld16(c.r64(0).wrapping_add(c.r64(15).wrapping_mul(2))) else { return RegfileExit::fault(0x70008d0aaa); };
            c.w32(11, t36);
            blk = 26; continue;
        }
        25 => {
            let Some(t37) = c.ld8(c.r64(15).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70008d0ab1); };
            c.w32(11, t37);
            blk = 26; continue;
        }
        26 => {
            c.w64(15, c.r64(11));
            let r38 = c.r32(15) & 0xfc00u64;
            c.w32(15, r38);
            c.logic(r38, 32);
            c.cmp(c.r32(15), 0xd800u64, 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(15, c.r64(7).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_ge() { blk = 52; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t39) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d0ada); };
            c.w32(9, t39);
            let r40 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r40);
            let Some(t41) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d0ae1); };
            c.w32(9, t41);
            c.logic(c.r8(9) & 0x7u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(3, c.r64(9));
            c.w64(9, c.r64(12));
            c.w64(0, c.r64(8));
            blk = 30; continue;
        }
        30 => {
            c.w64(2, c.r64(3));
            let r42 = c.r32(2) & 0x7u64;
            c.w32(2, r42);
            c.logic(r42, 32);
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_l() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 92; continue;
        }
        35 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(2), 0x1u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            blk = 92; continue;
        }
        38 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d0b2b); };
            c.w32(3, t43);
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d0b2e); };
            c.cmp(t44, c.r32(3), 32);
            if c.cc_ne() { blk = 93; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d0b3b); };
            c.w32(0, t45);
            let r46 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r46);
            let Some(t47) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d0b41); };
            c.w32(3, t47);
            let r48 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r48);
            let Some(t49) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d0b47); };
            c.w32(3, t49);
            blk = 30; continue;
        }
        40 => {
            let Some(t50) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d0b4d); };
            c.w32(3, t50);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r51 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r51);
            let Some(t52) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d0b59); };
            c.w32(0, t52);
            let r53 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r53);
            let Some(t54) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d0b5f); };
            c.w32(3, t54);
            let r55 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r55);
            let Some(t56) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d0b65); };
            c.w32(3, t56);
            blk = 30; continue;
        }
        41 => {
            let Some(t57) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d0b6b); };
            c.w32(0, t57);
            let r58 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r58);
            let Some(t59) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d0b71); };
            c.w32(3, t59);
            let r60 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r60);
            let Some(t61) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d0b77); };
            c.w32(3, t61);
            blk = 30; continue;
        }
        42 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(9));
            c.w64(9, c.r64(10));
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(10));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 45; continue;
        }
        43 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(9));
            c.w64(9, c.r64(10));
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(10));
            c.w32(2, 0x1u64);
            blk = 45; continue;
        }
        44 => {
            c.w64(0, c.r64(12));
            c.w64(3, c.r64(8));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(8, c.r64(3).wrapping_add(0xb));
            blk = 47; continue;
        }
        47 => {
            c.w64(12, c.r64(15).wrapping_add(c.r64(0)));
            let r62 = c.r32(9) & 0x8u64;
            c.w32(9, r62);
            c.logic(r62, 32);
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t63) = c.ld16(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(2))) else { return RegfileExit::fault(0x70008d0bd2); };
            c.w32(8, t63);
            blk = 50; continue;
        }
        49 => {
            let Some(t64) = c.ld8(c.r64(12).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008d0bd9); };
            c.w32(8, t64);
            blk = 50; continue;
        }
        50 => {
            c.w64(9, c.r64(8));
            let r65 = c.r32(9) & 0xfc00u64;
            c.w32(9, r65);
            c.logic(r65, 32);
            c.cmp(c.r32(9), 0xdc00u64, 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            let r66 = c.shl(c.r32(8), 0x10u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r66);
            let r67 = c.r32(8) | c.r32(11);
            c.w32(8, r67);
            c.logic(r67, 32);
            c.w64(11, c.r64(8));
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(11), 0x10000u64, 32);
            if c.cc_b() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d0c04); };
            c.w64(8, t68);
            let Some(t69) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008d0c0b); };
            c.w64(9, t69);
            c.w64(12, c.r64(9).wrapping_add(0x10));
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d0c12); };
            c.w64(15, t70);
            let Some(t71) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008d0c19); };
            c.cmp(t71, c.r64(12), 64);
            if c.cc_be() { blk = 109; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008d0c22); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 55; continue;
        }
        55 => {
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d0c29); };
            c.w64(9, t72);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008d0c30); }
            if !c.st32(c.r64(8).wrapping_add(0x7), 0x2u64) { return RegfileExit::fault(0x70008d0c34); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d0c3c); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008d0c44); }
            c.w64(0, c.r64(8));
            blk = 58; continue;
        }
        56 => {
            c.cmp(c.r32(11), 0xffu64, 32);
            if c.cc_g() { blk = 111; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008d0c62); };
            c.w64(9, t73);
            let Some(t74) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d0c69); };
            c.w32(0, t74);
            let r75 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r75);
            let Some(t76) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d0c71); };
            c.cmp(t76, c.r32(0), 32);
            if c.cc_e() { blk = 116; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t77) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d0c7e); };
            c.w32(8, t77);
            let r78 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r78);
            let r79 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r79);
            if !c.st32(c.r64(1).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008d0c87); }
            let Some(t80) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d0c8a); };
            c.w32(1, t80);
            let r81 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r81);
            let Some(t82) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d0c90); };
            c.w32(1, t82);
            let r83 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r83);
            let Some(t84) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d0c96); };
            c.w32(7, t84);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x68u64, 64);
            if c.cc_a() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            return RegfileExit::fault(0x70008d0ca5);
        }
        60 => {
            let Some(t85) = c.ld32(c.r64(1).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x70008d0ca6); };
            c.w32(1, t85);
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d0cac); };
            c.w64(7, t86);
            let Some(t87) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008d0cb3); };
            c.w64(8, t87);
            c.w64(9, c.r64(8).wrapping_add(0x14));
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d0cba); };
            c.w64(11, t88);
            let Some(t89) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008d0cc1); };
            c.cmp(t89, c.r64(9), 64);
            if c.cc_be() { blk = 125; continue; }
            blk = 61; continue;
        }
        61 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008d0cca); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 62; continue;
        }
        62 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008d0cd1); }
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008d0cd4); };
            c.w64(1, t90);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008d0cdb); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008d0cde); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008d0ce1); }
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008d0ce4); };
            c.w64(1, t91);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008d0ceb); }
            c.w64(0, c.r64(7));
            blk = 63; continue;
        }
        63 => {
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008d0cf1); };
            c.w64(1, t92);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d0cf8); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            return c.ret(0x8, 0x70008d0cff);
        }
        65 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d0d02); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d0d09); }
            return c.ret(0x0, 0x70008d0d0b);
        }
        66 => {
            let Some(t93) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d0d0c); };
            c.w32(1, t93);
            let r94 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r94);
            let Some(t95) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d0d12); };
            c.w32(1, t95);
            let r96 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r96);
            let Some(t97) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d0d18); };
            c.w32(7, t97);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x68u64, 64);
            if c.cc_a() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            return RegfileExit::fault(0x70008d0d27);
        }
        68 => {
            let Some(t98) = c.ld32(c.r64(1).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x70008d0d28); };
            c.w32(1, t98);
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d0d2e); };
            c.w64(7, t99);
            let Some(t100) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008d0d35); };
            c.w64(8, t100);
            c.w64(9, c.r64(8).wrapping_add(0x14));
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d0d3c); };
            c.w64(11, t101);
            let Some(t102) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008d0d43); };
            c.cmp(t102, c.r64(9), 64);
            if c.cc_be() { blk = 127; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008d0d4c); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 70; continue;
        }
        70 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008d0d53); }
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008d0d56); };
            c.w64(1, t103);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008d0d5d); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008d0d60); }
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d0d63); };
            c.w64(1, t104);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008d0d6a); }
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008d0d6d); };
            c.w64(1, t105);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008d0d74); }
            c.w64(0, c.r64(7));
            blk = 63; continue;
        }
        71 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d0d84); }
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d0d85); };
            c.w64(7, t106);
            let Some(t107) = c.ld32(c.r64(7).wrapping_add(0x367)) else { return RegfileExit::fault(0x70008d0d8c); };
            c.w32(7, t107);
            let r108 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r108);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d0d95); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d0d96); }
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d0d97); };
            c.w64(3, t109);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008d0da8, 0x70008d0da3);
        }
        72 => {
            return RegfileExit::fault(0x70008d0da8);
        }
        73 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d0da9); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d0daf); };
            c.w64(3, t110);
            return c.call(0x700086ed74, 0x70008d0dbb, 0x70008d0db6);
        }
        74 => {
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d0dbb); };
            c.w64(6, t111);
            blk = 1; continue;
        }
        75 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d0dc4); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008d0dc8); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008d0dcc); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008d0dd0); }
            blk = 79; continue;
        }
        76 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d0dd6); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008d0dda); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008d0dde); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008d0de2); }
            blk = 79; continue;
        }
        77 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d0de8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008d0dec); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008d0df0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008d0df4); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t112) = c.ld64(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d0dfe); };
            c.w64(0, t112);
            blk = 23; continue;
        }
        79 => {
            c.cmp(c.r64(7), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 81; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w32(11, c.r64(7).wrapping_add(c.r64(7)));
            blk = 90; continue;
        }
        81 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d0e19); };
            c.w64(11, t113);
            let Some(t114) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008d0e20); };
            c.w64(15, t114);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d0e27); };
            c.w64(3, t115);
            let Some(t116) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008d0e2e); };
            c.cmp(t116, c.r64(0), 64);
            if c.cc_a() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008d0e3d, 0x70008d0e38);
        }
        83 => {
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d0e3d); };
            c.w64(9, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d0e41); };
            c.w64(1, t118);
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d0e45); };
            c.w64(6, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d0e49); };
            c.w64(7, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d0e4d); };
            c.w64(8, t121);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 85; continue;
        }
        84 => {
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x70008d0e56); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 85; continue;
        }
        85 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(7) as i64));
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 89; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w64(10, c.r64(7));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 88; continue; }
            blk = 87; continue;
        }
        87 => {
            let r122 = c.r64(10) | 0x1u64;
            c.w64(10, r122);
            c.logic(r122, 64);
            blk = 88; continue;
        }
        88 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 89; continue;
        }
        89 => {
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008d0e87); };
            c.w64(11, t123);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008d0e8e); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008d0e92); }
            c.w64(11, c.r64(0));
            blk = 90; continue;
        }
        90 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008d0e9a); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008d0e9c); }
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008d0e9e); };
            c.w64(3, t124);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x70008d0eb2, 0x70008d0ead);
        }
        91 => {
            let r125 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r125);
            c.w64(11, c.r64(0));
            let Some(t126) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d0eb7); };
            c.w64(1, t126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d0ebb); };
            c.w64(6, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d0ebf); };
            c.w64(7, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d0ec3); };
            c.w64(8, t129);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d0eca); };
            c.w64(9, t130);
            blk = 26; continue;
        }
        92 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d0ed3); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008d0ed7); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008d0edb); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008d0edf); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008d0ee3); }
            blk = 96; continue;
        }
        93 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d0ee9); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008d0eed); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008d0ef1); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008d0ef5); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008d0ef9); }
            blk = 96; continue;
        }
        94 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d0eff); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008d0f03); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008d0f07); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008d0f0b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008d0f0f); }
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t131) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d0f19); };
            c.w64(8, t131);
            blk = 47; continue;
        }
        96 => {
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 98; continue; }
            blk = 97; continue;
        }
        97 => {
            c.w32(9, c.r64(15).wrapping_add(c.r64(15)));
            blk = 107; continue;
        }
        98 => {
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d0f34); };
            c.w64(9, t132);
            let Some(t133) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d0f3b); };
            c.w64(0, t133);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d0f42); };
            c.w64(2, t134);
            let Some(t135) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008d0f49); };
            c.cmp(t135, c.r64(3), 64);
            if c.cc_a() { blk = 101; continue; }
            blk = 99; continue;
        }
        99 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008d0f58, 0x70008d0f53);
        }
        100 => {
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t136) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d0f5b); };
            c.w64(1, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d0f5f); };
            c.w64(6, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d0f63); };
            c.w64(7, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d0f67); };
            c.w64(11, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d0f6b); };
            c.w64(8, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d0f6f); };
            c.w64(15, t141);
            blk = 102; continue;
        }
        101 => {
            if !c.st64(c.r64(9), c.r64(3)) { return RegfileExit::fault(0x70008d0f75); }
            c.w64(9, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 102; continue;
        }
        102 => {
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008d0f7f); };
            c.w64(9, t142);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008d0f86); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(15) as i64));
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 106; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w64(10, c.r64(15));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            let r143 = c.r64(10) | 0x1u64;
            c.w64(10, r143);
            c.logic(r143, 64);
            blk = 105; continue;
        }
        105 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 106; continue;
        }
        106 => {
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008d0fb1); }
            c.w64(9, c.r64(0));
            blk = 107; continue;
        }
        107 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008d0fb9); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008d0fbb); }
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008d0fbd); };
            c.w64(3, t144);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x70008d0fd1, 0x70008d0fcc);
        }
        108 => {
            let r145 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r145);
            c.w64(8, c.r64(0));
            let Some(t146) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d0fd6); };
            c.w64(1, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d0fda); };
            c.w64(6, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d0fde); };
            c.w64(7, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d0fe2); };
            c.w64(11, t149);
            blk = 50; continue;
        }
        109 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d0ff0); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008d0ff4); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008d0ff8); }
            return c.call(0x70007febf4, 0x70008d1001, 0x70008d0ffc);
        }
        110 => {
            c.w64(8, c.r64(0));
            let Some(t150) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d1004); };
            c.w64(1, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1008); };
            c.w64(6, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d100c); };
            c.w64(7, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d1010); };
            c.w64(11, t153);
            blk = 55; continue;
        }
        111 => {
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d1019); };
            c.w64(8, t154);
            let Some(t155) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008d1020); };
            c.w64(9, t155);
            c.w64(12, c.r64(9).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d1027); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008d102b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008d102f); }
            let Some(t156) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d1033); };
            c.w64(15, t156);
            let Some(t157) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008d103a); };
            c.cmp(t157, c.r64(12), 64);
            if c.cc_a() { blk = 114; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008d1049, 0x70008d1044);
        }
        113 => {
            blk = 115; continue;
        }
        114 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008d104b); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 115; continue;
        }
        115 => {
            let Some(t158) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d1055); };
            c.w64(1, t158);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008d105c); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008d105f); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d1066); }
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d106d); };
            c.w64(11, t159);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(11)) { return RegfileExit::fault(0x70008d1071); }
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d1076); };
            c.w64(7, t160);
            let Some(t161) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d107a); };
            c.w64(1, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d107e); };
            c.w64(6, t162);
            blk = 58; continue;
        }
        116 => {
            let Some(t163) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d1087); };
            c.w64(12, t163);
            let Some(t164) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008d108e); };
            c.w64(15, t164);
            c.w64(0, c.r64(15).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d1096); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008d109a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008d109e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008d10a2); }
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d10a6); };
            c.w64(3, t165);
            let Some(t166) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008d10ad); };
            c.cmp(t166, c.r64(0), 64);
            if c.cc_a() { blk = 119; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008d10bc, 0x70008d10b7);
        }
        118 => {
            blk = 120; continue;
        }
        119 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008d10be); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(12));
            blk = 120; continue;
        }
        120 => {
            let Some(t167) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d10c9); };
            c.w64(1, t167);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008d10d0); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008d10d3); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d10da); }
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d10e1); };
            c.w64(11, t168);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(11)) { return RegfileExit::fault(0x70008d10e5); }
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008d10e9); };
            c.w64(9, t169);
            let Some(t170) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d10f0); };
            c.w32(1, t170);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_b() { blk = 122; continue; }
            blk = 121; continue;
        }
        121 => {
            return RegfileExit::fault(0x70008d10ff);
        }
        122 => {
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d1100); };
            c.w64(8, t171);
            if !c.st32(c.r64(9).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x70008d1104); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 124; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r172 = c.r64(1) & c.r64(9);
            c.w64(1, r172);
            c.logic(r172, 64);
            let Some(t173) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d1116); };
            c.logic(t173 & 0x4u64, 8);
            if c.cc_ne() { blk = 129; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d111c); };
            c.w64(7, t174);
            let Some(t175) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d1120); };
            c.w64(1, t175);
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1124); };
            c.w64(6, t176);
            blk = 58; continue;
        }
        125 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008d1132); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008d1136); }
            return c.call(0x70007febf4, 0x70008d113f, 0x70008d113a);
        }
        126 => {
            c.w64(7, c.r64(0));
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d1142); };
            c.w64(0, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1146); };
            c.w64(1, t178);
            blk = 62; continue;
        }
        127 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008d1154); }
            return c.call(0x70007febf4, 0x70008d115d, 0x70008d1158);
        }
        128 => {
            c.w64(7, c.r64(0));
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1160); };
            c.w64(1, t179);
            blk = 70; continue;
        }
        129 => {
            c.w32(0, c.r32(0));
            let r180 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r180);
            c.w64(1, 0xfffffffffffc0000u64);
            let r181 = c.r64(1) & c.r64(0);
            c.w64(1, r181);
            c.logic(r181, 64);
            let Some(t182) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d1178); };
            c.logic(t182 & 0x2u64, 8);
            if c.cc_e() { blk = 124; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(7, c.r64(9).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d1182); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008d1183); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x70008d1190, 0x70008d118b);
        }
        131 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008d1190); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008d1192); }
            blk = 124; continue;
        }
        132 => {
            return RegfileExit::jump(0x70008d1198);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d11b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d11b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d11b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d11b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d11ba); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d11bf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 23; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d11c9); };
            c.w64(3, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d11cd); };
            c.cmp(t3, c.r32(3), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d11da); };
            c.cmp(t4, c.r32(3), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let r5 = c.r32(1) | c.r32(8);
            c.w32(1, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d1204); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d120b); };
            c.cmp(t8, 0x3fu64, 16);
            if c.cc_a() { blk = 25; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d1217); };
            c.w64(0, t9);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d1223); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008d122d); };
            c.w64(11, t12);
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d1234); };
            c.cmp(t13, c.r32(9), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d123e); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d1245); };
            c.w32(11, t16);
            let r17 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r17);
            let Some(t18) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d124c); };
            c.w32(12, t18);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x9au64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x70008d125f);
        }
        8 => {
            let Some(t19) = c.ld32(c.r64(11).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008d1260); };
            c.w32(15, t19);
            let r20 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r20);
            let Some(t21) = c.ld32(c.r64(15).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008d126a); };
            c.cmp(t21, c.r32(8), 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008d1270); };
            c.w32(15, t22);
            c.logic(c.r32(15) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(12), 0x9fu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008d1286);
        }
        12 => {
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d1287); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            let Some(t25) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d128e); };
            c.w32(8, t25);
            let r26 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r26);
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008d1295); };
            c.w32(12, t27);
            let r28 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r28);
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0x283)) else { return RegfileExit::fault(0x70008d129c); };
            c.cmp(t29, c.r32(8), 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t30) = c.ld32(c.r64(12).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008d12a5); };
            c.w32(8, t30);
            let Some(t31) = c.ld32(c.r64(12).wrapping_add(0xbb)) else { return RegfileExit::fault(0x70008d12aa); };
            c.w32(11, t31);
            let r32 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r32);
            let r33 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r33);
            let r34 = c.r32(11) & c.r32(8);
            c.w32(11, r34);
            c.logic(r34, 32);
            let r35 = c.r32(11) & 0x4u64;
            c.w32(11, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(11, 0x1u64);
            blk = 16; continue;
        }
        15 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 16; continue;
        }
        16 => {
            c.w32(8, 0x1u64);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            return c.call(0x70008c5034, 0x70008d12e1, 0x70008d12dc);
        }
        18 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008d12e1); };
            c.w64(1, t36);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d12e8); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return c.ret(0x10, 0x70008d12ef);
        }
        20 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d12f2); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d12f9); }
            return c.ret(0x0, 0x70008d12fb);
        }
        21 => {
            c.w32(1, 0x36u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d1301); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d1302); };
            c.w64(1, t37);
            let Some(t38) = c.ld32(c.r64(1).wrapping_add(0x36b)) else { return RegfileExit::fault(0x70008d1309); };
            c.w32(1, t38);
            let r39 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r39);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d1312); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d1313); };
            c.w64(3, t40);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008d1324, 0x70008d131f);
        }
        22 => {
            return RegfileExit::fault(0x70008d1324);
        }
        23 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d1325); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d1329); };
            c.w64(3, t41);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008d1337, 0x70008d1332);
        }
        24 => {
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1337); };
            c.w64(6, t42);
            blk = 1; continue;
        }
        25 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d1340); };
            c.w64(0, t43);
            blk = 26; continue;
        }
        26 => {
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d1344); };
            c.cmp(t44, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d1353); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008d1357); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008d135b); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d1369); };
            c.cmp(t45, c.r32(0), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xcd0)) else { return RegfileExit::fault(0x70008d1376); };
            c.w64(3, t46);
            c.w64(9, c.r64(3));
            return c.call(0x7000870914, 0x70008d1385, 0x70008d1380);
        }
        29 => {
            let Some(t47) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d1385); };
            c.cmp(t47, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t48) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d138e); };
            c.cmp(t48, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 31; continue;
        }
        31 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d139b); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008d13a1); };
            c.logic(t51 & 0x2u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d13a7); };
            if !c.push64(t52) { return RegfileExit::fault(0x70008d13a7); }
            let Some(t53) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d13aa); };
            if !c.push64(t53) { return RegfileExit::fault(0x70008d13aa); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d13b5); };
            c.w64(6, t54);
            return c.call(0x70007f2274, 0x70008d13be, 0x70008d13b9);
        }
        34 => {
            blk = 18; continue;
        }
        35 => {
            c.w32(1, 0xf6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d13c8); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d13c9); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0xcd0)) else { return RegfileExit::fault(0x70008d13ca); };
            c.w64(1, t55);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d13d1); }
            let Some(t56) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d13d2); };
            if !c.push64(t56) { return RegfileExit::fault(0x70008d13d2); }
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d13d5); };
            c.w64(3, t57);
            c.w32(0, 0x4u64);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d13e1); };
            c.w64(6, t58);
            return c.call(0x700086ed74, 0x70008d13ea, 0x70008d13e5);
        }
        36 => {
            return RegfileExit::fault(0x70008d13ea);
        }
        37 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d13eb); };
            c.w64(6, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d13ef); };
            c.w64(0, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d13f3); };
            c.w64(8, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d13f7); };
            c.w64(3, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d13fb); };
            c.w64(1, t63);
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t64) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d1404); };
            c.w32(1, t64);
            let r65 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r65);
            let Some(t66) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d140a); };
            c.cmp(t66, 0x40u64, 16);
            if c.cc_ae() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(1, c.r64(3));
            blk = 43; continue;
        }
        41 => {
            c.w64(0, c.r64(3));
            return c.call(0x700089c794, 0x70008d141e, 0x70008d1419);
        }
        42 => {
            c.w64(1, c.r64(0));
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1421); };
            c.w64(6, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d1425); };
            c.w64(0, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d1429); };
            c.w64(8, t69);
            blk = 43; continue;
        }
        43 => {
            let Some(t70) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d142d); };
            c.w32(9, t70);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008d1437); }
            c.cmp(c.r64(9), 0x9au64, 64);
            if c.cc_a() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            return RegfileExit::fault(0x70008d1444);
        }
        45 => {
            let Some(t71) = c.ld32(c.r64(6).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008d1445); };
            c.w32(9, t71);
            let r72 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r72);
            let Some(t73) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008d144f); };
            c.w32(9, t73);
            let r74 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r74);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008d1456); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 46; continue;
        }
        46 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t75) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d1464); };
            c.w32(8, t75);
            let r76 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r76);
            let Some(t77) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d146b); };
            c.cmp(t77, 0x40u64, 16);
            if c.cc_b() { blk = 51; continue; }
            blk = 48; continue;
        }
        48 => {
            return c.call(0x700089c794, 0x70008d1478, 0x70008d1473);
        }
        49 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d1478); };
            c.w64(9, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d147c); };
            c.w64(6, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d1480); };
            c.w64(1, t80);
            blk = 51; continue;
        }
        50 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d1486); };
            c.w64(0, t81);
            blk = 51; continue;
        }
        51 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008d148d); }
            c.w64(8, c.r64(9));
            let Some(t82) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d1494); };
            c.logic(t82 & 0x200000u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d149e); };
            c.w64(15, t83);
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008d14a5); };
            c.w64(11, t84);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d14af); };
            c.w64(7, t85);
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d14b6); };
            c.w64(9, t86);
            blk = 72; continue;
        }
        53 => {
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d14c2); };
            c.w64(9, t87);
            let Some(t88) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d14c9); };
            c.w64(11, t88);
            c.w64(12, c.r64(11).wrapping_add(0x4c));
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d14d0); };
            c.w64(15, t89);
            let Some(t90) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008d14d7); };
            c.cmp(t90, c.r64(12), 64);
            if c.cc_a() { blk = 56; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w32(2, 0x4cu64);
            return c.call(0x70007febf4, 0x70008d14e6, 0x70008d14e1);
        }
        55 => {
            c.w64(11, c.r64(0));
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d14e9); };
            c.w64(6, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d14ed); };
            c.w64(1, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d14f1); };
            c.w64(0, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d14f5); };
            c.w64(8, t94);
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d14f9); };
            c.w64(9, t95);
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d1500); };
            c.w64(15, t96);
            blk = 57; continue;
        }
        56 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008d1509); }
            let r97 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r97);
            blk = 57; continue;
        }
        57 => {
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008d1510); };
            c.w64(12, t98);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008d1517); }
            c.w32(12, 0x22u64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008d1521); }
            let Some(t99) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d1525); };
            c.w32(12, t99);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            return RegfileExit::fault(0x70008d1535);
        }
        59 => {
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008d1539); }
            let Some(t100) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d153d); };
            c.w32(3, t100);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_a() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            return RegfileExit::fault(0x70008d154d);
        }
        61 => {
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70008d154e); }
            let Some(t101) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d1552); };
            c.w32(3, t101);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x2u64, 64);
            if c.cc_a() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            return RegfileExit::fault(0x70008d1562);
        }
        63 => {
            c.w32(3, 0x8u64);
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(3)) { return RegfileExit::fault(0x70008d1568); }
            let Some(t102) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d156c); };
            c.w32(3, t102);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x3u64, 64);
            if c.cc_a() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            return RegfileExit::fault(0x70008d157c);
        }
        65 => {
            c.w32(3, 0x2u64);
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(3)) { return RegfileExit::fault(0x70008d1582); }
            let Some(t103) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d1586); };
            c.w32(3, t103);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x4u64, 64);
            if c.cc_a() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            return RegfileExit::fault(0x70008d1596);
        }
        67 => {
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x70008d1597); }
            c.w64(3, c.r64(11).wrapping_add(0x4b));
            c.w64(2, c.r64(11).wrapping_add(0x1b));
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_e() { blk = 71; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d15a8); };
            c.w64(7, t104);
            blk = 69; continue;
        }
        69 => {
            if !c.st32(c.r64(2), c.r32(7)) { return RegfileExit::fault(0x70008d15af); }
            let r105 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r105);
            c.cmp(c.r64(3), c.r64(2), 64);
            if c.cc_ne() { blk = 69; continue; }
            blk = 70; continue;
        }
        70 => {
            blk = 72; continue;
        }
        71 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d15bc); };
            c.w64(7, t106);
            blk = 72; continue;
        }
        72 => {
            let Some(t107) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d15c3); };
            c.w32(3, t107);
            c.w64(2, c.r64(3).wrapping_mul(4));
            let Some(t108) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d15d0); };
            c.w64(3, t108);
            c.w64(9, c.r64(2).wrapping_add(c.r64(3)));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008d15d7); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008d15db); }
            let Some(t109) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008d15df); };
            c.cmp(t109, c.r64(9), 64);
            if c.cc_a() { blk = 75; continue; }
            blk = 73; continue;
        }
        73 => {
            return c.call(0x70007febf4, 0x70008d15e9, 0x70008d15e4);
        }
        74 => {
            c.w64(9, c.r64(0));
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d15ec); };
            c.w64(6, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d15f0); };
            c.w64(1, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d15f4); };
            c.w64(0, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d15f8); };
            c.w64(8, t113);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d15ff); };
            c.w64(7, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d1606); };
            c.w64(11, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d160a); };
            c.w64(2, t116);
            blk = 76; continue;
        }
        75 => {
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d1610); };
            c.w64(15, t117);
            if !c.st64(c.r64(15), c.r64(9)) { return RegfileExit::fault(0x70008d1617); }
            c.w64(9, c.r64(3).wrapping_add(0x1));
            blk = 76; continue;
        }
        76 => {
            c.w64(15, c.r64(2));
            let r118 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r118);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008d1625); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008d1629); }
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008d162d); };
            c.w64(11, t119);
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008d1634); }
            let Some(t120) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d1638); };
            c.w32(11, t120);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008d163c); }
            c.w64(3, c.r64(11));
            let r121 = c.r32(3) & 0xe0000000u64;
            c.w32(3, r121);
            c.logic(r121, 32);
            if c.cc_ne() { blk = 81; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_e() { blk = 89; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(2, c.r64(7));
            blk = 79; continue;
        }
        79 => {
            let r122 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r122);
            if !c.st32(c.r64(15).wrapping_add(c.r64(9)), c.r32(2)) { return RegfileExit::fault(0x70008d165c); }
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_ne() { blk = 79; continue; }
            blk = 80; continue;
        }
        80 => {
            blk = 89; continue;
        }
        81 => {
            let r123 = c.sub(c.r32(11), 0x20000000u64, 32);
            c.w32(11, r123);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008d166f); }
            let Some(t124) = c.ld8(c.r64(8).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008d1673); };
            c.w32(11, t124);
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_e() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x70008d1685); };
            c.w64(2, t125);
            blk = 83; continue;
        }
        83 => {
            let r126 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r126);
            if !c.st32(c.r64(15).wrapping_add(c.r64(9)), c.r32(2)) { return RegfileExit::fault(0x70008d1690); }
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_ne() { blk = 83; continue; }
            blk = 84; continue;
        }
        84 => {
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            let r127 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r127);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(7)) { return RegfileExit::fault(0x70008d16a3); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 85; continue; }
            blk = 86; continue;
        }
        86 => {
            let r128 = c.add(c.r32(3), 0xe0000000u64, 32);
            c.w32(3, r128);
            if c.cc_ne() { blk = 89; continue; }
            blk = 87; continue;
        }
        87 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008d16b5); }
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x70008d16b7); };
            c.w64(3, t129);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x70008d16cb, 0x70008d16c6);
        }
        88 => {
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d16cb); };
            c.w64(6, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d16cf); };
            c.w64(9, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d16d3); };
            c.w64(0, t132);
            blk = 89; continue;
        }
        89 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008d16d7); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d16d9); }
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d16da); };
            c.w64(8, t133);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008d16e1); }
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0x2670)) else { return RegfileExit::fault(0x70008d16e3); };
            c.w64(3, t134);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008d16f4, 0x70008d16ef);
        }
        90 => {
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d16f4); };
            c.w64(6, t135);
            let Some(t136) = c.ld64(c.r64(13).wrapping_add(0xcd0)) else { return RegfileExit::fault(0x70008d16f8); };
            c.w64(3, t136);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008d16ff); }
            return c.call(0x7000870914, 0x70008d1708, 0x70008d1703);
        }
        91 => {
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d1708); };
            if !c.push64(t137) { return RegfileExit::fault(0x70008d1708); }
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d170b); };
            if !c.push64(t138) { return RegfileExit::fault(0x70008d170b); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1716); };
            c.w64(6, t139);
            return c.call(0x70007f2274, 0x70008d171f, 0x70008d171a);
        }
        92 => {
            blk = 18; continue;
        }
        93 => {
            return RegfileExit::jump(0x70008d1728);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d1734(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d1734); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d1738); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d1739); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d173a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d173f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 23; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d1749); };
            c.w64(3, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d174d); };
            c.cmp(t3, c.r32(3), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d175a); };
            c.cmp(t4, c.r32(3), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let r5 = c.r32(1) | c.r32(8);
            c.w32(1, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d1784); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d178b); };
            c.cmp(t8, 0x3fu64, 16);
            if c.cc_a() { blk = 25; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d1797); };
            c.w64(0, t9);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d17a3); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008d17ad); };
            c.w64(11, t12);
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d17b4); };
            c.cmp(t13, c.r32(9), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d17be); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d17c5); };
            c.w32(11, t16);
            let r17 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r17);
            let Some(t18) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d17cc); };
            c.w32(12, t18);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x9au64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x70008d17df);
        }
        8 => {
            let Some(t19) = c.ld32(c.r64(11).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008d17e0); };
            c.w32(15, t19);
            let r20 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r20);
            let Some(t21) = c.ld32(c.r64(15).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008d17ea); };
            c.cmp(t21, c.r32(8), 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008d17f0); };
            c.w32(15, t22);
            c.logic(c.r32(15) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(12), 0x9fu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008d1806);
        }
        12 => {
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d1807); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            let Some(t25) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d180e); };
            c.w32(8, t25);
            let r26 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r26);
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008d1815); };
            c.w32(12, t27);
            let r28 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r28);
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0x283)) else { return RegfileExit::fault(0x70008d181c); };
            c.cmp(t29, c.r32(8), 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t30) = c.ld32(c.r64(12).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008d1825); };
            c.w32(8, t30);
            let Some(t31) = c.ld32(c.r64(12).wrapping_add(0xdf)) else { return RegfileExit::fault(0x70008d182a); };
            c.w32(11, t31);
            let r32 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r32);
            let r33 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r33);
            let r34 = c.r32(11) & c.r32(8);
            c.w32(11, r34);
            c.logic(r34, 32);
            let r35 = c.r32(11) & 0x4u64;
            c.w32(11, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(11, 0x1u64);
            blk = 16; continue;
        }
        15 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 16; continue;
        }
        16 => {
            c.w32(8, 0x1u64);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            return c.call(0x70008c9e54, 0x70008d1861, 0x70008d185c);
        }
        18 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008d1861); };
            c.w64(1, t36);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d1868); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return c.ret(0x10, 0x70008d186f);
        }
        20 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d1872); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d1879); }
            return c.ret(0x0, 0x70008d187b);
        }
        21 => {
            c.w32(1, 0x36u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d1881); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d1882); };
            c.w64(1, t37);
            let Some(t38) = c.ld32(c.r64(1).wrapping_add(0x36f)) else { return RegfileExit::fault(0x70008d1889); };
            c.w32(1, t38);
            let r39 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r39);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d1892); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d1893); };
            c.w64(3, t40);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008d18a4, 0x70008d189f);
        }
        22 => {
            return RegfileExit::fault(0x70008d18a4);
        }
        23 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d18a5); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d18a9); };
            c.w64(3, t41);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008d18b7, 0x70008d18b2);
        }
        24 => {
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d18b7); };
            c.w64(6, t42);
            blk = 1; continue;
        }
        25 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d18c0); };
            c.w64(0, t43);
            blk = 26; continue;
        }
        26 => {
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d18c4); };
            c.cmp(t44, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d18d3); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008d18d7); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008d18db); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d18e9); };
            c.cmp(t45, c.r32(0), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xce0)) else { return RegfileExit::fault(0x70008d18f6); };
            c.w64(3, t46);
            c.w64(9, c.r64(3));
            return c.call(0x7000870914, 0x70008d1905, 0x70008d1900);
        }
        29 => {
            let Some(t47) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d1905); };
            c.cmp(t47, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t48) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d190e); };
            c.cmp(t48, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 31; continue;
        }
        31 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d191b); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008d1921); };
            c.logic(t51 & 0x2u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d1927); };
            if !c.push64(t52) { return RegfileExit::fault(0x70008d1927); }
            let Some(t53) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d192a); };
            if !c.push64(t53) { return RegfileExit::fault(0x70008d192a); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1935); };
            c.w64(6, t54);
            return c.call(0x70007f2274, 0x70008d193e, 0x70008d1939);
        }
        34 => {
            blk = 18; continue;
        }
        35 => {
            c.w32(1, 0xf6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d1948); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d1949); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0xce0)) else { return RegfileExit::fault(0x70008d194a); };
            c.w64(1, t55);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d1951); }
            let Some(t56) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d1952); };
            if !c.push64(t56) { return RegfileExit::fault(0x70008d1952); }
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d1955); };
            c.w64(3, t57);
            c.w32(0, 0x4u64);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1961); };
            c.w64(6, t58);
            return c.call(0x700086ed74, 0x70008d196a, 0x70008d1965);
        }
        36 => {
            return RegfileExit::fault(0x70008d196a);
        }
        37 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d196b); };
            c.w64(6, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d196f); };
            c.w64(0, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d1973); };
            c.w64(8, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d1977); };
            c.w64(3, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d197b); };
            c.w64(1, t63);
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t64) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d1984); };
            c.w32(1, t64);
            let r65 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r65);
            let Some(t66) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d198a); };
            c.cmp(t66, 0x40u64, 16);
            if c.cc_ae() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(1, c.r64(3));
            blk = 43; continue;
        }
        41 => {
            c.w64(0, c.r64(3));
            return c.call(0x700089c794, 0x70008d199e, 0x70008d1999);
        }
        42 => {
            c.w64(1, c.r64(0));
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d19a1); };
            c.w64(6, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d19a5); };
            c.w64(0, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d19a9); };
            c.w64(8, t69);
            blk = 43; continue;
        }
        43 => {
            let Some(t70) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d19ad); };
            c.w32(9, t70);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008d19b7); }
            c.cmp(c.r64(9), 0x9au64, 64);
            if c.cc_a() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            return RegfileExit::fault(0x70008d19c4);
        }
        45 => {
            let Some(t71) = c.ld32(c.r64(6).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008d19c5); };
            c.w32(9, t71);
            let r72 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r72);
            let Some(t73) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008d19cf); };
            c.w32(9, t73);
            let r74 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r74);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008d19d6); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 46; continue;
        }
        46 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t75) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d19e4); };
            c.w32(8, t75);
            let r76 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r76);
            let Some(t77) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d19eb); };
            c.cmp(t77, 0x40u64, 16);
            if c.cc_b() { blk = 51; continue; }
            blk = 48; continue;
        }
        48 => {
            return c.call(0x700089c794, 0x70008d19f8, 0x70008d19f3);
        }
        49 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d19f8); };
            c.w64(9, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d19fc); };
            c.w64(6, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d1a00); };
            c.w64(1, t80);
            blk = 51; continue;
        }
        50 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d1a06); };
            c.w64(0, t81);
            blk = 51; continue;
        }
        51 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008d1a0d); }
            c.w64(8, c.r64(9));
            let Some(t82) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d1a14); };
            c.logic(t82 & 0x200000u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d1a1e); };
            c.w64(15, t83);
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008d1a25); };
            c.w64(11, t84);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d1a2f); };
            c.w64(7, t85);
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d1a36); };
            c.w64(9, t86);
            blk = 72; continue;
        }
        53 => {
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d1a42); };
            c.w64(9, t87);
            let Some(t88) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d1a49); };
            c.w64(11, t88);
            c.w64(12, c.r64(11).wrapping_add(0x4c));
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d1a50); };
            c.w64(15, t89);
            let Some(t90) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008d1a57); };
            c.cmp(t90, c.r64(12), 64);
            if c.cc_a() { blk = 56; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w32(2, 0x4cu64);
            return c.call(0x70007febf4, 0x70008d1a66, 0x70008d1a61);
        }
        55 => {
            c.w64(11, c.r64(0));
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1a69); };
            c.w64(6, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d1a6d); };
            c.w64(1, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d1a71); };
            c.w64(0, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d1a75); };
            c.w64(8, t94);
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d1a79); };
            c.w64(9, t95);
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d1a80); };
            c.w64(15, t96);
            blk = 57; continue;
        }
        56 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008d1a89); }
            let r97 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r97);
            blk = 57; continue;
        }
        57 => {
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008d1a90); };
            c.w64(12, t98);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008d1a97); }
            c.w32(12, 0x22u64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008d1aa1); }
            let Some(t99) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d1aa5); };
            c.w32(12, t99);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            return RegfileExit::fault(0x70008d1ab5);
        }
        59 => {
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008d1ab9); }
            let Some(t100) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d1abd); };
            c.w32(3, t100);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_a() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            return RegfileExit::fault(0x70008d1acd);
        }
        61 => {
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70008d1ace); }
            let Some(t101) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d1ad2); };
            c.w32(3, t101);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x2u64, 64);
            if c.cc_a() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            return RegfileExit::fault(0x70008d1ae2);
        }
        63 => {
            c.w32(3, 0x8u64);
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(3)) { return RegfileExit::fault(0x70008d1ae8); }
            let Some(t102) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d1aec); };
            c.w32(3, t102);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x3u64, 64);
            if c.cc_a() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            return RegfileExit::fault(0x70008d1afc);
        }
        65 => {
            c.w32(3, 0x2u64);
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(3)) { return RegfileExit::fault(0x70008d1b02); }
            let Some(t103) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d1b06); };
            c.w32(3, t103);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x4u64, 64);
            if c.cc_a() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            return RegfileExit::fault(0x70008d1b16);
        }
        67 => {
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x70008d1b17); }
            c.w64(3, c.r64(11).wrapping_add(0x4b));
            c.w64(2, c.r64(11).wrapping_add(0x1b));
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_e() { blk = 71; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d1b28); };
            c.w64(7, t104);
            blk = 69; continue;
        }
        69 => {
            if !c.st32(c.r64(2), c.r32(7)) { return RegfileExit::fault(0x70008d1b2f); }
            let r105 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r105);
            c.cmp(c.r64(3), c.r64(2), 64);
            if c.cc_ne() { blk = 69; continue; }
            blk = 70; continue;
        }
        70 => {
            blk = 72; continue;
        }
        71 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d1b3c); };
            c.w64(7, t106);
            blk = 72; continue;
        }
        72 => {
            let Some(t107) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d1b43); };
            c.w32(3, t107);
            c.w64(2, c.r64(3).wrapping_mul(4));
            let Some(t108) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d1b50); };
            c.w64(3, t108);
            c.w64(9, c.r64(2).wrapping_add(c.r64(3)));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008d1b57); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008d1b5b); }
            let Some(t109) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008d1b5f); };
            c.cmp(t109, c.r64(9), 64);
            if c.cc_a() { blk = 75; continue; }
            blk = 73; continue;
        }
        73 => {
            return c.call(0x70007febf4, 0x70008d1b69, 0x70008d1b64);
        }
        74 => {
            c.w64(9, c.r64(0));
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1b6c); };
            c.w64(6, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d1b70); };
            c.w64(1, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d1b74); };
            c.w64(0, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d1b78); };
            c.w64(8, t113);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d1b7f); };
            c.w64(7, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d1b86); };
            c.w64(11, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d1b8a); };
            c.w64(2, t116);
            blk = 76; continue;
        }
        75 => {
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d1b90); };
            c.w64(15, t117);
            if !c.st64(c.r64(15), c.r64(9)) { return RegfileExit::fault(0x70008d1b97); }
            c.w64(9, c.r64(3).wrapping_add(0x1));
            blk = 76; continue;
        }
        76 => {
            c.w64(15, c.r64(2));
            let r118 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r118);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008d1ba5); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008d1ba9); }
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008d1bad); };
            c.w64(11, t119);
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008d1bb4); }
            let Some(t120) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d1bb8); };
            c.w32(11, t120);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008d1bbc); }
            c.w64(3, c.r64(11));
            let r121 = c.r32(3) & 0xe0000000u64;
            c.w32(3, r121);
            c.logic(r121, 32);
            if c.cc_ne() { blk = 81; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_e() { blk = 89; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(2, c.r64(7));
            blk = 79; continue;
        }
        79 => {
            let r122 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r122);
            if !c.st32(c.r64(15).wrapping_add(c.r64(9)), c.r32(2)) { return RegfileExit::fault(0x70008d1bdc); }
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_ne() { blk = 79; continue; }
            blk = 80; continue;
        }
        80 => {
            blk = 89; continue;
        }
        81 => {
            let r123 = c.sub(c.r32(11), 0x20000000u64, 32);
            c.w32(11, r123);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008d1bef); }
            let Some(t124) = c.ld8(c.r64(8).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008d1bf3); };
            c.w32(11, t124);
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_e() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x70008d1c05); };
            c.w64(2, t125);
            blk = 83; continue;
        }
        83 => {
            let r126 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r126);
            if !c.st32(c.r64(15).wrapping_add(c.r64(9)), c.r32(2)) { return RegfileExit::fault(0x70008d1c10); }
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_ne() { blk = 83; continue; }
            blk = 84; continue;
        }
        84 => {
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            let r127 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r127);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(7)) { return RegfileExit::fault(0x70008d1c23); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 85; continue; }
            blk = 86; continue;
        }
        86 => {
            let r128 = c.add(c.r32(3), 0xe0000000u64, 32);
            c.w32(3, r128);
            if c.cc_ne() { blk = 89; continue; }
            blk = 87; continue;
        }
        87 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008d1c35); }
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x70008d1c37); };
            c.w64(3, t129);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x70008d1c4b, 0x70008d1c46);
        }
        88 => {
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1c4b); };
            c.w64(6, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d1c4f); };
            c.w64(9, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d1c53); };
            c.w64(0, t132);
            blk = 89; continue;
        }
        89 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008d1c57); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d1c59); }
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d1c5a); };
            c.w64(8, t133);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008d1c61); }
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0x2670)) else { return RegfileExit::fault(0x70008d1c63); };
            c.w64(3, t134);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008d1c74, 0x70008d1c6f);
        }
        90 => {
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1c74); };
            c.w64(6, t135);
            let Some(t136) = c.ld64(c.r64(13).wrapping_add(0xce0)) else { return RegfileExit::fault(0x70008d1c78); };
            c.w64(3, t136);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008d1c7f); }
            return c.call(0x7000870914, 0x70008d1c88, 0x70008d1c83);
        }
        91 => {
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d1c88); };
            if !c.push64(t137) { return RegfileExit::fault(0x70008d1c88); }
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d1c8b); };
            if !c.push64(t138) { return RegfileExit::fault(0x70008d1c8b); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1c96); };
            c.w64(6, t139);
            return c.call(0x70007f2274, 0x70008d1c9f, 0x70008d1c9a);
        }
        92 => {
            blk = 18; continue;
        }
        93 => {
            return RegfileExit::jump(0x70008d1ca8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d1cb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d1cb4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d1cb8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d1cb9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d1cba); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008d1cbf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d1cc3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 44; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(7, c.r64(2).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d1cdb); };
            c.w64(8, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008d1ce0); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d1cee); };
            c.w32(9, t3);
            let r4 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r4);
            let Some(t5) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d1cf5); };
            c.cmp(t5, 0x40u64, 16);
            if c.cc_ae() { blk = 46; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d1d04); };
            c.w32(11, t6);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008d1d08); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008d1d0c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008d1d10); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 43; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d1d24); };
            c.w64(2, t7);
            blk = 7; continue;
        }
        6 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d1d2a); };
            c.w64(2, t8);
            blk = 7; continue;
        }
        7 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(0, c.r64(2));
            return c.call(0x70008a17f4, 0x70008d1d3e, 0x70008d1d39);
        }
        9 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d1d3e); };
            c.w64(7, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1d42); };
            c.w64(8, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d1d46); };
            c.w64(9, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d1d4a); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d1d4e); };
            c.w64(11, t13);
            blk = 14; continue;
        }
        10 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_l() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(3, c.r64(2));
            blk = 13; continue;
        }
        12 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(3));
            blk = 14; continue;
        }
        14 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w32(15, c.r64(11).wrapping_add(c.r64(11)));
            c.cmp(c.r32(0), c.r32(15), 32);
            if c.cc_g() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(2, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d1d88); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d1d89); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d1d8f); }
            return c.ret(0x0, 0x70008d1d91);
        }
        17 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_ne() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(15, 0x1u64);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d1d9e); };
            c.w64(7, t14);
            let Some(t15) = c.ld32(c.r64(7).wrapping_add(0x2db)) else { return RegfileExit::fault(0x70008d1da5); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            blk = 27; continue;
        }
        19 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t17) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d1db6); };
            c.w64(7, t17);
            blk = 22; continue;
        }
        21 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d1dbc); };
            c.w64(7, t18);
            blk = 22; continue;
        }
        22 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d1dc3); };
            c.cmp(t19, c.r32(7), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d1dcc); };
            c.w64(7, t20);
            let Some(t21) = c.ld32(c.r64(7).wrapping_add(0x2db)) else { return RegfileExit::fault(0x70008d1dd3); };
            c.w32(7, t21);
            let r22 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r22);
            c.w32(15, 0x1u64);
            blk = 27; continue;
        }
        24 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t23) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d1dee); };
            c.w32(15, t23);
            let r24 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r24);
            let Some(t25) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d1df5); };
            c.cmp(t25, 0x40u64, 16);
            if c.cc_ae() { blk = 53; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d1e01); };
            c.w32(15, t26);
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 42; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008d1e0f); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(0), 0x3fffffd0u64, 32);
            if c.cc_g() { blk = 38; continue; }
            blk = 29; continue;
        }
        29 => {
            let r27 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r27);
            c.w32(11, c.r32(11));
            let r28 = c.sub(c.r32(2), c.r32(11), 32);
            c.w32(2, r28);
            let r29 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r29);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008d1e35); }
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(11, c.r32(15));
            c.w64(12, c.r64(2));
            let r30 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r30);
            c.w64(0, c.r64(12));
            c.cdq();
            c.idiv(c.r32(11), 32);
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(12));
            c.cdq();
            c.idiv(c.r32(11), 32);
            c.w32(3, c.r64(15).wrapping_add(c.r64(15)));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008d1e5d); }
            c.w64(0, c.r64(7));
            return c.call(0x70008d2374, 0x70008d1e69, 0x70008d1e64);
        }
        31 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008d1e69); }
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d1e6d); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t32) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d1e73); };
            c.w64(1, (t32 as u32 as i32 as i64 as u64));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d1e77); };
            c.w64(0, t33);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d1e7d); };
            c.w64(6, t34);
            return c.call(0x70007fbc74, 0x70008d1e86, 0x70008d1e81);
        }
        33 => {
            c.w64(3, c.r64(0));
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d1e89); };
            c.w64(0, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d1e8d); };
            c.w64(6, t36);
            return c.call(0x700086f9d4, 0x70008d1e96, 0x70008d1e91);
        }
        34 => {
            blk = 36; continue;
        }
        35 => {
            c.w64(3, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            return c.call(0x70008d2374, 0x70008d1ea3, 0x70008d1e9e);
        }
        36 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d1ea3); };
            c.w64(6, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1ea7); };
            c.w64(3, t38);
            return c.call(0x700086f9d4, 0x70008d1eb0, 0x70008d1eab);
        }
        37 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d1eb0); };
            c.w64(2, t39);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d1eb7); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d1eb8); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d1ebe); }
            return c.ret(0x0, 0x70008d1ec0);
        }
        38 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x70008d1ec1); };
            c.w64(3, t40);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008d1ecf, 0x70008d1eca);
        }
        39 => {
            return RegfileExit::fault(0x70008d1ecf);
        }
        40 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x70008d1ed0); };
            c.w64(3, t41);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008d1ede, 0x70008d1ed9);
        }
        41 => {
            return RegfileExit::fault(0x70008d1ede);
        }
        42 => {
            c.w64(2, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d1ee9); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d1eea); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d1ef0); }
            return c.ret(0x0, 0x70008d1ef2);
        }
        43 => {
            c.w64(2, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d1efd); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d1efe); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d1f04); }
            return c.ret(0x0, 0x70008d1f06);
        }
        44 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008d1f07); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d1f0d); };
            c.w64(3, t42);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008d1f1c, 0x70008d1f17);
        }
        45 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d1f1c); };
            c.w64(6, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d1f20); };
            c.w64(0, t44);
            blk = 1; continue;
        }
        46 => {
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d1f29); };
            c.cmp(t45, c.r32(8), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let Some(t46) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d1f38); };
            c.cmp(t46, c.r32(8), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008d1f47); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008d1f4b); }
            let r47 = c.r32(9) | c.r32(11);
            c.w32(9, r47);
            c.logic(r47, 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(0, c.r64(8));
            return c.call(0x700089c794, 0x70008d1f5c, 0x70008d1f57);
        }
        48 => {
            c.w64(8, c.r64(0));
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d1f5f); };
            c.w64(6, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d1f63); };
            c.w64(7, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d1f67); };
            c.w64(2, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d1f6b); };
            c.w64(0, t51);
            blk = 3; continue;
        }
        49 => {
            c.w32(2, 0x36u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d1f79); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d1f7a); };
            c.w64(2, t52);
            let Some(t53) = c.ld32(c.r64(2).wrapping_add(0x373)) else { return RegfileExit::fault(0x70008d1f81); };
            c.w32(2, t53);
            let r54 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r54);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d1f8a); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d1f8b); };
            c.w64(3, t55);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008d1f9c, 0x70008d1f97);
        }
        50 => {
            return RegfileExit::fault(0x70008d1f9c);
        }
        51 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008d1f9d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008d1fa1); }
            c.w64(0, c.r64(8));
            return c.call(0x70008a1214, 0x70008d1fad, 0x70008d1fa8);
        }
        52 => {
            c.w64(8, c.r64(0));
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d1fb0); };
            c.w64(6, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d1fb4); };
            c.w64(7, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d1fb8); };
            c.w64(2, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d1fbc); };
            c.w64(0, t59);
            blk = 3; continue;
        }
        53 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008d1fc5); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008d1fc9); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008d1fcd); }
            c.w64(0, c.r64(7));
            return c.call(0x700089c794, 0x70008d1fd9, 0x70008d1fd4);
        }
        54 => {
            c.w64(7, c.r64(0));
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d1fdc); };
            c.w64(8, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d1fe0); };
            c.w64(9, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d1fe4); };
            c.w64(6, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d1fe8); };
            c.w64(2, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d1fec); };
            c.w64(11, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d1ff0); };
            c.w64(12, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d1ff4); };
            c.w64(0, t66);
            blk = 26; continue;
        }
        55 => {
            return RegfileExit::jump(0x70008d2000);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d2014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d2014); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d2018); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d2019); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d201a); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008d201f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d2023); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 44; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(7, c.r64(2).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d203b); };
            c.w64(8, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008d2040); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d204e); };
            c.w32(9, t3);
            let r4 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r4);
            let Some(t5) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2055); };
            c.cmp(t5, 0x40u64, 16);
            if c.cc_ae() { blk = 46; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2064); };
            c.w32(11, t6);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008d2068); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008d206c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008d2070); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 43; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d2084); };
            c.w64(2, t7);
            blk = 7; continue;
        }
        6 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d208a); };
            c.w64(2, t8);
            blk = 7; continue;
        }
        7 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(0, c.r64(2));
            return c.call(0x70008a17f4, 0x70008d209e, 0x70008d2099);
        }
        9 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d209e); };
            c.w64(7, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d20a2); };
            c.w64(8, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d20a6); };
            c.w64(9, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d20aa); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d20ae); };
            c.w64(11, t13);
            blk = 14; continue;
        }
        10 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_l() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(3, c.r64(2));
            blk = 13; continue;
        }
        12 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(3));
            blk = 14; continue;
        }
        14 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w32(15, c.r64(11).wrapping_add(c.r64(11)));
            c.cmp(c.r32(0), c.r32(15), 32);
            if c.cc_g() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(2, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d20e8); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d20e9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d20ef); }
            return c.ret(0x0, 0x70008d20f1);
        }
        17 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_ne() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(15, 0x1u64);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d20fe); };
            c.w64(7, t14);
            let Some(t15) = c.ld32(c.r64(7).wrapping_add(0x2db)) else { return RegfileExit::fault(0x70008d2105); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            blk = 27; continue;
        }
        19 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t17) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d2116); };
            c.w64(7, t17);
            blk = 22; continue;
        }
        21 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d211c); };
            c.w64(7, t18);
            blk = 22; continue;
        }
        22 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d2123); };
            c.cmp(t19, c.r32(7), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d212c); };
            c.w64(7, t20);
            let Some(t21) = c.ld32(c.r64(7).wrapping_add(0x2db)) else { return RegfileExit::fault(0x70008d2133); };
            c.w32(7, t21);
            let r22 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r22);
            c.w32(15, 0x1u64);
            blk = 27; continue;
        }
        24 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t23) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d214e); };
            c.w32(15, t23);
            let r24 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r24);
            let Some(t25) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2155); };
            c.cmp(t25, 0x40u64, 16);
            if c.cc_ae() { blk = 53; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2161); };
            c.w32(15, t26);
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 42; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008d216f); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(0), 0x3fffffd0u64, 32);
            if c.cc_g() { blk = 38; continue; }
            blk = 29; continue;
        }
        29 => {
            let r27 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r27);
            c.w32(11, c.r32(11));
            let r28 = c.sub(c.r32(2), c.r32(11), 32);
            c.w32(2, r28);
            let r29 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r29);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008d2195); }
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(11, c.r32(15));
            c.w64(12, c.r64(2));
            let r30 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r30);
            c.w64(0, c.r64(12));
            c.cdq();
            c.idiv(c.r32(11), 32);
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(12));
            c.cdq();
            c.idiv(c.r32(11), 32);
            c.w32(3, c.r64(15).wrapping_add(c.r64(15)));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008d21bd); }
            c.w64(0, c.r64(7));
            return c.call(0x70008d2374, 0x70008d21c9, 0x70008d21c4);
        }
        31 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008d21c9); }
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d21cd); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t32) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d21d3); };
            c.w64(1, (t32 as u32 as i32 as i64 as u64));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d21d7); };
            c.w64(0, t33);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d21dd); };
            c.w64(6, t34);
            return c.call(0x70007fbc74, 0x70008d21e6, 0x70008d21e1);
        }
        33 => {
            c.w64(3, c.r64(0));
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d21e9); };
            c.w64(0, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d21ed); };
            c.w64(6, t36);
            return c.call(0x700086f9d4, 0x70008d21f6, 0x70008d21f1);
        }
        34 => {
            blk = 36; continue;
        }
        35 => {
            c.w64(3, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            return c.call(0x70008d2374, 0x70008d2203, 0x70008d21fe);
        }
        36 => {
            c.w64(3, c.r64(0));
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d2206); };
            c.w64(0, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d220a); };
            c.w64(6, t38);
            return c.call(0x700086f9d4, 0x70008d2213, 0x70008d220e);
        }
        37 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d2213); };
            c.w64(2, t39);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d221a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d221b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d2221); }
            return c.ret(0x0, 0x70008d2223);
        }
        38 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x70008d2224); };
            c.w64(3, t40);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008d2232, 0x70008d222d);
        }
        39 => {
            return RegfileExit::fault(0x70008d2232);
        }
        40 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x70008d2233); };
            c.w64(3, t41);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008d2241, 0x70008d223c);
        }
        41 => {
            return RegfileExit::fault(0x70008d2241);
        }
        42 => {
            c.w64(2, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d224c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d224d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d2253); }
            return c.ret(0x0, 0x70008d2255);
        }
        43 => {
            c.w64(2, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d2260); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d2261); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d2267); }
            return c.ret(0x0, 0x70008d2269);
        }
        44 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008d226a); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d2270); };
            c.w64(3, t42);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008d227f, 0x70008d227a);
        }
        45 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d227f); };
            c.w64(6, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d2283); };
            c.w64(0, t44);
            blk = 1; continue;
        }
        46 => {
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d228c); };
            c.cmp(t45, c.r32(8), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let Some(t46) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d229b); };
            c.cmp(t46, c.r32(8), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008d22aa); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008d22ae); }
            let r47 = c.r32(9) | c.r32(11);
            c.w32(9, r47);
            c.logic(r47, 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(0, c.r64(8));
            return c.call(0x700089c794, 0x70008d22bf, 0x70008d22ba);
        }
        48 => {
            c.w64(8, c.r64(0));
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d22c2); };
            c.w64(6, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d22c6); };
            c.w64(7, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d22ca); };
            c.w64(2, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d22ce); };
            c.w64(0, t51);
            blk = 3; continue;
        }
        49 => {
            c.w32(2, 0x36u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d22dc); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d22dd); };
            c.w64(2, t52);
            let Some(t53) = c.ld32(c.r64(2).wrapping_add(0x377)) else { return RegfileExit::fault(0x70008d22e4); };
            c.w32(2, t53);
            let r54 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r54);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d22ed); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d22ee); };
            c.w64(3, t55);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008d22ff, 0x70008d22fa);
        }
        50 => {
            return RegfileExit::fault(0x70008d22ff);
        }
        51 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008d2300); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008d2304); }
            c.w64(0, c.r64(8));
            return c.call(0x70008a1214, 0x70008d2310, 0x70008d230b);
        }
        52 => {
            c.w64(8, c.r64(0));
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d2313); };
            c.w64(6, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d2317); };
            c.w64(7, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d231b); };
            c.w64(2, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d231f); };
            c.w64(0, t59);
            blk = 3; continue;
        }
        53 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008d2328); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008d232c); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008d2330); }
            c.w64(0, c.r64(7));
            return c.call(0x700089c794, 0x70008d233c, 0x70008d2337);
        }
        54 => {
            c.w64(7, c.r64(0));
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d233f); };
            c.w64(8, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d2343); };
            c.w64(9, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d2347); };
            c.w64(6, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d234b); };
            c.w64(2, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d234f); };
            c.w64(11, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d2353); };
            c.w64(12, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d2357); };
            c.w64(0, t66);
            blk = 26; continue;
        }
        55 => {
            return RegfileExit::jump(0x70008d2364);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d2374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d2374); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008d2378); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            c.w64(2, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d2384); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d2388); };
            c.w64(1, t1);
            blk = 3; continue;
        }
        1 => {
            c.w64(3, c.r64(0));
            return c.call(0x700086f9d4, 0x70008d2399, 0x70008d2394);
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d2399); };
            c.w64(2, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d239d); };
            c.w64(6, t3);
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008d23a1); };
            c.w64(1, t4);
            blk = 3; continue;
        }
        3 => {
            c.w64(7, c.r64(2));
            let r5 = c.r64(7) & 0x1u64;
            c.w64(7, r5);
            c.logic(r5, 64);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008d23b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008d23b6); }
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(1));
            return c.call(0x700086f9d4, 0x70008d23c5, 0x70008d23c0);
        }
        5 => {
            c.w64(1, c.r64(0));
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008d23c8); };
            c.w64(0, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d23cc); };
            c.w64(6, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d23d0); };
            c.w64(2, t8);
            blk = 6; continue;
        }
        6 => {
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008d23d7); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008d23db); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_ne() { blk = 1; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(0, c.r64(1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d23eb); }
            return c.ret(0x0, 0x70008d23ec);
        }
        8 => {
            return RegfileExit::jump(0x70008d23f0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d23f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d23f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d23f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d23f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d23fa); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d23ff); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d2403); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 23; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d240d); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d2419); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d241f); };
            c.cmp(t5, 0x40u64, 16);
            if c.cc_ae() { blk = 25; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d242a); };
            c.w64(2, t6);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008d242e); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(1, c.r64(2));
            blk = 7; continue;
        }
        5 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x70008d2444, 0x70008d243f);
        }
        6 => {
            c.w64(1, c.r64(0));
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d2447); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d244b); };
            c.w64(6, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d244f); };
            c.w64(0, t9);
            blk = 7; continue;
        }
        7 => {
            c.w32(7, c.r32(1));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_l() { blk = 21; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2468); };
            c.cmp(t10, 0x0u64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(1), 0x3fffffd0u64, 32);
            if c.cc_g() { blk = 32; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(3, c.r64(1));
            return c.call(0x70008d2374, 0x70008d2482, 0x70008d247d);
        }
        13 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008d2482); };
            c.w64(1, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d2489); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return c.ret(0x10, 0x70008d2490);
        }
        15 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d2493); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d249a); }
            return c.ret(0x0, 0x70008d249c);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008d249d); }
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x35);
            c.psrlq(1, 0x1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            if c.cc_e() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d24c2); };
            c.cmp(t12, 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d24cc); };
            c.w64(0, t13);
            blk = 13; continue;
        }
        21 => {
            c.w32(1, 0x198u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d24da); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d24db); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008d24dc); };
            c.w64(3, t14);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008d24ed, 0x70008d24e8);
        }
        22 => {
            return RegfileExit::fault(0x70008d24ed);
        }
        23 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d24f0); };
            c.w64(3, t15);
            return c.call(0x700086ed74, 0x70008d24fc, 0x70008d24f7);
        }
        24 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d24fc); };
            c.w64(6, t16);
            blk = 1; continue;
        }
        25 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d2505); };
            c.cmp(t17, c.r32(0), 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d2512); };
            c.cmp(t18, c.r32(0), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let r19 = c.r32(2) | c.r32(1);
            c.w32(2, r19);
            c.logic(r19, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            return c.call(0x700089c794, 0x70008d2528, 0x70008d2523);
        }
        27 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d2528); };
            c.w64(6, t20);
            blk = 3; continue;
        }
        28 => {
            c.w32(2, 0x36u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d2536); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d2537); };
            c.w64(2, t21);
            let Some(t22) = c.ld32(c.r64(2).wrapping_add(0x37b)) else { return RegfileExit::fault(0x70008d253e); };
            c.w32(2, t22);
            let r23 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r23);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d2547); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d2548); };
            c.w64(3, t24);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008d2559, 0x70008d2554);
        }
        29 => {
            return RegfileExit::fault(0x70008d2559);
        }
        30 => {
            return c.call(0x70008a1214, 0x70008d255f, 0x70008d255a);
        }
        31 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d255f); };
            c.w64(6, t25);
            blk = 3; continue;
        }
        32 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x70008d2568); };
            c.w64(3, t26);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008d2576, 0x70008d2571);
        }
        33 => {
            return RegfileExit::fault(0x70008d2576);
        }
        34 => {
            blk = 21; continue;
        }
        35 => {
            return RegfileExit::jump(0x70008d2580);
        }
            _ => unreachable!(),
        }
    }
}
