#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_7000859fd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000859fd4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000859fd8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000859fd9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000859fda); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000859fdf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000859fe3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 86; continue; }
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
            let Some(t2) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000859ff9); };
            c.w64(1, t2);
            c.w64(8, c.r64(1));
            blk = 4; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085a002); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085a009); };
            c.w64(1, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700085a010); }
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085a014); };
            c.cmp(t5, c.r32(2), 32);
            if c.cc_e() { blk = 170; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085a021); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            let Some(t8) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085a028); };
            c.w32(9, t8);
            let r9 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085a02f); };
            c.w32(11, t10);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700085a039); }
            c.cmp(c.r64(11), 0x72u64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x700085a043);
        }
        7 => {
            let Some(t11) = c.ld32(c.r64(9).wrapping_add(0x1cf)) else { return RegfileExit::fault(0x700085a044); };
            c.w32(11, t11);
            let r12 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r12);
            c.cmp(c.r32(11), c.r32(2), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085a057); };
            c.w32(12, t13);
            let r14 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r14);
            let Some(t15) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x700085a05e); };
            c.logic(t15 & 0xffffffffffffff80u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t16) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085a06a); };
            c.w32(12, t16);
            let r17 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r17);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085a07b); };
            c.w32(15, t18);
            let r19 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r19);
            let Some(t20) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085a083); };
            c.cmp(t20, 0xacu64, 16);
            if c.cc_ne() { blk = 36; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t21) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085a090); };
            c.cmp(t21, c.r32(11), 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t22) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085a09b); };
            c.logic(t22 & 0x200000u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085a0a6); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700085a0ad); };
            c.w64(15, t24);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085a0b6); };
            c.w64(11, t25);
            blk = 23; continue;
        }
        14 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085a0c2); };
            c.w64(11, t26);
            let Some(t27) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700085a0c9); };
            c.w64(15, t27);
            c.w64(0, c.r64(15).wrapping_add(0x4c));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085a0d0); };
            c.w64(3, t28);
            let Some(t29) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700085a0d7); };
            c.cmp(t29, c.r64(0), 64);
            if c.cc_be() { blk = 88; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x700085a0e0); }
            let r30 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r30);
            blk = 16; continue;
        }
        16 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x700085a0e7); };
            c.w64(0, t31);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700085a0ee); }
            c.w32(0, 0x22u64);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700085a0f7); }
            let Some(t32) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085a0fb); };
            c.w32(0, t32);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_be() { blk = 90; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700085a111); }
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085a115); };
            c.w32(2, t33);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_be() { blk = 91; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700085a129); }
            let Some(t34) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085a12d); };
            c.w32(2, t34);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_be() { blk = 92; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, 0x8u64);
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x700085a146); }
            let Some(t35) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085a14a); };
            c.w32(2, t35);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x3u64, 64);
            if c.cc_be() { blk = 93; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(2, 0x2u64);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x700085a163); }
            let Some(t36) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085a167); };
            c.w32(2, t36);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x4u64, 64);
            if c.cc_be() { blk = 94; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(0)) { return RegfileExit::fault(0x700085a17b); }
            c.w64(2, c.r64(15).wrapping_add(0x4b));
            c.w64(7, c.r64(15).wrapping_add(0x1b));
            c.cmp(c.r64(7), c.r64(2), 64);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.st32(c.r64(7), c.r32(1)) { return RegfileExit::fault(0x700085a18c); }
            let r37 = c.add(c.r64(7), 0x4u64, 64);
            c.w64(7, r37);
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_ne() { blk = 22; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t38) = c.ld8(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085a197); };
            c.w32(2, t38);
            if let Some(sr) = c.shl(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let Some(t39) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700085a1a1); };
            c.w64(7, t39);
            c.w64(8, c.r64(2).wrapping_add(c.r64(7)));
            let Some(t40) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700085a1a8); };
            c.cmp(t40, c.r64(8), 64);
            if c.cc_be() { blk = 95; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(11), c.r64(8)) { return RegfileExit::fault(0x700085a1b1); }
            c.w64(8, c.r64(7).wrapping_add(0x1));
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700085a1bc); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700085a1c0); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700085a1c4); };
            c.w64(15, t41);
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700085a1cb); }
            let Some(t42) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085a1cf); };
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
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x700085a1e9); }
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
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x700085a1fc); }
            let Some(t46) = c.ld8(c.r64(12).wrapping_add(0x5)) else { return RegfileExit::fault(0x700085a201); };
            c.w32(15, t46);
            c.w64(15, c.r64(15).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x700085a214); };
            c.w64(2, t47);
            blk = 31; continue;
        }
        31 => {
            let r48 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r48);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(2)) { return RegfileExit::fault(0x700085a21f); }
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
            if !c.st32(c.r64(15).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x700085a232); }
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_ne() { blk = 33; continue; }
            blk = 34; continue;
        }
        34 => {
            let r50 = c.add(c.r32(3), 0xe0000000u64, 32);
            c.w32(3, r50);
            if c.cc_e() { blk = 97; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(0, c.r64(8));
            blk = 37; continue;
        }
        36 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700085a24d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700085a24f); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2408)) else { return RegfileExit::fault(0x700085a250); };
            c.w64(3, t51);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700085a261, 0x700085a25c);
        }
        37 => {
            c.w64(12, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a264); };
            c.w64(6, t52);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085a268); };
            c.w64(1, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a26f); };
            c.w64(8, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085a273); };
            c.w64(9, t55);
            blk = 42; continue;
        }
        38 => {
            let Some(t56) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085a279); };
            c.w32(11, t56);
            let r57 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r57);
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085a280); };
            c.w64(12, t58);
            let Some(t59) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700085a287); };
            c.w64(15, t59);
            let Some(t60) = c.ld8(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085a28b); };
            c.w32(0, t60);
            c.w64(2, c.r64(0).wrapping_mul(4));
            c.w64(0, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085a29c); };
            c.w64(3, t61);
            let Some(t62) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700085a2a3); };
            c.cmp(t62, c.r64(0), 64);
            if c.cc_be() { blk = 99; continue; }
            blk = 39; continue;
        }
        39 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x700085a2ac); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 40; continue;
        }
        40 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700085a2b4); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700085a2b9); };
            c.w64(11, t63);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700085a2c0); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700085a2c5); }
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let r64 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r64);
            if !c.st32(c.r64(11).wrapping_add(c.r64(12)), c.r32(1)) { return RegfileExit::fault(0x700085a2d8); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 41; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700085a2e2); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x700085a2f0); };
            c.w64(11, t65);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t66) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085a2f9); };
            c.cmp(t66, c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t67) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085a303); };
            c.w32(11, t67);
            let r68 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r68);
            let Some(t69) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085a30a); };
            c.cmp(t69, 0x834u64, 16);
            if c.cc_ne() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(3, 0x1u64);
            blk = 47; continue;
        }
        46 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 47; continue;
        }
        47 => {
            c.w32(15, 0);
            c.logic(0, 32);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t70) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x700085a324); };
            c.w32(3, t70);
            let r71 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r71);
            let r72 = c.r32(3) & 0x3fu64;
            c.w32(3, r72);
            c.logic(r72, 32);
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_a() { blk = 56; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t73) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085a334); };
            c.w32(3, t73);
            let r74 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r74);
            let Some(t75) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085a33a); };
            c.w32(3, t75);
            let r76 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r76);
            let Some(t77) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085a340); };
            c.w32(2, t77);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x4cu64, 64);
            if c.cc_a() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            return RegfileExit::fault(0x700085a34f);
        }
        51 => {
            let Some(t78) = c.ld32(c.r64(3).wrapping_add(0x137)) else { return RegfileExit::fault(0x700085a350); };
            c.w32(3, t78);
            let Some(t79) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085a356); };
            c.cmp(t79, c.r32(3), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700085a35c); };
            c.w64(11, t80);
            let Some(t81) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085a363); };
            c.cmp(t81, c.r32(0), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w32(15, 0x1u64);
            blk = 56; continue;
        }
        54 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        55 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085a37f); };
            c.w64(11, t82);
            let Some(t83) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700085a386); };
            c.w64(3, t83);
            c.w64(2, c.r64(3).wrapping_add(0x3c));
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085a38d); };
            c.w64(7, t84);
            let Some(t85) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700085a394); };
            c.cmp(t85, c.r64(2), 64);
            if c.cc_be() { blk = 101; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x700085a39d); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            blk = 58; continue;
        }
        58 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x238)) else { return RegfileExit::fault(0x700085a3a4); };
            c.w64(3, t86);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700085a3ab); }
            c.w32(3, 0x1au64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x700085a3b4); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700085a3b8); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700085a3bc); }
            c.w32(3, 0x4u64);
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(3)) { return RegfileExit::fault(0x700085a3c5); }
            c.w64(3, 0xfffffffffffffffeu64);
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(3)) { return RegfileExit::fault(0x700085a3d0); }
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(3)) { return RegfileExit::fault(0x700085a3d4); }
            if !c.st32(c.r64(11).wrapping_add(0x1b), c.r32(1)) { return RegfileExit::fault(0x700085a3d8); }
            if !c.st32(c.r64(11).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x700085a3dc); }
            if !c.st32(c.r64(11).wrapping_add(0x23), c.r32(1)) { return RegfileExit::fault(0x700085a3e0); }
            if !c.st32(c.r64(11).wrapping_add(0x27), c.r32(1)) { return RegfileExit::fault(0x700085a3e4); }
            if !c.st32(c.r64(11).wrapping_add(0x2b), c.r32(1)) { return RegfileExit::fault(0x700085a3e8); }
            if !c.st32(c.r64(11).wrapping_add(0x2f), c.r32(1)) { return RegfileExit::fault(0x700085a3ec); }
            if !c.st32(c.r64(11).wrapping_add(0x33), c.r32(1)) { return RegfileExit::fault(0x700085a3f0); }
            if !c.st32(c.r64(11).wrapping_add(0x37), c.r32(1)) { return RegfileExit::fault(0x700085a3f4); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x700085a3f8); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r87 = c.r64(3) & c.r64(12);
            c.w64(3, r87);
            c.logic(r87, 64);
            let Some(t88) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700085a40d); };
            c.logic(t88 & 0x4u64, 8);
            if c.cc_ne() { blk = 172; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t89) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700085a417); };
            c.cmp(t89, c.r32(8), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let Some(t90) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085a426); };
            c.cmp(t90, c.r32(8), 32);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            let r91 = c.r32(11) | c.r32(3);
            c.w32(11, r91);
            c.logic(r91, 32);
            if c.cc_ne() { blk = 85; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t92) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085a43c); };
            c.w32(11, t92);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x57u64, 64);
            if c.cc_a() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            return RegfileExit::fault(0x700085a44c);
        }
        63 => {
            let Some(t93) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085a44d); };
            c.w32(3, t93);
            let r94 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r94);
            let Some(t95) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085a455); };
            c.w32(3, t95);
            let r96 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r96);
            let Some(t97) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085a45b); };
            c.w32(3, t97);
            let r98 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r98);
            let Some(t99) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085a461); };
            c.w32(2, t99);
            let r100 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r100);
            let Some(t101) = c.ld32(c.r64(9).wrapping_add(0x163)) else { return RegfileExit::fault(0x700085a467); };
            c.cmp(t101, c.r32(3), 32);
            if c.cc_ne() { blk = 103; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t102) = c.ld32(c.r64(2).wrapping_add(0x37)) else { return RegfileExit::fault(0x700085a474); };
            c.w32(3, t102);
            c.w32(3, c.r32(3));
            let r103 = c.r32(3) & 0x8u64;
            c.w32(3, r103);
            c.logic(r103, 32);
            c.cmp(c.r32(3), 0x8u64, 32);
            if c.cc_ne() { blk = 103; continue; }
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t104) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085a48f); };
            c.w32(15, t104);
            let r105 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r105);
            let Some(t106) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085a496); };
            c.w32(3, t106);
            let r107 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r107);
            let Some(t108) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x700085a49d); };
            c.w32(15, t108);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700085a4a2); }
            c.cmp(c.r64(11), 0xedu64, 64);
            if c.cc_a() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            return RegfileExit::fault(0x700085a4af);
        }
        68 => {
            let Some(t109) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085a4b0); };
            c.w32(8, t109);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t110) = c.ld32(c.r64(9).wrapping_add(0x3bb)) else { return RegfileExit::fault(0x700085a4b7); };
            c.w32(7, t110);
            let r111 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r111);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700085a4c4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700085a4c8); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 85; continue; }
            blk = 69; continue;
        }
        69 => {
            let r112 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r112);
            let r113 = c.r32(15) & 0x3fu64;
            c.w32(15, r113);
            c.logic(r113, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_be() { blk = 79; continue; }
            blk = 70; continue;
        }
        70 => {
            blk = 72; continue;
        }
        71 => {
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085a4ea); };
            c.w64(3, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a4ee); };
            c.w64(6, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085a4f2); };
            c.w64(7, t116);
            blk = 72; continue;
        }
        72 => {
            c.w64(9, 0xfff7fffffff7ffffu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700085a500); }
            let Some(t117) = c.ld64(c.r64(3).wrapping_add(c.r64(0).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085a504); };
            c.cmp(t117, c.r64(9), 64);
            if c.cc_e() { blk = 169; continue; }
            blk = 73; continue;
        }
        73 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(c.r64(0).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x700085a50f); }
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085a515); };
            c.w64(15, t118);
            let Some(t119) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700085a51c); };
            c.w64(2, t119);
            c.w64(8, c.r64(2).wrapping_add(0xc));
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085a523); };
            c.w64(9, t120);
            let Some(t121) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085a52a); };
            c.cmp(t121, c.r64(8), 64);
            if c.cc_be() { blk = 167; continue; }
            blk = 74; continue;
        }
        74 => {
            if !c.st64(c.r64(15), c.r64(8)) { return RegfileExit::fault(0x700085a533); }
            c.w64(8, c.r64(2).wrapping_add(0x1));
            blk = 75; continue;
        }
        75 => {
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085a53a); };
            c.w64(9, t122);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700085a541); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700085a545); }
            blk = 76; continue;
        }
        76 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085a54b); }
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085a54d); };
            if !c.push64(t123) { return RegfileExit::fault(0x700085a54d); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2274, 0x700085a55a, 0x700085a555);
        }
        77 => {
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085a55a); };
            c.w64(0, t124);
            let r125 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r125);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a562); };
            c.cmp(c.r64(0), t126, 64);
            if c.cc_ne() { blk = 71; continue; }
            blk = 78; continue;
        }
        78 => {
            blk = 85; continue;
        }
        79 => {
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(3));
            blk = 81; continue;
        }
        80 => {
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085a572); };
            c.w64(8, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a576); };
            c.w64(6, t128);
            c.w64(9, c.r64(1));
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085a57d); };
            c.w64(7, t129);
            blk = 81; continue;
        }
        81 => {
            let Some(t130) = c.ld32(c.r64(8).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085a581); };
            c.w32(11, t130);
            let r131 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r131);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700085a589); }
            let Some(t132) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700085a58d); };
            c.cmp(t132, c.r32(11), 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085a596); };
            c.w64(11, t133);
            blk = 83; continue;
        }
        83 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700085a59d); }
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085a59f); };
            if !c.push64(t134) { return RegfileExit::fault(0x700085a59f); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2274, 0x700085a5ac, 0x700085a5a7);
        }
        84 => {
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085a5ac); };
            c.w64(0, t135);
            let r136 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a5b4); };
            c.w64(1, t137);
            c.cmp(c.r64(1), c.r64(0), 64);
            if c.cc_ne() { blk = 80; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085a5bd); };
            c.w64(0, t138);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085a5c4); }
            return c.ret(0x0, 0x700085a5c5);
        }
        86 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700085a5c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700085a5ca); }
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085a5ce); };
            c.w64(3, t139);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x700085a5df, 0x700085a5da);
        }
        87 => {
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a5df); };
            c.w64(6, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085a5e3); };
            c.w64(2, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a5e7); };
            c.w64(0, t142);
            blk = 1; continue;
        }
        88 => {
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700085a5f5); }
            return c.call(0x70007febf4, 0x700085a5fe, 0x700085a5f9);
        }
        89 => {
            c.w64(15, c.r64(0));
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a601); };
            c.w64(6, t143);
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085a605); };
            c.w64(1, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a60c); };
            c.w64(8, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085a610); };
            c.w64(9, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085a614); };
            c.w64(12, t147);
            let Some(t148) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085a618); };
            c.w64(11, t148);
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085a61f); };
            c.w64(3, t149);
            blk = 16; continue;
        }
        90 => {
            return RegfileExit::fault(0x700085a62b);
        }
        91 => {
            return RegfileExit::fault(0x700085a62c);
        }
        92 => {
            return RegfileExit::fault(0x700085a62d);
        }
        93 => {
            return RegfileExit::fault(0x700085a62e);
        }
        94 => {
            return RegfileExit::fault(0x700085a62f);
        }
        95 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700085a630); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700085a634); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x700085a638); }
            return c.call(0x70007febf4, 0x700085a641, 0x700085a63c);
        }
        96 => {
            c.w64(8, c.r64(0));
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a644); };
            c.w64(6, t150);
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085a648); };
            c.w64(1, t151);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085a651); };
            c.w64(9, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085a655); };
            c.w64(12, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085a659); };
            c.w64(15, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085a65d); };
            c.w64(2, t155);
            blk = 25; continue;
        }
        97 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700085a666); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700085a668); }
            let Some(t156) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x700085a66c); };
            c.w64(3, t156);
            c.w64(6, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700085a683, 0x700085a67e);
        }
        98 => {
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a683); };
            c.w64(6, t157);
            let Some(t158) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085a687); };
            c.w64(1, t158);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085a690); };
            c.w64(9, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085a694); };
            c.w64(8, t160);
            blk = 35; continue;
        }
        99 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700085a69d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700085a6a1); }
            return c.call(0x70007febf4, 0x700085a6aa, 0x700085a6a5);
        }
        100 => {
            c.w64(12, c.r64(0));
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a6ad); };
            c.w64(6, t161);
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085a6b1); };
            c.w64(1, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a6b8); };
            c.w64(8, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085a6bc); };
            c.w64(9, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085a6c0); };
            c.w64(2, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085a6c4); };
            c.w64(11, t166);
            blk = 40; continue;
        }
        101 => {
            c.w32(2, 0x3cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700085a6d2); }
            return c.call(0x70007febb4, 0x700085a6db, 0x700085a6d6);
        }
        102 => {
            c.w64(11, c.r64(0));
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a6de); };
            c.w64(6, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085a6e2); };
            c.w64(12, t168);
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085a6e6); };
            c.w64(1, t169);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a6ef); };
            c.w64(8, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085a6f3); };
            c.w64(9, t171);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085a6f7); };
            c.w64(15, t172);
            blk = 58; continue;
        }
        103 => {
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0x4b8)) else { return RegfileExit::fault(0x700085a700); };
            c.w64(3, t173);
            c.w64(0, c.r64(12));
            return c.call(0x7000870914, 0x700085a70f, 0x700085a70a);
        }
        104 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700085a70f); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 165; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t174) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085a71b); };
            c.w32(1, t174);
            let r175 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r175);
            let Some(t176) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x700085a721); };
            c.logic(t176 & 0x2u64, 8);
            if c.cc_e() { blk = 165; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t177) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x700085a72b); };
            c.w64(3, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a732); };
            c.w64(0, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a736); };
            c.w64(6, t179);
            return c.call(0x7000870914, 0x700085a73f, 0x700085a73a);
        }
        107 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 163; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t180) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085a747); };
            c.w32(1, t180);
            let r181 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r181);
            let Some(t182) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x700085a74d); };
            c.logic(t182 & 0x2u64, 8);
            if c.cc_e() { blk = 163; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a757); };
            if !c.push64(t183) { return RegfileExit::fault(0x700085a757); }
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a75a); };
            c.w64(6, t184);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x700085a76b, 0x700085a766);
        }
        110 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700085a76b); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 161; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t185) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085a777); };
            c.w32(1, t185);
            let r186 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r186);
            let Some(t187) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085a77d); };
            c.cmp(t187, 0xbau64, 16);
            if c.cc_b() { blk = 161; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t188) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x700085a789); };
            c.w64(3, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a790); };
            c.w64(6, t189);
            return c.call(0x7000870914, 0x700085a799, 0x700085a794);
        }
        113 => {
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085a799); };
            c.w64(9, t190);
            let Some(t191) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085a79d); };
            c.w32(1, t191);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700085a7a7); }
            c.cmp(c.r64(1), 0x68u64, 64);
            if c.cc_a() { blk = 115; continue; }
            blk = 114; continue;
        }
        114 => {
            return RegfileExit::fault(0x700085a7b1);
        }
        115 => {
            let Some(t192) = c.ld32(c.r64(9).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x700085a7b2); };
            c.w32(1, t192);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700085a7b9); }
            blk = 116; continue;
        }
        116 => {
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085a7bd); };
            if !c.push64(t193) { return RegfileExit::fault(0x700085a7bd); }
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a7c0); };
            c.w64(6, t194);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x70007f21d4, 0x700085a7d1, 0x700085a7cc);
        }
        117 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700085a7d1); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 159; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t195) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085a7dd); };
            c.w32(1, t195);
            let r196 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r196);
            let Some(t197) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085a7e3); };
            c.cmp(c.r32(1), t197, 32);
            if c.cc_e() { blk = 136; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t198) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085a7ec); };
            c.cmp(t198, 0xbau64, 16);
            if c.cc_b() { blk = 159; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t199) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x700085a7f8); };
            c.w64(3, t199);
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a7ff); };
            c.w64(6, t200);
            return c.call(0x7000870914, 0x700085a808, 0x700085a803);
        }
        121 => {
            let Some(t201) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700085a808); };
            c.cmp(t201, c.r32(0), 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 122; continue;
        }
        122 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 133; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t202) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700085a81b); };
            c.cmp(t202, c.r32(0), 32);
            if c.cc_e() { blk = 132; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t203) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085a824); };
            c.w32(1, t203);
            let r204 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r204);
            let Some(t205) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x700085a82a); };
            c.logic(t205 & 0x10u64, 8);
            if c.cc_ne() { blk = 131; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t206) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085a830); };
            c.cmp(t206, c.r32(1), 32);
            if c.cc_e() { blk = 129; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t207) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085a839); };
            c.cmp(t207, 0x41u64, 16);
            if c.cc_ne() { blk = 85; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t208) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085a844); };
            c.logic(t208 & 0x7ffffffeu64, 32);
            if c.cc_ne() { blk = 85; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a851); };
            c.w64(0, t209);
            blk = 147; continue;
        }
        129 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700085a85a); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 85; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a86d); };
            c.w64(0, t210);
            blk = 147; continue;
        }
        131 => {
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a876); };
            c.w64(0, t211);
            blk = 147; continue;
        }
        132 => {
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a87f); };
            c.w64(0, t212);
            blk = 147; continue;
        }
        133 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 85; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a891); };
            c.w64(0, t213);
            blk = 147; continue;
        }
        135 => {
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a89a); };
            c.w64(0, t214);
            blk = 147; continue;
        }
        136 => {
            let Some(t215) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085a8a3); };
            c.w32(1, t215);
            let r216 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r216);
            let Some(t217) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700085a8a9); };
            c.cmp(t217, c.r32(1), 32);
            if c.cc_e() { blk = 147; continue; }
            blk = 137; continue;
        }
        137 => {
            c.w32(8, c.r32(1));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 146; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t218) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700085a8ba); };
            c.cmp(t218, c.r32(1), 32);
            if c.cc_e() { blk = 147; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t219) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085a8c3); };
            c.w32(8, t219);
            let r220 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r220);
            let Some(t221) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700085a8ca); };
            c.logic(t221 & 0x10u64, 8);
            if c.cc_ne() { blk = 147; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t222) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085a8d1); };
            c.cmp(t222, c.r32(8), 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t223) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085a8da); };
            c.cmp(t223, 0x41u64, 16);
            if c.cc_ne() { blk = 85; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t224) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085a8e6); };
            c.logic(t224 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 147; continue; }
            blk = 143; continue;
        }
        143 => {
            blk = 85; continue;
        }
        144 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700085a8f4); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 85; continue; }
            blk = 145; continue;
        }
        145 => {
            blk = 147; continue;
        }
        146 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 85; continue; }
            blk = 147; continue;
        }
        147 => {
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085a913); };
            c.w64(1, t225);
            let Some(t226) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085a917); };
            c.cmp(t226, c.r32(1), 32);
            if c.cc_e() { blk = 150; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t227) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x700085a91c); };
            c.w64(3, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a923); };
            c.w64(6, t228);
            return c.call(0x7000870914, 0x700085a92c, 0x700085a927);
        }
        149 => {
            blk = 151; continue;
        }
        150 => {
            let Some(t229) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085a92e); };
            c.w32(0, t229);
            let r230 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r230);
            blk = 151; continue;
        }
        151 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085a934); }
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085a935); };
            if !c.push64(t231) { return RegfileExit::fault(0x700085a935); }
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085a938); };
            c.w64(7, t232);
            c.w32(0, 0x1u64);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a941); };
            c.w64(6, t233);
            return c.call(0x70007f2274, 0x700085a94a, 0x700085a945);
        }
        152 => {
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085a94a); };
            c.w64(0, t234);
            blk = 116; continue;
        }
        153 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700085a953); }
            let Some(t235) = c.ld64(c.r64(13).wrapping_add(0x9e8)) else { return RegfileExit::fault(0x700085a957); };
            c.w64(3, t235);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085a95e); };
            c.w64(0, t236);
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a962); };
            c.w64(6, t237);
            return c.call(0x7000870914, 0x700085a96b, 0x700085a966);
        }
        154 => {
            let Some(t238) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085a96b); };
            c.cmp(t238, c.r32(0), 32);
            if c.cc_e() { blk = 157; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t239) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700085a974); };
            c.cmp(t239, c.r32(0), 32);
            if c.cc_e() { blk = 157; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085a97d); };
            if !c.push64(t240) { return RegfileExit::fault(0x700085a97d); }
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a980); };
            c.w64(6, t241);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x700085a991, 0x700085a98c);
        }
        157 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a991); };
            if !c.push64(t242) { return RegfileExit::fault(0x700085a991); }
            let Some(t243) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x700085a994); };
            c.w64(3, t243);
            c.w32(0, 0x1u64);
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a9a0); };
            c.w64(6, t244);
            return c.call(0x700086ed74, 0x700085a9a9, 0x700085a9a4);
        }
        158 => {
            return RegfileExit::fault(0x700085a9a9);
        }
        159 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085a9aa); }
            let Some(t245) = c.ld64(c.r64(13).wrapping_add(0x2200)) else { return RegfileExit::fault(0x700085a9ab); };
            c.w64(3, t245);
            c.w32(0, 0x1u64);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a9b7); };
            c.w64(6, t246);
            return c.call(0x700086ed74, 0x700085a9c0, 0x700085a9bb);
        }
        160 => {
            return RegfileExit::fault(0x700085a9c0);
        }
        161 => {
            let Some(t247) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x700085a9c1); };
            c.w64(3, t247);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a9ca); };
            c.w64(6, t248);
            return c.call(0x700086ed74, 0x700085a9d3, 0x700085a9ce);
        }
        162 => {
            return RegfileExit::fault(0x700085a9d3);
        }
        163 => {
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085a9d4); };
            if !c.push64(t249) { return RegfileExit::fault(0x700085a9d4); }
            let Some(t250) = c.ld64(c.r64(13).wrapping_add(0x21f0)) else { return RegfileExit::fault(0x700085a9d7); };
            c.w64(3, t250);
            c.w32(0, 0x1u64);
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085a9e3); };
            c.w64(6, t251);
            return c.call(0x700086ed74, 0x700085a9ec, 0x700085a9e7);
        }
        164 => {
            return RegfileExit::fault(0x700085a9ec);
        }
        165 => {
            c.w32(1, 0xf6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085a9f2); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085a9f3); }
            let Some(t252) = c.ld64(c.r64(13).wrapping_add(0x4b8)) else { return RegfileExit::fault(0x700085a9f4); };
            c.w64(1, t252);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085a9fb); }
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085a9fc); };
            if !c.push64(t253) { return RegfileExit::fault(0x700085a9fc); }
            let Some(t254) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085a9ff); };
            c.w64(3, t254);
            c.w32(0, 0x4u64);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085aa0b); };
            c.w64(6, t255);
            return c.call(0x700086ed74, 0x700085aa14, 0x700085aa0f);
        }
        166 => {
            return RegfileExit::fault(0x700085aa14);
        }
        167 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x700085aa1a); }
            return c.call(0x70007febf4, 0x700085aa24, 0x700085aa1f);
        }
        168 => {
            c.w64(8, c.r64(0));
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085aa27); };
            c.w64(6, t256);
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085aa2b); };
            c.w64(7, t257);
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085aa2f); };
            c.w64(3, t258);
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085aa33); };
            c.w64(0, t259);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x700085aa37); }
            blk = 75; continue;
        }
        169 => {
            let Some(t260) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085aa41); };
            c.w64(8, t260);
            blk = 76; continue;
        }
        170 => {
            c.w32(1, 0x52u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085aa52); }
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0xa20)) else { return RegfileExit::fault(0x700085aa53); };
            c.w64(1, t261);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085aa5a); }
            let Some(t262) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085aa5b); };
            c.w64(3, t262);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700085aa6c, 0x700085aa67);
        }
        171 => {
            return RegfileExit::fault(0x700085aa6c);
        }
        172 => {
            c.w32(11, c.r32(11));
            let r263 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r263);
            c.w64(3, 0xfffffffffffc0000u64);
            let r264 = c.r64(3) & c.r64(11);
            c.w64(3, r264);
            c.logic(r264, 64);
            let Some(t265) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700085aa7d); };
            c.logic(t265 & 0x2u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 173; continue;
        }
        173 => {
            c.w64(2, c.r64(12).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085aa8c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085aa8d); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x700085aa9a, 0x700085aa95);
        }
        174 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700085aa9a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700085aa9c); }
            blk = 60; continue;
        }
        175 => {
            return RegfileExit::jump(0x700085aaa4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085aab4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085aab4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700085aab8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085aab9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085aaba); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700085aabf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 73; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085aac9); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 76; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085aad7); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085aade); };
            c.cmp(t5, 0x830u64, 16);
            if c.cc_ne() { blk = 76; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085aaeb); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085aaf1); };
            c.w64(2, t8);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700085aaf5); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085ab02); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            let Some(t11) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085ab09); };
            c.w32(9, t11);
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 48; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085ab19); };
            c.cmp(t12, c.r32(8), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(8, c.r16(9));
            c.cmp(c.r32(8), 0xbau64, 32);
            if c.cc_ge() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(10, c.r64(4));
            let r13 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r13);
            let r14 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r14);
            c.logic(r14, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700085ab49); }
            c.w64(1, c.r64(13));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x700085ab51); };
            c.w64(0, t15);
            c.w64(10, 0x700085ab5fu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700085ab5f); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700085ab63); }
            return c.call(c.r64(0), 0x700085ab69, 0x700085ab67);
        }
        9 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700085ab69); }
            let Some(t16) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085ab71); };
            c.w64(4, t16);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(6, c.r64(7));
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085ab7f); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085ab83); };
            c.w64(2, t18);
            blk = 18; continue;
        }
        10 => {
            let Some(t19) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085ab8c); };
            c.w32(8, t19);
            let r20 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r20);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t21) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085ab99); };
            c.w32(9, t21);
            let r22 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r22);
            let Some(t23) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085aba0); };
            c.w32(9, t23);
            c.cmp(c.r16(9), 0xb0u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(9), 0x7bu64, 16);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 18; continue;
        }
        14 => {
            let Some(t24) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085abb8); };
            c.w32(9, t24);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_be() { blk = 75; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085abcc); };
            c.w32(8, t25);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(6, c.r64(8));
            blk = 18; continue;
        }
        16 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085abdb); };
            c.w32(8, t26);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if let Some(sr) = c.shr(c.r64(8), 0xau64, 64) { c.w64(8, sr); }
            let r27 = c.r64(8) & 0xfffffu64;
            c.w64(8, r27);
            c.logic(r27, 64);
            c.w64(6, c.r64(8));
            blk = 18; continue;
        }
        17 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(6, c.r64(8));
            blk = 18; continue;
        }
        18 => {
            let Some(t28) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085abfe); };
            c.w32(8, t28);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8).wrapping_sub(0x1));
            let r29 = c.r64(9) & c.r64(6);
            c.w64(9, r29);
            c.logic(r29, 64);
            let Some(t30) = c.ld32(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085ac0f); };
            c.w32(9, t30);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 20; continue;
        }
        19 => {
            let Some(t31) = c.ld32(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085ac1c); };
            c.w32(9, t31);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(9), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 72; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)));
            if let Some(sr) = c.shl(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let Some(t32) = c.ld32(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085ac39); };
            c.cmp(t32, c.r32(2), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 19; continue;
        }
        23 => {
            c.w64(10, c.r64(4));
            let r33 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r33);
            let r34 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r34);
            c.logic(r34, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700085ac51); }
            c.w64(1, c.r64(13));
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x700085ac59); };
            c.w64(0, t35);
            c.w64(10, 0x700085ac67u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700085ac67); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700085ac6b); }
            return c.call(c.r64(0), 0x700085ac71, 0x700085ac6f);
        }
        24 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700085ac71); }
            let Some(t36) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085ac79); };
            c.w64(4, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085ac7e); };
            c.w64(7, t37);
            let Some(t38) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085ac82); };
            c.w32(8, t38);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(8).wrapping_sub(0x1));
            let r39 = c.r64(11) & c.r64(9);
            c.w64(11, r39);
            c.logic(r39, 64);
            let Some(t40) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085ac99); };
            c.w32(9, t40);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700085aca4); }
            blk = 26; continue;
        }
        25 => {
            let Some(t41) = c.ld32(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085acaa); };
            c.w32(9, t41);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(9), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 72; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)));
            if let Some(sr) = c.shl(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let Some(t42) = c.ld32(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085acc7); };
            c.w32(11, t42);
            let r43 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r43);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700085accf); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t44) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085acd9); };
            c.w32(12, t44);
            let r45 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r45);
            let Some(t46) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085ace0); };
            c.cmp(t46, 0x41u64, 16);
            if c.cc_ne() { blk = 25; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085ace9); };
            if !c.push64(t47) { return RegfileExit::fault(0x700085ace9); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700085acec); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x700085acee); };
            c.w64(3, t48);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085ad01, 0x700085acfc);
        }
        30 => {
            let Some(t49) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700085ad01); };
            c.cmp(t49, c.r32(0), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085ad0e); };
            c.w64(7, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085ad12); };
            c.w64(9, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085ad16); };
            c.w64(8, t52);
            blk = 25; continue;
        }
        32 => {
            c.w64(10, c.r64(4));
            let r53 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r53);
            let r54 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r54);
            c.logic(r54, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700085ad27); }
            c.w64(1, c.r64(13));
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x700085ad2f); };
            c.w64(0, t55);
            c.w64(10, 0x700085ad3du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700085ad3d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700085ad41); }
            return c.call(c.r64(0), 0x700085ad47, 0x700085ad45);
        }
        33 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700085ad47); }
            let Some(t56) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085ad4f); };
            c.w64(4, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085ad54); };
            c.w64(7, t57);
            let Some(t58) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085ad58); };
            c.w32(8, t58);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t59) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085ad5f); };
            c.w64(2, t59);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700085ad63); }
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(8).wrapping_sub(0x1));
            let r60 = c.r64(11) & c.r64(9);
            c.w64(11, r60);
            c.logic(r60, 64);
            let Some(t61) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085ad78); };
            c.w32(9, t61);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 35; continue;
        }
        34 => {
            let Some(t62) = c.ld32(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085ad85); };
            c.w32(9, t62);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(9), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 72; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)));
            if let Some(sr) = c.shl(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let Some(t63) = c.ld32(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085ada2); };
            c.w32(11, t63);
            let r64 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r64);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t65) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085adb0); };
            c.w32(12, t65);
            let Some(t66) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085adb4); };
            c.cmp(t66, c.r32(12), 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 38; continue;
        }
        38 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700085adbd); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            if c.cc_e() { blk = 68; continue; }
            blk = 40; continue;
        }
        40 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            if c.cc_e() { blk = 34; continue; }
            blk = 42; continue;
        }
        42 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 68; continue; }
            blk = 43; continue;
        }
        43 => {
            if c.cc_e() { blk = 34; continue; }
            blk = 44; continue;
        }
        44 => {
            blk = 68; continue;
        }
        45 => {
            let r67 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r67);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 34; continue; }
            blk = 46; continue;
        }
        46 => {
            if c.cc_e() { blk = 68; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 34; continue;
        }
        48 => {
            let Some(t68) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085ae02); };
            c.w32(8, t68);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            let r69 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r69);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 52; continue;
        }
        50 => {
            c.w64(10, c.r64(4));
            let r70 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r70);
            let r71 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r71);
            c.logic(r71, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700085ae20); }
            c.w64(1, c.r64(13));
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x700085ae28); };
            c.w64(0, t72);
            c.w64(10, 0x700085ae36u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700085ae36); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700085ae3a); }
            return c.call(c.r64(0), 0x700085ae40, 0x700085ae3e);
        }
        51 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700085ae40); }
            let Some(t73) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085ae48); };
            c.w64(4, t73);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7));
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085ae56); };
            c.w64(7, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085ae5a); };
            c.w64(2, t75);
            blk = 52; continue;
        }
        52 => {
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085ae5e); };
            c.w32(9, t76);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r77 = c.r64(11) & c.r64(8);
            c.w64(11, r77);
            c.logic(r77, 64);
            let Some(t78) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085ae6f); };
            c.w32(8, t78);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700085ae7a); }
            blk = 54; continue;
        }
        53 => {
            let Some(t79) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085ae80); };
            c.w32(8, t79);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r64(8), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 72; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(8).wrapping_mul(2)));
            if let Some(sr) = c.shl(c.r64(8), 0x2u64, 64) { c.w64(8, sr); }
            let Some(t80) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085ae9d); };
            c.w32(0, t80);
            let r81 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r81);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700085aea5); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t82) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085aead); };
            c.w32(11, t82);
            let r83 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r83);
            let Some(t84) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085aeb4); };
            c.cmp(t84, 0x40u64, 16);
            if c.cc_ae() { blk = 53; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x70007fb534, 0x700085aec3, 0x700085aebe);
        }
        58 => {
            let Some(t85) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700085aec3); };
            c.cmp(t85, c.r32(0), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085aed0); };
            c.w64(7, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085aed4); };
            c.w64(2, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085aed8); };
            c.w64(8, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085aedc); };
            c.w64(9, t89);
            blk = 53; continue;
        }
        60 => {
            c.w64(8, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let Some(t90) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085aee8); };
            c.w32(9, t90);
            c.w32(11, c.r32(8));
            let r91 = c.shl(c.r32(8), 0xfu64, 32).unwrap_or(c.r32(8));
            c.w32(8, r91);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r92 = !c.r32(11);
            c.w32(11, r92);
            let r93 = c.add(c.r32(8), c.r32(11), 32);
            c.w32(8, r93);
            c.w64(11, c.r64(8));
            let r94 = c.shr(c.r32(11), 0xcu64, 32).unwrap_or(c.r32(11));
            c.w32(11, r94);
            let r95 = c.r32(11) ^ c.r32(8);
            c.w32(11, r95);
            c.logic(r95, 32);
            c.w32(8, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(4)));
            c.w64(11, c.r64(8));
            let r96 = c.shr(c.r32(11), 0x4u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r96);
            let r97 = c.r32(11) ^ c.r32(8);
            c.w32(11, r97);
            c.logic(r97, 32);
            let r98 = c.imul2(c.r32(11), 0x809u64, 32);
            c.w32(8, r98);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            c.w64(12, c.r64(8));
            let r99 = c.shr(c.r32(12), 0x10u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r99);
            let r100 = c.r32(12) ^ c.r32(8);
            c.w32(12, r100);
            c.logic(r100, 32);
            let r101 = c.r32(12) & 0x3fffffffu64;
            c.w32(12, r101);
            c.logic(r101, 32);
            c.w64(8, (c.r32(12) as u32 as i32 as i64 as u64));
            let r102 = c.r64(11) & c.r64(8);
            c.w64(11, r102);
            c.logic(r102, 64);
            let Some(t103) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085af39); };
            c.w32(8, t103);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(11, c.r64(2));
            blk = 62; continue;
        }
        61 => {
            let Some(t104) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085af49); };
            c.w32(8, t104);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 62; continue;
        }
        62 => {
            c.cmp(c.r64(8), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 72; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(8).wrapping_mul(2)));
            if let Some(sr) = c.shl(c.r64(8), 0x2u64, 64) { c.w64(8, sr); }
            let Some(t105) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085af62); };
            c.w32(12, t105);
            let r106 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r106);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 64; continue;
        }
        64 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 61; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t107) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085af75); };
            c.w32(15, t107);
            let Some(t108) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085af7a); };
            c.cmp(t108, c.r32(15), 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(15, c.r64(11));
            let r109 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r109);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700085af89); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(15) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 61; continue; }
            blk = 67; continue;
        }
        67 => {
            if c.cc_ne() { blk = 61; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700085afa1); };
            c.w64(0, t110);
            blk = 69; continue;
        }
        69 => {
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085afa8); };
            c.w64(1, t111);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085afaf); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 71; continue; }
            blk = 70; continue;
        }
        70 => {
            return c.ret(0x10, 0x700085afb6);
        }
        71 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085afb9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085afc0); }
            return c.ret(0x0, 0x700085afc2);
        }
        72 => {
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700085afc3); };
            c.w64(0, t112);
            blk = 69; continue;
        }
        73 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085afcc); }
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085afd0); };
            c.w64(3, t113);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085afde, 0x700085afd9);
        }
        74 => {
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085afde); };
            c.w64(6, t114);
            blk = 1; continue;
        }
        75 => {
            return RegfileExit::fault(0x700085afe7);
        }
        76 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085afee); }
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085aff0); };
            c.w64(8, t115);
            let Some(t116) = c.ld32(c.r64(8).wrapping_add(0x63)) else { return RegfileExit::fault(0x700085aff7); };
            c.w32(8, t116);
            let r117 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r117);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085affe); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085b000); }
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085b001); };
            c.w64(3, t118);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700085b012, 0x700085b00d);
        }
        77 => {
            return RegfileExit::fault(0x700085b012);
        }
        78 => {
            return RegfileExit::jump(0x700085b014);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085b034(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085b034); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700085b038); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085b039); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085b03a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085b03f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700085b043); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 91; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085b04d); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 94; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085b059); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085b05f); };
            c.cmp(t5, 0x830u64, 16);
            if c.cc_ne() { blk = 94; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085b06b); };
            c.w64(7, t6);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085b075); };
            c.w32(8, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085b079); };
            c.cmp(t8, c.r32(8), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700085b082); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(7));
            blk = 8; continue;
        }
        8 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085b098); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700085b09f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700085b0a3); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t11) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085b0b1); };
            c.w32(9, t11);
            let r12 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085b0b8); };
            c.w32(11, t13);
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 56; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085b0c8); };
            c.cmp(t14, c.r32(9), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(9, c.r16(11));
            c.cmp(c.r32(9), 0xbau64, 32);
            if c.cc_ge() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(10, c.r64(4));
            let r15 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r15);
            let r16 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r16);
            c.logic(r16, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700085b0f8); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x700085b103); };
            c.w64(0, t17);
            c.w64(10, 0x700085b111u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700085b111); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700085b115); }
            return c.call(c.r64(0), 0x700085b11b, 0x700085b119);
        }
        14 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700085b11b); }
            let Some(t18) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085b123); };
            c.w64(4, t18);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085b12e); };
            c.w64(8, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085b132); };
            c.w64(0, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085b136); };
            c.w64(6, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085b13a); };
            c.w64(2, t22);
            blk = 23; continue;
        }
        15 => {
            c.w64(2, c.r64(7));
            let Some(t23) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085b143); };
            c.w32(7, t23);
            let r24 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r24);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t25) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085b14f); };
            c.w32(9, t25);
            let r26 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r26);
            let Some(t27) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085b156); };
            c.w32(9, t27);
            c.cmp(c.r16(9), 0xb0u64, 16);
            if c.cc_e() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r16(9), 0x7bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 23; continue;
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085b16e); };
            c.w32(9, t28);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_be() { blk = 93; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t29) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085b182); };
            c.w32(7, t29);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            blk = 23; continue;
        }
        21 => {
            let Some(t30) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085b18d); };
            c.w32(7, t30);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if let Some(sr) = c.shr(c.r64(7), 0xau64, 64) { c.w64(7, sr); }
            let r31 = c.r64(7) & 0xfffffu64;
            c.w64(7, r31);
            c.logic(r31, 64);
            blk = 23; continue;
        }
        22 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            blk = 23; continue;
        }
        23 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085b1a9); };
            c.w32(9, t32);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r33 = c.r64(11) & c.r64(7);
            c.w64(11, r33);
            c.logic(r33, 64);
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085b1ba); };
            c.w32(11, t34);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 25; continue;
        }
        24 => {
            let Some(t35) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085b1c7); };
            c.w32(11, t35);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r64(11), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(11, c.r64(9).wrapping_add(c.r64(11).wrapping_mul(2)));
            if let Some(sr) = c.shl(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085b1e0); };
            c.cmp(t36, c.r32(2), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 27; continue;
        }
        27 => {
            blk = 24; continue;
        }
        28 => {
            c.w64(9, c.r64(7));
            c.w64(7, c.r64(2));
            blk = 81; continue;
        }
        29 => {
            c.w64(10, c.r64(4));
            let r37 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r37);
            let r38 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r38);
            c.logic(r38, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700085b203); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x700085b20e); };
            c.w64(0, t39);
            c.w64(10, 0x700085b21cu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700085b21c); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700085b220); }
            return c.call(c.r64(0), 0x700085b226, 0x700085b224);
        }
        30 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700085b226); }
            let Some(t40) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085b22e); };
            c.w64(4, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085b233); };
            c.w64(8, t41);
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085b237); };
            c.w32(7, t42);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(7).wrapping_sub(0x1));
            let r43 = c.r64(11) & c.r64(9);
            c.w64(11, r43);
            c.logic(r43, 64);
            let Some(t44) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085b24e); };
            c.w32(11, t44);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x700085b259); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700085b25d); }
            blk = 32; continue;
        }
        31 => {
            c.w64(9, c.r64(11));
            let Some(t45) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085b266); };
            c.w32(9, t45);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9));
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r64(11), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 38; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(11, c.r64(7).wrapping_add(c.r64(11).wrapping_mul(2)));
            if let Some(sr) = c.shl(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085b282); };
            c.w32(12, t46);
            let r47 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r47);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700085b28a); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t48) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085b294); };
            c.w32(15, t48);
            let r49 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r49);
            let Some(t50) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085b29c); };
            c.cmp(t50, 0x41u64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085b2a4); };
            if !c.push64(t51) { return RegfileExit::fault(0x700085b2a4); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700085b2a7); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x700085b2a9); };
            c.w64(3, t52);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085b2bc, 0x700085b2b7);
        }
        36 => {
            let Some(t53) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700085b2bc); };
            c.cmp(t53, c.r32(0), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085b2c9); };
            c.w64(8, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085b2cd); };
            c.w64(11, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085b2d1); };
            c.w64(7, t56);
            blk = 31; continue;
        }
        38 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085b2d7); };
            c.w64(9, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085b2db); };
            c.w64(7, t58);
            blk = 81; continue;
        }
        39 => {
            c.w64(10, c.r64(4));
            let r59 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r59);
            let r60 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r60);
            c.logic(r60, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700085b2ef); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x700085b2fa); };
            c.w64(0, t61);
            c.w64(10, 0x700085b308u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700085b308); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700085b30c); }
            return c.call(c.r64(0), 0x700085b312, 0x700085b310);
        }
        40 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700085b312); }
            let Some(t62) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085b31a); };
            c.w64(4, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085b31f); };
            c.w64(8, t63);
            let Some(t64) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085b323); };
            c.w32(7, t64);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085b32a); };
            c.w64(2, t65);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700085b32e); }
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(7).wrapping_sub(0x1));
            let r66 = c.r64(11) & c.r64(9);
            c.w64(11, r66);
            c.logic(r66, 64);
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085b343); };
            c.w32(11, t67);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 42; continue;
        }
        41 => {
            let Some(t68) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085b350); };
            c.w32(11, t68);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r64(11), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 55; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(11, c.r64(7).wrapping_add(c.r64(11).wrapping_mul(2)));
            if let Some(sr) = c.shl(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let Some(t69) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085b369); };
            c.w32(12, t69);
            let r70 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r70);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t71) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085b377); };
            c.w32(15, t71);
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085b37c); };
            c.cmp(t72, c.r32(15), 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 45; continue;
        }
        45 => {
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700085b385); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            if c.cc_e() { blk = 77; continue; }
            blk = 47; continue;
        }
        47 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            if c.cc_e() { blk = 41; continue; }
            blk = 49; continue;
        }
        49 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 77; continue; }
            blk = 50; continue;
        }
        50 => {
            if c.cc_e() { blk = 41; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 77; continue;
        }
        52 => {
            let r73 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r73);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 41; continue; }
            blk = 53; continue;
        }
        53 => {
            if c.cc_e() { blk = 77; continue; }
            blk = 54; continue;
        }
        54 => {
            blk = 41; continue;
        }
        55 => {
            c.w64(7, c.r64(2));
            blk = 81; continue;
        }
        56 => {
            let Some(t74) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085b3d3); };
            c.w32(9, t74);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            let r75 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r75);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 60; continue;
        }
        58 => {
            c.w64(10, c.r64(4));
            let r76 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r76);
            let r77 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r77);
            c.logic(r77, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700085b3f1); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x700085b3fc); };
            c.w64(0, t78);
            c.w64(10, 0x700085b40au64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700085b40a); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700085b40e); }
            return c.call(c.r64(0), 0x700085b414, 0x700085b412);
        }
        59 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700085b414); }
            let Some(t79) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085b41c); };
            c.w64(4, t79);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7));
            let Some(t80) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085b42a); };
            c.w64(0, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085b42e); };
            c.w64(6, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085b432); };
            c.w64(7, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085b436); };
            c.w64(8, t83);
            blk = 60; continue;
        }
        60 => {
            let Some(t84) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085b43a); };
            c.w32(11, t84);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(11).wrapping_sub(0x1));
            let r85 = c.r64(12) & c.r64(9);
            c.w64(12, r85);
            c.logic(r85, 64);
            let Some(t86) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085b44b); };
            c.w32(12, t86);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700085b456); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700085b45a); }
            blk = 62; continue;
        }
        61 => {
            c.w64(9, c.r64(12));
            let Some(t87) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085b463); };
            c.w32(9, t87);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(12, c.r64(9));
            blk = 62; continue;
        }
        62 => {
            c.cmp(c.r64(12), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 68; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(12, c.r64(11).wrapping_add(c.r64(12).wrapping_mul(2)));
            if let Some(sr) = c.shl(c.r64(12), 0x2u64, 64) { c.w64(12, sr); }
            let Some(t88) = c.ld32(c.r64(8).wrapping_add(c.r64(12)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085b47f); };
            c.w32(15, t88);
            let r89 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r89);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700085b487); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 61; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t90) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085b491); };
            c.w32(3, t90);
            let r91 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r91);
            let Some(t92) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085b498); };
            c.cmp(t92, 0x40u64, 16);
            if c.cc_ae() { blk = 61; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(2, c.r64(7));
            c.w64(0, c.r64(15));
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x70007fb534, 0x700085b4ac, 0x700085b4a7);
        }
        66 => {
            let Some(t93) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700085b4ac); };
            c.cmp(t93, c.r32(0), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085b4b9); };
            c.w64(7, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085b4bd); };
            c.w64(8, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085b4c1); };
            c.w64(12, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085b4c5); };
            c.w64(11, t97);
            blk = 61; continue;
        }
        68 => {
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085b4cb); };
            c.w64(9, t98);
            blk = 81; continue;
        }
        69 => {
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let Some(t99) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085b4da); };
            c.w32(11, t99);
            c.w32(12, c.r32(9));
            let r100 = c.shl(c.r32(9), 0xfu64, 32).unwrap_or(c.r32(9));
            c.w32(9, r100);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r101 = !c.r32(12);
            c.w32(12, r101);
            let r102 = c.add(c.r32(9), c.r32(12), 32);
            c.w32(9, r102);
            c.w64(12, c.r64(9));
            let r103 = c.shr(c.r32(12), 0xcu64, 32).unwrap_or(c.r32(12));
            c.w32(12, r103);
            let r104 = c.r32(12) ^ c.r32(9);
            c.w32(12, r104);
            c.logic(r104, 32);
            c.w32(9, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(4)));
            c.w64(12, c.r64(9));
            let r105 = c.shr(c.r32(12), 0x4u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r105);
            let r106 = c.r32(12) ^ c.r32(9);
            c.w32(12, r106);
            c.logic(r106, 32);
            let r107 = c.imul2(c.r32(12), 0x809u64, 32);
            c.w32(9, r107);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(11).wrapping_sub(0x1));
            c.w64(15, c.r64(9));
            let r108 = c.shr(c.r32(15), 0x10u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r108);
            let r109 = c.r32(15) ^ c.r32(9);
            c.w32(15, r109);
            c.logic(r109, 32);
            let r110 = c.r32(15) & 0x3fffffffu64;
            c.w32(15, r110);
            c.logic(r110, 32);
            c.w64(9, (c.r32(15) as u32 as i32 as i64 as u64));
            let r111 = c.r64(12) & c.r64(9);
            c.w64(12, r111);
            c.logic(r111, 64);
            let Some(t112) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085b52b); };
            c.w32(12, t112);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            blk = 71; continue;
        }
        70 => {
            let Some(t113) = c.ld32(c.r64(8).wrapping_add(c.r64(12)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085b538); };
            c.w32(12, t113);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r64(12), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 81; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(12, c.r64(11).wrapping_add(c.r64(12).wrapping_mul(2)));
            if let Some(sr) = c.shl(c.r64(12), 0x2u64, 64) { c.w64(12, sr); }
            let Some(t114) = c.ld32(c.r64(8).wrapping_add(c.r64(12)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085b551); };
            c.w32(15, t114);
            let r115 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r115);
            c.cmp(c.r32(15), c.r32(7), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 73; continue;
        }
        73 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 70; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t116) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085b564); };
            c.w32(3, t116);
            let Some(t117) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085b568); };
            c.cmp(t117, c.r32(3), 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(3, c.r64(7));
            let r118 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r118);
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x700085b576); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 70; continue; }
            blk = 76; continue;
        }
        76 => {
            if c.cc_ne() { blk = 70; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t119) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085b58c); };
            c.w64(0, t119);
            blk = 78; continue;
        }
        78 => {
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085b590); };
            c.w64(1, t120);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085b597); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            return c.ret(0x10, 0x700085b59e);
        }
        80 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085b5a1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085b5a8); }
            return c.ret(0x0, 0x700085b5aa);
        }
        81 => {
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_g() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            c.w64(10, c.r64(4));
            let r121 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r121);
            let r122 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r122);
            c.logic(r122, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700085b5bc); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x12d8)) else { return RegfileExit::fault(0x700085b5c7); };
            c.w64(0, t123);
            c.w64(10, 0x700085b5d5u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700085b5d5); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700085b5d9); }
            return c.call(c.r64(0), 0x700085b5df, 0x700085b5dd);
        }
        83 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700085b5df); }
            let Some(t124) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085b5e7); };
            c.w64(4, t124);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7));
            blk = 84; continue;
        }
        84 => {
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085b5f5); };
            c.w64(8, t125);
            let Some(t126) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085b5f9); };
            c.w32(7, t126);
            let Some(t127) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085b5fd); };
            c.w32(11, t127);
            let Some(t128) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085b601); };
            c.w32(12, t128);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r129 = c.add(c.r64(7), c.r64(11), 64);
            c.w64(7, r129);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700085b617); }
            c.cmp(c.r64(7), c.r64(12), 64);
            if c.cc_l() { blk = 87; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t130) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085b620); };
            if !c.push64(t130) { return RegfileExit::fault(0x700085b620); }
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x1de0)) else { return RegfileExit::fault(0x700085b623); };
            c.w64(3, t131);
            c.w32(0, 0x1u64);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085b62f); };
            c.w64(6, t132);
            return c.call(0x700086ed74, 0x700085b638, 0x700085b633);
        }
        86 => {
            let Some(t133) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085b638); };
            c.w64(0, t133);
            let Some(t134) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085b63c); };
            c.w32(8, t134);
            let r135 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r135);
            let Some(t136) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085b643); };
            c.w32(7, t136);
            let Some(t137) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085b647); };
            c.w32(9, t137);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t138) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085b651); };
            c.w32(11, t138);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r139 = c.add(c.r64(7), c.r64(9), 64);
            c.w64(7, r139);
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(12, c.r64(9));
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085b667); };
            c.w64(9, t140);
            blk = 88; continue;
        }
        87 => {
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            blk = 88; continue;
        }
        88 => {
            c.w64(11, c.r64(12).wrapping_sub(0x1));
            c.w64(12, c.r64(12).wrapping_add(c.r64(7).wrapping_mul(2)));
            let r141 = c.r64(11) & c.r64(9);
            c.w64(11, r141);
            c.logic(r141, 64);
            c.w64(9, c.r64(12).wrapping_mul(4));
            let Some(t142) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085b684); };
            c.w32(15, t142);
            c.w64(12, c.r64(12).wrapping_mul(4).wrapping_add(0x13));
            let r143 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085b693); };
            c.w64(0, t144);
            if !c.st32(c.r64(8).wrapping_add(c.r64(12)), c.r32(0)) { return RegfileExit::fault(0x700085b697); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r145 = c.r64(3) & c.r64(8);
            c.w64(3, r145);
            c.logic(r145, 64);
            let Some(t146) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700085b6a9); };
            c.logic(t146 & 0x4u64, 8);
            if c.cc_ne() { blk = 96; continue; }
            blk = 90; continue;
        }
        90 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x700085b6af); }
            if !c.st32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13), c.r32(7)) { return RegfileExit::fault(0x700085b6b4); }
            let Some(t147) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085b6b9); };
            c.w32(7, t147);
            let r148 = c.add(c.r32(7), 0x2u64, 32);
            c.w32(7, r148);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700085b6c3); }
            let Some(t149) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085b6c7); };
            c.w64(0, t149);
            blk = 78; continue;
        }
        91 => {
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085b6d0); };
            c.w64(3, t150);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085b6de, 0x700085b6d9);
        }
        92 => {
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085b6de); };
            c.w64(6, t151);
            blk = 1; continue;
        }
        93 => {
            return RegfileExit::fault(0x700085b6e7);
        }
        94 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085b6ed); }
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085b6ee); };
            c.w64(7, t152);
            let Some(t153) = c.ld32(c.r64(7).wrapping_add(0x67)) else { return RegfileExit::fault(0x700085b6f5); };
            c.w32(7, t153);
            let r154 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r154);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085b6fb); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085b6fc); }
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085b6fd); };
            c.w64(3, t155);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700085b70e, 0x700085b709);
        }
        95 => {
            return RegfileExit::fault(0x700085b70e);
        }
        96 => {
            c.w32(0, c.r32(0));
            let r156 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r156);
            c.w64(3, 0xfffffffffffc0000u64);
            let r157 = c.r64(3) & c.r64(0);
            c.w64(3, r157);
            c.logic(r157, 64);
            let Some(t158) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700085b71e); };
            c.logic(t158 & 0x2u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 97; continue;
        }
        97 => {
            c.w64(2, c.r64(8).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085b728); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085b729); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x700085b736, 0x700085b731);
        }
        98 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700085b736); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700085b738); }
            blk = 90; continue;
        }
        99 => {
            return RegfileExit::jump(0x700085b740);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085b754(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085b754); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700085b758); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085b759); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085b75a); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085b75f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700085b763); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 80; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085b76d); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 85; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085b77b); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085b782); };
            c.cmp(t5, 0x830u64, 16);
            if c.cc_ne() { blk = 85; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085b78f); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085b796); };
            c.w64(2, t8);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700085b79a); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085b7a7); };
            c.w32(9, t9);
            let r10 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r10);
            let Some(t11) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085b7ae); };
            c.w32(11, t11);
            c.cmp(c.r16(11), 0x40u64, 16);
            if c.cc_b() { blk = 49; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085b7be); };
            c.cmp(t12, c.r32(9), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(9, c.r16(11));
            c.cmp(c.r32(9), 0xbau64, 32);
            if c.cc_ge() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(10, c.r64(4));
            let r13 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r13);
            let r14 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r14);
            c.logic(r14, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700085b7ee); }
            c.w64(1, c.r64(13));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x700085b7f6); };
            c.w64(0, t15);
            c.w64(10, 0x700085b804u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700085b804); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700085b808); }
            return c.call(c.r64(0), 0x700085b80e, 0x700085b80c);
        }
        9 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700085b80e); }
            let Some(t16) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085b816); };
            c.w64(4, t16);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7));
            let Some(t17) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085b824); };
            c.w64(7, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085b828); };
            c.w64(6, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085b82c); };
            c.w64(8, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085b830); };
            c.w64(2, t20);
            blk = 18; continue;
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085b836); };
            c.w32(9, t21);
            let r22 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r22);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t23) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085b843); };
            c.w32(11, t23);
            let r24 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r24);
            let Some(t25) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085b84a); };
            c.w32(11, t25);
            c.cmp(c.r16(11), 0xb0u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(11), 0x7bu64, 16);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 18; continue;
        }
        14 => {
            let Some(t26) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085b863); };
            c.w32(11, t26);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4u64, 64);
            if c.cc_be() { blk = 82; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t27) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085b877); };
            c.w32(9, t27);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 18; continue;
        }
        16 => {
            let Some(t28) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085b883); };
            c.w32(9, t28);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if let Some(sr) = c.shr(c.r64(9), 0xau64, 64) { c.w64(9, sr); }
            let r29 = c.r64(9) & 0xfffffu64;
            c.w64(9, r29);
            c.logic(r29, 64);
            blk = 18; continue;
        }
        17 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 18; continue;
        }
        18 => {
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085b8a0); };
            c.w32(11, t30);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(11).wrapping_sub(0x1));
            let r31 = c.r64(12) & c.r64(9);
            c.w64(12, r31);
            c.logic(r31, 64);
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085b8b1); };
            c.w32(9, t32);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 20; continue;
        }
        19 => {
            let Some(t33) = c.ld32(c.r64(8).wrapping_add(c.r64(12)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085b8be); };
            c.w32(9, t33);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(9), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 79; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(9, c.r64(11).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(12, c.r64(9).wrapping_mul(4));
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(c.r64(12)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085b8df); };
            c.cmp(t34, c.r32(2), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 19; continue;
        }
        23 => {
            c.w64(10, c.r64(4));
            let r35 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r35);
            let r36 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r36);
            c.logic(r36, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700085b8f7); }
            c.w64(1, c.r64(13));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x700085b8ff); };
            c.w64(0, t37);
            c.w64(10, 0x700085b90du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700085b90d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700085b911); }
            return c.call(c.r64(0), 0x700085b917, 0x700085b915);
        }
        24 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700085b917); }
            let Some(t38) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085b91f); };
            c.w64(4, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085b924); };
            c.w64(7, t39);
            let Some(t40) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085b928); };
            c.w32(8, t40);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(8).wrapping_sub(0x1));
            let r41 = c.r64(11) & c.r64(9);
            c.w64(11, r41);
            c.logic(r41, 64);
            let Some(t42) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085b93f); };
            c.w32(9, t42);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700085b94a); }
            blk = 26; continue;
        }
        25 => {
            c.w64(9, c.r64(11));
            let Some(t43) = c.ld32(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085b953); };
            c.w32(9, t43);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(9), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 79; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(11, c.r64(9).wrapping_mul(4));
            let Some(t44) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085b974); };
            c.w32(12, t44);
            let r45 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r45);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700085b97c); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700085b980); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t46) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085b98a); };
            c.w32(15, t46);
            let r47 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r47);
            let Some(t48) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085b992); };
            c.cmp(t48, 0x41u64, 16);
            if c.cc_ne() { blk = 25; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085b99a); };
            if !c.push64(t49) { return RegfileExit::fault(0x700085b99a); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700085b99d); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x700085b99f); };
            c.w64(3, t50);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085b9b2, 0x700085b9ad);
        }
        30 => {
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700085b9b2); };
            c.cmp(t51, c.r32(0), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085b9bb); };
            c.w64(7, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085b9bf); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085b9c3); };
            c.w64(8, t54);
            blk = 25; continue;
        }
        32 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085b9c9); };
            c.w64(9, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085b9cd); };
            c.w64(8, t56);
            blk = 70; continue;
        }
        33 => {
            c.w64(10, c.r64(4));
            let r57 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r57);
            let r58 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r58);
            c.logic(r58, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700085b9e1); }
            c.w64(1, c.r64(13));
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x700085b9e9); };
            c.w64(0, t59);
            c.w64(10, 0x700085b9f7u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700085b9f7); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700085b9fb); }
            return c.call(c.r64(0), 0x700085ba01, 0x700085b9ff);
        }
        34 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700085ba01); }
            let Some(t60) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085ba09); };
            c.w64(4, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085ba0e); };
            c.w64(7, t61);
            let Some(t62) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085ba12); };
            c.w32(8, t62);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t63) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085ba19); };
            c.w64(2, t63);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700085ba1d); }
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(8).wrapping_sub(0x1));
            let r64 = c.r64(11) & c.r64(9);
            c.w64(11, r64);
            c.logic(r64, 64);
            let Some(t65) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085ba32); };
            c.w32(9, t65);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 36; continue;
        }
        35 => {
            let Some(t66) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085ba3f); };
            c.w32(9, t66);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(9), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 79; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(11, c.r64(9).wrapping_mul(4));
            let Some(t67) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085ba60); };
            c.w32(12, t67);
            let r68 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r68);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t69) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085ba6e); };
            c.w32(15, t69);
            let Some(t70) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085ba73); };
            c.cmp(t70, c.r32(15), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 39; continue;
        }
        39 => {
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700085ba7c); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            if c.cc_e() { blk = 48; continue; }
            blk = 41; continue;
        }
        41 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            if c.cc_e() { blk = 35; continue; }
            blk = 43; continue;
        }
        43 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 48; continue; }
            blk = 44; continue;
        }
        44 => {
            if c.cc_e() { blk = 35; continue; }
            blk = 45; continue;
        }
        45 => {
            blk = 48; continue;
        }
        46 => {
            let r71 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r71);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 35; continue; }
            blk = 47; continue;
        }
        47 => {
            if c.cc_ne() { blk = 35; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(8, c.r64(7));
            blk = 70; continue;
        }
        49 => {
            let Some(t72) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085bab9); };
            c.w32(9, t72);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            let r73 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r73);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 53; continue;
        }
        51 => {
            c.w64(10, c.r64(4));
            let r74 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r74);
            let r75 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r75);
            c.logic(r75, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700085bad7); }
            c.w64(1, c.r64(13));
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x1418)) else { return RegfileExit::fault(0x700085badf); };
            c.w64(0, t76);
            c.w64(10, 0x700085baedu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700085baed); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700085baf1); }
            return c.call(c.r64(0), 0x700085baf7, 0x700085baf5);
        }
        52 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700085baf7); }
            let Some(t77) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085baff); };
            c.w64(4, t77);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7));
            let Some(t78) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085bb0d); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085bb11); };
            c.w64(6, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085bb15); };
            c.w64(8, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085bb19); };
            c.w64(2, t81);
            blk = 53; continue;
        }
        53 => {
            let Some(t82) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085bb1d); };
            c.w32(11, t82);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(11).wrapping_sub(0x1));
            let r83 = c.r64(12) & c.r64(9);
            c.w64(12, r83);
            c.logic(r83, 64);
            let Some(t84) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085bb2e); };
            c.w32(9, t84);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700085bb39); }
            blk = 55; continue;
        }
        54 => {
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(12));
            let Some(t85) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085bb45); };
            c.w32(8, t85);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(7));
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r64(9), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 79; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(9, c.r64(11).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(12, c.r64(9).wrapping_mul(4));
            let Some(t86) = c.ld32(c.r64(8).wrapping_add(c.r64(12)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085bb6c); };
            c.w32(0, t86);
            let r87 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r87);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700085bb74); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x700085bb78); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t88) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085bb80); };
            c.w32(15, t88);
            let r89 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r89);
            let Some(t90) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085bb87); };
            c.cmp(t90, 0x40u64, 16);
            if c.cc_ae() { blk = 54; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x70007fb534, 0x700085bb96, 0x700085bb91);
        }
        59 => {
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700085bb96); };
            c.cmp(t91, c.r32(0), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085bb9f); };
            c.w64(8, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085bba3); };
            c.w64(2, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085bba7); };
            c.w64(12, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085bbab); };
            c.w64(11, t95);
            blk = 54; continue;
        }
        61 => {
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085bbb1); };
            c.w64(9, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085bbb5); };
            c.w64(8, t97);
            blk = 70; continue;
        }
        62 => {
            c.w64(9, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let Some(t98) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085bbc4); };
            c.w32(11, t98);
            c.w32(12, c.r32(9));
            let r99 = c.shl(c.r32(9), 0xfu64, 32).unwrap_or(c.r32(9));
            c.w32(9, r99);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r100 = !c.r32(12);
            c.w32(12, r100);
            let r101 = c.add(c.r32(9), c.r32(12), 32);
            c.w32(9, r101);
            c.w64(12, c.r64(9));
            let r102 = c.shr(c.r32(12), 0xcu64, 32).unwrap_or(c.r32(12));
            c.w32(12, r102);
            let r103 = c.r32(12) ^ c.r32(9);
            c.w32(12, r103);
            c.logic(r103, 32);
            c.w32(9, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(4)));
            c.w64(12, c.r64(9));
            let r104 = c.shr(c.r32(12), 0x4u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r104);
            let r105 = c.r32(12) ^ c.r32(9);
            c.w32(12, r105);
            c.logic(r105, 32);
            let r106 = c.imul2(c.r32(12), 0x809u64, 32);
            c.w32(9, r106);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(11).wrapping_sub(0x1));
            c.w64(15, c.r64(9));
            let r107 = c.shr(c.r32(15), 0x10u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r107);
            let r108 = c.r32(15) ^ c.r32(9);
            c.w32(15, r108);
            c.logic(r108, 32);
            let r109 = c.r32(15) & 0x3fffffffu64;
            c.w32(15, r109);
            c.logic(r109, 32);
            c.w64(9, (c.r32(15) as u32 as i32 as i64 as u64));
            let r110 = c.r64(12) & c.r64(9);
            c.w64(12, r110);
            c.logic(r110, 64);
            let Some(t111) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085bc15); };
            c.w32(9, t111);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(12, c.r64(2));
            blk = 64; continue;
        }
        63 => {
            let Some(t112) = c.ld32(c.r64(8).wrapping_add(c.r64(15)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700085bc25); };
            c.w32(9, t112);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r64(9), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 79; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(9, c.r64(11).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(15, c.r64(9).wrapping_mul(4));
            let Some(t113) = c.ld32(c.r64(8).wrapping_add(c.r64(15)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085bc46); };
            c.w32(0, t113);
            let r114 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r114);
            c.cmp(c.r32(12), c.r32(0), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 66; continue;
        }
        66 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 63; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t115) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085bc57); };
            c.w32(3, t115);
            let Some(t116) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085bc5a); };
            c.cmp(t116, c.r32(3), 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w64(3, c.r64(12));
            let r117 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r117);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700085bc68); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 63; continue; }
            blk = 69; continue;
        }
        69 => {
            if c.cc_ne() { blk = 63; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t118) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085bc7d); };
            c.w32(7, t118);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(11, c.r64(9).wrapping_add(0x3));
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 83; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700085bc94); };
            c.w64(7, t119);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x13), c.r32(7)) { return RegfileExit::fault(0x700085bc9b); }
            let Some(t120) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085bca0); };
            c.w32(7, t120);
            let r121 = c.sub(c.r32(7), 0x2u64, 32);
            c.w32(7, r121);
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700085bcaa); }
            let Some(t122) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085bcae); };
            c.w32(9, t122);
            let r123 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r123);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700085bcb9); }
            let Some(t124) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085bcbd); };
            c.w32(9, t124);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_be() { blk = 84; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t125) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085bccd); };
            c.w32(8, t125);
            let r126 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r126);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_l() { blk = 77; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700085bcd8); };
            c.w64(0, t127);
            blk = 74; continue;
        }
        74 => {
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085bcdf); };
            c.w64(1, t128);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085bce6); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 76; continue; }
            blk = 75; continue;
        }
        75 => {
            return c.ret(0x10, 0x700085bced);
        }
        76 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085bcf0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085bcf7); }
            return c.ret(0x0, 0x700085bcf9);
        }
        77 => {
            let Some(t129) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085bcfa); };
            if !c.push64(t129) { return RegfileExit::fault(0x700085bcfa); }
            let Some(t130) = c.ld64(c.r64(13).wrapping_add(0x1de8)) else { return RegfileExit::fault(0x700085bcfd); };
            c.w64(3, t130);
            c.w32(0, 0x1u64);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085bd09); };
            c.w64(6, t131);
            return c.call(0x700086ed74, 0x700085bd12, 0x700085bd0d);
        }
        78 => {
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700085bd12); };
            c.w64(0, t132);
            blk = 74; continue;
        }
        79 => {
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700085bd1b); };
            c.w64(0, t133);
            blk = 74; continue;
        }
        80 => {
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085bd24); };
            c.w64(3, t134);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085bd32, 0x700085bd2d);
        }
        81 => {
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085bd32); };
            c.w64(6, t135);
            blk = 1; continue;
        }
        82 => {
            return RegfileExit::fault(0x700085bd3b);
        }
        83 => {
            return RegfileExit::fault(0x700085bd3c);
        }
        84 => {
            return RegfileExit::fault(0x700085bd3d);
        }
        85 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085bd44); }
            let Some(t136) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085bd46); };
            c.w64(8, t136);
            let Some(t137) = c.ld32(c.r64(8).wrapping_add(0x6b)) else { return RegfileExit::fault(0x700085bd4d); };
            c.w32(8, t137);
            let r138 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r138);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085bd54); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085bd56); }
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085bd57); };
            c.w64(3, t139);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700085bd68, 0x700085bd63);
        }
        86 => {
            return RegfileExit::fault(0x700085bd68);
        }
        87 => {
            return RegfileExit::jump(0x700085bd6c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085bd74(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b50)) else { return RegfileExit::fault(0x700085bd74); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700085bd94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085bd94); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700085bd98); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085bd99); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085bd9a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700085bd9f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085bda9); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085bdb6); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085bdbc); };
            c.cmp(t5, 0x830u64, 16);
            if c.cc_ne() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085bdc8); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085bdce); };
            c.w32(7, t8);
            let r9 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085bdd4); };
            c.w32(8, t10);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085bddb); };
            c.w32(1, t11);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xacu64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x700085bdea);
        }
        5 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(0x2b7)) else { return RegfileExit::fault(0x700085bdeb); };
            c.w32(7, t12);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085bdf1); };
            c.w64(8, t13);
            let Some(t14) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085bdf8); };
            c.w64(9, t14);
            c.w64(11, c.r64(9).wrapping_add(0x14));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085bdff); };
            c.w64(12, t15);
            let Some(t16) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700085be06); };
            c.cmp(t16, c.r64(11), 64);
            if c.cc_be() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700085be0c); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085be13); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700085be17); };
            c.w64(7, t17);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700085be1e); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700085be22); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x700085be26); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x700085be2c); }
            c.w64(0, c.r64(8));
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085be33); };
            c.w64(1, t18);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085be3a); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x8, 0x700085be41);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085be44); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085be4b); }
            return c.ret(0x0, 0x700085be4d);
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085be4e); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085be52); };
            c.w64(3, t19);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085be60, 0x700085be5b);
        }
        11 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085be60); };
            c.w64(6, t20);
            blk = 1; continue;
        }
        12 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700085be6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700085be72); }
            return c.call(0x70007febf4, 0x700085be7b, 0x700085be76);
        }
        13 => {
            c.w64(8, c.r64(0));
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085be7e); };
            c.w64(1, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085be82); };
            c.w64(7, t22);
            blk = 7; continue;
        }
        14 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085be8d); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085be8e); };
            c.w64(7, t23);
            let Some(t24) = c.ld32(c.r64(7).wrapping_add(0x6f)) else { return RegfileExit::fault(0x700085be95); };
            c.w32(7, t24);
            let r25 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r25);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085be9b); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085be9c); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085be9d); };
            c.w64(3, t26);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700085beae, 0x700085bea9);
        }
        15 => {
            return RegfileExit::fault(0x700085beae);
        }
        16 => {
            return RegfileExit::jump(0x700085beb0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085beb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085beb4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700085beb8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085beb9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085beba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700085bebf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085bec5); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085bece); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085bed4); };
            c.cmp(t5, 0x830u64, 16);
            if c.cc_ne() { blk = 8; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085bedc); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085bee2); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085bee8); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085beef); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return c.ret(0x8, 0x700085bef6);
        }
        5 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085bef9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085bf00); }
            return c.ret(0x0, 0x700085bf02);
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085bf03); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085bf07); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085bf15, 0x700085bf10);
        }
        7 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085bf15); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        8 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085bf20); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085bf21); };
            c.w64(1, t13);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0x73)) else { return RegfileExit::fault(0x700085bf28); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085bf2e); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700085bf2f); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085bf30); };
            c.w64(3, t16);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700085bf41, 0x700085bf3c);
        }
        9 => {
            return RegfileExit::fault(0x700085bf41);
        }
        10 => {
            return RegfileExit::jump(0x700085bf44);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085bf54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085bf54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700085bf58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085bf59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085bf5a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700085bf5f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700085bf63); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, c.r64(2).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085bf7b); };
            c.w64(8, t2);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700085bf83); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700085bf87); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085bf91); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085bf97); };
            c.w64(2, t4);
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700085bf9e); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700085bfa8); };
            c.w64(1, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085bfae); };
            c.w64(1, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700085bfb5); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085bfc3); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085bfca); };
            c.cmp(t9, 0x830u64, 16);
            if c.cc_ne() { blk = 27; continue; }
            blk = 9; continue;
        }
        9 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t10) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085bfe0); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700085bfe7); };
            c.logic(t12 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t13) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085bff2); };
            c.w32(11, t13);
            let r14 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r14);
            c.w64(15, c.r64(11));
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(11));
            blk = 14; continue;
        }
        12 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085c004); };
            if !c.push64(t15) { return RegfileExit::fault(0x700085c004); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700085c007); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700085c008); }
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085c009); };
            if !c.push64(t16) { return RegfileExit::fault(0x700085c009); }
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(2));
            return c.call(0x70007f2274, 0x700085c019, 0x700085c014);
        }
        13 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085c019); };
            c.w64(15, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085c01d); };
            c.w64(11, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085c021); };
            c.w64(6, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085c025); };
            c.w64(2, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085c029); };
            c.w64(9, t21);
            blk = 14; continue;
        }
        14 => {
            let Some(t22) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085c02d); };
            c.w32(0, t22);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 22; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t23) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085c039); };
            c.w32(0, t23);
            let Some(t24) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085c03d); };
            c.w32(3, t24);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t25) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085c047); };
            c.w32(7, t25);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r26 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r26);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700085c054); }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_ge() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(3, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(3, c.r64(3).wrapping_add(c.r64(11).wrapping_mul(2)));
            let Some(t27) = c.ld32(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085c067); };
            c.w32(3, t27);
            let r28 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r28);
            let r29 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r29);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700085c073); }
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700085c077); };
            c.cmp(t30, c.r32(3), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 18; continue;
        }
        18 => {
            blk = 12; continue;
        }
        19 => {
            c.w64(2, c.r64(9));
            let r31 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r31);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085c089); };
            c.w64(0, t32);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085c093); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085c094); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085c09a); }
            return c.ret(0x0, 0x700085c09c);
        }
        20 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085c09d); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085c0a1); };
            c.w64(3, t33);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x700085c0b2, 0x700085c0ad);
        }
        21 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085c0b2); };
            c.w64(6, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085c0b6); };
            c.w64(0, t35);
            blk = 1; continue;
        }
        22 => {
            let Some(t36) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085c0bf); };
            c.w32(0, t36);
            let r37 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r37);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700085c0c6); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(3, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(0, c.r64(15));
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x70007fc694, 0x700085c0e0, 0x700085c0db);
        }
        24 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.w64(11, c.r64(2));
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085c0e9); };
            c.w64(15, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085c0ed); };
            c.w64(6, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085c0f1); };
            c.w64(2, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085c0f5); };
            c.w64(9, t41);
            blk = 22; continue;
        }
        25 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700085c0fb); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x21c8)) else { return RegfileExit::fault(0x700085c0fc); };
            c.w64(3, t42);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085c10d, 0x700085c108);
        }
        26 => {
            return RegfileExit::fault(0x700085c10d);
        }
        27 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700085c113); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085c114); };
            c.w64(2, t43);
            let Some(t44) = c.ld32(c.r64(2).wrapping_add(0x77)) else { return RegfileExit::fault(0x700085c11b); };
            c.w32(2, t44);
            let r45 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r45);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700085c121); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085c122); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085c124); };
            c.w64(3, t46);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700085c135, 0x700085c130);
        }
        28 => {
            return RegfileExit::fault(0x700085c135);
        }
        29 => {
            return RegfileExit::jump(0x700085c138);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085c154(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085c154); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700085c158); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085c159); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085c15a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700085c15f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085c169); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085c176); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085c17c); };
            c.cmp(t5, 0x830u64, 16);
            if c.cc_ne() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085c188); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085c18e); };
            c.w32(7, t8);
            let r9 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085c194); };
            c.w32(8, t10);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085c19b); };
            c.w32(1, t11);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xabu64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x700085c1aa);
        }
        5 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(0x2b3)) else { return RegfileExit::fault(0x700085c1ab); };
            c.w32(7, t12);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085c1b1); };
            c.w64(8, t13);
            let Some(t14) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085c1b8); };
            c.w64(9, t14);
            c.w64(11, c.r64(9).wrapping_add(0x14));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085c1bf); };
            c.w64(12, t15);
            let Some(t16) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700085c1c6); };
            c.cmp(t16, c.r64(11), 64);
            if c.cc_be() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700085c1cc); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085c1d3); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700085c1d7); };
            c.w64(7, t17);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700085c1de); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700085c1e2); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x700085c1e6); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x700085c1ec); }
            c.w64(0, c.r64(8));
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085c1f3); };
            c.w64(1, t18);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085c1fa); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x8, 0x700085c201);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085c204); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085c20b); }
            return c.ret(0x0, 0x700085c20d);
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085c20e); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085c212); };
            c.w64(3, t19);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085c220, 0x700085c21b);
        }
        11 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085c220); };
            c.w64(6, t20);
            blk = 1; continue;
        }
        12 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700085c22e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700085c232); }
            return c.call(0x70007febf4, 0x700085c23b, 0x700085c236);
        }
        13 => {
            c.w64(8, c.r64(0));
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085c23e); };
            c.w64(1, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085c242); };
            c.w64(7, t22);
            blk = 7; continue;
        }
        14 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085c24d); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085c24e); };
            c.w64(7, t23);
            let Some(t24) = c.ld32(c.r64(7).wrapping_add(0x7b)) else { return RegfileExit::fault(0x700085c255); };
            c.w32(7, t24);
            let r25 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r25);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085c25b); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085c25c); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085c25d); };
            c.w64(3, t26);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700085c26e, 0x700085c269);
        }
        15 => {
            return RegfileExit::fault(0x700085c26e);
        }
        16 => {
            return RegfileExit::jump(0x700085c270);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085c274(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085c274); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700085c278); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085c279); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085c27a); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700085c27f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 32; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085c289); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085c296); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085c29c); };
            c.w32(7, t5);
            c.cmp(c.r16(7), 0x829u64, 16);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r16(7), 0x828u64, 16);
            if c.cc_ne() { blk = 52; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085c2be); };
            c.w32(7, t6);
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085c2c1); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085c2cb); };
            c.w32(11, t9);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085c2dc); };
            c.w32(11, t10);
            let Some(t11) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085c2e0); };
            c.w32(12, t11);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085c2ea); };
            c.w32(15, t12);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r13 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r13);
            blk = 7; continue;
        }
        6 => {
            c.w64(7, c.r64(0));
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_ge() { blk = 41; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(12, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(0, c.r64(7).wrapping_add(0x1));
            c.w64(7, c.r64(12).wrapping_add(c.r64(7).wrapping_mul(2)));
            let Some(t14) = c.ld32(c.r64(9).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085c313); };
            c.w32(7, t14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700085c318); };
            c.cmp(t15, c.r32(7), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(9, c.r64(0).wrapping_add(c.r64(0)));
            if !c.st32(c.r64(1).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x700085c325); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t16) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085c333); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r17);
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085c339); };
            c.w64(8, t18);
            let Some(t19) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085c340); };
            c.w64(9, t19);
            let Some(t20) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085c343); };
            c.w32(1, t20);
            let r21 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r21);
            c.w64(11, c.r64(9).wrapping_add(0x10));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085c34d); };
            c.w64(12, t22);
            let Some(t23) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700085c354); };
            c.cmp(t23, c.r64(11), 64);
            if c.cc_be() { blk = 42; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700085c35e); }
            let r24 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r24);
            blk = 12; continue;
        }
        12 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700085c365); };
            c.w64(11, t25);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700085c36c); }
            c.w32(11, 0x4u64);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700085c376); }
            let Some(t26) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085c37a); };
            c.w32(15, t26);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_be() { blk = 44; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700085c38e); }
            let Some(t27) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085c392); };
            c.w32(15, t27);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_be() { blk = 45; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x700085c3a6); }
            let Some(t28) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085c3aa); };
            c.w32(7, t28);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x6bu64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x700085c3b9);
        }
        16 => {
            let Some(t29) = c.ld32(c.r64(1).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x700085c3ba); };
            c.w32(7, t29);
            let Some(t30) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085c3c0); };
            c.w64(15, t30);
            c.w64(0, c.r64(15).wrapping_add(0x10));
            let Some(t31) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700085c3c7); };
            c.cmp(t31, c.r64(0), 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700085c3d1); }
            let r32 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r32);
            blk = 18; continue;
        }
        18 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085c3d8); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700085c3dc); };
            c.w64(0, t33);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700085c3e3); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700085c3e7); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x700085c3eb); }
            let Some(t34) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085c3ef); };
            c.w32(7, t34);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x68u64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x700085c3fe);
        }
        20 => {
            let Some(t35) = c.ld32(c.r64(1).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x700085c3ff); };
            c.w32(1, t35);
            let Some(t36) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085c405); };
            c.w64(7, t36);
            c.w64(9, c.r64(7).wrapping_add(0x14));
            let Some(t37) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700085c40c); };
            c.cmp(t37, c.r64(9), 64);
            if c.cc_be() { blk = 48; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.st64(c.r64(8), c.r64(9)) { return RegfileExit::fault(0x700085c416); }
            let r38 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r38);
            blk = 22; continue;
        }
        22 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700085c41d); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x700085c420); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700085c423); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x700085c426); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700085c42a); };
            c.w64(1, t39);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x700085c431); }
            c.w64(0, c.r64(7));
            blk = 23; continue;
        }
        23 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085c437); };
            c.w64(1, t40);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085c43e); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            return c.ret(0x8, 0x700085c445);
        }
        25 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085c448); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085c44f); }
            return c.ret(0x0, 0x700085c451);
        }
        26 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700085c452); };
            c.w64(1, t41);
            blk = 27; continue;
        }
        27 => {
            let Some(t42) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085c459); };
            c.w32(8, t42);
            let r43 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r43);
            let Some(t44) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085c460); };
            c.w32(8, t44);
            let r45 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r45);
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085c467); };
            c.w32(9, t46);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x68u64, 64);
            if c.cc_a() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            return RegfileExit::fault(0x700085c477);
        }
        29 => {
            let Some(t47) = c.ld32(c.r64(8).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x700085c478); };
            c.w32(8, t47);
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085c47f); };
            c.w64(9, t48);
            let Some(t49) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085c486); };
            c.w64(11, t49);
            c.w64(15, c.r64(11).wrapping_add(0x14));
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085c48d); };
            c.w64(12, t50);
            let Some(t51) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700085c494); };
            c.cmp(t51, c.r64(15), 64);
            if c.cc_be() { blk = 50; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x700085c49e); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700085c4a5); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700085c4a9); };
            c.w64(11, t52);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700085c4b0); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700085c4b4); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x700085c4b8); }
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x700085c4bc); }
            c.w64(0, c.r64(9));
            blk = 23; continue;
        }
        32 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085c4c8); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085c4cc); };
            c.w64(3, t53);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085c4da, 0x700085c4d5);
        }
        33 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085c4da); };
            c.w64(6, t54);
            blk = 1; continue;
        }
        34 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085c4e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700085c4e7); }
            blk = 35; continue;
        }
        35 => {
            c.w32(3, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t55) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085c4ee); };
            c.w32(12, t55);
            let r56 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r56);
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x700085c4f8); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, c.r64(9));
            c.w64(6, c.r64(11));
            return c.call(0x70007fc694, 0x700085c50d, 0x700085c508);
        }
        37 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(7, c.r64(1));
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085c516); };
            c.w64(9, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700085c51a); };
            c.w64(1, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085c51e); };
            c.w64(6, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085c522); };
            c.w64(8, t60);
            blk = 35; continue;
        }
        38 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700085c528); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r61 = c.r64(11) & c.r64(1);
            c.w64(11, r61);
            c.logic(r61, 64);
            let Some(t62) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700085c53c); };
            c.logic(t62 & 0x4u64, 8);
            if c.cc_ne() { blk = 54; continue; }
            blk = 40; continue;
        }
        40 => {
            if !c.st32(c.r64(1).wrapping_add(0xf), c.r32(3)) { return RegfileExit::fault(0x700085c547); }
            blk = 5; continue;
        }
        41 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x408)) else { return RegfileExit::fault(0x700085c54f); };
            c.w64(7, t63);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x700085c556); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085c559); };
            c.w64(7, t64);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700085c560); };
            c.w64(1, t65);
            blk = 27; continue;
        }
        42 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700085c571); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700085c575); }
            return c.call(0x70007febf4, 0x700085c57e, 0x700085c579);
        }
        43 => {
            c.w64(9, c.r64(0));
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085c581); };
            c.w64(8, t66);
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085c588); };
            c.w64(12, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085c58f); };
            c.w64(7, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085c593); };
            c.w64(1, t69);
            blk = 12; continue;
        }
        44 => {
            return RegfileExit::fault(0x700085c59c);
        }
        45 => {
            return RegfileExit::fault(0x700085c59d);
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700085c59e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700085c5a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700085c5a6); }
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700085c5b4, 0x700085c5af);
        }
        47 => {
            c.w64(15, c.r64(0));
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085c5b7); };
            c.w64(8, t70);
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085c5be); };
            c.w64(12, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085c5c5); };
            c.w64(1, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085c5c9); };
            c.w64(9, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085c5cd); };
            c.w64(7, t74);
            c.w32(11, 0x4u64);
            blk = 18; continue;
        }
        48 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700085c5e1); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700085c5e5); }
            return c.call(0x70007febf4, 0x700085c5ee, 0x700085c5e9);
        }
        49 => {
            c.w64(7, c.r64(0));
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700085c5f1); };
            c.w64(0, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085c5f8); };
            c.w64(15, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085c5fc); };
            c.w64(1, t77);
            blk = 22; continue;
        }
        50 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700085c605); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700085c609); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700085c60d); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x700085c61b, 0x700085c616);
        }
        51 => {
            c.w64(9, c.r64(0));
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085c61e); };
            c.w64(1, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085c622); };
            c.w64(7, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085c626); };
            c.w64(8, t80);
            blk = 31; continue;
        }
        52 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085c634); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085c635); };
            c.w64(7, t81);
            let Some(t82) = c.ld32(c.r64(7).wrapping_add(0x7f)) else { return RegfileExit::fault(0x700085c63c); };
            c.w32(7, t82);
            let r83 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r83);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085c642); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085c643); }
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085c644); };
            c.w64(3, t84);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700085c655, 0x700085c650);
        }
        53 => {
            return RegfileExit::fault(0x700085c655);
        }
        54 => {
            c.w32(9, c.r32(9));
            let r85 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r85);
            c.w64(11, 0xfffffffffffc0000u64);
            let r86 = c.r64(11) & c.r64(9);
            c.w64(11, r86);
            c.logic(r86, 64);
            let Some(t87) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700085c666); };
            c.logic(t87 & 0x2u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(12, c.r64(1).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085c675); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085c676); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700085c683, 0x700085c67e);
        }
        56 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700085c683); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700085c685); }
            blk = 40; continue;
        }
        57 => {
            return RegfileExit::jump(0x700085c68c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085c694(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085c694); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700085c698); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085c69e); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085c6a4); };
            c.cmp(t3, 0x830u64, 16);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x700085c6b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700085c6b6); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085c6bf); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085c6c6); };
            c.w32(11, t6);
            let Some(t7) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085c6ca); };
            c.w32(12, t7);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            blk = 39; continue;
        }
        3 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085c6d9); };
            c.w32(9, t8);
            let r9 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r9);
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085c6e0); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085c6e7); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085c6ee); };
            c.w32(12, t14);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t15) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085c6f5); };
            c.w32(15, t15);
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700085c6fc); }
            c.cmp(c.r64(12), 0x6bu64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x700085c706);
        }
        6 => {
            c.w64(12, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x700085c70a); };
            c.w32(11, t16);
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x700085c717); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700085c71b); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(2, c.r64(12).wrapping_mul(2).wrapping_add(0x18));
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(15), 0x2000000u64, 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 51; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085c75c); };
            c.w64(3, t17);
            let Some(t18) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700085c763); };
            c.w64(2, t18);
            c.w64(1, c.r64(8).wrapping_add(0x8));
            c.w64(6, c.r64(1).wrapping_add(c.r64(2)));
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085c76e); };
            c.w64(9, t19);
            let Some(t20) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085c775); };
            c.cmp(t20, c.r64(6), 64);
            if c.cc_be() { blk = 47; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 47; continue; }
            blk = 12; continue;
        }
        12 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x700085c78b); }
            let r21 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r21);
            blk = 13; continue;
        }
        13 => {
            let r22 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r22);
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700085c795); };
            c.w64(1, t23);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700085c79c); }
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700085c79f); }
            c.w64(1, c.r64(2).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r24 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r24);
            let r25 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r25);
            c.logic(r25, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700085c7b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700085c7b7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x700085c7bb); }
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(2));
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x700085c7c4); };
            c.w64(0, t26);
            c.w64(10, 0x700085c7d2u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700085c7d2); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700085c7d6); }
            return c.call(c.r64(0), 0x700085c7dc, 0x700085c7da);
        }
        14 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700085c7dc); }
            let Some(t27) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085c7e4); };
            c.w64(4, t27);
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085c7e9); };
            c.w64(7, t28);
            let Some(t29) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700085c7f0); };
            c.w64(9, t29);
            c.w64(11, c.r64(9).wrapping_add(0x10));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085c7f7); };
            c.w64(12, t30);
            let Some(t31) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700085c7fe); };
            c.cmp(t31, c.r64(11), 64);
            if c.cc_be() { blk = 49; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x700085c808); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 16; continue;
        }
        16 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085c80f); };
            c.w64(9, t32);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700085c813); }
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085c817); };
            c.w64(9, t33);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700085c81b); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700085c81f); };
            c.w64(9, t34);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700085c826); }
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085c82a); };
            c.w64(0, t35);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700085c82e); }
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085c831); };
            c.w64(12, t36);
            c.w64(11, c.r64(0));
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(7));
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085c83e); };
            c.w64(0, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085c842); };
            c.w64(7, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085c846); };
            c.w64(9, t39);
            blk = 23; continue;
        }
        17 => {
            let r40 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r40);
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085c852); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700085c859); };
            c.w64(1, t42);
            c.w64(6, c.r64(2).wrapping_add(c.r64(1)));
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085c860); };
            c.w64(8, t43);
            let Some(t44) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085c867); };
            c.cmp(t44, c.r64(6), 64);
            if c.cc_be() { blk = 53; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x700085c870); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 19; continue;
        }
        19 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700085c877); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x700085c87b); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700085c87f); };
            c.w64(11, t45);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700085c886); }
            c.w64(11, c.r64(3).wrapping_add(0x10));
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700085c88e); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700085c892); };
            c.w64(2, t46);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700085c899); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700085c89d); }
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 23; continue;
        }
        20 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085c8a6); };
            c.w64(3, t47);
            let Some(t48) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700085c8ad); };
            c.w64(2, t48);
            c.w64(1, c.r64(2).wrapping_add(0x10));
            let r49 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r49);
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085c8b7); };
            c.w64(6, t50);
            let Some(t51) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700085c8be); };
            c.cmp(t51, c.r64(1), 64);
            if c.cc_be() { blk = 55; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x700085c8c7); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 22; continue;
        }
        22 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700085c8ce); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x700085c8d2); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700085c8d6); };
            c.w64(11, t52);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700085c8dd); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700085c8e1); }
            c.w32(15, 0x1u64);
            blk = 23; continue;
        }
        23 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700085c8eb); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(12, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_e() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700085c903); };
            c.w64(15, t53);
            blk = 26; continue;
        }
        26 => {
            let r54 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r54);
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(15)) { return RegfileExit::fault(0x700085c90e); }
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_ne() { blk = 26; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t55) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085c918); };
            c.w32(11, t55);
            let r56 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r56);
            c.w32(12, 0);
            c.logic(0, 32);
            c.w32(15, 0x7u64);
            blk = 31; continue;
        }
        28 => {
            c.w64(12, c.r64(15).wrapping_add(0x4));
            if !c.st32(c.r64(11).wrapping_add(c.r64(15)), c.r32(3)) { return RegfileExit::fault(0x700085c92e); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r57 = c.r64(2) & c.r64(11);
            c.w64(2, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700085c941); };
            c.logic(t58 & 0x4u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(15, c.r64(12));
            c.w64(12, c.r64(8));
            blk = 31; continue;
        }
        31 => {
            let Some(t59) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085c951); };
            c.w32(2, t59);
            let Some(t60) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085c955); };
            c.w32(1, t60);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t61) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085c95f); };
            c.w32(6, t61);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r62 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r62);
            c.w64(1, c.r64(12));
            blk = 33; continue;
        }
        32 => {
            c.w64(1, c.r64(8));
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r64(1), c.r64(2), 64);
            if c.cc_ge() { blk = 36; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(8, c.r64(1).wrapping_add(0x1));
            c.w64(3, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(3, c.r64(3).wrapping_add(c.r64(1).wrapping_mul(2)));
            let Some(t63) = c.ld32(c.r64(9).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700085c987); };
            c.w32(3, t63);
            let Some(t64) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700085c98c); };
            c.cmp(t64, c.r32(3), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 35; continue;
        }
        35 => {
            blk = 28; continue;
        }
        36 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w32(7, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x408)) else { return RegfileExit::fault(0x700085c9a0); };
            c.w64(9, t65);
            c.w64(11, c.r64(0));
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700085c9aa); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x700085c9ae); }
            blk = 38; continue;
        }
        38 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085c9b2); };
            c.w64(0, t66);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085c9b9); }
            return c.ret(0x0, 0x700085c9ba);
        }
        39 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x700085c9c1); }
            blk = 40; continue;
        }
        40 => {
            c.w32(3, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t67) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085c9c9); };
            c.w32(11, t67);
            let r68 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r68);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700085c9d0); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(9));
            c.w64(12, c.r64(6));
            return c.call(0x70007fc694, 0x700085c9e7, 0x700085c9e2);
        }
        42 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(11, c.r64(7));
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085c9f0); };
            c.w64(9, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085c9f4); };
            c.w64(0, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085c9f8); };
            c.w64(7, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085c9fc); };
            c.w64(6, t72);
            blk = 40; continue;
        }
        43 => {
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700085ca02); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r73 = c.r64(11) & c.r64(0);
            c.w64(11, r73);
            c.logic(r73, 64);
            let Some(t74) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700085ca16); };
            c.logic(t74 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 45; continue;
        }
        45 => {
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(3)) { return RegfileExit::fault(0x700085ca21); }
            blk = 4; continue;
        }
        46 => {
            return RegfileExit::fault(0x700085ca29);
        }
        47 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x700085ca2a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700085ca2e); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x700085ca3a, 0x700085ca35);
        }
        48 => {
            c.w64(2, c.r64(0));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085ca3d); };
            c.w64(15, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085ca41); };
            c.w64(8, t76);
            blk = 13; continue;
        }
        49 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700085ca54, 0x700085ca4f);
        }
        50 => {
            c.w64(7, c.r64(0));
            blk = 16; continue;
        }
        51 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700085ca5c); };
            c.w64(3, t77);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085ca6c, 0x700085ca67);
        }
        52 => {
            return RegfileExit::fault(0x700085ca6c);
        }
        53 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700085ca6d); }
            return c.call(0x70007febf4, 0x700085ca76, 0x700085ca71);
        }
        54 => {
            c.w64(3, c.r64(0));
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085ca79); };
            c.w64(0, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085ca7d); };
            c.w64(7, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085ca81); };
            c.w64(9, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085ca85); };
            c.w64(12, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085ca89); };
            c.w64(11, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085ca8d); };
            c.w64(15, t83);
            blk = 19; continue;
        }
        55 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700085ca9b); }
            return c.call(0x70007febf4, 0x700085caa4, 0x700085ca9f);
        }
        56 => {
            c.w64(3, c.r64(0));
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085caa7); };
            c.w64(0, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085caab); };
            c.w64(7, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085caaf); };
            c.w64(9, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085cab3); };
            c.w64(12, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085cab7); };
            c.w64(11, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700085cabb); };
            c.w64(15, t89);
            blk = 22; continue;
        }
        57 => {
            c.w32(9, c.r32(9));
            let r90 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r90);
            c.w64(11, 0xfffffffffffc0000u64);
            let r91 = c.r64(11) & c.r64(9);
            c.w64(11, r91);
            c.logic(r91, 64);
            let Some(t92) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700085cad4); };
            c.logic(t92 & 0x2u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(12, c.r64(0).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085cae3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085cae4); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700085caf1, 0x700085caec);
        }
        59 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700085caf1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700085caf3); }
            blk = 45; continue;
        }
        60 => {
            c.w32(3, c.r32(3));
            let r93 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r93);
            c.w64(2, 0xfffffffffffc0000u64);
            let r94 = c.r64(2) & c.r64(3);
            c.w64(2, r94);
            c.logic(r94, 64);
            let Some(t95) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700085cb08); };
            c.logic(t95 & 0x2u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(1, c.r64(11).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085cb16); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085cb17); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x700085cb24, 0x700085cb1f);
        }
        62 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700085cb24); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700085cb26); }
            blk = 30; continue;
        }
        63 => {
            return RegfileExit::jump(0x700085cb30);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085cb34(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b58)) else { return RegfileExit::fault(0x700085cb34); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700085cb54(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b60)) else { return RegfileExit::fault(0x700085cb54); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700085cb74(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b68)) else { return RegfileExit::fault(0x700085cb74); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_700085cb94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085cb94); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700085cb98); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085cb99); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085cb9a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085cb9f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700085cba3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 72; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085cbad); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 70; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085cbba); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700085cbc0); }
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085cbc4); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 70; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085cbd0); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085cbd7); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085cbde); };
            c.cmp(t9, 0x0u64, 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(8));
            blk = 8; continue;
        }
        6 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085cbea); }
            c.w32(0, 0x1u64);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2b28)) else { return RegfileExit::fault(0x700085cbf0); };
            c.w64(3, t10);
            return c.call(0x700086ed74, 0x700085cbfc, 0x700085cbf7);
        }
        7 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085cbfc); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085cc00); };
            c.w64(1, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085cc04); };
            c.w64(6, t13);
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700085cc08); }
            let Some(t14) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085cc0c); };
            c.logic(t14 & 0x4u64, 8);
            if c.cc_ne() { blk = 68; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t15) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x700085cc16); };
            c.w32(7, t15);
            let r16 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r16);
            let r17 = c.r32(7) & 0x3fu64;
            c.w32(7, r17);
            c.logic(r17, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x700085cc20); }
            c.cmp(c.r32(7), 0x17u64, 32);
            if c.cc_l() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(7), 0x19u64, 32);
            if c.cc_le() { blk = 70; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t18) = c.ld64(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085cc32); };
            c.w64(8, t18);
            let Some(t19) = c.ld64(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085cc36); };
            c.w64(9, t19);
            let Some(t20) = c.ld64(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700085cc3a); };
            c.w64(1, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700085cc3e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700085cc42); }
            let Some(t21) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085cc46); };
            c.w64(11, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x700085cc4a); }
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085cc4e); };
            c.cmp(t22, c.r32(11), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(11));
            return c.call(0x7000899a14, 0x700085cc69, 0x700085cc64);
        }
        14 => {
            c.w64(11, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085cc6c); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085cc70); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085cc74); };
            c.w64(9, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085cc78); };
            c.w64(7, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085cc7c); };
            c.w64(0, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085cc80); };
            c.w64(1, t28);
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 25; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700085cc9c); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_s() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(10, 0x8000000000000000u64);
            let r29 = c.r64(11) | c.r64(10);
            c.w64(11, r29);
            c.logic(r29, 64);
            blk = 23; continue;
        }
        23 => {
            blk = 25; continue;
        }
        24 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_b() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(1, 0x194u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085cd05); }
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700085cd06); };
            c.w64(3, t30);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085cd17, 0x700085cd12);
        }
        27 => {
            return RegfileExit::fault(0x700085cd17);
        }
        28 => {
            c.w64(1, c.r64(0));
            let Some(t31) = c.ld8(c.r64(1).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085cd1b); };
            c.logic(t31 & 0x4u64, 8);
            if c.cc_ne() { blk = 68; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(1, c.r64(7));
            c.cmp(c.r32(1), 0x15u64, 32);
            if c.cc_l() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(1), 0x1au64, 32);
            if c.cc_l() { blk = 33; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(1), 0x1au64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(1), 0x1bu64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(1), 0x15u64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(1), 0x16u64, 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(1), 0x13u64, 32);
            if c.cc_l() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(1), 0x13u64, 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(1), 0x14u64, 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(1), 0x11u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(1), 0x12u64, 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            return RegfileExit::fault(0x700085cd6f);
        }
        41 => {
            c.w64(1, c.r64(8));
            c.w64(7, c.r64(9));
            let r32 = c.add(c.r64(1), c.r64(7), 64);
            c.w64(1, r32);
            let Some(t33) = c.ld8(c.r64(1).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x700085cd79); };
            c.w32(1, (t33 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 42; continue;
        }
        42 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085cd81); };
            c.w64(1, t34);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085cd88); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            return c.ret(0x18, 0x700085cd8f);
        }
        44 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085cd92); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085cd99); }
            return c.ret(0x0, 0x700085cd9b);
        }
        45 => {
            c.w64(1, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t35) = c.ld8(c.r64(1).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x700085cda0); };
            c.w32(1, t35);
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 42; continue;
        }
        46 => {
            c.w64(1, c.r64(8).wrapping_add(c.r64(9)));
            c.w64(7, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t36) = c.ld16(c.r64(7).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x700085cdb2); };
            c.w32(1, (t36 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 42; continue;
        }
        47 => {
            c.w64(1, c.r64(8).wrapping_add(c.r64(9)));
            c.w64(7, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t37) = c.ld16(c.r64(7).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x700085cdc3); };
            c.w32(1, t37);
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 42; continue;
        }
        48 => {
            c.w64(1, c.r64(11).wrapping_mul(4));
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t38) = c.ld32(c.r64(1).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700085cdd8); };
            c.w32(1, t38);
            c.w64(7, c.r64(1));
            let r39 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r39);
            if c.cc_o() { blk = 74; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 42; continue;
        }
        50 => {
            c.w64(1, c.r64(8).wrapping_add(c.r64(9)));
            c.w64(7, c.r64(11).wrapping_mul(4));
            let Some(t40) = c.ld32(c.r64(7).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x700085cdf7); };
            c.w32(1, t40);
            c.cmp(c.r32(1), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 79; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w32(1, c.r32(1));
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 42; continue;
        }
        52 => {
            c.w64(1, c.r64(8).wrapping_add(c.r64(9)));
            c.w64(7, c.r64(11).wrapping_mul(8));
            let Some(t41) = c.ld64(c.r64(7).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x700085ce1c); };
            c.w64(1, t41);
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_e() { blk = 58; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085ce26); };
            c.w64(7, t42);
            let Some(t43) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700085ce2d); };
            c.w64(8, t43);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085ce34); };
            c.w64(11, t44);
            let Some(t45) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700085ce3b); };
            c.cmp(t45, c.r64(9), 64);
            if c.cc_be() { blk = 84; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700085ce44); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 55; continue;
        }
        55 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085ce4b); };
            c.w64(8, t46);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700085ce52); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700085ce5c); }
            let r47 = c.neg(c.r64(1), 64);
            c.w64(1, r47);
            if !c.st64(c.r64(7).wrapping_add(0x7), c.r64(1)) { return RegfileExit::fault(0x700085ce66); }
            c.w64(0, c.r64(7));
            blk = 42; continue;
        }
        57 => {
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700085ce72); }
            if !c.st64(c.r64(7).wrapping_add(0x7), c.r64(1)) { return RegfileExit::fault(0x700085ce79); }
            c.w64(0, c.r64(7));
            blk = 42; continue;
        }
        58 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085ce85); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085ce8c); };
            c.w64(7, t49);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085ce93); };
            c.w64(8, t50);
            let Some(t51) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085ce9a); };
            c.cmp(t51, c.r64(9), 64);
            if c.cc_be() { blk = 86; continue; }
            blk = 59; continue;
        }
        59 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700085cea3); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 60; continue;
        }
        60 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085ceaa); };
            c.w64(7, t52);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085ceb1); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700085ceb4); }
            c.w64(0, c.r64(1));
            blk = 42; continue;
        }
        61 => {
            c.w64(1, c.r64(8).wrapping_add(c.r64(9)));
            c.w64(7, c.r64(11).wrapping_mul(8));
            let Some(t53) = c.ld64(c.r64(7).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x700085cecf); };
            c.w64(1, t53);
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_e() { blk = 65; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085ced9); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700085cee0); };
            c.w64(8, t55);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085cee7); };
            c.w64(11, t56);
            let Some(t57) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700085ceee); };
            c.cmp(t57, c.r64(9), 64);
            if c.cc_be() { blk = 88; continue; }
            blk = 63; continue;
        }
        63 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700085cef7); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 64; continue;
        }
        64 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085cefe); };
            c.w64(8, t58);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700085cf05); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700085cf09); }
            if !c.st64(c.r64(7).wrapping_add(0x7), c.r64(1)) { return RegfileExit::fault(0x700085cf10); }
            c.w64(0, c.r64(7));
            blk = 42; continue;
        }
        65 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085cf1c); };
            c.w64(1, t59);
            let Some(t60) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085cf23); };
            c.w64(7, t60);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085cf2a); };
            c.w64(8, t61);
            let Some(t62) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085cf31); };
            c.cmp(t62, c.r64(9), 64);
            if c.cc_be() { blk = 90; continue; }
            blk = 66; continue;
        }
        66 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700085cf3a); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 67; continue;
        }
        67 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085cf41); };
            c.w64(7, t63);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085cf48); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700085cf4b); }
            c.w64(0, c.r64(1));
            blk = 42; continue;
        }
        68 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085cf5f); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085cf60); };
            c.w64(1, t64);
            let Some(t65) = c.ld32(c.r64(1).wrapping_add(0x83)) else { return RegfileExit::fault(0x700085cf67); };
            c.w32(1, t65);
            let r66 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r66);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085cf70); }
            c.w32(0, 0x2u64);
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085cf76); };
            c.w64(3, t67);
            return c.call(0x700086ed74, 0x700085cf82, 0x700085cf7d);
        }
        69 => {
            return RegfileExit::fault(0x700085cf82);
        }
        70 => {
            c.w32(7, 0xdcu64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085cf88); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085cf89); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085cf8a); };
            c.w64(3, t68);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700085cf9b, 0x700085cf96);
        }
        71 => {
            return RegfileExit::fault(0x700085cf9b);
        }
        72 => {
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085cf9c); };
            c.w64(3, t69);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085cfaa, 0x700085cfa5);
        }
        73 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085cfaa); };
            c.w64(6, t70);
            blk = 1; continue;
        }
        74 => {
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085cfb3); };
            c.w64(7, t71);
            let Some(t72) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700085cfba); };
            c.w64(8, t72);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700085cfc1); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085cfc5); };
            c.w64(11, t73);
            let Some(t74) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700085cfcc); };
            c.cmp(t74, c.r64(9), 64);
            if c.cc_a() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700085cfdb, 0x700085cfd6);
        }
        76 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085cfdb); };
            c.w64(1, t75);
            blk = 78; continue;
        }
        77 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700085cfe1); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 78; continue;
        }
        78 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085cff3); };
            c.w64(1, t76);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700085cffa); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700085cffd); }
            blk = 42; continue;
        }
        79 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085d007); };
            c.w64(7, t77);
            let Some(t78) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700085d00e); };
            c.w64(8, t78);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700085d015); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085d019); };
            c.w64(11, t79);
            let Some(t80) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700085d020); };
            c.cmp(t80, c.r64(9), 64);
            if c.cc_a() { blk = 82; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700085d02f, 0x700085d02a);
        }
        81 => {
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085d02f); };
            c.w64(1, t81);
            blk = 83; continue;
        }
        82 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700085d035); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 83; continue;
        }
        83 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085d03f); };
            c.w64(7, t82);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085d046); }
            c.w32(10, c.r32(1));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700085d055); }
            blk = 42; continue;
        }
        84 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700085d064); }
            return c.call(0x70007febb4, 0x700085d06d, 0x700085d068);
        }
        85 => {
            c.w64(7, c.r64(0));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085d070); };
            c.w64(1, t83);
            blk = 55; continue;
        }
        86 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700085d083, 0x700085d07e);
        }
        87 => {
            c.w64(1, c.r64(0));
            blk = 60; continue;
        }
        88 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700085d090); }
            return c.call(0x70007febb4, 0x700085d099, 0x700085d094);
        }
        89 => {
            c.w64(7, c.r64(0));
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085d09c); };
            c.w64(1, t84);
            blk = 64; continue;
        }
        90 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700085d0af, 0x700085d0aa);
        }
        91 => {
            c.w64(1, c.r64(0));
            blk = 67; continue;
        }
        92 => {
            return RegfileExit::jump(0x700085d0b8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085d0d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085d0d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700085d0d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085d0d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085d0da); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085d0df); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700085d0e3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 75; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085d0ed); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085d0fa); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700085d100); }
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085d104); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 73; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085d110); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085d116); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085d11c); };
            c.cmp(t9, 0x0u64, 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(7));
            blk = 8; continue;
        }
        6 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700085d128); }
            c.w32(0, 0x1u64);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2b28)) else { return RegfileExit::fault(0x700085d12e); };
            c.w64(3, t10);
            return c.call(0x700086ed74, 0x700085d13a, 0x700085d135);
        }
        7 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085d13a); };
            c.w64(1, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085d13e); };
            c.w64(2, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085d142); };
            c.w64(6, t13);
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700085d146); }
            let Some(t14) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085d14a); };
            c.logic(t14 & 0x4u64, 8);
            if c.cc_ne() { blk = 71; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t15) = c.ld8(c.r64(1).wrapping_add(0xa)) else { return RegfileExit::fault(0x700085d154); };
            c.w32(1, t15);
            let r16 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r16);
            let r17 = c.r32(1) & 0x3fu64;
            c.w32(1, r17);
            c.logic(r17, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x700085d15e); }
            c.cmp(c.r32(1), 0x17u64, 32);
            if c.cc_l() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(1), 0x19u64, 32);
            if c.cc_le() { blk = 73; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t18) = c.ld64(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085d170); };
            c.w64(7, t18);
            let Some(t19) = c.ld64(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085d174); };
            c.w64(8, t19);
            let Some(t20) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700085d178); };
            c.w64(2, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700085d17c); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700085d180); }
            let Some(t21) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085d184); };
            c.w64(9, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x700085d188); }
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085d18c); };
            c.cmp(t22, c.r32(9), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(9));
            return c.call(0x7000899a14, 0x700085d1a7, 0x700085d1a2);
        }
        14 => {
            c.w64(9, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085d1aa); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085d1ae); };
            c.w64(7, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085d1b2); };
            c.w64(8, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085d1b6); };
            c.w64(0, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085d1ba); };
            c.w64(1, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085d1be); };
            c.w64(2, t28);
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 25; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700085d1da); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(9, cv);
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_s() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(10, 0x8000000000000000u64);
            let r29 = c.r64(9) | c.r64(10);
            c.w64(9, r29);
            c.logic(r29, 64);
            blk = 23; continue;
        }
        23 => {
            blk = 25; continue;
        }
        24 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700085d239); }
            c.cmp(c.r64(9), c.r64(2), 64);
            if c.cc_b() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(2, 0x194u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700085d247); }
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700085d248); };
            c.w64(3, t30);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085d259, 0x700085d254);
        }
        27 => {
            return RegfileExit::fault(0x700085d259);
        }
        28 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700085d25a); };
            c.w64(2, t31);
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(1), 0x16u64, 32);
            if c.cc_g() { blk = 58; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x700085d280, 0x700085d27b);
        }
        31 => {
            c.w64(2, c.r64(0));
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085d283); };
            c.w64(6, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085d287); };
            c.w64(9, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085d28b); };
            c.w64(7, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085d28f); };
            c.w64(8, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085d293); };
            c.w64(0, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085d297); };
            c.w64(1, t37);
            blk = 32; continue;
        }
        32 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x700085d29b); }
            c.w64(12, c.r64(0));
            let Some(t38) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085d2a2); };
            c.logic(t38 & 0x4u64, 8);
            if c.cc_ne() { blk = 71; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(11, c.r64(2));
            blk = 35; continue;
        }
        34 => {
            c.w64(11, c.r64(0));
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t39) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085d2bc); };
            c.w32(12, t39);
            let r40 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r40);
            let Some(t41) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085d2c3); };
            c.cmp(t41, c.r32(12), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t42) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085d2cc); };
            c.cmp(t42, 0x43u64, 16);
            if c.cc_e() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a1014, 0x700085d2dd, 0x700085d2d8);
        }
        39 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085d2dd); };
            c.w64(6, t43);
            blk = 34; continue;
        }
        40 => {
            let Some(t44) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085d2e3); };
            c.w32(11, t44);
            let r45 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r45);
            c.w64(0, c.r64(11));
            blk = 34; continue;
        }
        41 => {
            let r46 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r46);
            blk = 44; continue;
        }
        42 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700085d2f4); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_o() { blk = 77; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w32(11, c.r32(11));
            blk = 44; continue;
        }
        44 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085d30c); };
            c.w64(2, t47);
            c.cmp(c.r32(2), 0x14u64, 32);
            if c.cc_l() { blk = 48; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(2), 0x14u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(2), 0x15u64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(2), 0x16u64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(2), 0x11u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(2), 0x12u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(2), 0x13u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 51; continue;
        }
        51 => {
            return RegfileExit::fault(0x700085d333);
        }
        52 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085d334); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085d338); };
            c.w64(7, t49);
            c.w64(2, c.r64(1).wrapping_add(c.r64(7)));
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085d340); };
            c.w64(9, t50);
            let Some(xm51) = c.xchg_mem(c.r64(2).wrapping_add(c.r64(9)), c.r8(11), 8) else { return RegfileExit::fault(0x700085d344); };
            c.w8(11, xm51);
            c.w32(11, (c.r8(11) as u8 as i8 as i64 as u64));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085d34c); };
            c.w64(0, t52);
            blk = 53; continue;
        }
        53 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085d350); };
            c.w64(1, t53);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085d357); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_g() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            return c.ret(0x20, 0x700085d35e);
        }
        55 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085d361); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085d368); }
            return c.ret(0x0, 0x700085d36a);
        }
        56 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085d36b); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085d36f); };
            c.w64(1, t55);
            c.w64(2, c.r64(1).wrapping_add(c.r64(7)));
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085d377); };
            c.w64(9, t56);
            c.w64(1, c.r64(9).wrapping_add(c.r64(9)));
            let Some(xm57) = c.xchg_mem(c.r64(2).wrapping_add(c.r64(1)), c.r16(11), 16) else { return RegfileExit::fault(0x700085d37f); };
            c.w16(11, xm57);
            c.w32(11, (c.r16(11) as u16 as i16 as i64 as u64));
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085d388); };
            c.w64(0, t58);
            blk = 53; continue;
        }
        57 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085d38e); };
            c.w64(7, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085d392); };
            c.w64(1, t60);
            c.w64(2, c.r64(1).wrapping_add(c.r64(7)));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085d39a); };
            c.w64(9, t61);
            c.w64(1, c.r64(9).wrapping_mul(4));
            let Some(xm62) = c.xchg_mem(c.r64(2).wrapping_add(c.r64(1)), c.r32(11), 32) else { return RegfileExit::fault(0x700085d3a6); };
            c.w32(11, xm62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085d3aa); };
            c.w64(0, t63);
            blk = 53; continue;
        }
        58 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 69; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t64) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085d3ba); };
            c.w32(1, t64);
            let r65 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r65);
            let Some(t66) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085d3c0); };
            c.cmp(t66, 0x41u64, 16);
            if c.cc_e() { blk = 62; continue; }
            blk = 60; continue;
        }
        60 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700085d3c7); }
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x700085d3c8); };
            c.w64(3, t67);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085d3d9, 0x700085d3d4);
        }
        61 => {
            c.w64(2, c.r64(0));
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085d3dc); };
            c.w64(6, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085d3e0); };
            c.w64(9, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085d3e4); };
            c.w64(7, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085d3e8); };
            c.w64(8, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085d3ec); };
            c.w64(0, t72);
            blk = 62; continue;
        }
        62 => {
            c.w64(1, c.r64(0));
            let Some(t73) = c.ld8(c.r64(1).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085d3f3); };
            c.logic(t73 & 0x4u64, 8);
            if c.cc_ne() { blk = 71; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t74) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085d3f9); };
            c.w32(1, t74);
            let r75 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r75);
            c.logic(c.r32(1) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t76) = c.ld64(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085d407); };
            c.w64(8, t76);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            let r77 = c.neg(c.r64(8), 64);
            c.w64(8, r77);
            c.w64(1, c.r64(8));
            blk = 68; continue;
        }
        66 => {
            c.w64(1, c.r64(8));
            blk = 68; continue;
        }
        67 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 68; continue;
        }
        68 => {
            c.w64(8, c.r64(9).wrapping_mul(8));
            let Some(xm78) = c.xchg_mem(c.r64(7).wrapping_add(c.r64(8)), c.r64(1), 64) else { return RegfileExit::fault(0x700085d427); };
            c.w64(1, xm78);
            c.w64(0, c.r64(2));
            blk = 53; continue;
        }
        69 => {
            c.w32(1, 0x28u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085d438); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700085d439); }
            c.w32(0, 0x2u64);
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085d43f); };
            c.w64(3, t79);
            return c.call(0x700086ed74, 0x700085d44b, 0x700085d446);
        }
        70 => {
            return RegfileExit::fault(0x700085d44b);
        }
        71 => {
            c.w32(2, 0x66u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700085d451); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085d452); };
            c.w64(2, t80);
            let Some(t81) = c.ld32(c.r64(2).wrapping_add(0x87)) else { return RegfileExit::fault(0x700085d459); };
            c.w32(2, t81);
            let r82 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r82);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700085d462); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085d463); };
            c.w64(3, t83);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700085d474, 0x700085d46f);
        }
        72 => {
            return RegfileExit::fault(0x700085d474);
        }
        73 => {
            c.w32(1, 0xdcu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085d47a); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700085d47b); }
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085d47c); };
            c.w64(3, t84);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700085d48d, 0x700085d488);
        }
        74 => {
            return RegfileExit::fault(0x700085d48d);
        }
        75 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085d48e); };
            c.w64(3, t85);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085d49c, 0x700085d497);
        }
        76 => {
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085d49c); };
            c.w64(6, t86);
            blk = 1; continue;
        }
        77 => {
            let r87 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r87);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700085d4a9); }
            return c.call(0x70008708b4, 0x700085d4b3, 0x700085d4ae);
        }
        78 => {
            let Some(t88) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700085d4b3); };
            c.w32(11, t88);
            let r89 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r89);
            blk = 43; continue;
        }
        79 => {
            return RegfileExit::jump(0x700085d4c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085d4d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085d4d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700085d4d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085d4d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085d4da); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085d4df); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700085d4e3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 97; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085d4ed); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 95; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085d4fa); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700085d500); }
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085d504); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 95; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085d510); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085d517); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085d51e); };
            c.cmp(t9, 0x0u64, 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(8));
            blk = 8; continue;
        }
        6 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085d52a); }
            c.w32(0, 0x1u64);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2b28)) else { return RegfileExit::fault(0x700085d530); };
            c.w64(3, t10);
            return c.call(0x700086ed74, 0x700085d53c, 0x700085d537);
        }
        7 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085d53c); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085d540); };
            c.w64(1, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085d544); };
            c.w64(6, t13);
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700085d548); }
            let Some(t14) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085d54c); };
            c.logic(t14 & 0x4u64, 8);
            if c.cc_ne() { blk = 93; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t15) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x700085d556); };
            c.w32(7, t15);
            c.w64(8, c.r64(7));
            let r16 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r16);
            let r17 = c.r32(8) & 0x3fu64;
            c.w32(8, r17);
            c.logic(r17, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700085d565); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700085d569); }
            c.cmp(c.r32(8), 0x17u64, 32);
            if c.cc_l() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(8), 0x19u64, 32);
            if c.cc_le() { blk = 95; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t18) = c.ld64(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085d57d); };
            c.w64(9, t18);
            let Some(t19) = c.ld64(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085d581); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700085d585); };
            c.w64(1, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700085d589); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700085d58d); }
            let Some(t21) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085d591); };
            c.w64(12, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x700085d595); }
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085d599); };
            c.cmp(t22, c.r32(12), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(12));
            return c.call(0x7000899a14, 0x700085d5b4, 0x700085d5af);
        }
        14 => {
            c.w64(12, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085d5b7); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085d5bb); };
            c.w64(9, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085d5bf); };
            c.w64(11, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085d5c3); };
            c.w64(7, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085d5c7); };
            c.w64(0, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085d5cb); };
            c.w64(8, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700085d5cf); };
            c.w64(1, t29);
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            blk = 25; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700085d5eb); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_s() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(10, 0x8000000000000000u64);
            let r30 = c.r64(12) | c.r64(10);
            c.w64(12, r30);
            c.logic(r30, 64);
            blk = 23; continue;
        }
        23 => {
            blk = 25; continue;
        }
        24 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700085d64b); }
            c.cmp(c.r64(12), c.r64(1), 64);
            if c.cc_b() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(1, 0x194u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085d659); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700085d65a); };
            c.w64(3, t31);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085d66b, 0x700085d666);
        }
        27 => {
            return RegfileExit::fault(0x700085d66b);
        }
        28 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700085d66c); };
            c.w64(1, t32);
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(8), 0x16u64, 32);
            if c.cc_g() { blk = 63; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x700085d693, 0x700085d68e);
        }
        31 => {
            c.w64(1, c.r64(0));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085d696); };
            c.w64(6, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085d69a); };
            c.w64(12, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085d69e); };
            c.w64(9, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085d6a2); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085d6a6); };
            c.w64(0, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085d6aa); };
            c.w64(8, t38);
            blk = 32; continue;
        }
        32 => {
            let Some(t39) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085d6ae); };
            c.logic(t39 & 0x4u64, 8);
            if c.cc_ne() { blk = 93; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 35; continue;
        }
        34 => {
            c.w64(1, c.r64(0));
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t40) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085d6c2); };
            c.w32(7, t40);
            let r41 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r41);
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085d6c8); };
            c.cmp(t42, c.r32(7), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t43) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085d6d1); };
            c.cmp(t43, 0x43u64, 16);
            if c.cc_e() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x700085d6e0, 0x700085d6db);
        }
        39 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085d6e0); };
            c.w64(6, t44);
            blk = 34; continue;
        }
        40 => {
            let Some(t45) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085d6e6); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            c.w64(0, c.r64(1));
            blk = 34; continue;
        }
        41 => {
            let r47 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r47);
            blk = 44; continue;
        }
        42 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700085d6f5); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 117; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w32(1, c.r32(1));
            blk = 44; continue;
        }
        44 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085d70b); };
            c.w64(7, t48);
            c.cmp(c.r32(7), 0x14u64, 32);
            if c.cc_l() { blk = 48; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(7), 0x14u64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(7), 0x15u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(7), 0x16u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(7), 0x11u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(7), 0x12u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(7), 0x13u64, 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 51; continue;
        }
        51 => {
            return RegfileExit::fault(0x700085d73a);
        }
        52 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085d73b); };
            c.w64(8, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085d73f); };
            c.w64(9, t50);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085d747); };
            c.w64(12, t51);
            let Some(xm52) = c.xchg_mem(c.r64(7).wrapping_add(c.r64(12)), c.r8(1), 8) else { return RegfileExit::fault(0x700085d74b); };
            c.w8(1, xm52);
            c.w32(1, (c.r8(1) as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 53; continue;
        }
        53 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085d755); };
            c.w64(1, t53);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085d75c); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_g() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            return c.ret(0x20, 0x700085d763);
        }
        55 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085d766); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085d76d); }
            return c.ret(0x0, 0x700085d76f);
        }
        56 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085d770); };
            c.w64(9, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085d774); };
            c.w64(8, t55);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085d77c); };
            c.w64(12, t56);
            let Some(xm57) = c.xchg_mem(c.r64(7).wrapping_add(c.r64(12)), c.r8(1), 8) else { return RegfileExit::fault(0x700085d780); };
            c.w8(1, xm57);
            c.w32(1, c.r8(1));
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 53; continue;
        }
        57 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085d78c); };
            c.w64(9, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085d790); };
            c.w64(8, t59);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085d798); };
            c.w64(12, t60);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            let Some(xm61) = c.xchg_mem(c.r64(7).wrapping_add(c.r64(8)), c.r16(1), 16) else { return RegfileExit::fault(0x700085d7a0); };
            c.w16(1, xm61);
            c.w32(1, (c.r16(1) as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 53; continue;
        }
        58 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085d7ad); };
            c.w64(9, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085d7b1); };
            c.w64(8, t63);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085d7b9); };
            c.w64(12, t64);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            let Some(xm65) = c.xchg_mem(c.r64(7).wrapping_add(c.r64(8)), c.r16(1), 16) else { return RegfileExit::fault(0x700085d7c1); };
            c.w16(1, xm65);
            c.w32(1, c.r16(1));
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 53; continue;
        }
        59 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085d7ce); };
            c.w64(12, t66);
            c.w64(7, c.r64(12).wrapping_mul(4));
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085d7da); };
            c.w64(8, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085d7de); };
            c.w64(9, t68);
            let r69 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r69);
            let Some(xm70) = c.xchg_mem(c.r64(8).wrapping_add(c.r64(7)), c.r32(1), 32) else { return RegfileExit::fault(0x700085d7e5); };
            c.w32(1, xm70);
            c.w64(7, c.r64(1));
            let r71 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r71);
            if c.cc_o() { blk = 99; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 53; continue;
        }
        61 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085d7fc); };
            c.w64(9, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085d800); };
            c.w64(8, t73);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085d808); };
            c.w64(12, t74);
            c.w64(8, c.r64(12).wrapping_mul(4));
            let Some(xm75) = c.xchg_mem(c.r64(7).wrapping_add(c.r64(8)), c.r32(1), 32) else { return RegfileExit::fault(0x700085d814); };
            c.w32(1, xm75);
            c.cmp(c.r32(1), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 104; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w32(1, c.r32(1));
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 53; continue;
        }
        63 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t76) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085d838); };
            c.w32(8, t76);
            let r77 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r77);
            let Some(t78) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085d83f); };
            c.cmp(t78, 0x41u64, 16);
            if c.cc_e() { blk = 67; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085d847); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x700085d848); };
            c.w64(3, t79);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085d859, 0x700085d854);
        }
        66 => {
            c.w64(1, c.r64(0));
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085d85c); };
            c.w64(6, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085d860); };
            c.w64(12, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085d864); };
            c.w64(9, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085d868); };
            c.w64(11, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085d86c); };
            c.w64(7, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085d870); };
            c.w64(0, t85);
            blk = 67; continue;
        }
        67 => {
            c.w64(8, c.r64(0));
            let Some(t86) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085d877); };
            c.logic(t86 & 0x4u64, 8);
            if c.cc_ne() { blk = 93; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t87) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085d882); };
            c.w32(8, t87);
            c.logic(c.r32(8) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t88) = c.ld64(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085d88f); };
            c.w64(1, t88);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 70; continue;
        }
        70 => {
            let r89 = c.neg(c.r64(1), 64);
            c.w64(1, r89);
            blk = 72; continue;
        }
        71 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 72; continue;
        }
        72 => {
            let r90 = c.r32(7) & 0xfcu64;
            c.w32(7, r90);
            c.logic(r90, 32);
            c.cmp(c.r32(7), 0x6cu64, 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(7), 0x68u64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            return RegfileExit::fault(0x700085d8b4);
        }
        75 => {
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            let r91 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r91);
            c.w64(8, c.r64(12).wrapping_mul(8));
            let Some(xm92) = c.xchg_mem(c.r64(7).wrapping_add(c.r64(8)), c.r64(1), 64) else { return RegfileExit::fault(0x700085d8c6); };
            c.w64(1, xm92);
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_e() { blk = 79; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085d8d0); };
            c.w64(7, t93);
            let Some(t94) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700085d8d7); };
            c.w64(8, t94);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085d8de); };
            c.w64(11, t95);
            let Some(t96) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700085d8e5); };
            c.cmp(t96, c.r64(9), 64);
            if c.cc_be() { blk = 109; continue; }
            blk = 77; continue;
        }
        77 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700085d8ee); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 78; continue;
        }
        78 => {
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085d8f5); };
            c.w64(8, t97);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700085d8fc); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700085d900); }
            if !c.st64(c.r64(7).wrapping_add(0x7), c.r64(1)) { return RegfileExit::fault(0x700085d907); }
            c.w64(0, c.r64(7));
            blk = 53; continue;
        }
        79 => {
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085d913); };
            c.w64(1, t98);
            let Some(t99) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085d91a); };
            c.w64(7, t99);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085d921); };
            c.w64(8, t100);
            let Some(t101) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085d928); };
            c.cmp(t101, c.r64(9), 64);
            if c.cc_be() { blk = 111; continue; }
            blk = 80; continue;
        }
        80 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700085d931); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 81; continue;
        }
        81 => {
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085d938); };
            c.w64(7, t102);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085d93f); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700085d942); }
            c.w64(0, c.r64(1));
            blk = 53; continue;
        }
        82 => {
            c.w64(7, c.r64(9).wrapping_add(c.r64(11)));
            c.w64(8, c.r64(12).wrapping_mul(8));
            let Some(xm103) = c.xchg_mem(c.r64(7).wrapping_add(c.r64(8)), c.r64(1), 64) else { return RegfileExit::fault(0x700085d95d); };
            c.w64(1, xm103);
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_e() { blk = 88; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085d967); };
            c.w64(7, t104);
            let Some(t105) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700085d96e); };
            c.w64(8, t105);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085d975); };
            c.w64(11, t106);
            let Some(t107) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700085d97c); };
            c.cmp(t107, c.r64(9), 64);
            if c.cc_be() { blk = 113; continue; }
            blk = 84; continue;
        }
        84 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700085d985); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 85; continue;
        }
        85 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085d98c); };
            c.w64(8, t108);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700085d993); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700085d99d); }
            let r109 = c.neg(c.r64(1), 64);
            c.w64(1, r109);
            if !c.st64(c.r64(7).wrapping_add(0x7), c.r64(1)) { return RegfileExit::fault(0x700085d9a7); }
            c.w64(0, c.r64(7));
            blk = 53; continue;
        }
        87 => {
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700085d9b3); }
            if !c.st64(c.r64(7).wrapping_add(0x7), c.r64(1)) { return RegfileExit::fault(0x700085d9ba); }
            c.w64(0, c.r64(7));
            blk = 53; continue;
        }
        88 => {
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085d9c6); };
            c.w64(1, t110);
            let Some(t111) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085d9cd); };
            c.w64(7, t111);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085d9d4); };
            c.w64(8, t112);
            let Some(t113) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085d9db); };
            c.cmp(t113, c.r64(9), 64);
            if c.cc_be() { blk = 115; continue; }
            blk = 89; continue;
        }
        89 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700085d9e4); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 90; continue;
        }
        90 => {
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085d9eb); };
            c.w64(7, t114);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085d9f2); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700085d9f5); }
            c.w64(0, c.r64(1));
            blk = 53; continue;
        }
        91 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085da09); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085da0a); }
            c.w32(0, 0x2u64);
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085da10); };
            c.w64(3, t115);
            return c.call(0x700086ed74, 0x700085da1c, 0x700085da17);
        }
        92 => {
            return RegfileExit::fault(0x700085da1c);
        }
        93 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085da22); }
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085da23); };
            c.w64(1, t116);
            let Some(t117) = c.ld32(c.r64(1).wrapping_add(0x8b)) else { return RegfileExit::fault(0x700085da2a); };
            c.w32(1, t117);
            let r118 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r118);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085da33); }
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085da34); };
            c.w64(3, t119);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700085da45, 0x700085da40);
        }
        94 => {
            return RegfileExit::fault(0x700085da45);
        }
        95 => {
            c.w32(7, 0xdcu64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085da4b); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085da4c); }
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085da4d); };
            c.w64(3, t120);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700085da5e, 0x700085da59);
        }
        96 => {
            return RegfileExit::fault(0x700085da5e);
        }
        97 => {
            let Some(t121) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085da5f); };
            c.w64(3, t121);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085da6d, 0x700085da68);
        }
        98 => {
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085da6d); };
            c.w64(6, t122);
            blk = 1; continue;
        }
        99 => {
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085da76); };
            c.w64(7, t123);
            let Some(t124) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700085da7d); };
            c.w64(8, t124);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700085da84); }
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085da88); };
            c.w64(11, t125);
            let Some(t126) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700085da8f); };
            c.cmp(t126, c.r64(9), 64);
            if c.cc_a() { blk = 102; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700085da9e, 0x700085da99);
        }
        101 => {
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085da9e); };
            c.w64(1, t127);
            blk = 103; continue;
        }
        102 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700085daa4); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 103; continue;
        }
        103 => {
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085daae); };
            c.w64(7, t128);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085dab5); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700085dac0); }
            blk = 53; continue;
        }
        104 => {
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085daca); };
            c.w64(7, t129);
            let Some(t130) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700085dad1); };
            c.w64(8, t130);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700085dad8); }
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085dadc); };
            c.w64(11, t131);
            let Some(t132) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700085dae3); };
            c.cmp(t132, c.r64(9), 64);
            if c.cc_a() { blk = 107; continue; }
            blk = 105; continue;
        }
        105 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700085daf2, 0x700085daed);
        }
        106 => {
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085daf2); };
            c.w64(1, t133);
            blk = 108; continue;
        }
        107 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700085daf8); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 108; continue;
        }
        108 => {
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085db02); };
            c.w64(7, t134);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085db09); }
            c.w32(10, c.r32(1));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700085db18); }
            blk = 53; continue;
        }
        109 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700085db27); }
            return c.call(0x70007febb4, 0x700085db30, 0x700085db2b);
        }
        110 => {
            c.w64(7, c.r64(0));
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085db33); };
            c.w64(1, t135);
            blk = 78; continue;
        }
        111 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700085db46, 0x700085db41);
        }
        112 => {
            c.w64(1, c.r64(0));
            blk = 81; continue;
        }
        113 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700085db53); }
            return c.call(0x70007febb4, 0x700085db5c, 0x700085db57);
        }
        114 => {
            c.w64(7, c.r64(0));
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085db5f); };
            c.w64(1, t136);
            blk = 85; continue;
        }
        115 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700085db72, 0x700085db6d);
        }
        116 => {
            c.w64(1, c.r64(0));
            blk = 90; continue;
        }
        117 => {
            let r137 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r137);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700085db7e); }
            return c.call(0x70008708b4, 0x700085db88, 0x700085db83);
        }
        118 => {
            let Some(t138) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700085db88); };
            c.w32(1, t138);
            let r139 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r139);
            blk = 43; continue;
        }
        119 => {
            return RegfileExit::jump(0x700085db98);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085dbb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085dbb4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700085dbb8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085dbb9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085dbba); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085dbbf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700085dbc3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 122; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085dbcd); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 120; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085dbda); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700085dbe0); }
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085dbe4); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 120; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085dbf0); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085dbf7); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085dbfe); };
            c.cmp(t9, 0x0u64, 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(8));
            blk = 8; continue;
        }
        6 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085dc0a); }
            c.w32(0, 0x1u64);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2b28)) else { return RegfileExit::fault(0x700085dc10); };
            c.w64(3, t10);
            return c.call(0x700086ed74, 0x700085dc1c, 0x700085dc17);
        }
        7 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085dc1c); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085dc20); };
            c.w64(1, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085dc24); };
            c.w64(6, t13);
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700085dc28); }
            let Some(t14) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085dc2c); };
            c.logic(t14 & 0x4u64, 8);
            if c.cc_ne() { blk = 118; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t15) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x700085dc36); };
            c.w32(7, t15);
            c.w64(8, c.r64(7));
            let r16 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r16);
            let r17 = c.r32(8) & 0x3fu64;
            c.w32(8, r17);
            c.logic(r17, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700085dc45); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700085dc49); }
            c.cmp(c.r32(8), 0x17u64, 32);
            if c.cc_l() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(8), 0x19u64, 32);
            if c.cc_le() { blk = 120; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t18) = c.ld64(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085dc5d); };
            c.w64(9, t18);
            let Some(t19) = c.ld64(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085dc61); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700085dc65); };
            c.w64(1, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700085dc69); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700085dc6d); }
            let Some(t21) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085dc71); };
            c.w64(12, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x700085dc75); }
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085dc79); };
            c.cmp(t22, c.r32(12), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(12));
            return c.call(0x7000899a14, 0x700085dc94, 0x700085dc8f);
        }
        14 => {
            c.w64(12, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085dc97); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085dc9b); };
            c.w64(9, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085dc9f); };
            c.w64(11, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085dca3); };
            c.w64(7, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085dca7); };
            c.w64(0, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085dcab); };
            c.w64(8, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700085dcaf); };
            c.w64(1, t29);
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            blk = 25; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700085dccb); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_s() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(10, 0x8000000000000000u64);
            let r30 = c.r64(12) | c.r64(10);
            c.w64(12, r30);
            c.logic(r30, 64);
            blk = 23; continue;
        }
        23 => {
            blk = 25; continue;
        }
        24 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700085dd2b); }
            c.cmp(c.r64(12), c.r64(1), 64);
            if c.cc_b() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(1, 0x194u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085dd39); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700085dd3a); };
            c.w64(3, t31);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085dd4b, 0x700085dd46);
        }
        27 => {
            return RegfileExit::fault(0x700085dd4b);
        }
        28 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700085dd4c); };
            c.w64(1, t32);
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(8), 0x16u64, 32);
            if c.cc_g() { blk = 76; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700085dd65); };
            c.w64(7, t33);
            c.w32(3, c.r32(7));
            let r34 = c.r32(3) & 0x1u64;
            c.w32(3, r34);
            c.logic(r34, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700085dd6e); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x700085dd80, 0x700085dd7b);
        }
        31 => {
            c.w64(1, c.r64(0));
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085dd83); };
            c.w64(6, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700085dd87); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085dd8b); };
            c.w64(12, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085dd8f); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085dd93); };
            c.w64(11, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085dd97); };
            c.w64(0, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085dd9b); };
            c.w64(8, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085dd9f); };
            c.w64(3, t42);
            blk = 32; continue;
        }
        32 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x700085dda3); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(0, c.r64(7));
            return c.call(0x7000899a14, 0x700085ddb4, 0x700085ddaf);
        }
        34 => {
            c.w64(7, c.r64(0));
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085ddb7); };
            c.w64(6, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085ddbb); };
            c.w64(12, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085ddbf); };
            c.w64(9, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085ddc3); };
            c.w64(11, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085ddc7); };
            c.w64(0, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085ddcb); };
            c.w64(8, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700085ddcf); };
            c.w64(1, t49);
            blk = 35; continue;
        }
        35 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x700085ddd3); }
            let Some(t50) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085ddd7); };
            c.logic(t50 & 0x4u64, 8);
            if c.cc_ne() { blk = 118; continue; }
            blk = 36; continue;
        }
        36 => {
            blk = 38; continue;
        }
        37 => {
            c.w64(1, c.r64(0));
            blk = 38; continue;
        }
        38 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t51) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085ddeb); };
            c.w32(15, t51);
            let r52 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r52);
            let Some(t53) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085ddf2); };
            c.cmp(t53, c.r32(15), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t54) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085ddfb); };
            c.cmp(t54, 0x43u64, 16);
            if c.cc_e() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x700085de0b, 0x700085de06);
        }
        42 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085de0b); };
            c.w64(6, t55);
            blk = 37; continue;
        }
        43 => {
            let Some(t56) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085de11); };
            c.w32(1, t56);
            let r57 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r57);
            c.w64(0, c.r64(1));
            blk = 37; continue;
        }
        44 => {
            let r58 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r58);
            blk = 47; continue;
        }
        45 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700085de20); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 144; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(1, c.r32(1));
            blk = 47; continue;
        }
        47 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700085de36); }
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700085de3a); };
            c.w64(0, t59);
            blk = 48; continue;
        }
        48 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t60) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085de42); };
            c.w32(7, t60);
            let r61 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r61);
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085de48); };
            c.cmp(t62, c.r32(7), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t63) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085de51); };
            c.cmp(t63, 0x43u64, 16);
            if c.cc_e() { blk = 53; continue; }
            blk = 51; continue;
        }
        51 => {
            return c.call(0x70008a1014, 0x700085de5d, 0x700085de58);
        }
        52 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085de5d); };
            c.w64(6, t64);
            blk = 48; continue;
        }
        53 => {
            let Some(t65) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085de63); };
            c.w32(7, t65);
            let r66 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r66);
            c.w64(0, c.r64(7));
            blk = 48; continue;
        }
        54 => {
            let r67 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r67);
            blk = 57; continue;
        }
        55 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700085de72); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 142; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w32(0, c.r32(0));
            blk = 57; continue;
        }
        57 => {
            c.w64(1, c.r64(0));
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085de8b); };
            c.w64(7, t68);
            c.cmp(c.r32(7), 0x14u64, 32);
            if c.cc_l() { blk = 61; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(7), 0x14u64, 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r32(7), 0x15u64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r32(7), 0x16u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(7), 0x11u64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 62; continue;
        }
        62 => {
            c.cmp(c.r32(7), 0x12u64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(7), 0x13u64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 64; continue;
        }
        64 => {
            return RegfileExit::fault(0x700085dec2);
        }
        65 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085dec3); };
            c.w64(8, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085dec7); };
            c.w64(9, t70);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085decf); };
            c.w64(0, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085ded3); };
            c.w64(12, t72);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(12)), c.r8(1), 8) { return RegfileExit::fault(0x700085ded7); }
            c.w32(0, (c.r8(0) as u8 as i8 as i64 as u64));
            let r73 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r73);
            blk = 66; continue;
        }
        66 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085dee2); };
            c.w64(1, t74);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085dee9); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_g() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            return c.ret(0x28, 0x700085def0);
        }
        68 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085def3); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085defa); }
            return c.ret(0x0, 0x700085defc);
        }
        69 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085defd); };
            c.w64(9, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085df01); };
            c.w64(8, t76);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085df09); };
            c.w64(12, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085df0d); };
            c.w64(0, t78);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(12)), c.r8(1), 8) { return RegfileExit::fault(0x700085df11); }
            c.w32(0, c.r8(0));
            let r79 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r79);
            blk = 66; continue;
        }
        70 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085df1e); };
            c.w64(9, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085df22); };
            c.w64(8, t81);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085df2a); };
            c.w64(12, t82);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085df32); };
            c.w64(0, t83);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r16(1), 16) { return RegfileExit::fault(0x700085df36); }
            c.w32(0, (c.r16(0) as u16 as i16 as i64 as u64));
            let r84 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r84);
            blk = 66; continue;
        }
        71 => {
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085df44); };
            c.w64(9, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085df48); };
            c.w64(8, t86);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085df50); };
            c.w64(12, t87);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085df58); };
            c.w64(0, t88);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r16(1), 16) { return RegfileExit::fault(0x700085df5c); }
            c.w32(0, c.r16(0));
            let r89 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r89);
            blk = 66; continue;
        }
        72 => {
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085df6d); };
            c.w64(12, t90);
            c.w64(7, c.r64(12).wrapping_mul(4));
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085df79); };
            c.w64(8, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085df7d); };
            c.w64(9, t92);
            let r93 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085df84); };
            c.w64(0, t94);
            if !c.lock_cmpxchg(c.r64(8).wrapping_add(c.r64(7)), c.r32(1), 32) { return RegfileExit::fault(0x700085df88); }
            c.w64(1, c.r64(0));
            let r95 = c.add(c.r32(1), c.r32(0), 32);
            c.w32(1, r95);
            if c.cc_o() { blk = 124; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 66; continue;
        }
        74 => {
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085dfa1); };
            c.w64(9, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085dfa5); };
            c.w64(8, t97);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085dfad); };
            c.w64(12, t98);
            c.w64(8, c.r64(12).wrapping_mul(4));
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085dfb9); };
            c.w64(0, t99);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r32(1), 32) { return RegfileExit::fault(0x700085dfbd); }
            c.cmp(c.r32(0), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 129; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w32(1, c.r32(0));
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 66; continue;
        }
        76 => {
            let Some(t100) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700085dfd8); };
            c.w64(8, t100);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 116; continue; }
            blk = 77; continue;
        }
        77 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            let Some(t101) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085dff2); };
            c.w32(3, t101);
            let r102 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r102);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700085dff8); }
            let Some(t103) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085dffc); };
            c.cmp(t103, 0x41u64, 16);
            if c.cc_e() { blk = 80; continue; }
            blk = 78; continue;
        }
        78 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085e003); }
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x700085e004); };
            c.w64(3, t104);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085e015, 0x700085e010);
        }
        79 => {
            c.w64(1, c.r64(0));
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085e018); };
            c.w64(6, t105);
            let Some(t106) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700085e01c); };
            c.w64(8, t106);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085e020); };
            c.w64(12, t107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085e024); };
            c.w64(9, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085e028); };
            c.w64(11, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085e02c); };
            c.w64(7, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085e030); };
            c.w64(0, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085e034); };
            c.w64(15, t112);
            blk = 80; continue;
        }
        80 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x700085e038); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 114; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t113) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085e046); };
            c.w32(15, t113);
            let r114 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r114);
            let Some(t115) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085e04d); };
            c.cmp(t115, 0x41u64, 16);
            if c.cc_e() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085e055); }
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x700085e057); };
            c.w64(3, t116);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085e068, 0x700085e063);
        }
        83 => {
            c.w64(8, c.r64(0));
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085e06b); };
            c.w64(6, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085e06f); };
            c.w64(12, t118);
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085e073); };
            c.w64(9, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085e077); };
            c.w64(11, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085e07b); };
            c.w64(7, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700085e07f); };
            c.w64(1, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085e083); };
            c.w64(0, t123);
            blk = 84; continue;
        }
        84 => {
            c.w64(15, c.r64(0));
            let Some(t124) = c.ld8(c.r64(15).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085e08a); };
            c.logic(t124 & 0x4u64, 8);
            if c.cc_ne() { blk = 118; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t125) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085e095); };
            c.w32(15, t125);
            c.logic(c.r32(15) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t126) = c.ld64(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085e0a2); };
            c.w64(1, t126);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 88; continue; }
            blk = 87; continue;
        }
        87 => {
            let r127 = c.neg(c.r64(1), 64);
            c.w64(1, r127);
            c.w64(0, c.r64(1));
            blk = 90; continue;
        }
        88 => {
            c.w64(0, c.r64(1));
            blk = 90; continue;
        }
        89 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 90; continue;
        }
        90 => {
            let Some(t128) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085e0bb); };
            c.w32(1, t128);
            c.logic(c.r32(1) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 94; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t129) = c.ld64(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085e0c7); };
            c.w64(8, t129);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let r130 = c.neg(c.r64(8), 64);
            c.w64(8, r130);
            c.w64(1, c.r64(8));
            blk = 95; continue;
        }
        93 => {
            c.w64(1, c.r64(8));
            blk = 95; continue;
        }
        94 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 95; continue;
        }
        95 => {
            let r131 = c.r32(7) & 0xfcu64;
            c.w32(7, r131);
            c.logic(r131, 32);
            c.cmp(c.r32(7), 0x6cu64, 32);
            if c.cc_e() { blk = 105; continue; }
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r32(7), 0x68u64, 32);
            if c.cc_e() { blk = 98; continue; }
            blk = 97; continue;
        }
        97 => {
            return RegfileExit::fault(0x700085e0f3);
        }
        98 => {
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            let r132 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r132);
            c.w64(8, c.r64(12).wrapping_mul(8));
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r64(1), 64) { return RegfileExit::fault(0x700085e105); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 102; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085e111); };
            c.w64(1, t133);
            let Some(t134) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085e118); };
            c.w64(7, t134);
            c.w64(8, c.r64(7).wrapping_add(0x10));
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085e11f); };
            c.w64(9, t135);
            let Some(t136) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085e126); };
            c.cmp(t136, c.r64(8), 64);
            if c.cc_be() { blk = 134; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085e12f); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 101; continue;
        }
        101 => {
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085e136); };
            c.w64(7, t137);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085e13d); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700085e140); }
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x700085e147); }
            c.w64(0, c.r64(1));
            blk = 66; continue;
        }
        102 => {
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085e153); };
            c.w64(1, t138);
            let Some(t139) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085e15a); };
            c.w64(7, t139);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085e161); };
            c.w64(8, t140);
            let Some(t141) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085e168); };
            c.cmp(t141, c.r64(9), 64);
            if c.cc_be() { blk = 136; continue; }
            blk = 103; continue;
        }
        103 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700085e171); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 104; continue;
        }
        104 => {
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085e178); };
            c.w64(7, t142);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085e17f); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700085e182); }
            c.w64(0, c.r64(1));
            blk = 66; continue;
        }
        105 => {
            c.w64(7, c.r64(9).wrapping_add(c.r64(11)));
            c.w64(8, c.r64(12).wrapping_mul(8));
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r64(1), 64) { return RegfileExit::fault(0x700085e19d); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 111; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085e1a9); };
            c.w64(1, t143);
            let Some(t144) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085e1b0); };
            c.w64(7, t144);
            c.w64(8, c.r64(7).wrapping_add(0x10));
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085e1b7); };
            c.w64(9, t145);
            let Some(t146) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085e1be); };
            c.cmp(t146, c.r64(8), 64);
            if c.cc_be() { blk = 138; continue; }
            blk = 107; continue;
        }
        107 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085e1c7); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 108; continue;
        }
        108 => {
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085e1ce); };
            c.w64(7, t147);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085e1d5); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_g() { blk = 110; continue; }
            blk = 109; continue;
        }
        109 => {
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700085e1de); }
            let r148 = c.neg(c.r64(0), 64);
            c.w64(0, r148);
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x700085e1e8); }
            c.w64(0, c.r64(1));
            blk = 66; continue;
        }
        110 => {
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700085e1f4); }
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x700085e1fb); }
            c.w64(0, c.r64(1));
            blk = 66; continue;
        }
        111 => {
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085e207); };
            c.w64(1, t149);
            let Some(t150) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085e20e); };
            c.w64(7, t150);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085e215); };
            c.w64(8, t151);
            let Some(t152) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085e21c); };
            c.cmp(t152, c.r64(9), 64);
            if c.cc_be() { blk = 140; continue; }
            blk = 112; continue;
        }
        112 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700085e225); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 113; continue;
        }
        113 => {
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085e22c); };
            c.w64(7, t153);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085e233); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700085e236); }
            c.w64(0, c.r64(1));
            blk = 66; continue;
        }
        114 => {
            c.w32(1, 0x28u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085e24a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700085e24b); }
            c.w32(0, 0x2u64);
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085e252); };
            c.w64(3, t154);
            return c.call(0x700086ed74, 0x700085e25e, 0x700085e259);
        }
        115 => {
            return RegfileExit::fault(0x700085e25e);
        }
        116 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085e264); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085e265); }
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085e266); };
            c.w64(3, t155);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700085e277, 0x700085e272);
        }
        117 => {
            return RegfileExit::fault(0x700085e277);
        }
        118 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085e27d); }
            let Some(t156) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085e27e); };
            c.w64(1, t156);
            let Some(t157) = c.ld32(c.r64(1).wrapping_add(0x87)) else { return RegfileExit::fault(0x700085e285); };
            c.w32(1, t157);
            let r158 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r158);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085e28e); }
            let Some(t159) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085e28f); };
            c.w64(3, t159);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700085e2a0, 0x700085e29b);
        }
        119 => {
            return RegfileExit::fault(0x700085e2a0);
        }
        120 => {
            c.w32(7, 0xdcu64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085e2a6); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085e2a7); }
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085e2a8); };
            c.w64(3, t160);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700085e2b9, 0x700085e2b4);
        }
        121 => {
            return RegfileExit::fault(0x700085e2b9);
        }
        122 => {
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085e2ba); };
            c.w64(3, t161);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085e2c8, 0x700085e2c3);
        }
        123 => {
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085e2c8); };
            c.w64(6, t162);
            blk = 1; continue;
        }
        124 => {
            let Some(t163) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085e2d1); };
            c.w64(1, t163);
            let Some(t164) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085e2d8); };
            c.w64(7, t164);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085e2df); }
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085e2e3); };
            c.w64(9, t165);
            let Some(t166) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085e2ea); };
            c.cmp(t166, c.r64(8), 64);
            if c.cc_a() { blk = 127; continue; }
            blk = 125; continue;
        }
        125 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700085e2f9, 0x700085e2f4);
        }
        126 => {
            c.w64(1, c.r64(0));
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085e2fc); };
            c.w64(0, t167);
            blk = 128; continue;
        }
        127 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085e302); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 128; continue;
        }
        128 => {
            let Some(t168) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085e309); };
            c.w64(7, t168);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085e310); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700085e31b); }
            c.w64(0, c.r64(1));
            blk = 66; continue;
        }
        129 => {
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085e328); };
            c.w64(1, t169);
            let Some(t170) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085e32f); };
            c.w64(7, t170);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085e336); }
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085e33a); };
            c.w64(9, t171);
            let Some(t172) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085e341); };
            c.cmp(t172, c.r64(8), 64);
            if c.cc_a() { blk = 132; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700085e350, 0x700085e34b);
        }
        131 => {
            c.w64(1, c.r64(0));
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085e353); };
            c.w64(0, t173);
            blk = 133; continue;
        }
        132 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085e359); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 133; continue;
        }
        133 => {
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085e360); };
            c.w64(7, t174);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085e367); }
            c.w32(10, c.r32(0));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700085e376); }
            c.w64(0, c.r64(1));
            blk = 66; continue;
        }
        134 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085e388); }
            return c.call(0x70007febb4, 0x700085e391, 0x700085e38c);
        }
        135 => {
            c.w64(1, c.r64(0));
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085e394); };
            c.w64(0, t175);
            blk = 101; continue;
        }
        136 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700085e3a7, 0x700085e3a2);
        }
        137 => {
            c.w64(1, c.r64(0));
            blk = 104; continue;
        }
        138 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085e3b4); }
            return c.call(0x70007febb4, 0x700085e3bd, 0x700085e3b8);
        }
        139 => {
            c.w64(1, c.r64(0));
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085e3c0); };
            c.w64(0, t176);
            blk = 108; continue;
        }
        140 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700085e3d3, 0x700085e3ce);
        }
        141 => {
            c.w64(1, c.r64(0));
            blk = 113; continue;
        }
        142 => {
            let r177 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r177);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700085e3df); }
            return c.call(0x70008708b4, 0x700085e3e9, 0x700085e3e4);
        }
        143 => {
            let Some(t178) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700085e3e9); };
            c.w32(0, t178);
            let r179 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r179);
            blk = 56; continue;
        }
        144 => {
            let r180 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r180);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700085e3f9); }
            return c.call(0x70008708b4, 0x700085e403, 0x700085e3fe);
        }
        145 => {
            let Some(t181) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700085e403); };
            c.w32(1, t181);
            let r182 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r182);
            blk = 46; continue;
        }
        146 => {
            return RegfileExit::jump(0x700085e410);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085e414(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085e414); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700085e418); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085e419); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085e41a); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085e41f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700085e423); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 113; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085e42d); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 111; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085e43a); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700085e440); }
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085e444); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 111; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085e450); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085e457); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085e45e); };
            c.cmp(t9, 0x0u64, 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(8));
            blk = 8; continue;
        }
        6 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085e46a); }
            c.w32(0, 0x1u64);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2b28)) else { return RegfileExit::fault(0x700085e470); };
            c.w64(3, t10);
            return c.call(0x700086ed74, 0x700085e47c, 0x700085e477);
        }
        7 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085e47c); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085e480); };
            c.w64(1, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085e484); };
            c.w64(6, t13);
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700085e488); }
            let Some(t14) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085e48c); };
            c.logic(t14 & 0x4u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t15) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x700085e496); };
            c.w32(7, t15);
            c.w64(8, c.r64(7));
            let r16 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r16);
            let r17 = c.r32(8) & 0x3fu64;
            c.w32(8, r17);
            c.logic(r17, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700085e4a5); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700085e4a9); }
            c.cmp(c.r32(8), 0x17u64, 32);
            if c.cc_l() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(8), 0x19u64, 32);
            if c.cc_le() { blk = 111; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t18) = c.ld64(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085e4bd); };
            c.w64(9, t18);
            let Some(t19) = c.ld64(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085e4c1); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700085e4c5); };
            c.w64(1, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700085e4c9); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700085e4cd); }
            let Some(t21) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085e4d1); };
            c.w64(12, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x700085e4d5); }
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085e4d9); };
            c.cmp(t22, c.r32(12), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(12));
            return c.call(0x7000899a14, 0x700085e4f4, 0x700085e4ef);
        }
        14 => {
            c.w64(12, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085e4f7); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085e4fb); };
            c.w64(9, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085e4ff); };
            c.w64(11, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085e503); };
            c.w64(7, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085e507); };
            c.w64(0, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085e50b); };
            c.w64(8, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700085e50f); };
            c.w64(1, t29);
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            blk = 25; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700085e52b); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_s() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(10, 0x8000000000000000u64);
            let r30 = c.r64(12) | c.r64(10);
            c.w64(12, r30);
            c.logic(r30, 64);
            blk = 23; continue;
        }
        23 => {
            blk = 25; continue;
        }
        24 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700085e58b); }
            c.cmp(c.r64(12), c.r64(1), 64);
            if c.cc_b() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(1, 0x194u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085e599); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700085e59a); };
            c.w64(3, t31);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085e5ab, 0x700085e5a6);
        }
        27 => {
            return RegfileExit::fault(0x700085e5ab);
        }
        28 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700085e5ac); };
            c.w64(1, t32);
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(8), 0x16u64, 32);
            if c.cc_g() { blk = 75; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x700085e5d3, 0x700085e5ce);
        }
        31 => {
            c.w64(1, c.r64(0));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085e5d6); };
            c.w64(6, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085e5da); };
            c.w64(12, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085e5de); };
            c.w64(9, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085e5e2); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085e5e6); };
            c.w64(0, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085e5ea); };
            c.w64(8, t38);
            blk = 32; continue;
        }
        32 => {
            let Some(t39) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085e5ee); };
            c.logic(t39 & 0x4u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 35; continue;
        }
        34 => {
            c.w64(1, c.r64(0));
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t40) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085e602); };
            c.w32(7, t40);
            let r41 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r41);
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085e608); };
            c.cmp(t42, c.r32(7), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t43) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085e611); };
            c.cmp(t43, 0x43u64, 16);
            if c.cc_e() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x700085e620, 0x700085e61b);
        }
        39 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085e620); };
            c.w64(6, t44);
            blk = 34; continue;
        }
        40 => {
            let Some(t45) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085e626); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            c.w64(0, c.r64(1));
            blk = 34; continue;
        }
        41 => {
            let r47 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r47);
            blk = 44; continue;
        }
        42 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700085e635); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 133; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w32(1, c.r32(1));
            blk = 44; continue;
        }
        44 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085e64b); };
            c.w64(7, t48);
            c.cmp(c.r32(7), 0x14u64, 32);
            if c.cc_l() { blk = 48; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(7), 0x14u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(7), 0x15u64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(7), 0x16u64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(7), 0x11u64, 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(7), 0x12u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(7), 0x13u64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 51; continue;
        }
        51 => {
            return RegfileExit::fault(0x700085e682);
        }
        52 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085e683); };
            c.w64(8, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085e687); };
            c.w64(9, t50);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085e68f); };
            c.w64(12, t51);
            blk = 53; continue;
        }
        53 => {
            let Some(t52) = c.ld8(c.r64(7).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700085e693); };
            c.w8(0, t52);
            c.w32(8, c.r32(0));
            let r53 = c.add(c.r32(8), c.r32(1), 32);
            c.w32(8, r53);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(12)), c.r8(8), 8) { return RegfileExit::fault(0x700085e69d); }
            if c.cc_ne() { blk = 53; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w32(0, (c.r8(0) as u8 as i8 as i64 as u64));
            let r54 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r54);
            blk = 55; continue;
        }
        55 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085e6aa); };
            c.w64(1, t55);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085e6b1); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_g() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            return c.ret(0x20, 0x700085e6b8);
        }
        57 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085e6bb); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085e6c2); }
            return c.ret(0x0, 0x700085e6c4);
        }
        58 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085e6c5); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085e6c9); };
            c.w64(8, t57);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085e6d1); };
            c.w64(12, t58);
            blk = 59; continue;
        }
        59 => {
            let Some(t59) = c.ld8(c.r64(7).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700085e6d5); };
            c.w8(0, t59);
            c.w32(8, c.r32(0));
            let r60 = c.add(c.r32(8), c.r32(1), 32);
            c.w32(8, r60);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(12)), c.r8(8), 8) { return RegfileExit::fault(0x700085e6df); }
            if c.cc_ne() { blk = 59; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w32(0, c.r8(0));
            let r61 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r61);
            blk = 55; continue;
        }
        61 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085e6ee); };
            c.w64(9, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085e6f2); };
            c.w64(8, t63);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085e6fa); };
            c.w64(12, t64);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            blk = 62; continue;
        }
        62 => {
            let Some(t65) = c.ld16(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085e702); };
            c.w16(0, t65);
            c.w32(9, c.r32(0));
            let r66 = c.add(c.r32(9), c.r32(1), 32);
            c.w32(9, r66);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r16(9), 16) { return RegfileExit::fault(0x700085e70d); }
            if c.cc_ne() { blk = 62; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w32(0, (c.r16(0) as u16 as i16 as i64 as u64));
            let r67 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r67);
            blk = 55; continue;
        }
        64 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085e71d); };
            c.w64(9, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085e721); };
            c.w64(8, t69);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085e729); };
            c.w64(12, t70);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            blk = 65; continue;
        }
        65 => {
            let Some(t71) = c.ld16(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085e731); };
            c.w16(0, t71);
            c.w32(9, c.r32(0));
            let r72 = c.add(c.r32(9), c.r32(1), 32);
            c.w32(9, r72);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r16(9), 16) { return RegfileExit::fault(0x700085e73c); }
            if c.cc_ne() { blk = 65; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(0, c.r16(0));
            let r73 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r73);
            blk = 55; continue;
        }
        67 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085e74f); };
            c.w64(12, t74);
            c.w64(7, c.r64(12).wrapping_mul(4));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085e75b); };
            c.w64(8, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085e75f); };
            c.w64(9, t76);
            let r77 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r77);
            blk = 68; continue;
        }
        68 => {
            let Some(t78) = c.ld32(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700085e766); };
            c.w32(0, t78);
            c.w32(9, c.r32(0));
            let r79 = c.add(c.r32(9), c.r32(1), 32);
            c.w32(9, r79);
            if !c.lock_cmpxchg(c.r64(8).wrapping_add(c.r64(7)), c.r32(9), 32) { return RegfileExit::fault(0x700085e770); }
            if c.cc_ne() { blk = 68; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w64(1, c.r64(0));
            let r80 = c.add(c.r32(1), c.r32(0), 32);
            c.w32(1, r80);
            if c.cc_o() { blk = 115; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 55; continue;
        }
        71 => {
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085e78b); };
            c.w64(9, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085e78f); };
            c.w64(8, t82);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085e797); };
            c.w64(12, t83);
            c.w64(8, c.r64(12).wrapping_mul(4));
            blk = 72; continue;
        }
        72 => {
            let Some(t84) = c.ld32(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085e7a3); };
            c.w32(0, t84);
            c.w32(9, c.r32(0));
            let r85 = c.add(c.r32(9), c.r32(1), 32);
            c.w32(9, r85);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r32(9), 32) { return RegfileExit::fault(0x700085e7ad); }
            if c.cc_ne() { blk = 72; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(0), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 120; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w32(1, c.r32(0));
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 55; continue;
        }
        75 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t86) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085e7d4); };
            c.w32(8, t86);
            let r87 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r87);
            let Some(t88) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085e7db); };
            c.cmp(t88, 0x41u64, 16);
            if c.cc_e() { blk = 79; continue; }
            blk = 77; continue;
        }
        77 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085e7e3); }
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x700085e7e4); };
            c.w64(3, t89);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085e7f5, 0x700085e7f0);
        }
        78 => {
            c.w64(1, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085e7f8); };
            c.w64(6, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085e7fc); };
            c.w64(12, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085e800); };
            c.w64(9, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085e804); };
            c.w64(11, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085e808); };
            c.w64(7, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085e80c); };
            c.w64(0, t95);
            blk = 79; continue;
        }
        79 => {
            c.w64(8, c.r64(0));
            let Some(t96) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085e813); };
            c.logic(t96 & 0x4u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t97) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085e81e); };
            c.w32(8, t97);
            c.logic(c.r32(8) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t98) = c.ld64(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085e82b); };
            c.w64(1, t98);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            let r99 = c.neg(c.r64(1), 64);
            c.w64(1, r99);
            blk = 84; continue;
        }
        83 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 84; continue;
        }
        84 => {
            let r100 = c.r32(7) & 0xfcu64;
            c.w32(7, r100);
            c.logic(r100, 32);
            c.cmp(c.r32(7), 0x6cu64, 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r32(7), 0x68u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            return RegfileExit::fault(0x700085e850);
        }
        87 => {
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            let r101 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r101);
            c.w64(8, c.r64(12).wrapping_mul(8));
            blk = 88; continue;
        }
        88 => {
            let Some(t102) = c.ld64(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085e862); };
            c.w64(0, t102);
            c.w64(9, c.r64(0));
            let r103 = c.add(c.r64(9), c.r64(1), 64);
            c.w64(9, r103);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r64(9), 64) { return RegfileExit::fault(0x700085e86c); }
            if c.cc_ne() { blk = 88; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 93; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085e87a); };
            c.w64(1, t104);
            let Some(t105) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085e881); };
            c.w64(7, t105);
            c.w64(8, c.r64(7).wrapping_add(0x10));
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085e888); };
            c.w64(9, t106);
            let Some(t107) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085e88f); };
            c.cmp(t107, c.r64(8), 64);
            if c.cc_be() { blk = 125; continue; }
            blk = 91; continue;
        }
        91 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085e898); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 92; continue;
        }
        92 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085e89f); };
            c.w64(7, t108);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085e8a6); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700085e8a9); }
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x700085e8b0); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        93 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085e8bc); };
            c.w64(1, t109);
            let Some(t110) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085e8c3); };
            c.w64(7, t110);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085e8ca); };
            c.w64(8, t111);
            let Some(t112) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085e8d1); };
            c.cmp(t112, c.r64(9), 64);
            if c.cc_be() { blk = 127; continue; }
            blk = 94; continue;
        }
        94 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700085e8da); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 95; continue;
        }
        95 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085e8e1); };
            c.w64(7, t113);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085e8e8); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700085e8eb); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        96 => {
            c.w64(7, c.r64(9).wrapping_add(c.r64(11)));
            c.w64(8, c.r64(12).wrapping_mul(8));
            blk = 97; continue;
        }
        97 => {
            let Some(t114) = c.ld64(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085e906); };
            c.w64(0, t114);
            c.w64(9, c.r64(0));
            let r115 = c.add(c.r64(9), c.r64(1), 64);
            c.w64(9, r115);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r64(9), 64) { return RegfileExit::fault(0x700085e910); }
            if c.cc_ne() { blk = 97; continue; }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 104; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085e91e); };
            c.w64(1, t116);
            let Some(t117) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085e925); };
            c.w64(7, t117);
            c.w64(8, c.r64(7).wrapping_add(0x10));
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085e92c); };
            c.w64(9, t118);
            let Some(t119) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085e933); };
            c.cmp(t119, c.r64(8), 64);
            if c.cc_be() { blk = 129; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085e93c); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 101; continue;
        }
        101 => {
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085e943); };
            c.w64(7, t120);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085e94a); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_g() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700085e953); }
            let r121 = c.neg(c.r64(0), 64);
            c.w64(0, r121);
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x700085e95d); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        103 => {
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700085e969); }
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x700085e970); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        104 => {
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085e97c); };
            c.w64(1, t122);
            let Some(t123) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085e983); };
            c.w64(7, t123);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085e98a); };
            c.w64(8, t124);
            let Some(t125) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085e991); };
            c.cmp(t125, c.r64(9), 64);
            if c.cc_be() { blk = 131; continue; }
            blk = 105; continue;
        }
        105 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700085e99a); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 106; continue;
        }
        106 => {
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085e9a1); };
            c.w64(7, t126);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085e9a8); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700085e9ab); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        107 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085e9bf); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085e9c0); }
            c.w32(0, 0x2u64);
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085e9c6); };
            c.w64(3, t127);
            return c.call(0x700086ed74, 0x700085e9d2, 0x700085e9cd);
        }
        108 => {
            return RegfileExit::fault(0x700085e9d2);
        }
        109 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085e9d8); }
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085e9d9); };
            c.w64(1, t128);
            let Some(t129) = c.ld32(c.r64(1).wrapping_add(0x8f)) else { return RegfileExit::fault(0x700085e9e0); };
            c.w32(1, t129);
            let r130 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r130);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085e9e9); }
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085e9ea); };
            c.w64(3, t131);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700085e9fb, 0x700085e9f6);
        }
        110 => {
            return RegfileExit::fault(0x700085e9fb);
        }
        111 => {
            c.w32(7, 0xdcu64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085ea01); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085ea02); }
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085ea03); };
            c.w64(3, t132);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700085ea14, 0x700085ea0f);
        }
        112 => {
            return RegfileExit::fault(0x700085ea14);
        }
        113 => {
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085ea15); };
            c.w64(3, t133);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085ea23, 0x700085ea1e);
        }
        114 => {
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085ea23); };
            c.w64(6, t134);
            blk = 1; continue;
        }
        115 => {
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085ea2c); };
            c.w64(1, t135);
            let Some(t136) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085ea33); };
            c.w64(7, t136);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085ea3a); }
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085ea3e); };
            c.w64(9, t137);
            let Some(t138) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085ea45); };
            c.cmp(t138, c.r64(8), 64);
            if c.cc_a() { blk = 118; continue; }
            blk = 116; continue;
        }
        116 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700085ea54, 0x700085ea4f);
        }
        117 => {
            c.w64(1, c.r64(0));
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085ea57); };
            c.w64(0, t139);
            blk = 119; continue;
        }
        118 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085ea5d); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 119; continue;
        }
        119 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085ea64); };
            c.w64(7, t140);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085ea6b); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700085ea76); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        120 => {
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085ea83); };
            c.w64(1, t141);
            let Some(t142) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085ea8a); };
            c.w64(7, t142);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085ea91); }
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085ea95); };
            c.w64(9, t143);
            let Some(t144) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085ea9c); };
            c.cmp(t144, c.r64(8), 64);
            if c.cc_a() { blk = 123; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700085eaab, 0x700085eaa6);
        }
        122 => {
            c.w64(1, c.r64(0));
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085eaae); };
            c.w64(0, t145);
            blk = 124; continue;
        }
        123 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085eab4); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 124; continue;
        }
        124 => {
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085eabb); };
            c.w64(7, t146);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085eac2); }
            c.w32(10, c.r32(0));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700085ead1); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        125 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085eae3); }
            return c.call(0x70007febb4, 0x700085eaec, 0x700085eae7);
        }
        126 => {
            c.w64(1, c.r64(0));
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085eaef); };
            c.w64(0, t147);
            blk = 92; continue;
        }
        127 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700085eb02, 0x700085eafd);
        }
        128 => {
            c.w64(1, c.r64(0));
            blk = 95; continue;
        }
        129 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085eb0f); }
            return c.call(0x70007febb4, 0x700085eb18, 0x700085eb13);
        }
        130 => {
            c.w64(1, c.r64(0));
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085eb1b); };
            c.w64(0, t148);
            blk = 101; continue;
        }
        131 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700085eb2e, 0x700085eb29);
        }
        132 => {
            c.w64(1, c.r64(0));
            blk = 106; continue;
        }
        133 => {
            let r149 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r149);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700085eb3a); }
            return c.call(0x70008708b4, 0x700085eb44, 0x700085eb3f);
        }
        134 => {
            let Some(t150) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700085eb44); };
            c.w32(1, t150);
            let r151 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r151);
            blk = 43; continue;
        }
        135 => {
            return RegfileExit::jump(0x700085eb54);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085eb74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085eb74); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700085eb78); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085eb79); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085eb7a); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085eb7f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700085eb83); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 113; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085eb8d); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 111; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085eb9a); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700085eba0); }
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085eba4); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 111; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085ebb0); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085ebb7); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085ebbe); };
            c.cmp(t9, 0x0u64, 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(8));
            blk = 8; continue;
        }
        6 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085ebca); }
            c.w32(0, 0x1u64);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2b28)) else { return RegfileExit::fault(0x700085ebd0); };
            c.w64(3, t10);
            return c.call(0x700086ed74, 0x700085ebdc, 0x700085ebd7);
        }
        7 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085ebdc); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085ebe0); };
            c.w64(1, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085ebe4); };
            c.w64(6, t13);
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700085ebe8); }
            let Some(t14) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085ebec); };
            c.logic(t14 & 0x4u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t15) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x700085ebf6); };
            c.w32(7, t15);
            c.w64(8, c.r64(7));
            let r16 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r16);
            let r17 = c.r32(8) & 0x3fu64;
            c.w32(8, r17);
            c.logic(r17, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700085ec05); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700085ec09); }
            c.cmp(c.r32(8), 0x17u64, 32);
            if c.cc_l() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(8), 0x19u64, 32);
            if c.cc_le() { blk = 111; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t18) = c.ld64(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085ec1d); };
            c.w64(9, t18);
            let Some(t19) = c.ld64(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085ec21); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700085ec25); };
            c.w64(1, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700085ec29); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700085ec2d); }
            let Some(t21) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085ec31); };
            c.w64(12, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x700085ec35); }
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085ec39); };
            c.cmp(t22, c.r32(12), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(12));
            return c.call(0x7000899a14, 0x700085ec54, 0x700085ec4f);
        }
        14 => {
            c.w64(12, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085ec57); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085ec5b); };
            c.w64(9, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085ec5f); };
            c.w64(11, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085ec63); };
            c.w64(7, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085ec67); };
            c.w64(0, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085ec6b); };
            c.w64(8, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700085ec6f); };
            c.w64(1, t29);
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            blk = 25; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700085ec8b); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_s() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(10, 0x8000000000000000u64);
            let r30 = c.r64(12) | c.r64(10);
            c.w64(12, r30);
            c.logic(r30, 64);
            blk = 23; continue;
        }
        23 => {
            blk = 25; continue;
        }
        24 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700085eceb); }
            c.cmp(c.r64(12), c.r64(1), 64);
            if c.cc_b() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(1, 0x194u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085ecf9); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700085ecfa); };
            c.w64(3, t31);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085ed0b, 0x700085ed06);
        }
        27 => {
            return RegfileExit::fault(0x700085ed0b);
        }
        28 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700085ed0c); };
            c.w64(1, t32);
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(8), 0x16u64, 32);
            if c.cc_g() { blk = 75; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x700085ed33, 0x700085ed2e);
        }
        31 => {
            c.w64(1, c.r64(0));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085ed36); };
            c.w64(6, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085ed3a); };
            c.w64(12, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085ed3e); };
            c.w64(9, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085ed42); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085ed46); };
            c.w64(0, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085ed4a); };
            c.w64(8, t38);
            blk = 32; continue;
        }
        32 => {
            let Some(t39) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085ed4e); };
            c.logic(t39 & 0x4u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 35; continue;
        }
        34 => {
            c.w64(1, c.r64(0));
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t40) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085ed62); };
            c.w32(7, t40);
            let r41 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r41);
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085ed68); };
            c.cmp(t42, c.r32(7), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t43) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085ed71); };
            c.cmp(t43, 0x43u64, 16);
            if c.cc_e() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x700085ed80, 0x700085ed7b);
        }
        39 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085ed80); };
            c.w64(6, t44);
            blk = 34; continue;
        }
        40 => {
            let Some(t45) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085ed86); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            c.w64(0, c.r64(1));
            blk = 34; continue;
        }
        41 => {
            let r47 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r47);
            blk = 44; continue;
        }
        42 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700085ed95); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 133; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w32(1, c.r32(1));
            blk = 44; continue;
        }
        44 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085edab); };
            c.w64(7, t48);
            c.cmp(c.r32(7), 0x14u64, 32);
            if c.cc_l() { blk = 48; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(7), 0x14u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(7), 0x15u64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(7), 0x16u64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(7), 0x11u64, 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(7), 0x12u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(7), 0x13u64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 51; continue;
        }
        51 => {
            return RegfileExit::fault(0x700085ede2);
        }
        52 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085ede3); };
            c.w64(8, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085ede7); };
            c.w64(9, t50);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085edef); };
            c.w64(12, t51);
            blk = 53; continue;
        }
        53 => {
            let Some(t52) = c.ld8(c.r64(7).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700085edf3); };
            c.w8(0, t52);
            c.w32(8, c.r32(0));
            let r53 = c.sub(c.r32(8), c.r32(1), 32);
            c.w32(8, r53);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(12)), c.r8(8), 8) { return RegfileExit::fault(0x700085edfd); }
            if c.cc_ne() { blk = 53; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w32(0, (c.r8(0) as u8 as i8 as i64 as u64));
            let r54 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r54);
            blk = 55; continue;
        }
        55 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085ee0a); };
            c.w64(1, t55);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085ee11); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_g() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            return c.ret(0x20, 0x700085ee18);
        }
        57 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085ee1b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085ee22); }
            return c.ret(0x0, 0x700085ee24);
        }
        58 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085ee25); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085ee29); };
            c.w64(8, t57);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085ee31); };
            c.w64(12, t58);
            blk = 59; continue;
        }
        59 => {
            let Some(t59) = c.ld8(c.r64(7).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700085ee35); };
            c.w8(0, t59);
            c.w32(8, c.r32(0));
            let r60 = c.sub(c.r32(8), c.r32(1), 32);
            c.w32(8, r60);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(12)), c.r8(8), 8) { return RegfileExit::fault(0x700085ee3f); }
            if c.cc_ne() { blk = 59; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w32(0, c.r8(0));
            let r61 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r61);
            blk = 55; continue;
        }
        61 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085ee4e); };
            c.w64(9, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085ee52); };
            c.w64(8, t63);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085ee5a); };
            c.w64(12, t64);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            blk = 62; continue;
        }
        62 => {
            let Some(t65) = c.ld16(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085ee62); };
            c.w16(0, t65);
            c.w32(9, c.r32(0));
            let r66 = c.sub(c.r32(9), c.r32(1), 32);
            c.w32(9, r66);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r16(9), 16) { return RegfileExit::fault(0x700085ee6d); }
            if c.cc_ne() { blk = 62; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w32(0, (c.r16(0) as u16 as i16 as i64 as u64));
            let r67 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r67);
            blk = 55; continue;
        }
        64 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085ee7d); };
            c.w64(9, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085ee81); };
            c.w64(8, t69);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085ee89); };
            c.w64(12, t70);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            blk = 65; continue;
        }
        65 => {
            let Some(t71) = c.ld16(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085ee91); };
            c.w16(0, t71);
            c.w32(9, c.r32(0));
            let r72 = c.sub(c.r32(9), c.r32(1), 32);
            c.w32(9, r72);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r16(9), 16) { return RegfileExit::fault(0x700085ee9c); }
            if c.cc_ne() { blk = 65; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(0, c.r16(0));
            let r73 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r73);
            blk = 55; continue;
        }
        67 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085eeaf); };
            c.w64(12, t74);
            c.w64(7, c.r64(12).wrapping_mul(4));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085eebb); };
            c.w64(8, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085eebf); };
            c.w64(9, t76);
            let r77 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r77);
            blk = 68; continue;
        }
        68 => {
            let Some(t78) = c.ld32(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700085eec6); };
            c.w32(0, t78);
            c.w32(9, c.r32(0));
            let r79 = c.sub(c.r32(9), c.r32(1), 32);
            c.w32(9, r79);
            if !c.lock_cmpxchg(c.r64(8).wrapping_add(c.r64(7)), c.r32(9), 32) { return RegfileExit::fault(0x700085eed0); }
            if c.cc_ne() { blk = 68; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w64(1, c.r64(0));
            let r80 = c.add(c.r32(1), c.r32(0), 32);
            c.w32(1, r80);
            if c.cc_o() { blk = 115; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 55; continue;
        }
        71 => {
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085eeeb); };
            c.w64(9, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085eeef); };
            c.w64(8, t82);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085eef7); };
            c.w64(12, t83);
            c.w64(8, c.r64(12).wrapping_mul(4));
            blk = 72; continue;
        }
        72 => {
            let Some(t84) = c.ld32(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085ef03); };
            c.w32(0, t84);
            c.w32(9, c.r32(0));
            let r85 = c.sub(c.r32(9), c.r32(1), 32);
            c.w32(9, r85);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r32(9), 32) { return RegfileExit::fault(0x700085ef0d); }
            if c.cc_ne() { blk = 72; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(0), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 120; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w32(1, c.r32(0));
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 55; continue;
        }
        75 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t86) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085ef34); };
            c.w32(8, t86);
            let r87 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r87);
            let Some(t88) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085ef3b); };
            c.cmp(t88, 0x41u64, 16);
            if c.cc_e() { blk = 79; continue; }
            blk = 77; continue;
        }
        77 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085ef43); }
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x700085ef44); };
            c.w64(3, t89);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085ef55, 0x700085ef50);
        }
        78 => {
            c.w64(1, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085ef58); };
            c.w64(6, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085ef5c); };
            c.w64(12, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085ef60); };
            c.w64(9, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085ef64); };
            c.w64(11, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085ef68); };
            c.w64(7, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085ef6c); };
            c.w64(0, t95);
            blk = 79; continue;
        }
        79 => {
            c.w64(8, c.r64(0));
            let Some(t96) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085ef73); };
            c.logic(t96 & 0x4u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t97) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085ef7e); };
            c.w32(8, t97);
            c.logic(c.r32(8) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t98) = c.ld64(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085ef8b); };
            c.w64(1, t98);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            let r99 = c.neg(c.r64(1), 64);
            c.w64(1, r99);
            blk = 84; continue;
        }
        83 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 84; continue;
        }
        84 => {
            let r100 = c.r32(7) & 0xfcu64;
            c.w32(7, r100);
            c.logic(r100, 32);
            c.cmp(c.r32(7), 0x6cu64, 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r32(7), 0x68u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            return RegfileExit::fault(0x700085efb0);
        }
        87 => {
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            let r101 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r101);
            c.w64(8, c.r64(12).wrapping_mul(8));
            blk = 88; continue;
        }
        88 => {
            let Some(t102) = c.ld64(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085efc2); };
            c.w64(0, t102);
            c.w64(9, c.r64(0));
            let r103 = c.sub(c.r64(9), c.r64(1), 64);
            c.w64(9, r103);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r64(9), 64) { return RegfileExit::fault(0x700085efcc); }
            if c.cc_ne() { blk = 88; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 93; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085efda); };
            c.w64(1, t104);
            let Some(t105) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085efe1); };
            c.w64(7, t105);
            c.w64(8, c.r64(7).wrapping_add(0x10));
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085efe8); };
            c.w64(9, t106);
            let Some(t107) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085efef); };
            c.cmp(t107, c.r64(8), 64);
            if c.cc_be() { blk = 125; continue; }
            blk = 91; continue;
        }
        91 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085eff8); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 92; continue;
        }
        92 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085efff); };
            c.w64(7, t108);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085f006); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700085f009); }
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x700085f010); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        93 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085f01c); };
            c.w64(1, t109);
            let Some(t110) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085f023); };
            c.w64(7, t110);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085f02a); };
            c.w64(8, t111);
            let Some(t112) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085f031); };
            c.cmp(t112, c.r64(9), 64);
            if c.cc_be() { blk = 127; continue; }
            blk = 94; continue;
        }
        94 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700085f03a); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 95; continue;
        }
        95 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085f041); };
            c.w64(7, t113);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085f048); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700085f04b); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        96 => {
            c.w64(7, c.r64(9).wrapping_add(c.r64(11)));
            c.w64(8, c.r64(12).wrapping_mul(8));
            blk = 97; continue;
        }
        97 => {
            let Some(t114) = c.ld64(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085f066); };
            c.w64(0, t114);
            c.w64(9, c.r64(0));
            let r115 = c.sub(c.r64(9), c.r64(1), 64);
            c.w64(9, r115);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r64(9), 64) { return RegfileExit::fault(0x700085f070); }
            if c.cc_ne() { blk = 97; continue; }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 104; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085f07e); };
            c.w64(1, t116);
            let Some(t117) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085f085); };
            c.w64(7, t117);
            c.w64(8, c.r64(7).wrapping_add(0x10));
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085f08c); };
            c.w64(9, t118);
            let Some(t119) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085f093); };
            c.cmp(t119, c.r64(8), 64);
            if c.cc_be() { blk = 129; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085f09c); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 101; continue;
        }
        101 => {
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085f0a3); };
            c.w64(7, t120);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085f0aa); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_g() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700085f0b3); }
            let r121 = c.neg(c.r64(0), 64);
            c.w64(0, r121);
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x700085f0bd); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        103 => {
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700085f0c9); }
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x700085f0d0); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        104 => {
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085f0dc); };
            c.w64(1, t122);
            let Some(t123) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085f0e3); };
            c.w64(7, t123);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085f0ea); };
            c.w64(8, t124);
            let Some(t125) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085f0f1); };
            c.cmp(t125, c.r64(9), 64);
            if c.cc_be() { blk = 131; continue; }
            blk = 105; continue;
        }
        105 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700085f0fa); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 106; continue;
        }
        106 => {
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085f101); };
            c.w64(7, t126);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085f108); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700085f10b); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        107 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085f11f); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085f120); }
            c.w32(0, 0x2u64);
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085f126); };
            c.w64(3, t127);
            return c.call(0x700086ed74, 0x700085f132, 0x700085f12d);
        }
        108 => {
            return RegfileExit::fault(0x700085f132);
        }
        109 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085f138); }
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085f139); };
            c.w64(1, t128);
            let Some(t129) = c.ld32(c.r64(1).wrapping_add(0x93)) else { return RegfileExit::fault(0x700085f140); };
            c.w32(1, t129);
            let r130 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r130);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085f149); }
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085f14a); };
            c.w64(3, t131);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700085f15b, 0x700085f156);
        }
        110 => {
            return RegfileExit::fault(0x700085f15b);
        }
        111 => {
            c.w32(7, 0xdcu64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085f161); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085f162); }
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085f163); };
            c.w64(3, t132);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700085f174, 0x700085f16f);
        }
        112 => {
            return RegfileExit::fault(0x700085f174);
        }
        113 => {
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085f175); };
            c.w64(3, t133);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085f183, 0x700085f17e);
        }
        114 => {
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085f183); };
            c.w64(6, t134);
            blk = 1; continue;
        }
        115 => {
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085f18c); };
            c.w64(1, t135);
            let Some(t136) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085f193); };
            c.w64(7, t136);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085f19a); }
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085f19e); };
            c.w64(9, t137);
            let Some(t138) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085f1a5); };
            c.cmp(t138, c.r64(8), 64);
            if c.cc_a() { blk = 118; continue; }
            blk = 116; continue;
        }
        116 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700085f1b4, 0x700085f1af);
        }
        117 => {
            c.w64(1, c.r64(0));
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085f1b7); };
            c.w64(0, t139);
            blk = 119; continue;
        }
        118 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085f1bd); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 119; continue;
        }
        119 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085f1c4); };
            c.w64(7, t140);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085f1cb); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700085f1d6); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        120 => {
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085f1e3); };
            c.w64(1, t141);
            let Some(t142) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085f1ea); };
            c.w64(7, t142);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085f1f1); }
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085f1f5); };
            c.w64(9, t143);
            let Some(t144) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085f1fc); };
            c.cmp(t144, c.r64(8), 64);
            if c.cc_a() { blk = 123; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700085f20b, 0x700085f206);
        }
        122 => {
            c.w64(1, c.r64(0));
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085f20e); };
            c.w64(0, t145);
            blk = 124; continue;
        }
        123 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085f214); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 124; continue;
        }
        124 => {
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085f21b); };
            c.w64(7, t146);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085f222); }
            c.w32(10, c.r32(0));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700085f231); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        125 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085f243); }
            return c.call(0x70007febb4, 0x700085f24c, 0x700085f247);
        }
        126 => {
            c.w64(1, c.r64(0));
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085f24f); };
            c.w64(0, t147);
            blk = 92; continue;
        }
        127 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700085f262, 0x700085f25d);
        }
        128 => {
            c.w64(1, c.r64(0));
            blk = 95; continue;
        }
        129 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085f26f); }
            return c.call(0x70007febb4, 0x700085f278, 0x700085f273);
        }
        130 => {
            c.w64(1, c.r64(0));
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085f27b); };
            c.w64(0, t148);
            blk = 101; continue;
        }
        131 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700085f28e, 0x700085f289);
        }
        132 => {
            c.w64(1, c.r64(0));
            blk = 106; continue;
        }
        133 => {
            let r149 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r149);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700085f29a); }
            return c.call(0x70008708b4, 0x700085f2a4, 0x700085f29f);
        }
        134 => {
            let Some(t150) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700085f2a4); };
            c.w32(1, t150);
            let r151 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r151);
            blk = 43; continue;
        }
        135 => {
            return RegfileExit::jump(0x700085f2b4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085f2d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085f2d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700085f2d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085f2d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085f2da); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085f2df); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700085f2e3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 113; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085f2ed); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 111; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085f2fa); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700085f300); }
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085f304); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 111; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085f310); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085f317); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085f31e); };
            c.cmp(t9, 0x0u64, 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(8));
            blk = 8; continue;
        }
        6 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085f32a); }
            c.w32(0, 0x1u64);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2b28)) else { return RegfileExit::fault(0x700085f330); };
            c.w64(3, t10);
            return c.call(0x700086ed74, 0x700085f33c, 0x700085f337);
        }
        7 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085f33c); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085f340); };
            c.w64(1, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085f344); };
            c.w64(6, t13);
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700085f348); }
            let Some(t14) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085f34c); };
            c.logic(t14 & 0x4u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t15) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x700085f356); };
            c.w32(7, t15);
            c.w64(8, c.r64(7));
            let r16 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r16);
            let r17 = c.r32(8) & 0x3fu64;
            c.w32(8, r17);
            c.logic(r17, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700085f365); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700085f369); }
            c.cmp(c.r32(8), 0x17u64, 32);
            if c.cc_l() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(8), 0x19u64, 32);
            if c.cc_le() { blk = 111; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t18) = c.ld64(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085f37d); };
            c.w64(9, t18);
            let Some(t19) = c.ld64(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085f381); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700085f385); };
            c.w64(1, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700085f389); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700085f38d); }
            let Some(t21) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085f391); };
            c.w64(12, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x700085f395); }
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085f399); };
            c.cmp(t22, c.r32(12), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(12));
            return c.call(0x7000899a14, 0x700085f3b4, 0x700085f3af);
        }
        14 => {
            c.w64(12, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085f3b7); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085f3bb); };
            c.w64(9, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085f3bf); };
            c.w64(11, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085f3c3); };
            c.w64(7, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085f3c7); };
            c.w64(0, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085f3cb); };
            c.w64(8, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700085f3cf); };
            c.w64(1, t29);
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            blk = 25; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700085f3eb); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_s() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(10, 0x8000000000000000u64);
            let r30 = c.r64(12) | c.r64(10);
            c.w64(12, r30);
            c.logic(r30, 64);
            blk = 23; continue;
        }
        23 => {
            blk = 25; continue;
        }
        24 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700085f44b); }
            c.cmp(c.r64(12), c.r64(1), 64);
            if c.cc_b() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(1, 0x194u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085f459); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700085f45a); };
            c.w64(3, t31);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085f46b, 0x700085f466);
        }
        27 => {
            return RegfileExit::fault(0x700085f46b);
        }
        28 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700085f46c); };
            c.w64(1, t32);
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(8), 0x16u64, 32);
            if c.cc_g() { blk = 75; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x700085f493, 0x700085f48e);
        }
        31 => {
            c.w64(1, c.r64(0));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085f496); };
            c.w64(6, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085f49a); };
            c.w64(12, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085f49e); };
            c.w64(9, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085f4a2); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085f4a6); };
            c.w64(0, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085f4aa); };
            c.w64(8, t38);
            blk = 32; continue;
        }
        32 => {
            let Some(t39) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085f4ae); };
            c.logic(t39 & 0x4u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 35; continue;
        }
        34 => {
            c.w64(1, c.r64(0));
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t40) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085f4c2); };
            c.w32(7, t40);
            let r41 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r41);
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085f4c8); };
            c.cmp(t42, c.r32(7), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t43) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085f4d1); };
            c.cmp(t43, 0x43u64, 16);
            if c.cc_e() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x700085f4e0, 0x700085f4db);
        }
        39 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085f4e0); };
            c.w64(6, t44);
            blk = 34; continue;
        }
        40 => {
            let Some(t45) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085f4e6); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            c.w64(0, c.r64(1));
            blk = 34; continue;
        }
        41 => {
            let r47 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r47);
            blk = 44; continue;
        }
        42 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700085f4f5); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 133; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w32(1, c.r32(1));
            blk = 44; continue;
        }
        44 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085f50b); };
            c.w64(7, t48);
            c.cmp(c.r32(7), 0x14u64, 32);
            if c.cc_l() { blk = 48; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(7), 0x14u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(7), 0x15u64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(7), 0x16u64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(7), 0x11u64, 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(7), 0x12u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(7), 0x13u64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 51; continue;
        }
        51 => {
            return RegfileExit::fault(0x700085f542);
        }
        52 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085f543); };
            c.w64(8, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085f547); };
            c.w64(9, t50);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085f54f); };
            c.w64(12, t51);
            blk = 53; continue;
        }
        53 => {
            let Some(t52) = c.ld8(c.r64(7).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700085f553); };
            c.w8(0, t52);
            c.w32(8, c.r32(0));
            let r53 = c.r32(8) & c.r32(1);
            c.w32(8, r53);
            c.logic(r53, 32);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(12)), c.r8(8), 8) { return RegfileExit::fault(0x700085f55d); }
            if c.cc_ne() { blk = 53; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w32(0, (c.r8(0) as u8 as i8 as i64 as u64));
            let r54 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r54);
            blk = 55; continue;
        }
        55 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085f56a); };
            c.w64(1, t55);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085f571); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_g() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            return c.ret(0x20, 0x700085f578);
        }
        57 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085f57b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085f582); }
            return c.ret(0x0, 0x700085f584);
        }
        58 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085f585); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085f589); };
            c.w64(8, t57);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085f591); };
            c.w64(12, t58);
            blk = 59; continue;
        }
        59 => {
            let Some(t59) = c.ld8(c.r64(7).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700085f595); };
            c.w8(0, t59);
            c.w32(8, c.r32(0));
            let r60 = c.r32(8) & c.r32(1);
            c.w32(8, r60);
            c.logic(r60, 32);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(12)), c.r8(8), 8) { return RegfileExit::fault(0x700085f59f); }
            if c.cc_ne() { blk = 59; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w32(0, c.r8(0));
            let r61 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r61);
            blk = 55; continue;
        }
        61 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085f5ae); };
            c.w64(9, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085f5b2); };
            c.w64(8, t63);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085f5ba); };
            c.w64(12, t64);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            blk = 62; continue;
        }
        62 => {
            let Some(t65) = c.ld16(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085f5c2); };
            c.w16(0, t65);
            c.w32(9, c.r32(0));
            let r66 = c.r32(9) & c.r32(1);
            c.w32(9, r66);
            c.logic(r66, 32);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r16(9), 16) { return RegfileExit::fault(0x700085f5cd); }
            if c.cc_ne() { blk = 62; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w32(0, (c.r16(0) as u16 as i16 as i64 as u64));
            let r67 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r67);
            blk = 55; continue;
        }
        64 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085f5dd); };
            c.w64(9, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085f5e1); };
            c.w64(8, t69);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085f5e9); };
            c.w64(12, t70);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            blk = 65; continue;
        }
        65 => {
            let Some(t71) = c.ld16(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085f5f1); };
            c.w16(0, t71);
            c.w32(9, c.r32(0));
            let r72 = c.r32(9) & c.r32(1);
            c.w32(9, r72);
            c.logic(r72, 32);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r16(9), 16) { return RegfileExit::fault(0x700085f5fc); }
            if c.cc_ne() { blk = 65; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(0, c.r16(0));
            let r73 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r73);
            blk = 55; continue;
        }
        67 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085f60f); };
            c.w64(12, t74);
            c.w64(7, c.r64(12).wrapping_mul(4));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085f61b); };
            c.w64(8, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085f61f); };
            c.w64(9, t76);
            let r77 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r77);
            blk = 68; continue;
        }
        68 => {
            let Some(t78) = c.ld32(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700085f626); };
            c.w32(0, t78);
            c.w32(9, c.r32(0));
            let r79 = c.r32(9) & c.r32(1);
            c.w32(9, r79);
            c.logic(r79, 32);
            if !c.lock_cmpxchg(c.r64(8).wrapping_add(c.r64(7)), c.r32(9), 32) { return RegfileExit::fault(0x700085f630); }
            if c.cc_ne() { blk = 68; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w64(1, c.r64(0));
            let r80 = c.add(c.r32(1), c.r32(0), 32);
            c.w32(1, r80);
            if c.cc_o() { blk = 115; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 55; continue;
        }
        71 => {
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085f64b); };
            c.w64(9, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085f64f); };
            c.w64(8, t82);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085f657); };
            c.w64(12, t83);
            c.w64(8, c.r64(12).wrapping_mul(4));
            blk = 72; continue;
        }
        72 => {
            let Some(t84) = c.ld32(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085f663); };
            c.w32(0, t84);
            c.w32(9, c.r32(0));
            let r85 = c.r32(9) & c.r32(1);
            c.w32(9, r85);
            c.logic(r85, 32);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r32(9), 32) { return RegfileExit::fault(0x700085f66d); }
            if c.cc_ne() { blk = 72; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(0), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 120; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w32(1, c.r32(0));
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 55; continue;
        }
        75 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t86) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085f694); };
            c.w32(8, t86);
            let r87 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r87);
            let Some(t88) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085f69b); };
            c.cmp(t88, 0x41u64, 16);
            if c.cc_e() { blk = 79; continue; }
            blk = 77; continue;
        }
        77 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085f6a3); }
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x700085f6a4); };
            c.w64(3, t89);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085f6b5, 0x700085f6b0);
        }
        78 => {
            c.w64(1, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085f6b8); };
            c.w64(6, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085f6bc); };
            c.w64(12, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085f6c0); };
            c.w64(9, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085f6c4); };
            c.w64(11, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085f6c8); };
            c.w64(7, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085f6cc); };
            c.w64(0, t95);
            blk = 79; continue;
        }
        79 => {
            c.w64(8, c.r64(0));
            let Some(t96) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085f6d3); };
            c.logic(t96 & 0x4u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t97) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085f6de); };
            c.w32(8, t97);
            c.logic(c.r32(8) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t98) = c.ld64(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085f6eb); };
            c.w64(1, t98);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            let r99 = c.neg(c.r64(1), 64);
            c.w64(1, r99);
            blk = 84; continue;
        }
        83 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 84; continue;
        }
        84 => {
            let r100 = c.r32(7) & 0xfcu64;
            c.w32(7, r100);
            c.logic(r100, 32);
            c.cmp(c.r32(7), 0x6cu64, 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r32(7), 0x68u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            return RegfileExit::fault(0x700085f710);
        }
        87 => {
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            let r101 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r101);
            c.w64(8, c.r64(12).wrapping_mul(8));
            blk = 88; continue;
        }
        88 => {
            let Some(t102) = c.ld64(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085f722); };
            c.w64(0, t102);
            c.w64(9, c.r64(0));
            let r103 = c.r64(9) & c.r64(1);
            c.w64(9, r103);
            c.logic(r103, 64);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r64(9), 64) { return RegfileExit::fault(0x700085f72c); }
            if c.cc_ne() { blk = 88; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 93; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085f73a); };
            c.w64(1, t104);
            let Some(t105) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085f741); };
            c.w64(7, t105);
            c.w64(8, c.r64(7).wrapping_add(0x10));
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085f748); };
            c.w64(9, t106);
            let Some(t107) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085f74f); };
            c.cmp(t107, c.r64(8), 64);
            if c.cc_be() { blk = 125; continue; }
            blk = 91; continue;
        }
        91 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085f758); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 92; continue;
        }
        92 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085f75f); };
            c.w64(7, t108);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085f766); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700085f769); }
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x700085f770); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        93 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085f77c); };
            c.w64(1, t109);
            let Some(t110) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085f783); };
            c.w64(7, t110);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085f78a); };
            c.w64(8, t111);
            let Some(t112) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085f791); };
            c.cmp(t112, c.r64(9), 64);
            if c.cc_be() { blk = 127; continue; }
            blk = 94; continue;
        }
        94 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700085f79a); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 95; continue;
        }
        95 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085f7a1); };
            c.w64(7, t113);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085f7a8); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700085f7ab); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        96 => {
            c.w64(7, c.r64(9).wrapping_add(c.r64(11)));
            c.w64(8, c.r64(12).wrapping_mul(8));
            blk = 97; continue;
        }
        97 => {
            let Some(t114) = c.ld64(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085f7c6); };
            c.w64(0, t114);
            c.w64(9, c.r64(0));
            let r115 = c.r64(9) & c.r64(1);
            c.w64(9, r115);
            c.logic(r115, 64);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r64(9), 64) { return RegfileExit::fault(0x700085f7d0); }
            if c.cc_ne() { blk = 97; continue; }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 104; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085f7de); };
            c.w64(1, t116);
            let Some(t117) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085f7e5); };
            c.w64(7, t117);
            c.w64(8, c.r64(7).wrapping_add(0x10));
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085f7ec); };
            c.w64(9, t118);
            let Some(t119) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085f7f3); };
            c.cmp(t119, c.r64(8), 64);
            if c.cc_be() { blk = 129; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085f7fc); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 101; continue;
        }
        101 => {
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085f803); };
            c.w64(7, t120);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085f80a); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_g() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700085f813); }
            let r121 = c.neg(c.r64(0), 64);
            c.w64(0, r121);
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x700085f81d); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        103 => {
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700085f829); }
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x700085f830); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        104 => {
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085f83c); };
            c.w64(1, t122);
            let Some(t123) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085f843); };
            c.w64(7, t123);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085f84a); };
            c.w64(8, t124);
            let Some(t125) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085f851); };
            c.cmp(t125, c.r64(9), 64);
            if c.cc_be() { blk = 131; continue; }
            blk = 105; continue;
        }
        105 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700085f85a); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 106; continue;
        }
        106 => {
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085f861); };
            c.w64(7, t126);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085f868); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700085f86b); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        107 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085f87f); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085f880); }
            c.w32(0, 0x2u64);
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085f886); };
            c.w64(3, t127);
            return c.call(0x700086ed74, 0x700085f892, 0x700085f88d);
        }
        108 => {
            return RegfileExit::fault(0x700085f892);
        }
        109 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085f898); }
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085f899); };
            c.w64(1, t128);
            let Some(t129) = c.ld32(c.r64(1).wrapping_add(0x97)) else { return RegfileExit::fault(0x700085f8a0); };
            c.w32(1, t129);
            let r130 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r130);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085f8a9); }
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085f8aa); };
            c.w64(3, t131);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700085f8bb, 0x700085f8b6);
        }
        110 => {
            return RegfileExit::fault(0x700085f8bb);
        }
        111 => {
            c.w32(7, 0xdcu64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085f8c1); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085f8c2); }
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085f8c3); };
            c.w64(3, t132);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700085f8d4, 0x700085f8cf);
        }
        112 => {
            return RegfileExit::fault(0x700085f8d4);
        }
        113 => {
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700085f8d5); };
            c.w64(3, t133);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700085f8e3, 0x700085f8de);
        }
        114 => {
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085f8e3); };
            c.w64(6, t134);
            blk = 1; continue;
        }
        115 => {
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085f8ec); };
            c.w64(1, t135);
            let Some(t136) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085f8f3); };
            c.w64(7, t136);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085f8fa); }
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085f8fe); };
            c.w64(9, t137);
            let Some(t138) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085f905); };
            c.cmp(t138, c.r64(8), 64);
            if c.cc_a() { blk = 118; continue; }
            blk = 116; continue;
        }
        116 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700085f914, 0x700085f90f);
        }
        117 => {
            c.w64(1, c.r64(0));
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085f917); };
            c.w64(0, t139);
            blk = 119; continue;
        }
        118 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085f91d); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 119; continue;
        }
        119 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085f924); };
            c.w64(7, t140);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085f92b); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700085f936); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        120 => {
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085f943); };
            c.w64(1, t141);
            let Some(t142) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085f94a); };
            c.w64(7, t142);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085f951); }
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085f955); };
            c.w64(9, t143);
            let Some(t144) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085f95c); };
            c.cmp(t144, c.r64(8), 64);
            if c.cc_a() { blk = 123; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700085f96b, 0x700085f966);
        }
        122 => {
            c.w64(1, c.r64(0));
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085f96e); };
            c.w64(0, t145);
            blk = 124; continue;
        }
        123 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085f974); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 124; continue;
        }
        124 => {
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085f97b); };
            c.w64(7, t146);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085f982); }
            c.w32(10, c.r32(0));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700085f991); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        125 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085f9a3); }
            return c.call(0x70007febb4, 0x700085f9ac, 0x700085f9a7);
        }
        126 => {
            c.w64(1, c.r64(0));
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085f9af); };
            c.w64(0, t147);
            blk = 92; continue;
        }
        127 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700085f9c2, 0x700085f9bd);
        }
        128 => {
            c.w64(1, c.r64(0));
            blk = 95; continue;
        }
        129 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700085f9cf); }
            return c.call(0x70007febb4, 0x700085f9d8, 0x700085f9d3);
        }
        130 => {
            c.w64(1, c.r64(0));
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085f9db); };
            c.w64(0, t148);
            blk = 101; continue;
        }
        131 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700085f9ee, 0x700085f9e9);
        }
        132 => {
            c.w64(1, c.r64(0));
            blk = 106; continue;
        }
        133 => {
            let r149 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r149);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700085f9fa); }
            return c.call(0x70008708b4, 0x700085fa04, 0x700085f9ff);
        }
        134 => {
            let Some(t150) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700085fa04); };
            c.w32(1, t150);
            let r151 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r151);
            blk = 43; continue;
        }
        135 => {
            return RegfileExit::jump(0x700085fa14);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700085fa34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700085fa34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700085fa38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085fa39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700085fa3a); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700085fa3f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700085fa43); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 113; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085fa4d); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 111; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085fa5a); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700085fa60); }
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085fa64); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 111; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700085fa70); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085fa77); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085fa7e); };
            c.cmp(t9, 0x0u64, 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(8));
            blk = 8; continue;
        }
        6 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085fa8a); }
            c.w32(0, 0x1u64);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2b28)) else { return RegfileExit::fault(0x700085fa90); };
            c.w64(3, t10);
            return c.call(0x700086ed74, 0x700085fa9c, 0x700085fa97);
        }
        7 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085fa9c); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700085faa0); };
            c.w64(1, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085faa4); };
            c.w64(6, t13);
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700085faa8); }
            let Some(t14) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085faac); };
            c.logic(t14 & 0x4u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t15) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x700085fab6); };
            c.w32(7, t15);
            c.w64(8, c.r64(7));
            let r16 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r16);
            let r17 = c.r32(8) & 0x3fu64;
            c.w32(8, r17);
            c.logic(r17, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700085fac5); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700085fac9); }
            c.cmp(c.r32(8), 0x17u64, 32);
            if c.cc_l() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(8), 0x19u64, 32);
            if c.cc_le() { blk = 111; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t18) = c.ld64(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700085fadd); };
            c.w64(9, t18);
            let Some(t19) = c.ld64(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085fae1); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700085fae5); };
            c.w64(1, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700085fae9); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700085faed); }
            let Some(t21) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700085faf1); };
            c.w64(12, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x700085faf5); }
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700085faf9); };
            c.cmp(t22, c.r32(12), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(12));
            return c.call(0x7000899a14, 0x700085fb14, 0x700085fb0f);
        }
        14 => {
            c.w64(12, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085fb17); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085fb1b); };
            c.w64(9, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085fb1f); };
            c.w64(11, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085fb23); };
            c.w64(7, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085fb27); };
            c.w64(0, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085fb2b); };
            c.w64(8, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700085fb2f); };
            c.w64(1, t29);
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            blk = 25; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700085fb4b); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_s() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(10, 0x8000000000000000u64);
            let r30 = c.r64(12) | c.r64(10);
            c.w64(12, r30);
            c.logic(r30, 64);
            blk = 23; continue;
        }
        23 => {
            blk = 25; continue;
        }
        24 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700085fbab); }
            c.cmp(c.r64(12), c.r64(1), 64);
            if c.cc_b() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(1, 0x194u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085fbb9); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700085fbba); };
            c.w64(3, t31);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085fbcb, 0x700085fbc6);
        }
        27 => {
            return RegfileExit::fault(0x700085fbcb);
        }
        28 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700085fbcc); };
            c.w64(1, t32);
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(8), 0x16u64, 32);
            if c.cc_g() { blk = 75; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x700085fbf3, 0x700085fbee);
        }
        31 => {
            c.w64(1, c.r64(0));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085fbf6); };
            c.w64(6, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085fbfa); };
            c.w64(12, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085fbfe); };
            c.w64(9, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085fc02); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085fc06); };
            c.w64(0, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085fc0a); };
            c.w64(8, t38);
            blk = 32; continue;
        }
        32 => {
            let Some(t39) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085fc0e); };
            c.logic(t39 & 0x4u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 35; continue;
        }
        34 => {
            c.w64(1, c.r64(0));
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t40) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085fc22); };
            c.w32(7, t40);
            let r41 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r41);
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700085fc28); };
            c.cmp(t42, c.r32(7), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t43) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085fc31); };
            c.cmp(t43, 0x43u64, 16);
            if c.cc_e() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x700085fc40, 0x700085fc3b);
        }
        39 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085fc40); };
            c.w64(6, t44);
            blk = 34; continue;
        }
        40 => {
            let Some(t45) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700085fc46); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            c.w64(0, c.r64(1));
            blk = 34; continue;
        }
        41 => {
            let r47 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r47);
            blk = 44; continue;
        }
        42 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700085fc55); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 133; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w32(1, c.r32(1));
            blk = 44; continue;
        }
        44 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700085fc6b); };
            c.w64(7, t48);
            c.cmp(c.r32(7), 0x14u64, 32);
            if c.cc_l() { blk = 48; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(7), 0x14u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(7), 0x15u64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(7), 0x16u64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(7), 0x11u64, 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(7), 0x12u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(7), 0x13u64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 51; continue;
        }
        51 => {
            return RegfileExit::fault(0x700085fca2);
        }
        52 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085fca3); };
            c.w64(8, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085fca7); };
            c.w64(9, t50);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085fcaf); };
            c.w64(12, t51);
            blk = 53; continue;
        }
        53 => {
            let Some(t52) = c.ld8(c.r64(7).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700085fcb3); };
            c.w8(0, t52);
            c.w32(8, c.r32(0));
            let r53 = c.r32(8) | c.r32(1);
            c.w32(8, r53);
            c.logic(r53, 32);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(12)), c.r8(8), 8) { return RegfileExit::fault(0x700085fcbd); }
            if c.cc_ne() { blk = 53; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w32(0, (c.r8(0) as u8 as i8 as i64 as u64));
            let r54 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r54);
            blk = 55; continue;
        }
        55 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700085fcca); };
            c.w64(1, t55);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700085fcd1); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_g() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            return c.ret(0x20, 0x700085fcd8);
        }
        57 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700085fcdb); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700085fce2); }
            return c.ret(0x0, 0x700085fce4);
        }
        58 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085fce5); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085fce9); };
            c.w64(8, t57);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085fcf1); };
            c.w64(12, t58);
            blk = 59; continue;
        }
        59 => {
            let Some(t59) = c.ld8(c.r64(7).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700085fcf5); };
            c.w8(0, t59);
            c.w32(8, c.r32(0));
            let r60 = c.r32(8) | c.r32(1);
            c.w32(8, r60);
            c.logic(r60, 32);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(12)), c.r8(8), 8) { return RegfileExit::fault(0x700085fcff); }
            if c.cc_ne() { blk = 59; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w32(0, c.r8(0));
            let r61 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r61);
            blk = 55; continue;
        }
        61 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085fd0e); };
            c.w64(9, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085fd12); };
            c.w64(8, t63);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085fd1a); };
            c.w64(12, t64);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            blk = 62; continue;
        }
        62 => {
            let Some(t65) = c.ld16(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085fd22); };
            c.w16(0, t65);
            c.w32(9, c.r32(0));
            let r66 = c.r32(9) | c.r32(1);
            c.w32(9, r66);
            c.logic(r66, 32);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r16(9), 16) { return RegfileExit::fault(0x700085fd2d); }
            if c.cc_ne() { blk = 62; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w32(0, (c.r16(0) as u16 as i16 as i64 as u64));
            let r67 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r67);
            blk = 55; continue;
        }
        64 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085fd3d); };
            c.w64(9, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085fd41); };
            c.w64(8, t69);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085fd49); };
            c.w64(12, t70);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            blk = 65; continue;
        }
        65 => {
            let Some(t71) = c.ld16(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085fd51); };
            c.w16(0, t71);
            c.w32(9, c.r32(0));
            let r72 = c.r32(9) | c.r32(1);
            c.w32(9, r72);
            c.logic(r72, 32);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r16(9), 16) { return RegfileExit::fault(0x700085fd5c); }
            if c.cc_ne() { blk = 65; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(0, c.r16(0));
            let r73 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r73);
            blk = 55; continue;
        }
        67 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085fd6f); };
            c.w64(12, t74);
            c.w64(7, c.r64(12).wrapping_mul(4));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085fd7b); };
            c.w64(8, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085fd7f); };
            c.w64(9, t76);
            let r77 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r77);
            blk = 68; continue;
        }
        68 => {
            let Some(t78) = c.ld32(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700085fd86); };
            c.w32(0, t78);
            c.w32(9, c.r32(0));
            let r79 = c.r32(9) | c.r32(1);
            c.w32(9, r79);
            c.logic(r79, 32);
            if !c.lock_cmpxchg(c.r64(8).wrapping_add(c.r64(7)), c.r32(9), 32) { return RegfileExit::fault(0x700085fd90); }
            if c.cc_ne() { blk = 68; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w64(1, c.r64(0));
            let r80 = c.add(c.r32(1), c.r32(0), 32);
            c.w32(1, r80);
            if c.cc_o() { blk = 115; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 55; continue;
        }
        71 => {
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085fdab); };
            c.w64(9, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085fdaf); };
            c.w64(8, t82);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085fdb7); };
            c.w64(12, t83);
            c.w64(8, c.r64(12).wrapping_mul(4));
            blk = 72; continue;
        }
        72 => {
            let Some(t84) = c.ld32(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085fdc3); };
            c.w32(0, t84);
            c.w32(9, c.r32(0));
            let r85 = c.r32(9) | c.r32(1);
            c.w32(9, r85);
            c.logic(r85, 32);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r32(9), 32) { return RegfileExit::fault(0x700085fdcd); }
            if c.cc_ne() { blk = 72; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(0), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 120; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w32(1, c.r32(0));
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 55; continue;
        }
        75 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t86) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700085fdf4); };
            c.w32(8, t86);
            let r87 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r87);
            let Some(t88) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085fdfb); };
            c.cmp(t88, 0x41u64, 16);
            if c.cc_e() { blk = 79; continue; }
            blk = 77; continue;
        }
        77 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085fe03); }
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x700085fe04); };
            c.w64(3, t89);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700085fe15, 0x700085fe10);
        }
        78 => {
            c.w64(1, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700085fe18); };
            c.w64(6, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700085fe1c); };
            c.w64(12, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700085fe20); };
            c.w64(9, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700085fe24); };
            c.w64(11, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700085fe28); };
            c.w64(7, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700085fe2c); };
            c.w64(0, t95);
            blk = 79; continue;
        }
        79 => {
            c.w64(8, c.r64(0));
            let Some(t96) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700085fe33); };
            c.logic(t96 & 0x4u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t97) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700085fe3e); };
            c.w32(8, t97);
            c.logic(c.r32(8) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t98) = c.ld64(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700085fe4b); };
            c.w64(1, t98);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            let r99 = c.neg(c.r64(1), 64);
            c.w64(1, r99);
            blk = 84; continue;
        }
        83 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 84; continue;
        }
        84 => {
            let r100 = c.r32(7) & 0xfcu64;
            c.w32(7, r100);
            c.logic(r100, 32);
            c.cmp(c.r32(7), 0x6cu64, 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r32(7), 0x68u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            return RegfileExit::fault(0x700085fe70);
        }
        87 => {
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            let r101 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r101);
            c.w64(8, c.r64(12).wrapping_mul(8));
            blk = 88; continue;
        }
        88 => {
            let Some(t102) = c.ld64(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085fe82); };
            c.w64(0, t102);
            c.w64(9, c.r64(0));
            let r103 = c.r64(9) | c.r64(1);
            c.w64(9, r103);
            c.logic(r103, 64);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r64(9), 64) { return RegfileExit::fault(0x700085fe8c); }
            if c.cc_ne() { blk = 88; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 93; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085fe9a); };
            c.w64(1, t104);
            let Some(t105) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085fea1); };
            c.w64(7, t105);
            c.w64(8, c.r64(7).wrapping_add(0x10));
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085fea8); };
            c.w64(9, t106);
            let Some(t107) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085feaf); };
            c.cmp(t107, c.r64(8), 64);
            if c.cc_be() { blk = 125; continue; }
            blk = 91; continue;
        }
        91 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085feb8); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 92; continue;
        }
        92 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085febf); };
            c.w64(7, t108);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085fec6); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700085fec9); }
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x700085fed0); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        93 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085fedc); };
            c.w64(1, t109);
            let Some(t110) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085fee3); };
            c.w64(7, t110);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085feea); };
            c.w64(8, t111);
            let Some(t112) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085fef1); };
            c.cmp(t112, c.r64(9), 64);
            if c.cc_be() { blk = 127; continue; }
            blk = 94; continue;
        }
        94 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700085fefa); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 95; continue;
        }
        95 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085ff01); };
            c.w64(7, t113);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085ff08); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700085ff0b); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        96 => {
            c.w64(7, c.r64(9).wrapping_add(c.r64(11)));
            c.w64(8, c.r64(12).wrapping_mul(8));
            blk = 97; continue;
        }
        97 => {
            let Some(t114) = c.ld64(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700085ff26); };
            c.w64(0, t114);
            c.w64(9, c.r64(0));
            let r115 = c.r64(9) | c.r64(1);
            c.w64(9, r115);
            c.logic(r115, 64);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r64(9), 64) { return RegfileExit::fault(0x700085ff30); }
            if c.cc_ne() { blk = 97; continue; }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 104; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085ff3e); };
            c.w64(1, t116);
            let Some(t117) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085ff45); };
            c.w64(7, t117);
            c.w64(8, c.r64(7).wrapping_add(0x10));
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085ff4c); };
            c.w64(9, t118);
            let Some(t119) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700085ff53); };
            c.cmp(t119, c.r64(8), 64);
            if c.cc_be() { blk = 129; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700085ff5c); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 101; continue;
        }
        101 => {
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085ff63); };
            c.w64(7, t120);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085ff6a); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_g() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700085ff73); }
            let r121 = c.neg(c.r64(0), 64);
            c.w64(0, r121);
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x700085ff7d); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        103 => {
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700085ff89); }
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x700085ff90); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        104 => {
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700085ff9c); };
            c.w64(1, t122);
            let Some(t123) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700085ffa3); };
            c.w64(7, t123);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700085ffaa); };
            c.w64(8, t124);
            let Some(t125) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700085ffb1); };
            c.cmp(t125, c.r64(9), 64);
            if c.cc_be() { blk = 131; continue; }
            blk = 105; continue;
        }
        105 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700085ffba); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 106; continue;
        }
        106 => {
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700085ffc1); };
            c.w64(7, t126);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700085ffc8); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700085ffcb); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        107 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700085ffdf); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085ffe0); }
            c.w32(0, 0x2u64);
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700085ffe6); };
            c.w64(3, t127);
            return c.call(0x700086ed74, 0x700085fff2, 0x700085ffed);
        }
        108 => {
            return RegfileExit::fault(0x700085fff2);
        }
        109 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700085fff8); }
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700085fff9); };
            c.w64(1, t128);
            let Some(t129) = c.ld32(c.r64(1).wrapping_add(0x9b)) else { return RegfileExit::fault(0x7000860000); };
            c.w32(1, t129);
            let r130 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r130);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000860009); }
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700086000a); };
            c.w64(3, t131);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700086001b, 0x7000860016);
        }
        110 => {
            return RegfileExit::fault(0x700086001b);
        }
        111 => {
            c.w32(7, 0xdcu64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000860021); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000860022); }
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000860023); };
            c.w64(3, t132);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000860034, 0x700086002f);
        }
        112 => {
            return RegfileExit::fault(0x7000860034);
        }
        113 => {
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000860035); };
            c.w64(3, t133);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000860043, 0x700086003e);
        }
        114 => {
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000860043); };
            c.w64(6, t134);
            blk = 1; continue;
        }
        115 => {
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086004c); };
            c.w64(1, t135);
            let Some(t136) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000860053); };
            c.w64(7, t136);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700086005a); }
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086005e); };
            c.w64(9, t137);
            let Some(t138) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000860065); };
            c.cmp(t138, c.r64(8), 64);
            if c.cc_a() { blk = 118; continue; }
            blk = 116; continue;
        }
        116 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000860074, 0x700086006f);
        }
        117 => {
            c.w64(1, c.r64(0));
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000860077); };
            c.w64(0, t139);
            blk = 119; continue;
        }
        118 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700086007d); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 119; continue;
        }
        119 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000860084); };
            c.w64(7, t140);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700086008b); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000860096); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        120 => {
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008600a3); };
            c.w64(1, t141);
            let Some(t142) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008600aa); };
            c.w64(7, t142);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008600b1); }
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008600b5); };
            c.w64(9, t143);
            let Some(t144) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008600bc); };
            c.cmp(t144, c.r64(8), 64);
            if c.cc_a() { blk = 123; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008600cb, 0x70008600c6);
        }
        122 => {
            c.w64(1, c.r64(0));
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008600ce); };
            c.w64(0, t145);
            blk = 124; continue;
        }
        123 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008600d4); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 124; continue;
        }
        124 => {
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008600db); };
            c.w64(7, t146);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008600e2); }
            c.w32(10, c.r32(0));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008600f1); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        125 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000860103); }
            return c.call(0x70007febb4, 0x700086010c, 0x7000860107);
        }
        126 => {
            c.w64(1, c.r64(0));
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086010f); };
            c.w64(0, t147);
            blk = 92; continue;
        }
        127 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x7000860122, 0x700086011d);
        }
        128 => {
            c.w64(1, c.r64(0));
            blk = 95; continue;
        }
        129 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700086012f); }
            return c.call(0x70007febb4, 0x7000860138, 0x7000860133);
        }
        130 => {
            c.w64(1, c.r64(0));
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086013b); };
            c.w64(0, t148);
            blk = 101; continue;
        }
        131 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700086014e, 0x7000860149);
        }
        132 => {
            c.w64(1, c.r64(0));
            blk = 106; continue;
        }
        133 => {
            let r149 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r149);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700086015a); }
            return c.call(0x70008708b4, 0x7000860164, 0x700086015f);
        }
        134 => {
            let Some(t150) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000860164); };
            c.w32(1, t150);
            let r151 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r151);
            blk = 43; continue;
        }
        135 => {
            return RegfileExit::jump(0x7000860174);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000860194(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000860194); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000860198); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000860199); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086019a); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700086019f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008601a3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 113; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008601ad); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 111; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008601ba); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008601c0); }
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008601c4); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 111; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008601d0); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008601d7); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008601de); };
            c.cmp(t9, 0x0u64, 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(8));
            blk = 8; continue;
        }
        6 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008601ea); }
            c.w32(0, 0x1u64);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2b28)) else { return RegfileExit::fault(0x70008601f0); };
            c.w64(3, t10);
            return c.call(0x700086ed74, 0x70008601fc, 0x70008601f7);
        }
        7 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008601fc); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000860200); };
            c.w64(1, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000860204); };
            c.w64(6, t13);
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x7000860208); }
            let Some(t14) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700086020c); };
            c.logic(t14 & 0x4u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t15) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000860216); };
            c.w32(7, t15);
            c.w64(8, c.r64(7));
            let r16 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r16);
            let r17 = c.r32(8) & 0x3fu64;
            c.w32(8, r17);
            c.logic(r17, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000860225); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x7000860229); }
            c.cmp(c.r32(8), 0x17u64, 32);
            if c.cc_l() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(8), 0x19u64, 32);
            if c.cc_le() { blk = 111; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t18) = c.ld64(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086023d); };
            c.w64(9, t18);
            let Some(t19) = c.ld64(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000860241); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000860245); };
            c.w64(1, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000860249); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700086024d); }
            let Some(t21) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000860251); };
            c.w64(12, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x7000860255); }
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000860259); };
            c.cmp(t22, c.r32(12), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(12));
            return c.call(0x7000899a14, 0x7000860274, 0x700086026f);
        }
        14 => {
            c.w64(12, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000860277); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086027b); };
            c.w64(9, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086027f); };
            c.w64(11, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000860283); };
            c.w64(7, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000860287); };
            c.w64(0, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086028b); };
            c.w64(8, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086028f); };
            c.w64(1, t29);
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            blk = 25; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008602ab); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_s() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(10, 0x8000000000000000u64);
            let r30 = c.r64(12) | c.r64(10);
            c.w64(12, r30);
            c.logic(r30, 64);
            blk = 23; continue;
        }
        23 => {
            blk = 25; continue;
        }
        24 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700086030b); }
            c.cmp(c.r64(12), c.r64(1), 64);
            if c.cc_b() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(1, 0x194u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000860319); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700086031a); };
            c.w64(3, t31);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700086032b, 0x7000860326);
        }
        27 => {
            return RegfileExit::fault(0x700086032b);
        }
        28 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700086032c); };
            c.w64(1, t32);
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(8), 0x16u64, 32);
            if c.cc_g() { blk = 75; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x7000860353, 0x700086034e);
        }
        31 => {
            c.w64(1, c.r64(0));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000860356); };
            c.w64(6, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086035a); };
            c.w64(12, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086035e); };
            c.w64(9, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000860362); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000860366); };
            c.w64(0, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086036a); };
            c.w64(8, t38);
            blk = 32; continue;
        }
        32 => {
            let Some(t39) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700086036e); };
            c.logic(t39 & 0x4u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 35; continue;
        }
        34 => {
            c.w64(1, c.r64(0));
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t40) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000860382); };
            c.w32(7, t40);
            let r41 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r41);
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000860388); };
            c.cmp(t42, c.r32(7), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t43) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000860391); };
            c.cmp(t43, 0x43u64, 16);
            if c.cc_e() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x70008603a0, 0x700086039b);
        }
        39 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008603a0); };
            c.w64(6, t44);
            blk = 34; continue;
        }
        40 => {
            let Some(t45) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008603a6); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            c.w64(0, c.r64(1));
            blk = 34; continue;
        }
        41 => {
            let r47 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r47);
            blk = 44; continue;
        }
        42 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008603b5); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 133; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w32(1, c.r32(1));
            blk = 44; continue;
        }
        44 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008603cb); };
            c.w64(7, t48);
            c.cmp(c.r32(7), 0x14u64, 32);
            if c.cc_l() { blk = 48; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(7), 0x14u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(7), 0x15u64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(7), 0x16u64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(7), 0x11u64, 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(7), 0x12u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(7), 0x13u64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 51; continue;
        }
        51 => {
            return RegfileExit::fault(0x7000860402);
        }
        52 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000860403); };
            c.w64(8, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000860407); };
            c.w64(9, t50);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086040f); };
            c.w64(12, t51);
            blk = 53; continue;
        }
        53 => {
            let Some(t52) = c.ld8(c.r64(7).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000860413); };
            c.w8(0, t52);
            c.w32(8, c.r32(0));
            let r53 = c.r32(8) ^ c.r32(1);
            c.w32(8, r53);
            c.logic(r53, 32);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(12)), c.r8(8), 8) { return RegfileExit::fault(0x700086041d); }
            if c.cc_ne() { blk = 53; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w32(0, (c.r8(0) as u8 as i8 as i64 as u64));
            let r54 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r54);
            blk = 55; continue;
        }
        55 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086042a); };
            c.w64(1, t55);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000860431); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_g() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            return c.ret(0x20, 0x7000860438);
        }
        57 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700086043b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000860442); }
            return c.ret(0x0, 0x7000860444);
        }
        58 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000860445); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000860449); };
            c.w64(8, t57);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000860451); };
            c.w64(12, t58);
            blk = 59; continue;
        }
        59 => {
            let Some(t59) = c.ld8(c.r64(7).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000860455); };
            c.w8(0, t59);
            c.w32(8, c.r32(0));
            let r60 = c.r32(8) ^ c.r32(1);
            c.w32(8, r60);
            c.logic(r60, 32);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(12)), c.r8(8), 8) { return RegfileExit::fault(0x700086045f); }
            if c.cc_ne() { blk = 59; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w32(0, c.r8(0));
            let r61 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r61);
            blk = 55; continue;
        }
        61 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086046e); };
            c.w64(9, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000860472); };
            c.w64(8, t63);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086047a); };
            c.w64(12, t64);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            blk = 62; continue;
        }
        62 => {
            let Some(t65) = c.ld16(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000860482); };
            c.w16(0, t65);
            c.w32(9, c.r32(0));
            let r66 = c.r32(9) ^ c.r32(1);
            c.w32(9, r66);
            c.logic(r66, 32);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r16(9), 16) { return RegfileExit::fault(0x700086048d); }
            if c.cc_ne() { blk = 62; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w32(0, (c.r16(0) as u16 as i16 as i64 as u64));
            let r67 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r67);
            blk = 55; continue;
        }
        64 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086049d); };
            c.w64(9, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008604a1); };
            c.w64(8, t69);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008604a9); };
            c.w64(12, t70);
            c.w64(8, c.r64(12).wrapping_add(c.r64(12)));
            blk = 65; continue;
        }
        65 => {
            let Some(t71) = c.ld16(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008604b1); };
            c.w16(0, t71);
            c.w32(9, c.r32(0));
            let r72 = c.r32(9) ^ c.r32(1);
            c.w32(9, r72);
            c.logic(r72, 32);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r16(9), 16) { return RegfileExit::fault(0x70008604bc); }
            if c.cc_ne() { blk = 65; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(0, c.r16(0));
            let r73 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r73);
            blk = 55; continue;
        }
        67 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008604cf); };
            c.w64(12, t74);
            c.w64(7, c.r64(12).wrapping_mul(4));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008604db); };
            c.w64(8, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008604df); };
            c.w64(9, t76);
            let r77 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r77);
            blk = 68; continue;
        }
        68 => {
            let Some(t78) = c.ld32(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008604e6); };
            c.w32(0, t78);
            c.w32(9, c.r32(0));
            let r79 = c.r32(9) ^ c.r32(1);
            c.w32(9, r79);
            c.logic(r79, 32);
            if !c.lock_cmpxchg(c.r64(8).wrapping_add(c.r64(7)), c.r32(9), 32) { return RegfileExit::fault(0x70008604f0); }
            if c.cc_ne() { blk = 68; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w64(1, c.r64(0));
            let r80 = c.add(c.r32(1), c.r32(0), 32);
            c.w32(1, r80);
            if c.cc_o() { blk = 115; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 55; continue;
        }
        71 => {
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086050b); };
            c.w64(9, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086050f); };
            c.w64(8, t82);
            c.w64(7, c.r64(8).wrapping_add(c.r64(9)));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000860517); };
            c.w64(12, t83);
            c.w64(8, c.r64(12).wrapping_mul(4));
            blk = 72; continue;
        }
        72 => {
            let Some(t84) = c.ld32(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000860523); };
            c.w32(0, t84);
            c.w32(9, c.r32(0));
            let r85 = c.r32(9) ^ c.r32(1);
            c.w32(9, r85);
            c.logic(r85, 32);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r32(9), 32) { return RegfileExit::fault(0x700086052d); }
            if c.cc_ne() { blk = 72; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(0), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 120; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w32(1, c.r32(0));
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            blk = 55; continue;
        }
        75 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t86) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000860554); };
            c.w32(8, t86);
            let r87 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r87);
            let Some(t88) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086055b); };
            c.cmp(t88, 0x41u64, 16);
            if c.cc_e() { blk = 79; continue; }
            blk = 77; continue;
        }
        77 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000860563); }
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x7000860564); };
            c.w64(3, t89);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000860575, 0x7000860570);
        }
        78 => {
            c.w64(1, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000860578); };
            c.w64(6, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086057c); };
            c.w64(12, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000860580); };
            c.w64(9, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000860584); };
            c.w64(11, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000860588); };
            c.w64(7, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086058c); };
            c.w64(0, t95);
            blk = 79; continue;
        }
        79 => {
            c.w64(8, c.r64(0));
            let Some(t96) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000860593); };
            c.logic(t96 & 0x4u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t97) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086059e); };
            c.w32(8, t97);
            c.logic(c.r32(8) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t98) = c.ld64(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008605ab); };
            c.w64(1, t98);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            let r99 = c.neg(c.r64(1), 64);
            c.w64(1, r99);
            blk = 84; continue;
        }
        83 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 84; continue;
        }
        84 => {
            let r100 = c.r32(7) & 0xfcu64;
            c.w32(7, r100);
            c.logic(r100, 32);
            c.cmp(c.r32(7), 0x6cu64, 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r32(7), 0x68u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            return RegfileExit::fault(0x70008605d0);
        }
        87 => {
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            let r101 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r101);
            c.w64(8, c.r64(12).wrapping_mul(8));
            blk = 88; continue;
        }
        88 => {
            let Some(t102) = c.ld64(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008605e2); };
            c.w64(0, t102);
            c.w64(9, c.r64(0));
            let r103 = c.r64(9) ^ c.r64(1);
            c.w64(9, r103);
            c.logic(r103, 64);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r64(9), 64) { return RegfileExit::fault(0x70008605ec); }
            if c.cc_ne() { blk = 88; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 93; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008605fa); };
            c.w64(1, t104);
            let Some(t105) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000860601); };
            c.w64(7, t105);
            c.w64(8, c.r64(7).wrapping_add(0x10));
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000860608); };
            c.w64(9, t106);
            let Some(t107) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700086060f); };
            c.cmp(t107, c.r64(8), 64);
            if c.cc_be() { blk = 125; continue; }
            blk = 91; continue;
        }
        91 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x7000860618); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 92; continue;
        }
        92 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700086061f); };
            c.w64(7, t108);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000860626); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x7000860629); }
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x7000860630); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        93 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086063c); };
            c.w64(1, t109);
            let Some(t110) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000860643); };
            c.w64(7, t110);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086064a); };
            c.w64(8, t111);
            let Some(t112) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000860651); };
            c.cmp(t112, c.r64(9), 64);
            if c.cc_be() { blk = 127; continue; }
            blk = 94; continue;
        }
        94 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700086065a); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 95; continue;
        }
        95 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000860661); };
            c.w64(7, t113);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000860668); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700086066b); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        96 => {
            c.w64(7, c.r64(9).wrapping_add(c.r64(11)));
            c.w64(8, c.r64(12).wrapping_mul(8));
            blk = 97; continue;
        }
        97 => {
            let Some(t114) = c.ld64(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000860686); };
            c.w64(0, t114);
            c.w64(9, c.r64(0));
            let r115 = c.r64(9) ^ c.r64(1);
            c.w64(9, r115);
            c.logic(r115, 64);
            if !c.lock_cmpxchg(c.r64(7).wrapping_add(c.r64(8)), c.r64(9), 64) { return RegfileExit::fault(0x7000860690); }
            if c.cc_ne() { blk = 97; continue; }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 104; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086069e); };
            c.w64(1, t116);
            let Some(t117) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008606a5); };
            c.w64(7, t117);
            c.w64(8, c.r64(7).wrapping_add(0x10));
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008606ac); };
            c.w64(9, t118);
            let Some(t119) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008606b3); };
            c.cmp(t119, c.r64(8), 64);
            if c.cc_be() { blk = 129; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008606bc); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 101; continue;
        }
        101 => {
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008606c3); };
            c.w64(7, t120);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008606ca); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_g() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008606d3); }
            let r121 = c.neg(c.r64(0), 64);
            c.w64(0, r121);
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x70008606dd); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        103 => {
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70008606e9); }
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(0)) { return RegfileExit::fault(0x70008606f0); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        104 => {
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008606fc); };
            c.w64(1, t122);
            let Some(t123) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000860703); };
            c.w64(7, t123);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086070a); };
            c.w64(8, t124);
            let Some(t125) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000860711); };
            c.cmp(t125, c.r64(9), 64);
            if c.cc_be() { blk = 131; continue; }
            blk = 105; continue;
        }
        105 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700086071a); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 106; continue;
        }
        106 => {
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000860721); };
            c.w64(7, t126);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000860728); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700086072b); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        107 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700086073f); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000860740); }
            c.w32(0, 0x2u64);
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000860746); };
            c.w64(3, t127);
            return c.call(0x700086ed74, 0x7000860752, 0x700086074d);
        }
        108 => {
            return RegfileExit::fault(0x7000860752);
        }
        109 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000860758); }
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000860759); };
            c.w64(1, t128);
            let Some(t129) = c.ld32(c.r64(1).wrapping_add(0x9f)) else { return RegfileExit::fault(0x7000860760); };
            c.w32(1, t129);
            let r130 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r130);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000860769); }
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700086076a); };
            c.w64(3, t131);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700086077b, 0x7000860776);
        }
        110 => {
            return RegfileExit::fault(0x700086077b);
        }
        111 => {
            c.w32(7, 0xdcu64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000860781); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000860782); }
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000860783); };
            c.w64(3, t132);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000860794, 0x700086078f);
        }
        112 => {
            return RegfileExit::fault(0x7000860794);
        }
        113 => {
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000860795); };
            c.w64(3, t133);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008607a3, 0x700086079e);
        }
        114 => {
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008607a3); };
            c.w64(6, t134);
            blk = 1; continue;
        }
        115 => {
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008607ac); };
            c.w64(1, t135);
            let Some(t136) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008607b3); };
            c.w64(7, t136);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008607ba); }
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008607be); };
            c.w64(9, t137);
            let Some(t138) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008607c5); };
            c.cmp(t138, c.r64(8), 64);
            if c.cc_a() { blk = 118; continue; }
            blk = 116; continue;
        }
        116 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008607d4, 0x70008607cf);
        }
        117 => {
            c.w64(1, c.r64(0));
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008607d7); };
            c.w64(0, t139);
            blk = 119; continue;
        }
        118 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008607dd); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 119; continue;
        }
        119 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008607e4); };
            c.w64(7, t140);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008607eb); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008607f6); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        120 => {
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000860803); };
            c.w64(1, t141);
            let Some(t142) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700086080a); };
            c.w64(7, t142);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000860811); }
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000860815); };
            c.w64(9, t143);
            let Some(t144) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700086081c); };
            c.cmp(t144, c.r64(8), 64);
            if c.cc_a() { blk = 123; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700086082b, 0x7000860826);
        }
        122 => {
            c.w64(1, c.r64(0));
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086082e); };
            c.w64(0, t145);
            blk = 124; continue;
        }
        123 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x7000860834); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 124; continue;
        }
        124 => {
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700086083b); };
            c.w64(7, t146);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000860842); }
            c.w32(10, c.r32(0));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000860851); }
            c.w64(0, c.r64(1));
            blk = 55; continue;
        }
        125 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000860863); }
            return c.call(0x70007febb4, 0x700086086c, 0x7000860867);
        }
        126 => {
            c.w64(1, c.r64(0));
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086086f); };
            c.w64(0, t147);
            blk = 92; continue;
        }
        127 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x7000860882, 0x700086087d);
        }
        128 => {
            c.w64(1, c.r64(0));
            blk = 95; continue;
        }
        129 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700086088f); }
            return c.call(0x70007febb4, 0x7000860898, 0x7000860893);
        }
        130 => {
            c.w64(1, c.r64(0));
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086089b); };
            c.w64(0, t148);
            blk = 101; continue;
        }
        131 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70008608ae, 0x70008608a9);
        }
        132 => {
            c.w64(1, c.r64(0));
            blk = 106; continue;
        }
        133 => {
            let r149 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r149);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008608ba); }
            return c.call(0x70008708b4, 0x70008608c4, 0x70008608bf);
        }
        134 => {
            let Some(t150) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008608c4); };
            c.w32(1, t150);
            let r151 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r151);
            blk = 43; continue;
        }
        135 => {
            return RegfileExit::jump(0x70008608d4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008608f4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b70)) else { return RegfileExit::fault(0x70008608f4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000860914(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b78)) else { return RegfileExit::fault(0x7000860914); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000860934(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b80)) else { return RegfileExit::fault(0x7000860934); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000860954(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b88)) else { return RegfileExit::fault(0x7000860954); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000860974(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b90)) else { return RegfileExit::fault(0x7000860974); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000860994(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000860994); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000860998); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000860999); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086099a); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700086099f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008609a3); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008609a7); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 73; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008609b4); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_e() { blk = 59; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x70008609cd); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(8, 0xcu64);
            let r2 = c.add(c.r64(8), c.r64(1), 64);
            c.w64(8, r2);
            if c.cc_o() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let r3 = c.add(c.r64(8), 0x3u64, 64);
            c.w64(8, r3);
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008609e5); };
            c.w64(9, t4);
            let Some(t5) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008609ec); };
            c.w64(9, t5);
            let r6 = c.r64(8) & 0xfffffffffffffffcu64;
            c.w64(8, r6);
            c.logic(r6, 64);
            c.w64(11, c.r64(9).wrapping_add(c.r64(8)));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008609f7); };
            c.w64(12, t7);
            let Some(t8) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008609fe); };
            c.cmp(t8, c.r64(11), 64);
            if c.cc_be() { blk = 75; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(8), 0x20000u64, 64);
            if c.cc_ae() { blk = 75; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000860a15); };
            c.w64(8, t9);
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000860a1c); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x7000860a23); };
            c.w64(9, t10);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000860a2a); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000860a2e); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000860a36); }
            blk = 10; continue;
        }
        8 => {
            return RegfileExit::fault(0x7000860a3c);
        }
        9 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000860a3d); };
            c.w64(8, t11);
            blk = 10; continue;
        }
        10 => {
            c.w64(9, c.r64(5));
            let r12 = c.add(c.r64(9), 0x18u64, 64);
            c.w64(9, r12);
            c.w64(11, c.r64(9).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000860a4f); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000860a53); }
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_e() { blk = 57; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 13; continue;
        }
        12 => {
            let r13 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r13);
            c.w64(8, c.r64(7));
            blk = 13; continue;
        }
        13 => {
            let Some(t14) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000860a6c); };
            c.w64(15, t14);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000860a6f); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x7000860a73); }
            blk = 15; continue;
        }
        14 => {
            c.w64(15, c.r64(7));
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t15) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000860a82); };
            c.w32(3, t15);
            let r16 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000860a89); };
            c.cmp(t17, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t18) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000860a92); };
            c.cmp(t18, 0x43u64, 16);
            if c.cc_e() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(0, c.r64(15));
            return c.call(0x70008a1014, 0x7000860aa1, 0x7000860a9c);
        }
        19 => {
            c.w64(7, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000860aa4); };
            c.w64(1, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000860aa8); };
            c.w64(6, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000860aac); };
            c.w64(8, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000860ab0); };
            c.w64(11, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000860ab4); };
            c.w64(12, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000860ab8); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000860abc); };
            c.w64(9, t25);
            blk = 14; continue;
        }
        20 => {
            let Some(t26) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000860ac2); };
            c.w32(15, t26);
            let r27 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r27);
            c.w64(7, c.r64(15));
            blk = 14; continue;
        }
        21 => {
            let r28 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r28);
            blk = 24; continue;
        }
        22 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000860ad3); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(15, cv);
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_o() { blk = 100; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(15, c.r32(15));
            blk = 24; continue;
        }
        24 => {
            c.w32(7, c.r16(15));
            c.cmp(c.r32(7), 0xffu64, 32);
            if c.cc_g() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(7, c.r64(8));
            if !c.st8(c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0xb), c.r8(15)) { return RegfileExit::fault(0x7000860afa); }
            let r29 = c.add(c.r64(9), 0x8u64, 64);
            c.w64(9, r29);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_ne() { blk = 12; continue; }
            blk = 26; continue;
        }
        26 => {
            blk = 58; continue;
        }
        27 => {
            let Some(t30) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000860b11); };
            c.cmp(t30, 0x0u64, 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(7, c.r64(0));
            let r31 = c.add(c.r32(0), c.r32(7), 32);
            c.w32(0, r31);
            if c.cc_o() { blk = 35; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w32(0, 0xcu64);
            let r32 = c.add(c.r64(9), c.r64(0), 64);
            c.w64(9, r32);
            if c.cc_o() { blk = 34; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000860b2b); };
            c.w64(0, t33);
            let Some(t34) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000860b32); };
            c.w64(0, t34);
            let r35 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r35);
            let r36 = c.r64(9) & 0xfffffffffffffffcu64;
            c.w64(9, r36);
            c.logic(r36, 64);
            c.w64(3, c.r64(0).wrapping_add(c.r64(9)));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000860b41); };
            c.w64(2, t37);
            let Some(t38) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000860b48); };
            c.cmp(t38, c.r64(3), 64);
            if c.cc_be() { blk = 77; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r64(9), 0x20000u64, 64);
            if c.cc_ae() { blk = 77; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000860b5e); };
            c.w64(9, t39);
            if !c.st64(c.r64(9), c.r64(3)) { return RegfileExit::fault(0x7000860b65); }
            c.w64(9, c.r64(0).wrapping_add(0x1));
            blk = 33; continue;
        }
        33 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x7000860b6c); };
            c.w64(0, t40);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000860b73); }
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000860b77); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x7000860b7f); }
            c.w64(0, c.r64(9));
            blk = 37; continue;
        }
        34 => {
            return RegfileExit::fault(0x7000860b88);
        }
        35 => {
            return RegfileExit::fault(0x7000860b89);
        }
        36 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000860b8a); };
            c.w64(0, t41);
            blk = 37; continue;
        }
        37 => {
            c.w64(7, c.r64(12).wrapping_add(0xb));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x7000860b96); }
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_e() { blk = 41; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(3, 0xbu64);
            c.w64(9, c.r64(3));
            blk = 40; continue;
        }
        39 => {
            let r42 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r42);
            c.w64(3, c.r64(2));
            blk = 40; continue;
        }
        40 => {
            c.w64(2, c.r64(3).wrapping_add(0x1));
            let Some(t43) = c.ld8(c.r64(8).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000860bb5); };
            c.w32(3, t43);
            if !c.st16(c.r64(0).wrapping_add(c.r64(9)), c.r16(3)) { return RegfileExit::fault(0x7000860bba); }
            c.cmp(c.r64(7), c.r64(2), 64);
            if c.cc_ne() { blk = 39; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(7, c.r64(5));
            c.w64(8, c.r64(12).wrapping_add(0x1));
            c.w64(7, c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x18));
            if !c.st16(c.r64(0).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0xb), c.r16(15)) { return RegfileExit::fault(0x7000860bd1); }
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_e() { blk = 56; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(9, c.r64(11));
            blk = 44; continue;
        }
        43 => {
            let r44 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r44);
            c.w64(9, c.r64(1));
            blk = 44; continue;
        }
        44 => {
            let Some(t45) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000860bec); };
            c.w64(11, t45);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000860bef); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x7000860bf3); }
            blk = 46; continue;
        }
        45 => {
            c.w64(11, c.r64(1));
            blk = 46; continue;
        }
        46 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t46) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000860c02); };
            c.w32(12, t46);
            let r47 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r47);
            let Some(t48) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000860c09); };
            c.cmp(t48, c.r32(12), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t49) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000860c12); };
            c.cmp(t49, 0x43u64, 16);
            if c.cc_e() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a1014, 0x7000860c23, 0x7000860c1e);
        }
        50 => {
            c.w64(1, c.r64(0));
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000860c26); };
            c.w64(6, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000860c2a); };
            c.w64(0, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000860c2e); };
            c.w64(8, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000860c32); };
            c.w64(9, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000860c36); };
            c.w64(7, t54);
            blk = 45; continue;
        }
        51 => {
            let Some(t55) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000860c3c); };
            c.w32(11, t55);
            let r56 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r56);
            c.w64(1, c.r64(11));
            blk = 45; continue;
        }
        52 => {
            let r57 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r57);
            blk = 55; continue;
        }
        53 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000860c4d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_o() { blk = 98; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w32(11, c.r32(11));
            blk = 55; continue;
        }
        55 => {
            let r58 = c.add(c.r64(7), 0x8u64, 64);
            c.w64(7, r58);
            if !c.st16(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0xb), c.r16(11)) { return RegfileExit::fault(0x7000860c69); }
            c.w64(1, c.r64(9));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_ne() { blk = 43; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000860c7b); };
            c.w64(1, t59);
            let r60 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r60);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000860c86); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000860c87); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000860c8d); }
            return c.ret(0x0, 0x7000860c8f);
        }
        57 => {
            c.w64(7, c.r64(8));
            blk = 58; continue;
        }
        58 => {
            let r61 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r61);
            c.w64(0, c.r64(7));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000860c9d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000860c9e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000860ca4); }
            return c.ret(0x0, 0x7000860ca6);
        }
        59 => {
            c.w64(7, c.r64(5));
            let Some(t62) = c.ld64(c.r64(7).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000860caa); };
            c.w64(7, t62);
            blk = 61; continue;
        }
        60 => {
            c.w64(7, c.r64(0));
            blk = 61; continue;
        }
        61 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 67; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t63) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000860cb9); };
            c.w32(8, t63);
            let r64 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r64);
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000860cc0); };
            c.cmp(t65, c.r32(8), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t66) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000860cc9); };
            c.cmp(t66, 0x43u64, 16);
            if c.cc_e() { blk = 66; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(0, c.r64(7));
            return c.call(0x70008a1014, 0x7000860cd9, 0x7000860cd4);
        }
        65 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000860cd9); };
            c.w64(6, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000860cdd); };
            c.w64(1, t68);
            blk = 60; continue;
        }
        66 => {
            let Some(t69) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000860ce3); };
            c.w32(7, t69);
            let r70 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r70);
            c.w64(0, c.r64(7));
            blk = 60; continue;
        }
        67 => {
            let r71 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r71);
            blk = 70; continue;
        }
        68 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000860cf2); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 96; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w32(7, c.r32(7));
            blk = 70; continue;
        }
        70 => {
            c.w32(8, c.r16(7));
            c.cmp(c.r32(8), 0xffu64, 32);
            if c.cc_g() { blk = 79; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(9, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x7000860d21); };
            c.w64(11, t72);
            let Some(t73) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000860d28); };
            c.w32(0, t73);
            let r74 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r74);
            let Some(t75) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000860d30); };
            c.cmp(t75, c.r32(0), 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 72; continue;
        }
        72 => {
            let r76 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r76);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000860d44); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000860d45); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000860d4b); }
            return c.ret(0x0, 0x7000860d4d);
        }
        73 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000860d4e); };
            c.w64(3, t77);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x7000860d5f, 0x7000860d5a);
        }
        74 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000860d5f); };
            c.w64(6, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000860d63); };
            c.w64(0, t79);
            blk = 1; continue;
        }
        75 => {
            c.w64(2, c.r64(8));
            return c.call(0x70007febb4, 0x7000860d74, 0x7000860d6f);
        }
        76 => {
            c.w64(8, c.r64(0));
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000860d77); };
            c.w64(1, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000860d7b); };
            c.w64(6, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000860d7f); };
            c.w64(0, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000860d83); };
            c.w64(7, t83);
            blk = 7; continue;
        }
        77 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x7000860d8c); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febb4, 0x7000860d98, 0x7000860d93);
        }
        78 => {
            c.w64(9, c.r64(0));
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000860d9b); };
            c.w64(1, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000860d9f); };
            c.w64(6, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000860da3); };
            c.w64(8, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000860da7); };
            c.w64(11, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000860dab); };
            c.w64(12, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000860daf); };
            c.w64(15, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000860db3); };
            c.w64(7, t90);
            blk = 33; continue;
        }
        79 => {
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000860dbc); };
            c.w64(8, t91);
            let Some(t92) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000860dc3); };
            c.w64(9, t92);
            c.w64(11, c.r64(9).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000860dca); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000860dce); };
            c.w64(12, t93);
            let Some(t94) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000860dd5); };
            c.cmp(t94, c.r64(11), 64);
            if c.cc_a() { blk = 82; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x7000860de5, 0x7000860de0);
        }
        81 => {
            blk = 83; continue;
        }
        82 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000860de7); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 83; continue;
        }
        83 => {
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x7000860df1); };
            c.w64(1, t95);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000860df8); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x7000860dfb); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000860e02); }
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000860e09); };
            c.w64(7, t96);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(7)) { return RegfileExit::fault(0x7000860e0d); }
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000860e11); };
            c.w64(1, t97);
            let r98 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r98);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000860e1c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000860e1d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000860e23); }
            return c.ret(0x0, 0x7000860e25);
        }
        84 => {
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000860e26); };
            c.w64(12, t99);
            let Some(t100) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000860e2d); };
            c.w64(15, t100);
            c.w64(0, c.r64(15).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000860e35); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000860e39); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000860e3d); }
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000860e41); };
            c.w64(3, t101);
            let Some(t102) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000860e48); };
            c.cmp(t102, c.r64(0), 64);
            if c.cc_a() { blk = 87; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x7000860e57, 0x7000860e52);
        }
        86 => {
            blk = 88; continue;
        }
        87 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x7000860e59); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(12));
            blk = 88; continue;
        }
        88 => {
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x7000860e64); };
            c.w64(1, t103);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000860e6b); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x7000860e6e); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000860e75); }
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000860e7c); };
            c.w64(7, t104);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(7)) { return RegfileExit::fault(0x7000860e80); }
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x7000860e84); };
            c.w64(11, t105);
            let Some(t106) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000860e8b); };
            c.w32(1, t106);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000860e95); };
            c.w64(8, t107);
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_b() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            return RegfileExit::fault(0x7000860e9e);
        }
        90 => {
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000860e9f); };
            c.w64(9, t108);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x7000860ea3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r109 = c.r64(1) & c.r64(11);
            c.w64(1, r109);
            c.logic(r109, 64);
            let Some(t110) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000860eb5); };
            c.logic(t110 & 0x4u64, 8);
            if c.cc_ne() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000860ebb); };
            c.w64(1, t111);
            let r112 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r112);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000860ec6); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000860ec7); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000860ecd); }
            return c.ret(0x0, 0x7000860ecf);
        }
        93 => {
            c.w32(0, c.r32(0));
            let r113 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r113);
            c.w64(1, 0xfffffffffffc0000u64);
            let r114 = c.r64(1) & c.r64(0);
            c.w64(1, r114);
            c.logic(r114, 64);
            let Some(t115) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000860edf); };
            c.logic(t115 & 0x2u64, 8);
            if c.cc_e() { blk = 92; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w64(7, c.r64(11).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000860ee9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000860eea); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(11));
            return c.call(0x70007f0e54, 0x7000860ef7, 0x7000860ef2);
        }
        95 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000860ef7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000860ef9); }
            blk = 92; continue;
        }
        96 => {
            let r116 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r116);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000860f00); }
            return c.call(0x70008708b4, 0x7000860f0a, 0x7000860f05);
        }
        97 => {
            let Some(t117) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000860f0a); };
            c.w32(7, t117);
            let r118 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r118);
            blk = 69; continue;
        }
        98 => {
            let r119 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r119);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000860f1a); }
            return c.call(0x70008708b4, 0x7000860f24, 0x7000860f1f);
        }
        99 => {
            let Some(t120) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000860f24); };
            c.w32(11, t120);
            let r121 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r121);
            blk = 54; continue;
        }
        100 => {
            let r122 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r122);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000860f35); }
            return c.call(0x70008708b4, 0x7000860f3f, 0x7000860f3a);
        }
        101 => {
            let Some(t123) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000860f3f); };
            c.w32(15, t123);
            let r124 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r124);
            blk = 23; continue;
        }
        102 => {
            return RegfileExit::jump(0x7000860f50);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000860f54(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1b98)) else { return RegfileExit::fault(0x7000860f54); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000860f74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000860f74); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000860f78); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000860f79); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000860f7a); }
            let r0 = c.sub(c.r64(4), 0x68u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000860f7f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000860f83); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 200; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000860f8d); };
            c.w32(8, t2);
            let r3 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000860f94); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000860f9b); };
            c.w64(0, t6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000860f9f); };
            c.cmp(t7, c.r32(0), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000860fae); };
            c.cmp(t8, c.r32(0), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000860fbd); }
            let r9 = c.r32(9) | c.r32(11);
            c.w32(9, r9);
            c.logic(r9, 32);
            if c.cc_ne() { blk = 198; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000860fca); };
            c.w64(9, t10);
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000860fda); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t11) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000860fe8); };
            c.w32(12, t11);
            let r12 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r12);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x7000860ff1); };
            c.w64(15, t13);
            let Some(t14) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000860ff8); };
            c.cmp(t14, c.r32(1), 32);
            if c.cc_e() { blk = 202; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000861002); };
            c.w32(15, t15);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x9au64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x7000861015);
        }
        6 => {
            let Some(t16) = c.ld32(c.r64(8).wrapping_add(0x26f)) else { return RegfileExit::fault(0x7000861016); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            let Some(t18) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000861020); };
            c.cmp(t18, c.r32(12), 32);
            if c.cc_ne() { blk = 202; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t19) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700086102a); };
            c.w32(3, t19);
            c.logic(c.r32(3) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 202; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(15), 0x9fu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x7000861043);
        }
        10 => {
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000861044); };
            c.w32(15, t20);
            let r21 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r21);
            let Some(t22) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086104c); };
            c.w32(15, t22);
            let r23 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r23);
            let Some(t24) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000861053); };
            c.w32(3, t24);
            let r25 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r25);
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x283)) else { return RegfileExit::fault(0x700086105a); };
            c.cmp(t26, c.r32(15), 32);
            if c.cc_ne() { blk = 202; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t27) = c.ld32(c.r64(3).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000861067); };
            c.w32(15, t27);
            c.w32(15, c.r32(15));
            let r28 = c.r32(15) & 0x8u64;
            c.w32(15, r28);
            c.logic(r28, 32);
            c.cmp(c.r32(15), 0x8u64, 32);
            if c.cc_ne() { blk = 202; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t29) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086107c); };
            c.w32(12, t29);
            c.w32(12, c.r32(12));
            let r30 = c.r32(12) & 0x2u64;
            c.w32(12, r30);
            c.logic(r30, 32);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let r31 = c.r64(12) & 0xfffffffffffffffeu64;
            c.w64(12, r31);
            c.logic(r31, 64);
            let r32 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r32);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 239; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 15; continue;
        }
        14 => {
            c.w32(12, 0x1u64);
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 154; continue; }
            blk = 16; continue;
        }
        16 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 152; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t33) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008610b8); };
            c.w32(12, t33);
            let r34 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r34);
            let Some(t35) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008610bf); };
            c.w32(12, t35);
            c.cmp(c.r16(12), 0x40u64, 16);
            if c.cc_b() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 153; continue;
        }
        19 => {
            let Some(t36) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008610d3); };
            c.w32(15, t36);
            let r37 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r37);
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008610da); };
            c.w64(3, t38);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t39) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008610e3); };
            c.cmp(t39, c.r32(1), 32);
            if c.cc_e() { blk = 153; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t40) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008610ec); };
            c.w32(3, t40);
            let r41 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r41);
            let Some(t42) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008610f2); };
            c.w32(3, t42);
            let r43 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r43);
            let Some(t44) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008610f8); };
            c.w32(2, t44);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x9au64, 64);
            if c.cc_a() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x700086110a);
        }
        22 => {
            let Some(t45) = c.ld32(c.r64(3).wrapping_add(0x26f)) else { return RegfileExit::fault(0x700086110b); };
            c.w32(7, t45);
            let r46 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r46);
            let Some(t47) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000861114); };
            c.cmp(t47, c.r32(15), 32);
            if c.cc_ne() { blk = 153; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t48) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700086111e); };
            c.w32(7, t48);
            c.logic(c.r32(7) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 153; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r64(2), 0x9fu64, 64);
            if c.cc_a() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            return RegfileExit::fault(0x7000861137);
        }
        26 => {
            let Some(t49) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000861138); };
            c.w32(2, t49);
            let r50 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r50);
            let Some(t51) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086113f); };
            c.w32(2, t51);
            let r52 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r52);
            let Some(t53) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000861145); };
            c.w32(7, t53);
            let r54 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r54);
            let Some(t55) = c.ld32(c.r64(3).wrapping_add(0x283)) else { return RegfileExit::fault(0x700086114b); };
            c.cmp(t55, c.r32(2), 32);
            if c.cc_ne() { blk = 153; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t56) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000861157); };
            c.w32(3, t56);
            let Some(t57) = c.ld32(c.r64(7).wrapping_add(0xc7)) else { return RegfileExit::fault(0x700086115a); };
            c.w32(2, t57);
            let r58 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r58);
            let r59 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r59);
            let r60 = c.r32(2) & c.r32(3);
            c.w32(2, r60);
            c.logic(r60, 32);
            let r61 = c.r32(2) & 0x4u64;
            c.w32(2, r61);
            c.logic(r61, 32);
            c.cmp(c.r32(2), 0x4u64, 32);
            if c.cc_ne() { blk = 153; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 268; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t62) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086117c); };
            c.cmp(t62, 0xbau64, 16);
            if c.cc_b() { blk = 268; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(11, c.r16(12));
            c.cmp(c.r32(11), 0x40u64, 32);
            if c.cc_ge() { blk = 245; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t63) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000861197); };
            c.w32(11, t63);
            let r64 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x700086119e); }
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008611a2); };
            c.w64(12, t65);
            let Some(t66) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008611a9); };
            c.cmp(t66, c.r32(1), 32);
            if c.cc_e() { blk = 117; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t67) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008611b4); };
            c.w32(12, t67);
            let r68 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r68);
            let Some(t69) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008611bb); };
            c.w32(12, t69);
            let r70 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r70);
            let Some(t71) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008611c3); };
            c.w32(15, t71);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x9au64, 64);
            if c.cc_a() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            return RegfileExit::fault(0x70008611d7);
        }
        34 => {
            let Some(t72) = c.ld32(c.r64(12).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008611d8); };
            c.w32(3, t72);
            let r73 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r73);
            let Some(t74) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008611e3); };
            c.cmp(t74, c.r32(11), 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t75) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008611ed); };
            c.w32(3, t75);
            c.logic(c.r32(3) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(15), 0x9fu64, 64);
            if c.cc_a() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            return RegfileExit::fault(0x7000861206);
        }
        38 => {
            let Some(t76) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000861207); };
            c.w32(11, t76);
            let r77 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r77);
            let Some(t78) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086120e); };
            c.w32(11, t78);
            let r79 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r79);
            let Some(t80) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000861215); };
            c.w32(15, t80);
            let r81 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r81);
            let Some(t82) = c.ld32(c.r64(12).wrapping_add(0x283)) else { return RegfileExit::fault(0x700086121c); };
            c.cmp(t82, c.r32(11), 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t83) = c.ld32(c.r64(15).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700086122a); };
            c.w32(11, t83);
            c.w32(11, c.r32(11));
            let r84 = c.r32(11) & 0x8u64;
            c.w32(11, r84);
            c.logic(r84, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t85) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086123f); };
            c.w32(11, t85);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let Some(t86) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000861249); };
            c.w32(12, t86);
            let r87 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r87);
            c.logic(c.r8(11) & 0xffffffffffffff80u64, 8);
            c.w8(15, (c.cc_ne() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x700086125c); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 43; continue;
        }
        42 => {
            c.w32(3, 0x1u64);
            blk = 43; continue;
        }
        43 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(2, (c.cc_ne() as u64));
            c.w32(2, c.r8(2));
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let r88 = c.add(c.r32(3), 0x1u64, 32);
            c.w32(3, r88);
            blk = 45; continue;
        }
        45 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            c.w8(7, (c.cc_ne() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            let r89 = c.add(c.r32(3), 0x1u64, 32);
            c.w32(3, r89);
            blk = 47; continue;
        }
        47 => {
            c.logic(c.r8(11) & 0x40u64, 8);
            c.w8(6, (c.cc_ne() as u64));
            c.w32(6, c.r8(6));
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            let r90 = c.add(c.r32(3), 0x1u64, 32);
            c.w32(3, r90);
            blk = 49; continue;
        }
        49 => {
            c.logic(c.r8(11) & 0x4u64, 8);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            let r91 = c.add(c.r32(3), 0x1u64, 32);
            c.w32(3, r91);
            blk = 51; continue;
        }
        51 => {
            c.logic(c.r8(11) & 0x20u64, 8);
            c.w8(0, (c.cc_ne() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            let r92 = c.add(c.r32(3), 0x1u64, 32);
            c.w32(3, r92);
            blk = 53; continue;
        }
        53 => {
            c.logic(c.r8(11) & 0x10u64, 8);
            c.w8(1, (c.cc_ne() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008612da); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            let r93 = c.add(c.r32(3), 0x1u64, 32);
            c.w32(3, r93);
            blk = 55; continue;
        }
        55 => {
            c.logic(c.r8(11) & 0x8u64, 8);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008612f2); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            let r94 = c.add(c.r32(3), 0x1u64, 32);
            c.w32(3, r94);
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(12, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w32(8, 0xcu64);
            let r95 = c.add(c.r64(12), c.r64(8), 64);
            c.w64(12, r95);
            if c.cc_o() { blk = 63; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000861316); };
            c.w64(8, t96);
            let Some(t97) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700086131d); };
            c.w64(8, t97);
            let r98 = c.add(c.r64(12), 0x3u64, 64);
            c.w64(12, r98);
            let r99 = c.r64(12) & 0xfffffffffffffffcu64;
            c.w64(12, r99);
            c.logic(r99, 64);
            c.w64(11, c.r64(8).wrapping_add(c.r64(12)));
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086132c); };
            c.w64(1, t100);
            let Some(t101) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000861333); };
            c.cmp(t101, c.r64(11), 64);
            if c.cc_be() { blk = 247; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 247; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000861349); };
            c.w64(12, t102);
            if !c.st64(c.r64(12), c.r64(11)) { return RegfileExit::fault(0x7000861350); }
            let r103 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r103);
            blk = 62; continue;
        }
        62 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x7000861358); };
            c.w64(11, t104);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700086135f); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000861363); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x700086136b); }
            c.w64(3, c.r64(8));
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000861372); };
            c.w64(8, t105);
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000861376); };
            c.w64(12, t106);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086137a); };
            c.w64(11, t107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086137e); };
            c.w64(1, t108);
            blk = 65; continue;
        }
        63 => {
            return RegfileExit::fault(0x7000861384);
        }
        64 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000861385); };
            c.w64(3, t109);
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(15, 0xbu64);
            blk = 68; continue;
        }
        67 => {
            if !c.st8(c.r64(3).wrapping_add(0xb), 0x64u64) { return RegfileExit::fault(0x700086139a); }
            c.w32(15, 0xcu64);
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.st8(c.r64(3).wrapping_add(c.r64(15)), 0x67u64) { return RegfileExit::fault(0x70008613a9); }
            let r110 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r110);
            blk = 70; continue;
        }
        70 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            if !c.st8(c.r64(3).wrapping_add(c.r64(15)), 0x69u64) { return RegfileExit::fault(0x70008613b7); }
            let r111 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r111);
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            if !c.st8(c.r64(3).wrapping_add(c.r64(15)), 0x6cu64) { return RegfileExit::fault(0x70008613c5); }
            let r112 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r112);
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 75; continue;
        }
        75 => {
            if !c.st8(c.r64(3).wrapping_add(c.r64(15)), 0x6du64) { return RegfileExit::fault(0x70008613d4); }
            let r113 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r113);
            blk = 76; continue;
        }
        76 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            if !c.st8(c.r64(3).wrapping_add(c.r64(15)), 0x73u64) { return RegfileExit::fault(0x70008613e2); }
            let r114 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r114);
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(9, c.r64(15).wrapping_add(0x1));
            if !c.st8(c.r64(3).wrapping_add(c.r64(15)), 0x75u64) { return RegfileExit::fault(0x70008613f4); }
            c.w64(15, c.r64(9));
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            if !c.st8(c.r64(3).wrapping_add(c.r64(15)), 0x79u64) { return RegfileExit::fault(0x7000861402); }
            blk = 82; continue;
        }
        82 => {
            let Some(t115) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000861407); };
            c.w32(9, t115);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x9au64, 64);
            if c.cc_a() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            return RegfileExit::fault(0x700086141a);
        }
        84 => {
            let Some(t116) = c.ld32(c.r64(8).wrapping_add(0x26f)) else { return RegfileExit::fault(0x700086141b); };
            c.w32(9, t116);
            let r117 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r117);
            let Some(t118) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000861425); };
            c.w32(9, t118);
            let r119 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r119);
            c.w64(0, c.r64(12));
            let Some(t120) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086142f); };
            c.cmp(t120, c.r32(0), 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 85; continue;
        }
        85 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 249; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t121) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000861440); };
            c.w32(11, t121);
            let r122 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r122);
            let Some(t123) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000861447); };
            c.cmp(t123, 0x40u64, 16);
            if c.cc_b() { blk = 89; continue; }
            blk = 87; continue;
        }
        87 => {
            blk = 249; continue;
        }
        88 => {
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000861454); };
            c.w64(0, t124);
            c.w64(12, c.r64(0));
            blk = 89; continue;
        }
        89 => {
            let Some(t125) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700086145e); };
            c.logic(t125 & 0x200000u64, 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000861468); };
            c.w64(2, t126);
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700086146f); };
            c.w64(12, t127);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000861478); };
            c.w64(7, t128);
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086147f); };
            c.w64(11, t129);
            blk = 103; continue;
        }
        91 => {
            let Some(t130) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700086148b); };
            c.w64(11, t130);
            let Some(t131) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000861492); };
            c.w64(12, t131);
            c.w64(15, c.r64(12).wrapping_add(0x4c));
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700086149a); };
            c.w64(2, t132);
            let Some(t133) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008614a1); };
            c.cmp(t133, c.r64(15), 64);
            if c.cc_be() { blk = 251; continue; }
            blk = 92; continue;
        }
        92 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008614aa); }
            let r134 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r134);
            blk = 93; continue;
        }
        93 => {
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008614b1); };
            c.w64(15, t135);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008614b8); }
            c.w32(15, 0x22u64);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008614c3); }
            let Some(t136) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008614c8); };
            c.w32(15, t136);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_be() { blk = 253; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70008614df); }
            let Some(t137) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008614e4); };
            c.w32(15, t137);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_be() { blk = 254; continue; }
            blk = 95; continue;
        }
        95 => {
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x70008614f9); }
            let Some(t138) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008614fe); };
            c.w32(15, t138);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x2u64, 64);
            if c.cc_be() { blk = 255; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w32(15, 0x8u64);
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x7000861519); }
            let Some(t139) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086151e); };
            c.w32(15, t139);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x3u64, 64);
            if c.cc_be() { blk = 256; continue; }
            blk = 97; continue;
        }
        97 => {
            c.w32(15, 0x2u64);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(15)) { return RegfileExit::fault(0x7000861539); }
            let Some(t140) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086153e); };
            c.w32(15, t140);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x4u64, 64);
            if c.cc_be() { blk = 257; continue; }
            blk = 98; continue;
        }
        98 => {
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(6)) { return RegfileExit::fault(0x7000861553); }
            c.w64(15, c.r64(12).wrapping_add(0x4b));
            c.w64(1, c.r64(12).wrapping_add(0x1b));
            c.cmp(c.r64(1), c.r64(15), 64);
            if c.cc_e() { blk = 102; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000861567); };
            c.w64(7, t141);
            blk = 100; continue;
        }
        100 => {
            if !c.st32(c.r64(1), c.r32(7)) { return RegfileExit::fault(0x700086156e); }
            let r142 = c.add(c.r64(1), 0x4u64, 64);
            c.w64(1, r142);
            c.cmp(c.r64(15), c.r64(1), 64);
            if c.cc_ne() { blk = 100; continue; }
            blk = 101; continue;
        }
        101 => {
            blk = 103; continue;
        }
        102 => {
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086157b); };
            c.w64(7, t143);
            blk = 103; continue;
        }
        103 => {
            let Some(t144) = c.ld8(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000861582); };
            c.w32(15, t144);
            if let Some(sr) = c.shl(c.r64(15), 0x2u64, 64) { c.w64(15, sr); }
            let Some(t145) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700086158b); };
            c.w64(1, t145);
            c.w64(8, c.r64(15).wrapping_add(c.r64(1)));
            let Some(t146) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000861592); };
            c.cmp(t146, c.r64(8), 64);
            if c.cc_be() { blk = 258; continue; }
            blk = 104; continue;
        }
        104 => {
            if !c.st64(c.r64(11), c.r64(8)) { return RegfileExit::fault(0x700086159b); }
            c.w64(8, c.r64(1).wrapping_add(0x1));
            blk = 105; continue;
        }
        105 => {
            c.w64(11, c.r64(15).wrapping_sub(0x1));
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008615a6); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008615aa); }
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008615ae); };
            c.w64(12, t147);
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008615b5); }
            let Some(t148) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008615b9); };
            c.w32(12, t148);
            c.w64(15, c.r64(12));
            let r149 = c.r32(15) & 0xe0000000u64;
            c.w32(15, r149);
            c.logic(r149, 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 106; continue;
        }
        106 => {
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 115; continue; }
            blk = 107; continue;
        }
        107 => {
            let r150 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r150);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(7)) { return RegfileExit::fault(0x70008615d3); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 107; continue; }
            blk = 108; continue;
        }
        108 => {
            blk = 115; continue;
        }
        109 => {
            let r151 = c.sub(c.r32(12), 0x20000000u64, 32);
            c.w32(12, r151);
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70008615e6); }
            let Some(t152) = c.ld8(c.r64(9).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008615ea); };
            c.w32(12, t152);
            c.w64(12, c.r64(12).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_e() { blk = 112; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x70008615fc); };
            c.w64(2, t153);
            blk = 111; continue;
        }
        111 => {
            let r154 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r154);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(2)) { return RegfileExit::fault(0x7000861607); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ne() { blk = 111; continue; }
            blk = 112; continue;
        }
        112 => {
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_e() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            let r155 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r155);
            if !c.st32(c.r64(12).wrapping_add(c.r64(8)), c.r32(7)) { return RegfileExit::fault(0x700086161a); }
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_ne() { blk = 113; continue; }
            blk = 114; continue;
        }
        114 => {
            let r156 = c.add(c.r32(15), 0xe0000000u64, 32);
            c.w32(15, r156);
            if c.cc_e() { blk = 260; continue; }
            blk = 115; continue;
        }
        115 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000861631); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000861633); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000861634); }
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0x2670)) else { return RegfileExit::fault(0x7000861635); };
            c.w64(3, t157);
            c.w32(0, 0x3u64);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861641); };
            c.w64(6, t158);
            return c.call(0x700086ed74, 0x700086164a, 0x7000861645);
        }
        116 => {
            let Some(t159) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086164a); };
            c.w64(8, t159);
            let Some(t160) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700086164e); };
            c.w32(8, t160);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(8)) { return RegfileExit::fault(0x7000861652); }
            let Some(t161) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000861656); };
            c.w32(8, t161);
            c.w32(8, c.r32(8));
            c.w64(9, c.r64(8));
            let r162 = c.r32(9) & 0x20u64;
            c.w32(9, r162);
            c.logic(r162, 32);
            let r163 = c.r32(8) & 0x2u64;
            c.w32(8, r163);
            c.logic(r163, 32);
            let r164 = c.shr(c.r32(9), 0x4u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r164);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r165 = c.r64(8) & 0xfffffffffffffffeu64;
            c.w64(8, r165);
            c.logic(r165, 64);
            let r166 = c.r64(9) & 0xfffffffffffffffeu64;
            c.w64(9, r166);
            c.logic(r166, 64);
            let r167 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r167);
            let r168 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r168);
            blk = 144; continue;
        }
        117 => {
            let Some(t169) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000861685); };
            c.w32(11, t169);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x9au64, 64);
            if c.cc_a() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            return RegfileExit::fault(0x7000861698);
        }
        119 => {
            let Some(t170) = c.ld32(c.r64(8).wrapping_add(0x26f)) else { return RegfileExit::fault(0x7000861699); };
            c.w32(11, t170);
            let r171 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r171);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008616a3); }
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x70008616a7); };
            c.w64(3, t172);
            c.w64(0, c.r64(9));
            return c.call(0x7000870914, 0x70008616b6, 0x70008616b1);
        }
        120 => {
            let Some(t173) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008616b6); };
            c.cmp(t173, c.r32(0), 32);
            if c.cc_e() { blk = 130; continue; }
            blk = 121; continue;
        }
        121 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 262; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t174) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008616cb); };
            c.w32(8, t174);
            let r175 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r175);
            let Some(t176) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008616d2); };
            c.cmp(t176, 0xbau64, 16);
            if c.cc_b() { blk = 262; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t177) = c.ld64(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x70008616df); };
            c.w64(3, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008616e6); };
            c.w64(6, t178);
            return c.call(0x7000870914, 0x70008616ef, 0x70008616ea);
        }
        124 => {
            let Some(t179) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008616ef); };
            c.cmp(t179, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t180) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008616fe); };
            c.cmp(t180, c.r32(0), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let r181 = c.r32(8) | c.r32(9);
            c.w32(8, r181);
            c.logic(r181, 32);
            if c.cc_ne() { blk = 130; continue; }
            blk = 125; continue;
        }
        125 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 128; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t182) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000861716); };
            c.w32(8, t182);
            let r183 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r183);
            let Some(t184) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700086171d); };
            c.logic(t184 & 0x40u64, 8);
            if c.cc_e() { blk = 128; continue; }
            blk = 127; continue;
        }
        127 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x7000861724); }
            blk = 130; continue;
        }
        128 => {
            c.w32(8, 0x26eu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000861730); }
            let Some(t185) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000861732); };
            c.w64(3, t185);
            c.w32(0, 0x1u64);
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086173e); };
            c.w64(6, t186);
            return c.call(0x700086ed74, 0x7000861747, 0x7000861742);
        }
        129 => {
            return RegfileExit::fault(0x7000861747);
        }
        130 => {
            let Some(t187) = c.ld64(c.r64(13).wrapping_add(0x700)) else { return RegfileExit::fault(0x7000861748); };
            c.w64(3, t187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086174f); };
            c.w64(0, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861753); };
            c.w64(6, t189);
            return c.call(0x7000870914, 0x700086175c, 0x7000861757);
        }
        131 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 264; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t190) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000861764); };
            c.w32(8, t190);
            let r191 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r191);
            let Some(t192) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086176b); };
            c.cmp(t192, 0x40u64, 16);
            if c.cc_ae() { blk = 264; continue; }
            blk = 133; continue;
        }
        133 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000861777); }
            let Some(t193) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000861778); };
            if !c.push64(t193) { return RegfileExit::fault(0x7000861778); }
            let Some(t194) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086177b); };
            c.w64(8, t194);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000861782); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000861784); }
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861788); };
            c.w64(6, t195);
            c.w32(0, 0x2u64);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000861791); };
            c.w64(2, t196);
            c.w64(8, c.r64(0));
            c.w64(7, c.r64(2));
            return c.call(0x70007f7214, 0x70008617a0, 0x700086179b);
        }
        134 => {
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008617a0); };
            c.w64(6, t197);
            let Some(t198) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008617a4); };
            c.w64(3, t198);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008617ab); }
            let Some(t199) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008617af); };
            c.w64(0, t199);
            return c.call(0x7000870914, 0x70008617b8, 0x70008617b3);
        }
        135 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 138; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008617bc); };
            c.w64(6, t200);
            return c.call(0x70008a17f4, 0x70008617c5, 0x70008617c0);
        }
        137 => {
            blk = 140; continue;
        }
        138 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 140; continue; }
            blk = 139; continue;
        }
        139 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 140; continue;
        }
        140 => {
            c.w64(1, c.r64(0));
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008617d1); };
            c.w64(0, t201);
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008617d5); };
            c.w64(3, t202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008617dc); };
            c.w64(6, t203);
            return c.call(0x7000871cb4, 0x70008617e5, 0x70008617e0);
        }
        141 => {
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008617e5); };
            c.w64(6, t204);
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008617e9); };
            c.w64(3, t205);
            let Some(t206) = c.ld32(c.r64(3).wrapping_add(0xa3)) else { return RegfileExit::fault(0x70008617f0); };
            c.w32(3, t206);
            let r207 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008617f9); };
            c.w64(0, t208);
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70008e8b74, 0x7000861804, 0x70008617ff);
        }
        142 => {
            c.cmp(c.r32(0), 0xfffffffffffffffeu64, 32);
            c.w8(8, (c.cc_ne() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700086180f); }
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000861813); };
            c.w64(3, t209);
            let Some(t210) = c.ld32(c.r64(3).wrapping_add(0xa7)) else { return RegfileExit::fault(0x700086181a); };
            c.w32(3, t210);
            let r211 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000861823); };
            c.w64(0, t212);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861829); };
            c.w64(6, t213);
            return c.call(0x70008e8b74, 0x7000861832, 0x700086182d);
        }
        143 => {
            c.cmp(c.r32(0), 0xfffffffffffffffeu64, 32);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086183d); };
            c.w64(8, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000861841); };
            c.w64(0, t215);
            blk = 144; continue;
        }
        144 => {
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000861845); };
            c.w64(12, t216);
            let Some(t217) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000861849); };
            c.w32(11, t217);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xa6u64, 64);
            if c.cc_a() { blk = 146; continue; }
            blk = 145; continue;
        }
        145 => {
            return RegfileExit::fault(0x700086185d);
        }
        146 => {
            let Some(t218) = c.ld32(c.r64(12).wrapping_add(0x29f)) else { return RegfileExit::fault(0x700086185e); };
            c.w32(11, t218);
            let Some(t219) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000861866); };
            c.w64(12, t219);
            let Some(t220) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700086186d); };
            c.w64(15, t220);
            c.w64(3, c.r64(15).wrapping_add(0x18));
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000861875); };
            c.w64(2, t221);
            let Some(t222) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700086187c); };
            c.cmp(t222, c.r64(3), 64);
            if c.cc_be() { blk = 266; continue; }
            blk = 147; continue;
        }
        147 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x7000861885); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 148; continue;
        }
        148 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700086188d); }
            let r223 = c.shl(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r223);
            let r224 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r224);
            let Some(t225) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000861899); };
            c.w64(11, t225);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008618a0); }
            let r226 = c.r32(9) | c.r32(8);
            c.w32(9, r226);
            c.logic(r226, 32);
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008618a8); }
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008618b1); }
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008618b6); };
            c.w64(0, t227);
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008618ba); }
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(8)) { return RegfileExit::fault(0x70008618bf); }
            c.w64(0, c.r64(12));
            blk = 149; continue;
        }
        149 => {
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008618c7); };
            c.w64(1, t228);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008618ce); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 151; continue; }
            blk = 150; continue;
        }
        150 => {
            return c.ret(0x10, 0x70008618d5);
        }
        151 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008618d8); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008618df); }
            return c.ret(0x0, 0x70008618e1);
        }
        152 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 153; continue;
        }
        153 => {
            let Some(t229) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008618e4); };
            c.cmp(t229, c.r32(9), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t230) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008618f3); };
            c.cmp(t230, c.r32(9), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            let r231 = c.r32(15) | c.r32(12);
            c.w32(15, r231);
            c.logic(r231, 32);
            if c.cc_ne() { blk = 159; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t232) = c.ld64(c.r64(13).wrapping_add(0xcc8)) else { return RegfileExit::fault(0x7000861907); };
            c.w64(3, t232);
            c.w64(0, c.r64(9));
            return c.call(0x7000870914, 0x7000861916, 0x7000861911);
        }
        155 => {
            let Some(t233) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000861916); };
            c.cmp(t233, c.r32(0), 32);
            if c.cc_e() { blk = 159; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t234) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700086191f); };
            c.cmp(t234, c.r32(0), 32);
            if c.cc_e() { blk = 159; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t235) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000861928); };
            if !c.push64(t235) { return RegfileExit::fault(0x7000861928); }
            let Some(t236) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086192b); };
            if !c.push64(t236) { return RegfileExit::fault(0x700086192b); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861936); };
            c.w64(6, t237);
            return c.call(0x70007f2274, 0x700086193f, 0x700086193a);
        }
        158 => {
            blk = 149; continue;
        }
        159 => {
            let Some(t238) = c.ld8(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000861941); };
            c.logic(t238 & 0x1u64, 8);
            if c.cc_e() { blk = 271; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t239) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700086194b); };
            c.w64(0, t239);
            let Some(t240) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086194f); };
            c.w32(8, t240);
            let r241 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r241);
            let Some(t242) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000861956); };
            c.cmp(t242, 0x40u64, 16);
            if c.cc_ae() { blk = 272; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000861962); };
            c.w64(8, t243);
            let Some(t244) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000861966); };
            c.w32(9, t244);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000861970); }
            c.cmp(c.r64(9), 0x9au64, 64);
            if c.cc_a() { blk = 163; continue; }
            blk = 162; continue;
        }
        162 => {
            return RegfileExit::fault(0x700086197d);
        }
        163 => {
            let Some(t245) = c.ld32(c.r64(8).wrapping_add(0x26f)) else { return RegfileExit::fault(0x700086197e); };
            c.w32(8, t245);
            let r246 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r246);
            let Some(t247) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000861988); };
            c.w32(8, t247);
            let r248 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r248);
            let Some(t249) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086198f); };
            c.w64(9, t249);
            let Some(t250) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000861993); };
            c.cmp(t250, c.r32(9), 32);
            if c.cc_e() { blk = 167; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t251) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086199c); };
            c.cmp(t251, 0x0u64, 32);
            if c.cc_ne() { blk = 274; continue; }
            blk = 165; continue;
        }
        165 => {
            let Some(t252) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008619a6); };
            c.w32(11, t252);
            let r253 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r253);
            let Some(t254) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008619ad); };
            c.cmp(t254, 0x40u64, 16);
            if c.cc_b() { blk = 168; continue; }
            blk = 166; continue;
        }
        166 => {
            blk = 274; continue;
        }
        167 => {
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008619ba); };
            c.w64(9, t255);
            blk = 168; continue;
        }
        168 => {
            let Some(t256) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008619c1); };
            c.logic(t256 & 0x200000u64, 32);
            if c.cc_ne() { blk = 170; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t257) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008619cb); };
            c.w64(3, t257);
            let Some(t258) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008619d2); };
            c.w64(12, t258);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008619db); };
            c.w64(1, t259);
            let Some(t260) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008619e2); };
            c.w64(11, t260);
            blk = 182; continue;
        }
        170 => {
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008619ee); };
            c.w64(11, t261);
            let Some(t262) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008619f5); };
            c.w64(12, t262);
            c.w64(15, c.r64(12).wrapping_add(0x4c));
            let Some(t263) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008619fd); };
            c.w64(3, t263);
            let Some(t264) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000861a04); };
            c.cmp(t264, c.r64(15), 64);
            if c.cc_be() { blk = 276; continue; }
            blk = 171; continue;
        }
        171 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000861a0d); }
            let r265 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r265);
            blk = 172; continue;
        }
        172 => {
            let Some(t266) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x7000861a14); };
            c.w64(15, t266);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000861a1b); }
            c.w32(15, 0x22u64);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x7000861a26); }
            let Some(t267) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000861a2b); };
            c.w32(15, t267);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_be() { blk = 278; continue; }
            blk = 173; continue;
        }
        173 => {
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000861a42); }
            let Some(t268) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000861a47); };
            c.w32(15, t268);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_be() { blk = 279; continue; }
            blk = 174; continue;
        }
        174 => {
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x7000861a5c); }
            let Some(t269) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000861a61); };
            c.w32(15, t269);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x2u64, 64);
            if c.cc_be() { blk = 280; continue; }
            blk = 175; continue;
        }
        175 => {
            c.w32(15, 0x8u64);
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x7000861a7c); }
            let Some(t270) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000861a81); };
            c.w32(15, t270);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x3u64, 64);
            if c.cc_be() { blk = 281; continue; }
            blk = 176; continue;
        }
        176 => {
            c.w32(15, 0x2u64);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(15)) { return RegfileExit::fault(0x7000861a9c); }
            let Some(t271) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000861aa1); };
            c.w32(15, t271);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x4u64, 64);
            if c.cc_be() { blk = 282; continue; }
            blk = 177; continue;
        }
        177 => {
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(6)) { return RegfileExit::fault(0x7000861ab6); }
            c.w64(15, c.r64(12).wrapping_add(0x4b));
            c.w64(2, c.r64(12).wrapping_add(0x1b));
            c.cmp(c.r64(2), c.r64(15), 64);
            if c.cc_e() { blk = 181; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t272) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000861aca); };
            c.w64(1, t272);
            blk = 179; continue;
        }
        179 => {
            if !c.st32(c.r64(2), c.r32(1)) { return RegfileExit::fault(0x7000861ad1); }
            let r273 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r273);
            c.cmp(c.r64(15), c.r64(2), 64);
            if c.cc_ne() { blk = 179; continue; }
            blk = 180; continue;
        }
        180 => {
            blk = 182; continue;
        }
        181 => {
            let Some(t274) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000861ade); };
            c.w64(1, t274);
            blk = 182; continue;
        }
        182 => {
            let Some(t275) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000861ae5); };
            c.w32(15, t275);
            c.w64(2, c.r64(15).wrapping_mul(4));
            let Some(t276) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000861af2); };
            c.w64(15, t276);
            c.w64(7, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t277) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000861af9); };
            c.cmp(t277, c.r64(7), 64);
            if c.cc_be() { blk = 283; continue; }
            blk = 183; continue;
        }
        183 => {
            if !c.st64(c.r64(11), c.r64(7)) { return RegfileExit::fault(0x7000861b02); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            blk = 184; continue;
        }
        184 => {
            c.w64(15, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000861b0d); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000861b11); }
            let Some(t278) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000861b15); };
            c.w64(12, t278);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x7000861b1c); }
            let Some(t279) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000861b20); };
            c.w32(12, t279);
            c.w64(3, c.r64(12));
            let r280 = c.r32(3) & 0xe0000000u64;
            c.w32(3, r280);
            c.logic(r280, 32);
            if c.cc_ne() { blk = 188; continue; }
            blk = 185; continue;
        }
        185 => {
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_e() { blk = 194; continue; }
            blk = 186; continue;
        }
        186 => {
            let r281 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r281);
            if !c.st32(c.r64(15).wrapping_add(c.r64(11)), c.r32(1)) { return RegfileExit::fault(0x7000861b39); }
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_ne() { blk = 186; continue; }
            blk = 187; continue;
        }
        187 => {
            blk = 194; continue;
        }
        188 => {
            let r282 = c.sub(c.r32(12), 0x20000000u64, 32);
            c.w32(12, r282);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x7000861b4c); }
            let Some(t283) = c.ld8(c.r64(8).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000861b50); };
            c.w32(12, t283);
            c.w64(12, c.r64(12).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_e() { blk = 191; continue; }
            blk = 189; continue;
        }
        189 => {
            let Some(t284) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x7000861b62); };
            c.w64(2, t284);
            blk = 190; continue;
        }
        190 => {
            let r285 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r285);
            if !c.st32(c.r64(15).wrapping_add(c.r64(11)), c.r32(2)) { return RegfileExit::fault(0x7000861b6d); }
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_ne() { blk = 190; continue; }
            blk = 191; continue;
        }
        191 => {
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_e() { blk = 193; continue; }
            blk = 192; continue;
        }
        192 => {
            let r286 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r286);
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(1)) { return RegfileExit::fault(0x7000861b80); }
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_ne() { blk = 192; continue; }
            blk = 193; continue;
        }
        193 => {
            let r287 = c.add(c.r32(3), 0xe0000000u64, 32);
            c.w32(3, r287);
            if c.cc_e() { blk = 285; continue; }
            blk = 194; continue;
        }
        194 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000861b96); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000861b98); }
            let Some(t288) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000861b9a); };
            c.w64(8, t288);
            let Some(t289) = c.ld32(c.r64(8).wrapping_add(0xa3)) else { return RegfileExit::fault(0x7000861ba1); };
            c.w32(8, t289);
            let r290 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r290);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000861bab); }
            let Some(t291) = c.ld64(c.r64(13).wrapping_add(0x2670)) else { return RegfileExit::fault(0x7000861bad); };
            c.w64(3, t291);
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861bb4); };
            c.w64(6, t292);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000861bc5, 0x7000861bc0);
        }
        195 => {
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861bc5); };
            c.w64(6, t293);
            let Some(t294) = c.ld64(c.r64(13).wrapping_add(0xcc8)) else { return RegfileExit::fault(0x7000861bc9); };
            c.w64(3, t294);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000861bd0); }
            return c.call(0x7000870914, 0x7000861bd9, 0x7000861bd4);
        }
        196 => {
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000861bd9); };
            if !c.push64(t295) { return RegfileExit::fault(0x7000861bd9); }
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000861bdc); };
            if !c.push64(t296) { return RegfileExit::fault(0x7000861bdc); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t297) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861be7); };
            c.w64(6, t297);
            return c.call(0x70007f2274, 0x7000861bf0, 0x7000861beb);
        }
        197 => {
            blk = 149; continue;
        }
        198 => {
            c.w32(8, 0x36u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000861bfb); }
            let Some(t298) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000861bfd); };
            c.w64(8, t298);
            let Some(t299) = c.ld32(c.r64(8).wrapping_add(0xab)) else { return RegfileExit::fault(0x7000861c04); };
            c.w32(8, t299);
            let r300 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r300);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000861c0e); }
            let Some(t301) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000861c10); };
            c.w64(3, t301);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000861c21, 0x7000861c1c);
        }
        199 => {
            return RegfileExit::fault(0x7000861c21);
        }
        200 => {
            let Some(t302) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000861c22); };
            c.w64(3, t302);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000861c30, 0x7000861c2b);
        }
        201 => {
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861c30); };
            c.w64(6, t303);
            blk = 1; continue;
        }
        202 => {
            let Some(t304) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000861c39); };
            c.w32(12, t304);
            let r305 = c.sub(c.r32(12), 0xbau64, 32);
            c.w32(12, r305);
            c.cmp(c.r32(12), 0x78eu64, 32);
            if c.cc_a() { blk = 243; continue; }
            blk = 203; continue;
        }
        203 => {
            let Some(t306) = c.ld64(c.r64(13).wrapping_add(0xcd0)) else { return RegfileExit::fault(0x7000861c53); };
            c.w64(3, t306);
            c.w64(0, c.r64(9));
            c.w64(6, c.r64(8));
            return c.call(0x7000870914, 0x7000861c65, 0x7000861c60);
        }
        204 => {
            let Some(t307) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000861c65); };
            c.cmp(t307, c.r32(0), 32);
            if c.cc_e() { blk = 225; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t308) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000861c72); };
            c.cmp(t308, c.r32(0), 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 206; continue;
        }
        206 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 215; continue; }
            blk = 207; continue;
        }
        207 => {
            let Some(t309) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000861c86); };
            c.cmp(t309, c.r32(0), 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t310) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000861c93); };
            c.w32(8, t310);
            let r311 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r311);
            let Some(t312) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000861c9a); };
            c.logic(t312 & 0x10u64, 8);
            if c.cc_ne() { blk = 222; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t313) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000861ca5); };
            c.cmp(t313, c.r32(8), 32);
            if c.cc_e() { blk = 213; continue; }
            blk = 210; continue;
        }
        210 => {
            let Some(t314) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000861cae); };
            c.cmp(t314, 0x41u64, 16);
            if c.cc_ne() { blk = 216; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t315) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000861cb6); };
            c.logic(t315 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 212; continue;
        }
        212 => {
            blk = 216; continue;
        }
        213 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000861cc5); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 216; continue; }
            blk = 214; continue;
        }
        214 => {
            blk = 222; continue;
        }
        215 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 216; continue;
        }
        216 => {
            let Some(t316) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000861cdc); };
            c.w64(9, t316);
            let Some(t317) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000861ce0); };
            c.w32(8, t317);
            let r318 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r318);
            let Some(t319) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000861ce7); };
            c.cmp(t319, 0x83eu64, 16);
            if c.cc_ne() { blk = 218; continue; }
            blk = 217; continue;
        }
        217 => {
            c.w32(12, 0x1u64);
            blk = 219; continue;
        }
        218 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 219; continue;
        }
        219 => {
            c.w32(0, 0x1u64);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 230; continue; }
            blk = 220; continue;
        }
        220 => {
            c.w32(8, 0x90u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000861d10); }
            let Some(t320) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x7000861d12); };
            c.w64(3, t320);
            let Some(t321) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000861d19); };
            c.w64(6, t321);
            return c.call(0x700086ed74, 0x7000861d22, 0x7000861d1d);
        }
        221 => {
            c.w32(0, 0x1u64);
            let Some(t322) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000861d27); };
            c.w64(9, t322);
            blk = 230; continue;
        }
        222 => {
            let Some(t323) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000861d2d); };
            c.w64(9, t323);
            let Some(t324) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000861d31); };
            c.w32(8, t324);
            let r325 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r325);
            let Some(t326) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000861d38); };
            c.cmp(t326, 0x83eu64, 16);
            if c.cc_ne() { blk = 224; continue; }
            blk = 223; continue;
        }
        223 => {
            c.w32(8, 0x92u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000861d47); }
            let Some(t327) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x7000861d49); };
            c.w64(3, t327);
            c.w32(0, 0x1u64);
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000861d55); };
            c.w64(6, t328);
            return c.call(0x700086ed74, 0x7000861d5e, 0x7000861d59);
        }
        224 => {
            let Some(t329) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000861d5e); };
            c.w64(0, t329);
            blk = 244; continue;
        }
        225 => {
            let Some(t330) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000861d67); };
            c.w64(0, t330);
            let Some(t331) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000861d6b); };
            c.w32(8, t331);
            let r332 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r332);
            let Some(t333) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000861d72); };
            c.cmp(t333, 0x83eu64, 16);
            if c.cc_ne() { blk = 227; continue; }
            blk = 226; continue;
        }
        226 => {
            c.w32(8, 0x1u64);
            blk = 228; continue;
        }
        227 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 228; continue;
        }
        228 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 244; continue; }
            blk = 229; continue;
        }
        229 => {
            c.w64(9, c.r64(0));
            blk = 230; continue;
        }
        230 => {
            let Some(t334) = c.ld64(c.r64(13).wrapping_add(0x700)) else { return RegfileExit::fault(0x7000861d93); };
            c.w64(3, t334);
            c.w64(0, c.r64(9));
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861d9d); };
            c.w64(6, t335);
            return c.call(0x7000870914, 0x7000861da6, 0x7000861da1);
        }
        231 => {
            let Some(t336) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000861da6); };
            c.cmp(t336, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t337) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000861db5); };
            c.cmp(t337, c.r32(0), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let r338 = c.r32(8) | c.r32(9);
            c.w32(8, r338);
            c.logic(r338, 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 232; continue;
        }
        232 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 234; continue; }
            blk = 233; continue;
        }
        233 => {
            let Some(t339) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000861dd1); };
            c.w32(8, t339);
            let r340 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r340);
            let Some(t341) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000861dd8); };
            c.cmp(t341, 0x40u64, 16);
            if c.cc_b() { blk = 235; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t342) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861de0); };
            c.w64(6, t342);
            return c.call(0x700089c794, 0x7000861de9, 0x7000861de4);
        }
        235 => {
            let Some(t343) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000861de9); };
            c.w64(3, t343);
            let Some(t344) = c.ld32(c.r64(3).wrapping_add(0xa3)) else { return RegfileExit::fault(0x7000861df0); };
            c.w32(3, t344);
            let r345 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r345);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t346) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861dfb); };
            c.w64(6, t346);
            return c.call(0x70008e8b74, 0x7000861e04, 0x7000861dff);
        }
        236 => {
            c.cmp(c.r32(0), 0xfffffffffffffffeu64, 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 237; continue;
        }
        237 => {
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t347) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861e0c); };
            c.w64(6, t347);
            let Some(t348) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000861e10); };
            c.w64(9, t348);
            let Some(t349) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000861e14); };
            c.w64(11, t349);
            let Some(t350) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000861e18); };
            c.w64(8, t350);
            let Some(t351) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000861e1c); };
            c.w64(0, t351);
            blk = 15; continue;
        }
        238 => {
            let Some(t352) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861e25); };
            c.w64(6, t352);
            blk = 239; continue;
        }
        239 => {
            c.w32(8, 0x148u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000861e2f); }
            let Some(t353) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000861e31); };
            c.w64(8, t353);
            let Some(t354) = c.ld32(c.r64(8).wrapping_add(0xab)) else { return RegfileExit::fault(0x7000861e38); };
            c.w32(8, t354);
            let r355 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r355);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000861e42); }
            let Some(t356) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000861e44); };
            c.w64(3, t356);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000861e55, 0x7000861e50);
        }
        240 => {
            return RegfileExit::fault(0x7000861e55);
        }
        241 => {
            c.w32(8, 0x15eu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000861e5c); }
            let Some(t357) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000861e5e); };
            c.w64(3, t357);
            c.w32(0, 0x1u64);
            let Some(t358) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861e6a); };
            c.w64(6, t358);
            return c.call(0x700086ed74, 0x7000861e73, 0x7000861e6e);
        }
        242 => {
            return RegfileExit::fault(0x7000861e73);
        }
        243 => {
            c.w64(0, c.r64(9));
            blk = 244; continue;
        }
        244 => {
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t359) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861e7a); };
            c.w64(6, t359);
            c.w64(9, c.r64(0));
            let Some(t360) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000861e81); };
            c.w64(11, t360);
            let Some(t361) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000861e85); };
            c.w64(8, t361);
            let Some(t362) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000861e89); };
            c.w64(0, t362);
            blk = 15; continue;
        }
        245 => {
            return c.call(0x700089c794, 0x7000861e97, 0x7000861e92);
        }
        246 => {
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000861e97); };
            c.w64(8, t363);
            let Some(t364) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861e9b); };
            c.w64(6, t364);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t365) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000861ea1); };
            c.w64(9, t365);
            blk = 31; continue;
        }
        247 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x7000861eaa); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x7000861eae); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(6)) { return RegfileExit::fault(0x7000861eb2); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x7000861eb6); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(2)) { return RegfileExit::fault(0x7000861eba); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(15)) { return RegfileExit::fault(0x7000861ebe); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000861ec2); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x7000861ece, 0x7000861ec9);
        }
        248 => {
            c.w64(8, c.r64(0));
            let Some(t366) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000861ed1); };
            c.w64(0, t366);
            let Some(t367) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000861ed5); };
            c.w64(9, t367);
            let Some(t368) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000861ed9); };
            c.w64(6, t368);
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000861edd); };
            c.w64(7, t369);
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000861ee1); };
            c.w64(2, t370);
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000861ee5); };
            c.w64(15, t371);
            let Some(t372) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000861ee9); };
            c.w64(3, t372);
            blk = 62; continue;
        }
        249 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000861ef2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000861ef6); }
            let Some(t373) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861efa); };
            c.w64(6, t373);
            return c.call(0x700089c794, 0x7000861f03, 0x7000861efe);
        }
        250 => {
            let Some(t374) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000861f03); };
            c.w64(9, t374);
            let Some(t375) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000861f07); };
            c.w64(8, t375);
            let Some(t376) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000861f0b); };
            c.w64(3, t376);
            blk = 89; continue;
        }
        251 => {
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x7000861f19); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000861f1d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000861f21); }
            c.w64(12, c.r64(2));
            return c.call(0x70007febf4, 0x7000861f2d, 0x7000861f28);
        }
        252 => {
            c.w64(12, c.r64(0));
            let Some(t377) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000861f30); };
            c.w64(8, t377);
            let Some(t378) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000861f34); };
            c.w64(0, t378);
            let Some(t379) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000861f38); };
            c.w64(3, t379);
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000861f3c); };
            c.w64(9, t380);
            let Some(t381) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000861f40); };
            c.w64(11, t381);
            let Some(t382) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000861f47); };
            c.w64(2, t382);
            blk = 93; continue;
        }
        253 => {
            return RegfileExit::fault(0x7000861f53);
        }
        254 => {
            return RegfileExit::fault(0x7000861f54);
        }
        255 => {
            return RegfileExit::fault(0x7000861f55);
        }
        256 => {
            return RegfileExit::fault(0x7000861f56);
        }
        257 => {
            return RegfileExit::fault(0x7000861f57);
        }
        258 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x7000861f58); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000861f5c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000861f60); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000861f64); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x7000861f68); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x7000861f74, 0x7000861f6f);
        }
        259 => {
            c.w64(8, c.r64(0));
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t383) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000861f79); };
            c.w64(7, t383);
            let Some(t384) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000861f80); };
            c.w64(0, t384);
            let Some(t385) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000861f84); };
            c.w64(3, t385);
            let Some(t386) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000861f88); };
            c.w64(9, t386);
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000861f8c); };
            c.w64(12, t387);
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000861f90); };
            c.w64(15, t388);
            blk = 105; continue;
        }
        260 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000861f99); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000861f9b); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x7000861f9f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000861fa3); }
            c.w32(0, 0x1u64);
            let Some(t389) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x7000861fac); };
            c.w64(3, t389);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x7000861fbb, 0x7000861fb6);
        }
        261 => {
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000861fbb); };
            c.w64(8, t390);
            let Some(t391) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000861fbf); };
            c.w64(0, t391);
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000861fc3); };
            c.w64(3, t392);
            blk = 115; continue;
        }
        262 => {
            c.w32(8, 0x54u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000861fd2); }
            let Some(t393) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000861fd4); };
            c.w64(8, t393);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000861fdb); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000861fdd); }
            let Some(t394) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000861fde); };
            c.w64(3, t394);
            c.w32(0, 0x3u64);
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861fea); };
            c.w64(6, t395);
            return c.call(0x700086ed74, 0x7000861ff3, 0x7000861fee);
        }
        263 => {
            return RegfileExit::fault(0x7000861ff3);
        }
        264 => {
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000861ff4); };
            c.w64(6, t396);
            return c.call(0x700089c794, 0x7000861ffd, 0x7000861ff8);
        }
        265 => {
            blk = 133; continue;
        }
        266 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x7000862007); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700086200b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700086200f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000862013); }
            return c.call(0x70007febf4, 0x700086201c, 0x7000862017);
        }
        267 => {
            c.w64(12, c.r64(0));
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086201f); };
            c.w64(0, t397);
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000862023); };
            c.w64(8, t398);
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862027); };
            c.w64(9, t399);
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086202b); };
            c.w64(11, t400);
            blk = 148; continue;
        }
        268 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086203a); }
            let Some(t401) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700086203c); };
            c.w64(8, t401);
            let Some(t402) = c.ld32(c.r64(8).wrapping_add(0xaf)) else { return RegfileExit::fault(0x7000862043); };
            c.w32(8, t402);
            let r403 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r403);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086204d); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700086204f); }
            let Some(t404) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000862051); };
            c.w64(3, t404);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000862062, 0x700086205d);
        }
        269 => {
            return RegfileExit::fault(0x7000862062);
        }
        270 => {
            blk = 272; continue;
        }
        271 => {
            let Some(t405) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000862065); };
            c.w64(0, t405);
            blk = 272; continue;
        }
        272 => {
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862069); };
            c.w64(6, t406);
            return c.call(0x700089c794, 0x7000862072, 0x700086206d);
        }
        273 => {
            blk = 161; continue;
        }
        274 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000862077); }
            c.w64(0, c.r64(9));
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086207e); };
            c.w64(6, t407);
            return c.call(0x700089c794, 0x7000862087, 0x7000862082);
        }
        275 => {
            c.w64(9, c.r64(0));
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086208a); };
            c.w64(0, t408);
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086208e); };
            c.w64(8, t409);
            blk = 168; continue;
        }
        276 => {
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700086209c); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008620a0); }
            return c.call(0x70007febf4, 0x70008620a9, 0x70008620a4);
        }
        277 => {
            c.w64(12, c.r64(0));
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008620ac); };
            c.w64(0, t410);
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008620b0); };
            c.w64(9, t411);
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008620b4); };
            c.w64(8, t412);
            let Some(t413) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008620b8); };
            c.w64(11, t413);
            let Some(t414) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008620bf); };
            c.w64(3, t414);
            blk = 172; continue;
        }
        278 => {
            return RegfileExit::fault(0x70008620cb);
        }
        279 => {
            return RegfileExit::fault(0x70008620cc);
        }
        280 => {
            return RegfileExit::fault(0x70008620cd);
        }
        281 => {
            return RegfileExit::fault(0x70008620ce);
        }
        282 => {
            return RegfileExit::fault(0x70008620cf);
        }
        283 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008620d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008620d4); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008620d8); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008620dc); }
            return c.call(0x70007febf4, 0x70008620e5, 0x70008620e0);
        }
        284 => {
            c.w64(11, c.r64(0));
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008620e8); };
            c.w64(0, t415);
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008620ec); };
            c.w64(9, t416);
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008620f0); };
            c.w64(8, t417);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t418) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008620f6); };
            c.w64(1, t418);
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008620fd); };
            c.w64(12, t419);
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000862101); };
            c.w64(2, t420);
            blk = 184; continue;
        }
        285 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086210a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700086210c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000862110); }
            let Some(t421) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x7000862114); };
            c.w64(3, t421);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000862125, 0x7000862120);
        }
        286 => {
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000862125); };
            c.w64(11, t422);
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000862129); };
            c.w64(9, t423);
            blk = 194; continue;
        }
        287 => {
            return RegfileExit::jump(0x7000862134);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000862154(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1ba0)) else { return RegfileExit::fault(0x7000862154); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000862174(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000862174); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000862178); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000862179); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086217a); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700086217f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000862183); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 73; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700086218d); };
            c.w64(3, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000862191); };
            c.cmp(t3, c.r32(3), 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086219e); };
            c.cmp(t4, c.r32(3), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let r5 = c.r32(2) | c.r32(8);
            c.w32(2, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008621b6); };
            c.w64(0, t6);
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008621c2); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008621d8); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008621df); };
            c.cmp(t9, 0x40u64, 16);
            if c.cc_b() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 20; continue;
        }
        6 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008621ee); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008621f7); };
            c.w64(9, t12);
            let Some(t13) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008621fe); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000862208); };
            c.w32(9, t14);
            let r15 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700086220f); };
            c.w32(9, t16);
            let r17 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r17);
            let Some(t18) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000862216); };
            c.w32(11, t18);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x9au64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x7000862229);
        }
        9 => {
            let Some(t19) = c.ld32(c.r64(9).wrapping_add(0x26f)) else { return RegfileExit::fault(0x700086222a); };
            c.w32(12, t19);
            let r20 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r20);
            let Some(t21) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000862234); };
            c.cmp(t21, c.r32(8), 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x700086223f); };
            c.w32(12, t22);
            c.logic(c.r32(12) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(11), 0x9fu64, 64);
            if c.cc_a() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x7000862255);
        }
        13 => {
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000862256); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            let Some(t25) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086225d); };
            c.w32(8, t25);
            let r26 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r26);
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000862264); };
            c.w32(11, t27);
            let r28 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r28);
            let Some(t29) = c.ld32(c.r64(9).wrapping_add(0x283)) else { return RegfileExit::fault(0x700086226b); };
            c.cmp(t29, c.r32(8), 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t30) = c.ld32(c.r64(11).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000862274); };
            c.w32(8, t30);
            let Some(t31) = c.ld32(c.r64(11).wrapping_add(0xd3)) else { return RegfileExit::fault(0x7000862278); };
            c.w32(9, t31);
            let r32 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r32);
            let r33 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r33);
            let r34 = c.r32(9) & c.r32(8);
            c.w32(9, r34);
            c.logic(r34, 32);
            let r35 = c.r32(9) & 0x4u64;
            c.w32(9, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000862292); };
            c.w64(1, t36);
            return c.call(0x70008c8294, 0x700086229b, 0x7000862296);
        }
        16 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700086229b); };
            c.w64(1, t37);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008622a2); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return c.ret(0x18, 0x70008622a9);
        }
        18 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008622ac); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008622b3); }
            return c.ret(0x0, 0x70008622b5);
        }
        19 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 20; continue;
        }
        20 => {
            let Some(t38) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008622b8); };
            c.cmp(t38, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008622c7); };
            c.cmp(t39, c.r32(0), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let r40 = c.r32(8) | c.r32(9);
            c.w32(8, r40);
            c.logic(r40, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0xcd8)) else { return RegfileExit::fault(0x70008622db); };
            c.w64(3, t41);
            c.w64(8, c.r64(3));
            return c.call(0x7000870914, 0x70008622ea, 0x70008622e5);
        }
        22 => {
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008622ea); };
            c.cmp(t42, c.r32(0), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008622f3); };
            c.cmp(t43, c.r32(0), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008622fc); };
            if !c.push64(t44) { return RegfileExit::fault(0x70008622fc); }
            let Some(t45) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008622ff); };
            if !c.push64(t45) { return RegfileExit::fault(0x70008622ff); }
            let Some(t46) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000862302); };
            if !c.push64(t46) { return RegfileExit::fault(0x7000862302); }
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862305); };
            c.w64(6, t47);
            c.w64(7, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(2, c.r64(0));
            return c.call(0x70007f2274, 0x7000862319, 0x7000862314);
        }
        25 => {
            blk = 16; continue;
        }
        26 => {
            let Some(t48) = c.ld8(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700086231b); };
            c.logic(t48 & 0x1u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000862325); };
            c.w64(0, t49);
            let Some(t50) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000862329); };
            c.w32(2, t50);
            let r51 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r51);
            let Some(t52) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086232f); };
            c.cmp(t52, 0x40u64, 16);
            if c.cc_ae() { blk = 76; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700086233a); }
            let Some(t53) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086233e); };
            c.cmp(t53, 0x0u64, 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000862348); };
            c.w64(2, t54);
            let Some(t55) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086234c); };
            c.w32(8, t55);
            let r56 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r56);
            let Some(t57) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000862353); };
            c.cmp(t57, 0x40u64, 16);
            if c.cc_ae() { blk = 79; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t58) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086235f); };
            c.w32(1, t58);
            let Some(t59) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000862362); };
            c.w32(8, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000862366); };
            c.w64(9, t60);
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x7000862376); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x700086237a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700086237e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000862382); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_ne() { blk = 40; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r64(1), 0xffu64, 64);
            if c.cc_le() { blk = 40; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t61) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008623a7); };
            c.w32(12, t61);
            let r62 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r62);
            let Some(t63) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008623ae); };
            c.cmp(t63, 0x40u64, 16);
            if c.cc_ae() { blk = 40; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t64) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008623b7); };
            c.w32(12, t64);
            let r65 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r65);
            let Some(t66) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008623be); };
            c.w32(12, t66);
            let r67 = c.r32(12) & 0x7u64;
            c.w32(12, r67);
            c.logic(r67, 32);
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008623ce); };
            c.w64(3, t68);
            let Some(t69) = c.ld32(c.r64(3).wrapping_add(0xb3)) else { return RegfileExit::fault(0x70008623d5); };
            c.w32(3, t69);
            let r70 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r70);
            c.w64(0, c.r64(9));
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008623e3); };
            c.w64(6, t71);
            return c.call(0x70008e8b74, 0x70008623ec, 0x70008623e7);
        }
        36 => {
            c.logic(c.r32(0) & 0x80000001u64, 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008623f3); };
            if !c.push64(t72) { return RegfileExit::fault(0x70008623f3); }
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008623f6); };
            if !c.push64(t73) { return RegfileExit::fault(0x70008623f6); }
            let Some(t74) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008623f9); };
            if !c.push64(t74) { return RegfileExit::fault(0x70008623f9); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x27b8)) else { return RegfileExit::fault(0x70008623fc); };
            c.w64(3, t75);
            c.w32(0, 0x3u64);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862408); };
            c.w64(6, t76);
            return c.call(0x700086ed74, 0x7000862411, 0x700086240c);
        }
        38 => {
            blk = 16; continue;
        }
        39 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000862416); };
            c.w64(0, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700086241a); };
            c.w64(2, t78);
            blk = 40; continue;
        }
        40 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086241e); };
            c.w64(6, t79);
            c.w64(3, c.r64(2));
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70008e8b74, 0x700086242c, 0x7000862427);
        }
        41 => {
            c.w32(2, c.r32(0));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700086242e); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 67; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000862446); };
            c.w64(0, t80);
            c.w32(3, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(2));
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086244f); };
            c.w64(6, t81);
            return c.call(0x70007fbc74, 0x7000862458, 0x7000862453);
        }
        44 => {
            blk = 46; continue;
        }
        45 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700086245a); };
            c.w64(0, t82);
            c.w64(2, c.r64(0));
            blk = 46; continue;
        }
        46 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000862464); };
            c.w64(2, t83);
            let r84 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086246a); };
            c.w64(8, t85);
            let r86 = c.add(c.r32(2), c.r32(8), 32);
            c.w32(2, r86);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x7000862477); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700086247b); }
            let Some(t87) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086247f); };
            c.cmp(t87, 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t88) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000862485); };
            c.w64(7, t88);
            let Some(t89) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000862489); };
            c.w32(9, t89);
            let r90 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r90);
            let Some(t91) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000862490); };
            c.logic(t91 & 0x2u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 55; continue;
        }
        49 => {
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086249c); };
            if !c.push64(t92) { return RegfileExit::fault(0x700086249c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086249f); }
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008624a1); };
            if !c.push64(t93) { return RegfileExit::fault(0x70008624a1); }
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008624a4); };
            c.w64(8, t94);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008624ab); }
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008624ad); };
            c.w64(6, t95);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2134, 0x70008624be, 0x70008624b9);
        }
        50 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 81; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t96) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008624c6); };
            c.w32(2, t96);
            let r97 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r97);
            let Some(t98) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008624cc); };
            c.cmp(t98, 0x40u64, 16);
            if c.cc_ae() { blk = 81; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(3, c.r64(0));
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008624da); };
            c.w64(0, t99);
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008624de); };
            c.w64(6, t100);
            return c.call(0x700086f9d4, 0x70008624e7, 0x70008624e2);
        }
        53 => {
            blk = 64; continue;
        }
        54 => {
            c.w32(9, 0x1u64);
            let Some(t101) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008624f2); };
            c.w64(7, t101);
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t102) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000862500); };
            c.w32(9, t102);
            let r103 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r103);
            let Some(t104) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000862507); };
            c.cmp(t104, 0x40u64, 16);
            if c.cc_ae() { blk = 83; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000862513); };
            c.w64(3, t105);
            let Some(t106) = c.ld32(c.r64(3).wrapping_add(0xb3)) else { return RegfileExit::fault(0x700086251a); };
            c.w32(3, t106);
            let r107 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r107);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000862523); }
            c.w64(0, c.r64(7));
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086252c); };
            c.w64(6, t108);
            return c.call(0x70008e8b74, 0x7000862535, 0x7000862530);
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000862535); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 62; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t109) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086253e); };
            c.w64(2, (t109 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000862545); };
            c.w64(0, t110);
            c.w64(3, c.r64(2));
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700086254c); };
            c.w64(1, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862550); };
            c.w64(6, t112);
            return c.call(0x70007fbc74, 0x7000862559, 0x7000862554);
        }
        60 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000862559); }
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086255a); };
            if !c.push64(t113) { return RegfileExit::fault(0x700086255a); }
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700086255d); };
            if !c.push64(t114) { return RegfileExit::fault(0x700086255d); }
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000862560); };
            if !c.push64(t115) { return RegfileExit::fault(0x7000862560); }
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000862563); };
            if !c.push64(t116) { return RegfileExit::fault(0x7000862563); }
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x2750)) else { return RegfileExit::fault(0x7000862566); };
            c.w64(3, t117);
            c.w32(0, 0x5u64);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862572); };
            c.w64(6, t118);
            return c.call(0x700086ed74, 0x700086257b, 0x7000862576);
        }
        61 => {
            blk = 63; continue;
        }
        62 => {
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086257d); };
            c.w64(0, t119);
            blk = 63; continue;
        }
        63 => {
            c.w64(3, c.r64(0));
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000862584); };
            c.w64(0, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862588); };
            c.w64(6, t121);
            return c.call(0x700086f9d4, 0x7000862591, 0x700086258c);
        }
        64 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000862591); }
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000862595); };
            c.w64(0, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000862599); };
            c.w64(3, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700086259d); };
            c.w64(1, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008625a1); };
            c.w64(6, t125);
            return c.call(0x70007fbc74, 0x70008625aa, 0x70008625a5);
        }
        65 => {
            c.w64(3, c.r64(0));
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008625ad); };
            c.w64(0, t126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008625b1); };
            c.w64(6, t127);
            return c.call(0x700086f9d4, 0x70008625ba, 0x70008625b5);
        }
        66 => {
            blk = 16; continue;
        }
        67 => {
            let Some(t128) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008625bf); };
            c.cmp(t128, 0x0u64, 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t129) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008625c5); };
            c.w64(0, t129);
            let Some(t130) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008625c9); };
            c.w32(2, t130);
            let r131 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r131);
            let Some(t132) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008625cf); };
            c.logic(t132 & 0x2u64, 8);
            if c.cc_ne() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t133) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008625d5); };
            c.cmp(t133, 0x40u64, 16);
            if c.cc_ae() { blk = 85; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008625e0); };
            c.w64(0, t134);
            blk = 16; continue;
        }
        71 => {
            c.w32(2, 0x36u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008625ee); }
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008625ef); };
            c.w64(2, t135);
            let Some(t136) = c.ld32(c.r64(2).wrapping_add(0xb7)) else { return RegfileExit::fault(0x70008625f6); };
            c.w32(2, t136);
            let r137 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r137);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008625ff); }
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000862600); };
            c.w64(3, t138);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000862611, 0x700086260c);
        }
        72 => {
            return RegfileExit::fault(0x7000862611);
        }
        73 => {
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000862612); };
            c.w64(3, t139);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000862620, 0x700086261b);
        }
        74 => {
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862620); };
            c.w64(6, t140);
            blk = 1; continue;
        }
        75 => {
            let Some(t141) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000862629); };
            c.w64(0, t141);
            blk = 76; continue;
        }
        76 => {
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086262d); };
            c.w64(6, t142);
            return c.call(0x700089c794, 0x7000862636, 0x7000862631);
        }
        77 => {
            blk = 28; continue;
        }
        78 => {
            let Some(t143) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700086263b); };
            c.w64(2, t143);
            blk = 79; continue;
        }
        79 => {
            c.w64(0, c.r64(2));
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862642); };
            c.w64(6, t144);
            return c.call(0x700089c794, 0x700086264b, 0x7000862646);
        }
        80 => {
            c.w64(2, c.r64(0));
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086264e); };
            c.w64(0, t145);
            blk = 30; continue;
        }
        81 => {
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862657); };
            c.w64(6, t146);
            return c.call(0x700089c794, 0x7000862660, 0x700086265b);
        }
        82 => {
            blk = 52; continue;
        }
        83 => {
            c.w64(0, c.r64(7));
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862668); };
            c.w64(6, t147);
            return c.call(0x700089c794, 0x7000862671, 0x700086266c);
        }
        84 => {
            c.w64(7, c.r64(0));
            blk = 57; continue;
        }
        85 => {
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862679); };
            c.w64(6, t148);
            return c.call(0x700089c794, 0x7000862682, 0x700086267d);
        }
        86 => {
            blk = 70; continue;
        }
        87 => {
            return RegfileExit::jump(0x7000862688);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000862694(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000862694); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000862698); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000862699); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086269a); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700086269f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008626a3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 98; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(5));
            c.w64(11, c.r64(9).wrapping_add(0x18));
            c.w64(12, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008626be); };
            c.w64(12, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008626c3); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(12)) { return RegfileExit::fault(0x70008626c7); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(9).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008626d1); };
            c.w64(9, t3);
            c.w64(15, c.r64(9));
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008626da); };
            c.w64(15, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008626e1); };
            c.w64(9, t5);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008626e8); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008626f2); };
            c.w64(9, t6);
            blk = 6; continue;
        }
        6 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008626f6); };
            c.cmp(t7, c.r32(12), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000862705); };
            c.cmp(t8, c.r32(12), 32);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x7000862712); }
            let r9 = c.r32(11) | c.r32(0);
            c.w32(11, r9);
            c.logic(r9, 32);
            if c.cc_ne() { blk = 96; continue; }
            blk = 7; continue;
        }
        7 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700086272b); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000862743); };
            c.w32(0, t10);
            let r11 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r11);
            let Some(t12) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086274b); };
            c.cmp(t12, 0x40u64, 16);
            if c.cc_b() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 23; continue;
        }
        11 => {
            let Some(t13) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000862759); };
            c.w32(0, t13);
            let r14 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r14);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x7000862762); };
            c.w64(2, t15);
            let Some(t16) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000862769); };
            c.cmp(t16, c.r32(3), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t17) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000862772); };
            c.w32(2, t17);
            let r18 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r18);
            let Some(t19) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000862778); };
            c.w32(2, t19);
            let r20 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r20);
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700086277e); };
            c.w32(1, t21);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x9au64, 64);
            if c.cc_a() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return RegfileExit::fault(0x7000862790);
        }
        14 => {
            let Some(t22) = c.ld32(c.r64(2).wrapping_add(0x26f)) else { return RegfileExit::fault(0x7000862791); };
            c.w32(7, t22);
            let r23 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r23);
            let Some(t24) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700086279a); };
            c.cmp(t24, c.r32(0), 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t25) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x700086279f); };
            c.w32(7, t25);
            c.logic(c.r32(7) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(1), 0x9fu64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x70008627b4);
        }
        18 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008627b5); };
            c.w32(0, t26);
            let r27 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r27);
            let Some(t28) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008627bb); };
            c.w32(0, t28);
            let r29 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r29);
            let Some(t30) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008627c1); };
            c.w32(1, t30);
            let r31 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r31);
            let Some(t32) = c.ld32(c.r64(2).wrapping_add(0x283)) else { return RegfileExit::fault(0x70008627c7); };
            c.cmp(t32, c.r32(0), 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t33) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008627cf); };
            c.w32(0, t33);
            let Some(t34) = c.ld32(c.r64(1).wrapping_add(0xeb)) else { return RegfileExit::fault(0x70008627d2); };
            c.w32(2, t34);
            let r35 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r35);
            let r36 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r36);
            let r37 = c.r32(2) & c.r32(0);
            c.w32(2, r37);
            c.logic(r37, 32);
            let r38 = c.r32(2) & 0x4u64;
            c.w32(2, r38);
            c.logic(r38, 32);
            c.cmp(c.r32(2), 0x4u64, 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(0, c.r64(15));
            c.w64(3, c.r64(12));
            c.w64(1, c.r64(9));
            return c.call(0x70008cab94, 0x70008627f4, 0x70008627ef);
        }
        21 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008627f4); };
            c.w64(8, t39);
            let r40 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r40);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008627ff); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000862800); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000862806); }
            return c.ret(0x0, 0x7000862808);
        }
        22 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 23; continue;
        }
        23 => {
            let Some(t41) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700086280b); };
            c.cmp(t41, c.r32(15), 32);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000862818); };
            c.cmp(t42, c.r32(15), 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            let r43 = c.r32(2) | c.r32(0);
            c.w32(2, r43);
            c.logic(r43, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0xcf0)) else { return RegfileExit::fault(0x7000862829); };
            c.w64(3, t44);
            c.w64(0, c.r64(15));
            return c.call(0x7000870914, 0x7000862838, 0x7000862833);
        }
        25 => {
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000862838); };
            c.cmp(t45, c.r32(0), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t46) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000862841); };
            c.cmp(t46, c.r32(0), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700086284a); };
            if !c.push64(t47) { return RegfileExit::fault(0x700086284a); }
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700086284d); };
            if !c.push64(t48) { return RegfileExit::fault(0x700086284d); }
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000862850); };
            if !c.push64(t49) { return RegfileExit::fault(0x7000862850); }
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862853); };
            c.w64(6, t50);
            c.w64(7, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x7000862867, 0x7000862862);
        }
        28 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000862867); };
            c.w64(8, t51);
            let r52 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r52);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000862872); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000862873); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000862879); }
            return c.ret(0x0, 0x700086287b);
        }
        29 => {
            let Some(t53) = c.ld8(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700086287c); };
            c.logic(t53 & 0x1u64, 8);
            if c.cc_e() { blk = 100; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000862886); };
            c.w64(0, t54);
            let Some(t55) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700086288a); };
            c.w32(8, t55);
            let r56 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r56);
            let Some(t57) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000862891); };
            c.cmp(t57, 0x40u64, 16);
            if c.cc_ae() { blk = 101; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(0)) { return RegfileExit::fault(0x700086289d); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008628a1); };
            c.w64(8, t58);
            let Some(t59) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008628a5); };
            c.cmp(t59, c.r32(8), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            c.logic(c.r32(8) & 0x80000001u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 103; continue;
        }
        34 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008628bc); };
            c.w64(8, t60);
            let Some(t61) = c.ld32(c.r64(8).wrapping_add(0xbb)) else { return RegfileExit::fault(0x70008628c3); };
            c.w32(8, t61);
            let r62 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r62);
            c.w64(9, c.r64(8));
            blk = 35; continue;
        }
        35 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(8)) { return RegfileExit::fault(0x70008628d0); }
            let Some(t63) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008628d4); };
            c.cmp(t63, 0x0u64, 32);
            if c.cc_ne() { blk = 163; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008628de); };
            c.w64(9, t64);
            let Some(t65) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008628e2); };
            c.w32(11, t65);
            let r66 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r66);
            let Some(t67) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008628e9); };
            c.cmp(t67, 0x40u64, 16);
            if c.cc_ae() { blk = 164; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(11, c.r64(9));
            blk = 38; continue;
        }
        38 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.cmp(c.r32(6), c.r32(8), 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t68) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000862903); };
            c.cmp(t68, c.r32(9), 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t69) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000862910); };
            c.w32(9, t69);
            let r70 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r70);
            c.w32(9, c.r32(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000862920); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000862921); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000862923); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2698)) else { return RegfileExit::fault(0x7000862925); };
            c.w64(3, t71);
            c.w32(0, 0x3u64);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862931); };
            c.w64(6, t72);
            return c.call(0x700086ed74, 0x700086293a, 0x7000862935);
        }
        42 => {
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700086293a); };
            c.w64(9, t73);
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000862945); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000862946); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700086294c); }
            return c.ret(0x0, 0x700086294e);
        }
        43 => {
            let Some(t74) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086294f); };
            c.w32(9, t74);
            let r75 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r75);
            c.w32(11, c.r32(9));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t76) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000862963); };
            c.w32(11, t76);
            let r77 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r77);
            let Some(t78) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700086296a); };
            c.w32(11, t78);
            c.w64(12, c.r64(11));
            let r79 = c.r32(12) & 0x8u64;
            c.w32(12, r79);
            c.logic(r79, 32);
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_ne() { blk = 179; continue; }
            blk = 45; continue;
        }
        45 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_ge() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(9, c.r64(8));
            blk = 48; continue;
        }
        48 => {
            c.w32(12, c.r16(11));
            c.logic(c.r8(11) & 0x7u64, 8);
            if c.cc_e() { blk = 64; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(11, c.r64(6));
            c.w64(15, c.r64(12));
            c.w64(12, c.r64(0));
            blk = 50; continue;
        }
        50 => {
            c.w64(3, c.r64(15));
            let r80 = c.r32(3) & 0x7u64;
            c.w32(3, r80);
            c.logic(r80, 32);
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_l() { blk = 55; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 54; continue;
        }
        54 => {
            blk = 179; continue;
        }
        55 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            blk = 179; continue;
        }
        58 => {
            let Some(t81) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008629d7); };
            c.w32(15, t81);
            let Some(t82) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008629dc); };
            c.cmp(t82, c.r32(15), 32);
            if c.cc_ne() { blk = 179; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t83) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008629e9); };
            c.w32(12, t83);
            let r84 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r84);
            let Some(t85) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008629f1); };
            c.w32(15, t85);
            let r86 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r86);
            let Some(t87) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008629f9); };
            c.w32(15, t87);
            blk = 50; continue;
        }
        60 => {
            let Some(t88) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000862a00); };
            c.w32(15, t88);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r89 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r89);
            let Some(t90) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000862a0e); };
            c.w32(12, t90);
            let r91 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r91);
            let Some(t92) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000862a16); };
            c.w32(15, t92);
            let r93 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r93);
            let Some(t94) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000862a1e); };
            c.w32(15, t94);
            blk = 50; continue;
        }
        61 => {
            let Some(t95) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000862a25); };
            c.w32(12, t95);
            let r96 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r96);
            let Some(t97) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000862a2d); };
            c.w32(15, t97);
            let r98 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r98);
            let Some(t99) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000862a35); };
            c.w32(15, t99);
            blk = 50; continue;
        }
        62 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 65; continue;
        }
        63 => {
            c.w32(3, 0x1u64);
            blk = 65; continue;
        }
        64 => {
            c.w32(3, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(6));
            c.w64(15, c.r64(12));
            c.w64(12, c.r64(0));
            blk = 65; continue;
        }
        65 => {
            c.w64(2, c.r64(15));
            let r100 = c.r32(2) & 0x8u64;
            c.w32(2, r100);
            c.logic(r100, 32);
            c.cmp(c.r32(2), 0x8u64, 32);
            if c.cc_ne() { blk = 179; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(2, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(2));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000862a6d); }
            c.cmp(c.r64(1), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 181; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000862a7e); };
            c.w64(7, t101);
            let Some(t102) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000862a85); };
            c.w64(7, t102);
            c.w64(0, c.r64(2).wrapping_mul(2).wrapping_add(0x8));
            c.w64(8, c.r64(0).wrapping_add(c.r64(7)));
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000862a94); };
            c.w64(2, t103);
            let Some(t104) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000862a9b); };
            c.cmp(t104, c.r64(8), 64);
            if c.cc_be() { blk = 166; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r64(0), 0x20000u64, 64);
            if c.cc_ae() { blk = 166; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000862ab0); };
            c.w64(0, t105);
            if !c.st64(c.r64(0), c.r64(8)) { return RegfileExit::fault(0x7000862ab7); }
            c.w64(8, c.r64(7).wrapping_add(0x1));
            blk = 70; continue;
        }
        70 => {
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000862ac1); };
            c.w64(2, t106);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000862ac8); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x7000862acc); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 168; continue; }
            blk = 71; continue;
        }
        71 => {
            let r107 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r107);
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_e() { blk = 78; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x7000862ae3); };
            c.w64(15, t108);
            blk = 74; continue;
        }
        74 => {
            c.w64(0, c.r64(6).wrapping_add(c.r64(11)));
            let Some(t109) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000862aee); };
            c.w32(3, t109);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t110) = c.ld8(c.r64(12).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x7000862af5); };
            c.w32(0, t110);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(0), c.r64(3), 64);
            if c.cc_ae() { blk = 170; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t111) = c.ld32(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000862b06); };
            c.w32(0, t111);
            let Some(t112) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000862b0b); };
            c.cmp(t112, c.r32(0), 32);
            if c.cc_e() { blk = 175; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t113) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000862b18); };
            c.w32(3, t113);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(6), c.r64(3), 64);
            if c.cc_ae() { blk = 171; continue; }
            blk = 77; continue;
        }
        77 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000862b2b); }
            let r114 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r114);
            c.cmp(c.r64(1), c.r64(6), 64);
            if c.cc_ne() { blk = 74; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862b39); };
            c.w64(11, t115);
            let Some(t116) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000862b3d); };
            c.w32(12, t116);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x6bu64, 64);
            if c.cc_a() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            return RegfileExit::fault(0x7000862b4d);
        }
        80 => {
            let Some(t117) = c.ld32(c.r64(11).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000862b4e); };
            c.w32(11, t117);
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000862b55); };
            c.w64(12, t118);
            let Some(t119) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000862b5c); };
            c.w64(15, t119);
            c.w64(0, c.r64(15).wrapping_add(0x10));
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000862b64); };
            c.w64(3, t120);
            let Some(t121) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000862b6b); };
            c.cmp(t121, c.r64(0), 64);
            if c.cc_be() { blk = 173; continue; }
            blk = 81; continue;
        }
        81 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x7000862b74); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 82; continue;
        }
        82 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000862b7c); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x7000862b81); }
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000862b86); };
            c.w64(9, t122);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x7000862b8d); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000862b92); }
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000862b97); };
            c.w64(9, t123);
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(12));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000862ba5); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000862ba6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000862bac); }
            return c.ret(0x0, 0x7000862bae);
        }
        83 => {
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862baf); };
            c.w64(8, t124);
            let Some(t125) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000862bb3); };
            c.w32(8, t125);
            let r126 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r126);
            let Some(t127) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000862bba); };
            c.w32(8, t127);
            let r128 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r128);
            let Some(t129) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000862bc1); };
            c.w32(9, t129);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            return RegfileExit::fault(0x7000862bd1);
        }
        85 => {
            let Some(t130) = c.ld32(c.r64(8).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000862bd2); };
            c.w32(8, t130);
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000862bd9); };
            c.w64(9, t131);
            let Some(t132) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000862be0); };
            c.w64(11, t132);
            c.w64(12, c.r64(11).wrapping_add(0x1c));
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000862be7); };
            c.w64(15, t133);
            let Some(t134) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000862bee); };
            c.cmp(t134, c.r64(12), 64);
            if c.cc_be() { blk = 183; continue; }
            blk = 86; continue;
        }
        86 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000862bf7); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 87; continue;
        }
        87 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000862bfe); }
            c.w32(8, 0x2u64);
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000862c08); }
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000862c0c); };
            c.w64(11, t135);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000862c13); }
            c.w64(11, c.r64(9).wrapping_add(0x10));
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000862c1b); }
            let Some(t136) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000862c1f); };
            c.w64(12, t136);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000862c26); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000862c2a); }
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000862c2e); };
            c.w64(8, t137);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000862c35); }
            let Some(t138) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000862c39); };
            c.w32(8, t138);
            let r139 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r139);
            let Some(t140) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000862c40); };
            c.w32(11, t140);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 185; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w64(12, c.r64(0));
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x7000862c57); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r141 = c.r64(11) & c.r64(8);
            c.w64(11, r141);
            c.logic(r141, 64);
            let Some(t142) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000862c6b); };
            c.logic(t142 & 0x4u64, 8);
            if c.cc_ne() { blk = 188; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000862c76); };
            c.w64(11, t143);
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000862c84); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000862c85); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000862c8b); }
            return c.ret(0x0, 0x7000862c8d);
        }
        91 => {
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862c8e); };
            c.w64(8, t144);
            let Some(t145) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000862c92); };
            c.w32(8, t145);
            let r146 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r146);
            let Some(t147) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000862c99); };
            c.w32(8, t147);
            let r148 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r148);
            let Some(t149) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000862ca0); };
            c.w32(9, t149);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            return RegfileExit::fault(0x7000862cb0);
        }
        93 => {
            let Some(t150) = c.ld32(c.r64(8).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x7000862cb1); };
            c.w32(8, t150);
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000862cb8); };
            c.w64(9, t151);
            let Some(t152) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000862cbf); };
            c.w64(11, t152);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000862cc6); };
            c.w64(15, t153);
            let r154 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r154);
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000862cce); };
            c.w64(0, t155);
            let Some(t156) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000862cd5); };
            c.cmp(t156, c.r64(12), 64);
            if c.cc_be() { blk = 186; continue; }
            blk = 94; continue;
        }
        94 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000862cde); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 95; continue;
        }
        95 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000862ce5); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x7000862ce9); }
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000862ced); };
            c.w64(8, t157);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000862cf4); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000862cf8); }
            c.w64(0, c.r64(9));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000862d02); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000862d03); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(15).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000862d09); }
            return c.ret(0x0, 0x7000862d0b);
        }
        96 => {
            c.w32(8, 0x36u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000862d12); }
            let Some(t158) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000862d14); };
            c.w64(8, t158);
            let Some(t159) = c.ld32(c.r64(8).wrapping_add(0xbf)) else { return RegfileExit::fault(0x7000862d1b); };
            c.w32(8, t159);
            let r160 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r160);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000862d25); }
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000862d27); };
            c.w64(3, t161);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000862d38, 0x7000862d33);
        }
        97 => {
            return RegfileExit::fault(0x7000862d38);
        }
        98 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000862d39); }
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000862d3d); };
            c.w64(3, t162);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000862d4e, 0x7000862d49);
        }
        99 => {
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862d4e); };
            c.w64(6, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000862d52); };
            c.w64(0, t164);
            blk = 1; continue;
        }
        100 => {
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000862d5b); };
            c.w64(0, t165);
            blk = 101; continue;
        }
        101 => {
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862d5f); };
            c.w64(6, t166);
            return c.call(0x700089c794, 0x7000862d68, 0x7000862d63);
        }
        102 => {
            blk = 31; continue;
        }
        103 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 121; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t167) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000862d77); };
            c.w32(9, t167);
            let r168 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r168);
            let Some(t169) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000862d7e); };
            c.cmp(t169, c.r32(9), 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t170) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000862d8b); };
            c.cmp(t170, 0xbau64, 16);
            if c.cc_b() { blk = 110; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w64(0, c.r64(8));
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862d97); };
            c.w64(6, t171);
            return c.call(0x70008a2474, 0x7000862da0, 0x7000862d9b);
        }
        107 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 120; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t172) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000862da8); };
            c.w32(8, t172);
            let r173 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r173);
            let Some(t174) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000862daf); };
            c.cmp(t174, c.r32(8), 32);
            if c.cc_e() { blk = 120; continue; }
            blk = 109; continue;
        }
        109 => {
            c.w64(8, c.r64(0));
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000862dbf); };
            c.w64(0, t175);
            blk = 110; continue;
        }
        110 => {
            let Some(t176) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000862dc3); };
            c.w32(9, t176);
            let r177 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r177);
            let Some(t178) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000862dca); };
            c.w32(9, t178);
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 115; continue; }
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_e() { blk = 114; continue; }
            blk = 112; continue;
        }
        112 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000862ddd); }
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000862ddf); };
            c.w64(6, t179);
            let Some(t180) = c.ld64(c.r64(13).wrapping_add(0x24b8)) else { return RegfileExit::fault(0x7000862de3); };
            c.w64(3, t180);
            c.w32(0, 0x1u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000862df7, 0x7000862df2);
        }
        113 => {
            blk = 120; continue;
        }
        114 => {
            let Some(t181) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000862df9); };
            c.w32(9, t181);
            let r182 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r182);
            c.w64(8, c.r64(9));
            blk = 119; continue;
        }
        115 => {
            let Some(t183) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000862e05); };
            c.w32(9, t183);
            c.logic(c.r32(9) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            if let Some(sr) = c.shr(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let r184 = c.r64(9) & 0xffffffu64;
            c.w64(9, r184);
            c.logic(r184, 64);
            let r185 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r185);
            c.w64(8, c.r64(9));
            blk = 119; continue;
        }
        117 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000862e25); }
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x7000862e27); };
            c.w64(3, t186);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000862e3a, 0x7000862e35);
        }
        118 => {
            c.w64(8, c.r64(0));
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000862e3d); };
            c.w64(0, t187);
            blk = 119; continue;
        }
        119 => {
            c.w64(0, c.r64(8));
            blk = 120; continue;
        }
        120 => {
            c.w64(8, c.r64(0));
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000862e47); };
            c.w64(0, t188);
            blk = 121; continue;
        }
        121 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(8)) { return RegfileExit::fault(0x7000862e4b); }
            c.logic(c.r32(8) & 0x80000001u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 122; continue;
        }
        122 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 158; continue; }
            blk = 123; continue;
        }
        123 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000862e66); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 125; continue; }
            blk = 124; continue;
        }
        124 => {
            if c.cc_e() { blk = 157; continue; }
            blk = 125; continue;
        }
        125 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 157; continue; }
            blk = 126; continue;
        }
        126 => {
            if c.cc_ne() { blk = 157; continue; }
            blk = 127; continue;
        }
        127 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x35);
            c.psrlq(2, 0x1);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_p() { blk = 129; continue; }
            blk = 128; continue;
        }
        128 => {
            if c.cc_e() { blk = 157; continue; }
            blk = 129; continue;
        }
        129 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x34);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_p() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            if c.cc_e() { blk = 157; continue; }
            blk = 131; continue;
        }
        131 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 137; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w64(10, 0xc330000000000000u64);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_ae() { blk = 141; continue; }
            blk = 133; continue;
        }
        133 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_be() { blk = 141; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t189) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x7000862ee4); };
            c.w64(10, t189);
            if !c.xorpd_rm(0, c.r64(10)) { return RegfileExit::fault(0x7000862eeb); }
            c.w64(10, 0x4330000000000000u64);
            c.movq_xr(1, 10);
            c.mov128_rr(2, 0);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_be() { blk = 136; continue; }
            blk = 135; continue;
        }
        135 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(1, 2, c.xmm_u64(0));
            blk = 136; continue;
        }
        136 => {
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x7000862f21); };
            c.w64(10, t190);
            if !c.xorpd_rm(2, c.r64(10)) { return RegfileExit::fault(0x7000862f28); }
            c.mov128_rr(0, 2);
            blk = 141; continue;
        }
        137 => {
            let Some(t191) = c.ld64(0x7000862ef1u64) else { return RegfileExit::fault(0x7000862f31); };
            c.w64(10, t191);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 141; continue; }
            blk = 138; continue;
        }
        138 => {
            c.mov128_rr(2, 0);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_a() { blk = 140; continue; }
            blk = 139; continue;
        }
        139 => {
            c.mov128_rr(0, 2);
            blk = 141; continue;
        }
        140 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(1, 2, c.xmm_u64(0));
            c.mov128_rr(0, 2);
            blk = 141; continue;
        }
        141 => {
            c.w64(10, 0x41f0000000000000u64);
            c.movq_xr(1, 10);
            c.w64(12, c.r64(0));
            let r192 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r192);
            if !c.movsd_mr(c.r64(4), 1) { return RegfileExit::fault(0x7000862f84); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x7000862f89); }
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000862f8c); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x7000862f91); }
            blk = 142; continue;
        }
        142 => {
            c.fprem();
            c.fnstsw_ax();
            let r193 = c.shr(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r193);
            let r194 = c.r32(0) & 0xffu64;
            c.w32(0, r194);
            c.logic(r194, 32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000862fa0); }
            if !c.popfq() { return RegfileExit::fault(0x7000862fa1); }
            if c.cc_p() { blk = 142; continue; }
            blk = 143; continue;
        }
        143 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x7000862fa6); }
            if !c.movsd_rm(0, c.r64(4)) { return RegfileExit::fault(0x7000862fa9); }
            let r195 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r195);
            c.arith_sd(0, 0, c.xmm_u64(1));
            let r196 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r196);
            if !c.movsd_mr(c.r64(4), 1) { return RegfileExit::fault(0x7000862fba); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x7000862fbf); }
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000862fc2); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x7000862fc7); }
            blk = 144; continue;
        }
        144 => {
            c.fprem();
            c.fnstsw_ax();
            let r197 = c.shr(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r197);
            let r198 = c.r32(0) & 0xffu64;
            c.w32(0, r198);
            c.logic(r198, 32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000862fd6); }
            if !c.popfq() { return RegfileExit::fault(0x7000862fd7); }
            if c.cc_p() { blk = 144; continue; }
            blk = 145; continue;
        }
        145 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x7000862fdc); }
            if !c.movsd_rm(0, c.r64(4)) { return RegfileExit::fault(0x7000862fdf); }
            let r199 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r199);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x7000862ff6); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 151; continue; }
            blk = 146; continue;
        }
        146 => {
            if c.cc_ne() { blk = 151; continue; }
            blk = 147; continue;
        }
        147 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 149; continue; }
            blk = 148; continue;
        }
        148 => {
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 151; continue; }
            blk = 149; continue;
        }
        149 => {
            c.w64(9, c.r64(8));
            let r200 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r200);
            if c.cc_o() { blk = 151; continue; }
            blk = 150; continue;
        }
        150 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(9));
            blk = 156; continue;
        }
        151 => {
            let Some(t201) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000863028); };
            c.w64(8, t201);
            let Some(t202) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700086302f); };
            c.w64(9, t202);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t203) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000863036); };
            c.w64(15, t203);
            let Some(t204) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700086303d); };
            c.cmp(t204, c.r64(11), 64);
            if c.cc_a() { blk = 154; continue; }
            blk = 152; continue;
        }
        152 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700086304c, 0x7000863047);
        }
        153 => {
            blk = 155; continue;
        }
        154 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700086304e); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 155; continue;
        }
        155 => {
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000863058); };
            c.w64(8, t205);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700086305f); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x7000863063); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000863068); }
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700086306d); };
            c.w64(12, t206);
            blk = 156; continue;
        }
        156 => {
            c.w64(8, c.r64(0));
            c.w64(0, c.r64(12));
            blk = 35; continue;
        }
        157 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 35; continue;
        }
        158 => {
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000863084); };
            c.w64(9, t207);
            let Some(t208) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700086308b); };
            c.w64(11, t208);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000863092); };
            c.w64(15, t209);
            let Some(t210) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000863099); };
            c.cmp(t210, c.r64(12), 64);
            if c.cc_a() { blk = 161; continue; }
            blk = 159; continue;
        }
        159 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008630a8, 0x70008630a3);
        }
        160 => {
            c.w64(9, c.r64(0));
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008630ab); };
            c.w64(0, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008630af); };
            c.w64(8, t212);
            blk = 162; continue;
        }
        161 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008630b5); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 162; continue;
        }
        162 => {
            let r213 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r213);
            let Some(t214) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008630bf); };
            c.w64(11, t214);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008630c6); }
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008630d6); }
            c.w64(8, c.r64(9));
            blk = 35; continue;
        }
        163 => {
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008630e4); };
            c.w64(9, t215);
            blk = 164; continue;
        }
        164 => {
            c.w64(0, c.r64(9));
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008630eb); };
            c.w64(6, t216);
            return c.call(0x700089c794, 0x70008630f4, 0x70008630ef);
        }
        165 => {
            c.w64(11, c.r64(0));
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008630f7); };
            c.w64(0, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008630fb); };
            c.w64(8, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008630ff); };
            c.w64(9, t219);
            blk = 38; continue;
        }
        166 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000863108); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x700086310c); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x7000863110); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x7000863114); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x7000863118); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x700086311c); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febb4, 0x7000863128, 0x7000863123);
        }
        167 => {
            c.w64(8, c.r64(0));
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086312d); };
            c.w64(9, t220);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000863131); };
            c.w64(1, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000863135); };
            c.w64(11, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000863139); };
            c.w64(12, t223);
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700086313d); };
            c.w64(15, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000863141); };
            c.w64(3, t225);
            blk = 70; continue;
        }
        168 => {
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 175; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t226) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000863150); };
            c.w64(12, t226);
            blk = 72; continue;
        }
        170 => {
            return RegfileExit::fault(0x700086315a);
        }
        171 => {
            return RegfileExit::fault(0x700086315b);
        }
        172 => {
            blk = 175; continue;
        }
        173 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000863163); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000863167); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700086316b); }
            return c.call(0x70007febf4, 0x7000863174, 0x700086316f);
        }
        174 => {
            c.w64(12, c.r64(0));
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000863177); };
            c.w64(8, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700086317b); };
            c.w64(9, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086317f); };
            c.w64(11, t229);
            blk = 82; continue;
        }
        175 => {
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000863188); };
            c.w64(9, t230);
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 178; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700086319a); };
            c.w64(11, t231);
            blk = 177; continue;
        }
        177 => {
            let r232 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r232);
            if !c.st32(c.r64(9).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x70008631a5); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 177; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008631af); };
            c.w64(0, t233);
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008631b3); };
            c.w64(8, t234);
            blk = 179; continue;
        }
        179 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008631b7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008631b8); }
            let Some(t235) = c.ld64(c.r64(13).wrapping_add(0x27c8)) else { return RegfileExit::fault(0x70008631ba); };
            c.w64(3, t235);
            c.w32(0, 0x2u64);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008631c6); };
            c.w64(6, t236);
            return c.call(0x700086ed74, 0x70008631cf, 0x70008631ca);
        }
        180 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008631cf); };
            c.w64(9, t237);
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008631da); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008631db); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008631e1); }
            return c.ret(0x0, 0x70008631e3);
        }
        181 => {
            let Some(t238) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008631e4); };
            c.w64(3, t238);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008631f2, 0x70008631ed);
        }
        182 => {
            return RegfileExit::fault(0x70008631f2);
        }
        183 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008631f8); }
            return c.call(0x70007febf4, 0x7000863201, 0x70008631fc);
        }
        184 => {
            c.w64(9, c.r64(0));
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000863204); };
            c.w64(0, t239);
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000863208); };
            c.w64(8, t240);
            blk = 87; continue;
        }
        185 => {
            return RegfileExit::fault(0x7000863211);
        }
        186 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x7000863217); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700086321b); }
            return c.call(0x70007febf4, 0x7000863224, 0x700086321f);
        }
        187 => {
            c.w64(9, c.r64(0));
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000863227); };
            c.w64(15, t241);
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086322b); };
            c.w64(8, t242);
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 95; continue;
        }
        188 => {
            c.w32(12, c.r32(12));
            let r243 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r243);
            c.w64(11, 0xfffffffffffc0000u64);
            let r244 = c.r64(11) & c.r64(12);
            c.w64(11, r244);
            c.logic(r244, 64);
            let Some(t245) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000863246); };
            c.logic(t245 & 0x2u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 189; continue;
        }
        189 => {
            c.w64(15, c.r64(8).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000863255); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000863256); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000863263, 0x700086325e);
        }
        190 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000863263); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000863265); }
            blk = 90; continue;
        }
        191 => {
            return RegfileExit::jump(0x700086326c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000863274(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1ba8)) else { return RegfileExit::fault(0x7000863274); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000863294(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1bb0)) else { return RegfileExit::fault(0x7000863294); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008632b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1bb8)) else { return RegfileExit::fault(0x70008632b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008632d4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1bc0)) else { return RegfileExit::fault(0x70008632d4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008632f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008632f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008632f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008632f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008632fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008632ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(2, 0x48u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086330a); }
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700086330b); };
            c.w64(2, t2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(0xc3)) else { return RegfileExit::fault(0x7000863312); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700086331b); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700086331c); };
            c.w64(3, t5);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700086332d, 0x7000863328);
        }
        2 => {
            return RegfileExit::fault(0x700086332d);
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700086332e); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000863332); };
            c.w64(3, t6);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000863340, 0x700086333b);
        }
        4 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000863340); };
            c.w64(6, t7);
            blk = 1; continue;
        }
        5 => {
            return RegfileExit::jump(0x7000863348);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000863354(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000863354); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000863358); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000863359); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086335a); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700086335f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 12; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700086336c); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(9, c.r64(5));
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000863379); };
            c.w64(9, t2);
            c.w64(11, c.r64(9));
            blk = 4; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000863382); };
            c.w64(11, t3);
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000863389); };
            c.w64(9, t4);
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(12, c.r64(5));
            let r5 = c.add(c.r64(12), 0x18u64, 64);
            c.w64(12, r5);
            let Some(t6) = c.ld64(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700086339d); };
            c.w64(12, t6);
            blk = 7; continue;
        }
        6 => {
            c.w64(12, c.r64(9));
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(9, c.r64(5));
            let r7 = c.add(c.r64(9), 0x18u64, 64);
            c.w64(9, r7);
            let Some(t8) = c.ld64(c.r64(9).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008633b4); };
            c.w64(9, t8);
            blk = 9; continue;
        }
        9 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008633b8); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(0, c.r64(7));
            c.w64(3, c.r64(2));
            c.w64(1, c.r64(11));
            c.w64(2, c.r64(12));
            c.w64(7, c.r64(9));
            return c.call(0x70008dd9b4, 0x70008633d5, 0x70008633d0);
        }
        11 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008633d5); };
            c.w64(8, t10);
            let r11 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008633e0); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008633e1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008633e7); }
            return c.ret(0x0, 0x70008633e9);
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008633ea); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008633ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008633f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008633f6); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008633fa); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700086340b, 0x7000863406);
        }
        13 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086340b); };
            c.w64(6, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700086340f); };
            c.w64(7, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000863413); };
            c.w64(2, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000863417); };
            c.w64(0, t16);
            blk = 1; continue;
        }
        14 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000863420); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000863421); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2388)) else { return RegfileExit::fault(0x7000863425); };
            c.w64(3, t17);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000863436, 0x7000863431);
        }
        15 => {
            c.w32(8, 0x52u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700086343c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700086343e); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700086343f); };
            c.w64(3, t18);
            c.w32(0, 0x2u64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700086344b); };
            c.w64(6, t19);
            return c.call(0x700086ed74, 0x7000863454, 0x700086344f);
        }
        16 => {
            return RegfileExit::fault(0x7000863454);
        }
        17 => {
            return RegfileExit::jump(0x7000863458);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000863474(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1bc8)) else { return RegfileExit::fault(0x7000863474); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}
