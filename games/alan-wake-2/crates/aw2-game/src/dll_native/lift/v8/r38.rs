#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_70009346b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009346b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009346b8); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009346be); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009346c2); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009346c6); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009346cd); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009346d4); };
            c.w32(0, t7);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009346d8); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009346dc); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009346e0); }
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009346e4); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(0, c.r64(11));
            blk = 3; continue;
        }
        2 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009346f2); };
            c.w64(0, t9);
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009346f9); };
            c.w64(3, (t10 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000934701); };
            c.w32(2, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000934707); };
            c.w64(11, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700093470e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000934712); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000934716); };
            c.cmp(t13, c.r32(0), 32);
            if c.cc_e() { blk = 98; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(2) & 0x20u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700093472c); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000934733); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000934740); };
            c.w32(6, t16);
            let r17 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r17);
            let Some(t18) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000934746); };
            c.w32(8, t18);
            let r19 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r19);
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700093474d); };
            c.w32(3, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x7000934751); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x7000934755); }
            c.logic(c.r32(3) & 0x1000000u64, 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 7; continue;
        }
        7 => {
            c.logic(c.r32(3) & 0x200000u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700093476d); };
            c.w32(11, t21);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(3, c.r64(11));
            blk = 11; continue;
        }
        10 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700093477c); };
            c.w64(3, t22);
            blk = 11; continue;
        }
        11 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000934783); };
            c.w64(11, t23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700093478a); };
            c.cmp(t24, c.r32(3), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 12; continue;
        }
        12 => {
            blk = 98; continue;
        }
        13 => {
            let Some(t25) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700093479c); };
            c.w32(3, t25);
            let r26 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r26);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70009347a7); };
            c.w64(3, t27);
            blk = 15; continue;
        }
        15 => {
            let Some(t28) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009347ae); };
            c.w32(0, t28);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x1000u64, 64);
            if c.cc_a() { blk = 97; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009347c8); };
            c.w64(12, t29);
            let Some(t30) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70009347cf); };
            c.w64(7, t30);
            c.w64(1, c.r64(2).wrapping_mul(4).wrapping_add(0x8));
            c.w64(8, c.r64(1).wrapping_add(c.r64(7)));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009347df); };
            c.w64(9, t31);
            let Some(t32) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70009347e6); };
            c.cmp(t32, c.r64(8), 64);
            if c.cc_be() { blk = 76; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.st64(c.r64(12), c.r64(8)) { return RegfileExit::fault(0x70009347ef); }
            let r33 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r33);
            blk = 18; continue;
        }
        18 => {
            let r34 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r34);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70009347f9); };
            c.w64(8, t35);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000934800); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x7000934804); }
            let Some(t36) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000934807); };
            c.w32(2, t36);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 78; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x700093481c); }
            let Some(t37) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700093481f); };
            c.w32(8, t37);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 79; continue; }
            blk = 20; continue;
        }
        20 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x7000934833); }
            let Some(t38) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000934836); };
            c.w32(2, t38);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_be() { blk = 80; continue; }
            blk = 21; continue;
        }
        21 => {
            let r39 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r39);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x700093484b); }
            let Some(t40) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700093484e); };
            c.w32(0, t40);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x3u64, 64);
            if c.cc_be() { blk = 81; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(0, 0x2u64);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x7000934866); }
            let Some(t41) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000934869); };
            c.w32(0, t41);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x4u64, 64);
            if c.cc_be() { blk = 82; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(0, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(0)) { return RegfileExit::fault(0x700093487e); }
            c.w64(0, c.r64(7).wrapping_add(c.r64(1)).wrapping_sub(0x1));
            c.w64(2, c.r64(7).wrapping_add(0x1b));
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st32(c.r64(2), c.r32(11)) { return RegfileExit::fault(0x700093488f); }
            let r42 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r42);
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_ne() { blk = 24; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t43) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700093489b); };
            c.w32(11, t43);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(11).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            let r44 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r44);
            let Some(t45) = c.ld32(c.r64(11).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x70009348b4); };
            c.w32(0, t45);
            if !c.st32(c.r64(11).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x70009348b8); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 26; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(11, c.r64(7));
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009348c5); };
            c.w64(9, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009348c9); };
            c.w64(7, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009348cd); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009348d1); };
            c.w64(8, t49);
            blk = 28; continue;
        }
        28 => {
            let Some(t50) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009348d5); };
            c.w32(0, t50);
            let r51 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r51);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70009348db); }
            let Some(t52) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70009348df); };
            c.cmp(t52, c.r32(0), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t53) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009348ec); };
            c.w32(3, t53);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t54) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009348f2); };
            c.w32(2, t54);
            c.w64(12, c.r64(3));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 50; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000934905); };
            c.cmp(t55, c.r32(2), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t56) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000934912); };
            c.cmp(t56, c.r32(2), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 85; continue; }
            blk = 33; continue;
        }
        33 => {
            let r57 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r57);
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000934932); };
            c.w64(7, t58);
            let Some(t59) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000934939); };
            c.w64(11, t59);
            c.w64(1, c.r64(3).wrapping_add(0x8));
            c.w64(6, c.r64(1).wrapping_add(c.r64(11)));
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000934944); };
            c.w64(8, t60);
            let Some(t61) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700093494b); };
            c.cmp(t61, c.r64(6), 64);
            if c.cc_be() { blk = 83; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st64(c.r64(7), c.r64(6)) { return RegfileExit::fault(0x7000934954); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 35; continue;
        }
        35 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700093495b); }
            c.w64(11, c.r64(7));
            let r62 = c.r64(11) & 0xfffffffffffc0000u64;
            c.w64(11, r62);
            c.logic(r62, 64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000934968); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700093496c); }
            let Some(t63) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000934970); };
            c.logic(t63 & 0x4u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(11, c.r64(3).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 42; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w32(3, 0x7u64);
            blk = 38; continue;
        }
        38 => {
            let Some(t64) = c.ld32(c.r64(0).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x7000934986); };
            c.w32(2, t64);
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)), c.r32(2)) { return RegfileExit::fault(0x7000934989); }
            let r65 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r65);
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_ne() { blk = 38; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 42; continue;
        }
        40 => {
            c.w64(2, c.r64(0).wrapping_add(0x7));
            c.w64(1, c.r64(7).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r66 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r66);
            let r67 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r67);
            c.logic(r67, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70009349aa); }
            c.w64(8, c.r64(3));
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x70009349b2); };
            c.w64(0, t68);
            c.w64(10, 0x70009349c0u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70009349c0); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70009349c4); }
            return c.call(c.r64(0), 0x70009349ca, 0x70009349c8);
        }
        41 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70009349ca); }
            let Some(t69) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70009349d2); };
            c.w64(4, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009349d7); };
            c.w64(15, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009349db); };
            c.w64(9, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009349df); };
            c.w64(7, t72);
            blk = 42; continue;
        }
        42 => {
            c.w64(0, c.r64(7));
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009349e6); };
            c.w64(7, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009349ea); };
            c.w64(6, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009349ee); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009349f2); };
            c.w64(11, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009349f6); };
            c.w64(8, t77);
            blk = 51; continue;
        }
        43 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 91; continue; }
            blk = 44; continue;
        }
        44 => {
            if let Some(sr) = c.shl(c.r64(3), 0x2u64, 64) { c.w64(3, sr); }
            c.w64(11, c.r64(3).wrapping_add(0x8));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000934a14); }
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 87; continue; }
            blk = 45; continue;
        }
        45 => {
            let r78 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r78);
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000934a28); };
            c.w64(3, t79);
            let Some(t80) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000934a2f); };
            c.w64(1, t80);
            c.w64(6, c.r64(11).wrapping_add(c.r64(1)));
            let Some(t81) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000934a36); };
            c.cmp(t81, c.r64(6), 64);
            if c.cc_be() { blk = 89; continue; }
            blk = 46; continue;
        }
        46 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x7000934a40); }
            c.w64(11, c.r64(1).wrapping_add(0x1));
            blk = 47; continue;
        }
        47 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x7000934a47); }
            c.w64(3, 0xfffffffffffc0000u64);
            let r82 = c.r64(3) & c.r64(11);
            c.w64(3, r82);
            c.logic(r82, 64);
            let Some(t83) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000934a55); };
            c.logic(t83 & 0x4u64, 8);
            if c.cc_ne() { blk = 103; continue; }
            blk = 48; continue;
        }
        48 => {
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000934a5f); }
            c.w64(2, c.r64(0).wrapping_add(0x7));
            c.w64(1, c.r64(11).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r84 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r84);
            let r85 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r85);
            c.logic(r85, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000934a76); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000934a7b); }
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000934a7f); };
            c.w64(8, t86);
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000934a83); };
            c.w64(0, t87);
            c.w64(10, 0x7000934a91u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000934a91); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000934a95); }
            return c.call(c.r64(0), 0x7000934a9b, 0x7000934a99);
        }
        49 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000934a9b); }
            let Some(t88) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000934aa3); };
            c.w64(4, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000934aa8); };
            c.w64(0, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000934aac); };
            c.w64(15, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000934ab0); };
            c.w64(9, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000934ab4); };
            c.w64(7, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000934ab8); };
            c.w64(6, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000934abc); };
            c.w64(1, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000934ac0); };
            c.w64(11, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000934ac4); };
            c.w64(8, t96);
            blk = 51; continue;
        }
        50 => {
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000934aca); };
            c.w64(0, t97);
            blk = 51; continue;
        }
        51 => {
            c.w64(3, c.r64(8));
            let Some(t98) = c.ld8(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000934ad4); };
            c.w32(2, t98);
            if let Some(sr) = c.shl(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000934adc); };
            c.w64(8, t99);
            let Some(t100) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000934ae3); };
            c.w64(12, t100);
            c.w64(15, c.r64(2).wrapping_add(0x8));
            c.w64(6, c.r64(15).wrapping_add(c.r64(12)));
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000934aee); };
            c.w64(1, t101);
            let Some(t102) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000934af5); };
            c.cmp(t102, c.r64(6), 64);
            if c.cc_be() { blk = 93; continue; }
            blk = 52; continue;
        }
        52 => {
            if !c.st64(c.r64(8), c.r64(6)) { return RegfileExit::fault(0x7000934afe); }
            c.w64(1, c.r64(12).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000934b06); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000934b09); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000934b0d); }
            c.w64(11, c.r64(2).wrapping_add(c.r64(1)));
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000934b14); };
            c.w64(0, t103);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000934b1b); }
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000934b1f); };
            c.w64(0, t104);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x7000934b23); }
            let Some(t105) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000934b27); };
            c.w32(11, t105);
            let r106 = c.add(c.r32(11), 0x1u64, 32);
            c.w32(11, r106);
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x7000934b2f); }
            c.cmp(c.r64(2), 0xcu64, 64);
            if c.cc_e() { blk = 74; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000934b3d); };
            c.w64(3, t107);
            c.w32(0, 0xcu64);
            c.w64(11, c.r64(0));
            blk = 55; continue;
        }
        55 => {
            let Some(t108) = c.ld32(c.r64(3).wrapping_add(c.r64(0)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000934b49); };
            c.w32(6, t108);
            let r109 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r109);
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t110) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000934b56); };
            c.w32(8, t110);
            let Some(t111) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000934b5a); };
            c.cmp(t111, c.r32(8), 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.st32(c.r64(1).wrapping_add(c.r64(0)).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000934b63); }
            let r112 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r112);
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_ne() { blk = 55; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 74; continue;
        }
        59 => {
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_e() { blk = 63; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(6, c.r64(0));
            blk = 61; continue;
        }
        61 => {
            let Some(t113) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000934b7d); };
            c.w32(8, t113);
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000934b82); }
            let r114 = c.add(c.r64(6), 0x4u64, 64);
            c.w64(6, r114);
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_ne() { blk = 61; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 64; continue;
        }
        63 => {
            c.w32(3, 0x1u64);
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000934ba2); };
            c.w64(3, t115);
            blk = 66; continue;
        }
        66 => {
            c.w64(6, c.r64(0).wrapping_sub(0x1));
            let Some(t116) = c.ld32(c.r64(1).wrapping_add(c.r64(0)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000934bad); };
            c.w32(8, t116);
            let r117 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r117);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t118) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000934bbb); };
            c.w32(12, t118);
            let Some(t119) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000934bbf); };
            c.cmp(t119, c.r32(12), 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 68; continue;
        }
        68 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000934bc8); }
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000934bce); };
            c.w64(8, t120);
            let Some(t121) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000934bd5); };
            c.w64(12, t121);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000934bdd); };
            c.w64(7, t122);
            let Some(t123) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000934be4); };
            c.cmp(t123, c.r64(15), 64);
            if c.cc_be() { blk = 95; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x7000934bed); }
            c.w64(7, c.r64(12).wrapping_add(0x1));
            blk = 70; continue;
        }
        70 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000934bf5); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000934bf8); }
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)), c.r32(7)) { return RegfileExit::fault(0x7000934bfd); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r124 = c.r64(8) & c.r64(1);
            c.w64(8, r124);
            c.logic(r124, 64);
            let Some(t125) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000934c10); };
            c.logic(t125 & 0x4u64, 8);
            if c.cc_ne() { blk = 100; continue; }
            blk = 72; continue;
        }
        72 => {
            let r126 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r126);
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_ne() { blk = 66; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000934c24); };
            c.w64(7, t127);
            blk = 74; continue;
        }
        74 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0xa));
            let Some(t128) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000934c2f); };
            c.w64(7, t128);
            let Some(t129) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000934c33); };
            c.w32(11, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000934c38); };
            c.w64(15, t130);
            let Some(t131) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000934c3c); };
            c.w64(11, t131);
            let Some(t132) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000934c40); };
            c.w64(5, t132);
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(11));
            c.w64(12, c.r64(7));
            let r133 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r133);
            return RegfileExit::jump(c.r64(1));
        }
        75 => {
            blk = 98; continue;
        }
        76 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000934c58); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000934c5c); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000934c60); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000934c64); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x7000934c70, 0x7000934c6b);
        }
        77 => {
            c.w64(7, c.r64(0));
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000934c73); };
            c.w64(15, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000934c77); };
            c.w64(6, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000934c7b); };
            c.w64(3, t136);
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000934c7f); };
            c.w64(11, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000934c86); };
            c.w64(1, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000934c8a); };
            c.w64(0, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000934c8e); };
            c.w64(2, t140);
            blk = 18; continue;
        }
        78 => {
            return RegfileExit::fault(0x7000934c97);
        }
        79 => {
            return RegfileExit::fault(0x7000934c98);
        }
        80 => {
            return RegfileExit::fault(0x7000934c99);
        }
        81 => {
            return RegfileExit::fault(0x7000934c9a);
        }
        82 => {
            return RegfileExit::fault(0x7000934c9b);
        }
        83 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000934c9c); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x7000934ca0); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000934ca4); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x7000934ca8); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x7000934cb4, 0x7000934caf);
        }
        84 => {
            c.w64(7, c.r64(0));
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000934cb7); };
            c.w64(15, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000934cbb); };
            c.w64(9, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000934cbf); };
            c.w64(0, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000934cc3); };
            c.w64(2, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000934cc7); };
            c.w64(3, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000934ccb); };
            c.w64(12, t146);
            blk = 35; continue;
        }
        85 => {
            c.w64(11, c.r64(9));
            let r147 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r147);
            let r148 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r148);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000934cde); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000934ce6); };
            c.w64(3, t149);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x7000934cf5, 0x7000934cf0);
        }
        86 => {
            return RegfileExit::fault(0x7000934cf5);
        }
        87 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r150 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r150);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000934cfd); }
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x7000934d01); };
            c.w64(3, t151);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000934d11, 0x7000934d0c);
        }
        88 => {
            return RegfileExit::fault(0x7000934d11);
        }
        89 => {
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let r152 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r152);
            let r153 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r153);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000934d1b); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000934d1f); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000934d23); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000934d24); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x7000934d28); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x7000934d2c); }
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000934d30); };
            c.w64(3, t154);
            c.w32(0, 0x2u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x7000934d44, 0x7000934d3f);
        }
        90 => {
            c.w64(11, c.r64(0));
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000934d47); };
            c.w64(0, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000934d4b); };
            c.w64(2, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000934d4f); };
            c.w64(12, t157);
            blk = 47; continue;
        }
        91 => {
            c.w64(11, c.r64(9));
            let r158 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r158);
            let r159 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r159);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000934d62); }
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000934d66); };
            c.w64(3, t160);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000934d76, 0x7000934d71);
        }
        92 => {
            return RegfileExit::fault(0x7000934d76);
        }
        93 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000934d77); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000934d7b); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x7000934d87, 0x7000934d82);
        }
        94 => {
            c.w64(1, c.r64(0));
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000934d8a); };
            c.w64(9, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000934d8e); };
            c.w64(7, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000934d92); };
            c.w64(2, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000934d96); };
            c.w64(0, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000934d9a); };
            c.w64(11, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000934d9e); };
            c.w64(3, t166);
            blk = 53; continue;
        }
        95 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000934da7); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000934dab); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x7000934daf); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x7000934db3); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x7000934db7); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febf4, 0x7000934dc4, 0x7000934dbf);
        }
        96 => {
            c.w64(7, c.r64(0));
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000934dc7); };
            c.w64(9, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000934dcb); };
            c.w64(1, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000934dcf); };
            c.w64(2, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000934dd3); };
            c.w64(0, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000934dd7); };
            c.w64(6, t171);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x7000934ddb); }
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000934de0); };
            c.w64(3, t172);
            c.w32(11, 0xcu64);
            blk = 70; continue;
        }
        97 => {
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000934df2); };
            c.w64(0, t173);
            blk = 98; continue;
        }
        98 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r174 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r174);
            let Some(t175) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000934dfd); };
            c.w32(3, t175);
            let Some(t176) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000934e02); };
            c.w32(1, t176);
            let r177 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r177);
            let r178 = c.r64(2) & 0x1fu64;
            c.w64(2, r178);
            c.logic(r178, 64);
            let Some(t179) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000934e0e); };
            c.w64(6, t179);
            let Some(t180) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000934e12); };
            c.w32(3, t180);
            let r181 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r181);
            let r182 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r182);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000934e1b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000934e1f); }
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000934e20); };
            if !c.push64(t183) { return RegfileExit::fault(0x7000934e20); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000934e23); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000934e24); }
            let Some(t184) = c.ld64(c.r64(13).wrapping_add(0x2280)) else { return RegfileExit::fault(0x7000934e25); };
            c.w64(3, t184);
            c.w32(0, 0x4u64);
            return c.call(0x700086ed74, 0x7000934e36, 0x7000934e31);
        }
        99 => {
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000934e36); };
            c.w64(7, t185);
            let Some(t186) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000934e3a); };
            c.w64(7, t186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000934e3e); };
            c.w64(11, t187);
            c.w64(9, c.r64(11).wrapping_add(0xa));
            let Some(t188) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000934e46); };
            c.w32(11, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000934e4b); };
            c.w64(15, t189);
            let Some(t190) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000934e4f); };
            c.w64(1, t190);
            let Some(t191) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000934e53); };
            c.w64(5, t191);
            c.w64(12, c.r64(7));
            let r192 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r192);
            return RegfileExit::jump(c.r64(1));
        }
        100 => {
            c.w32(7, c.r32(7));
            let r193 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r193);
            c.w64(8, 0xfffffffffffc0000u64);
            let r194 = c.r64(8) & c.r64(7);
            c.w64(8, r194);
            c.logic(r194, 64);
            let Some(t195) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000934e6f); };
            c.logic(t195 & 0x2u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 101; continue;
        }
        101 => {
            c.w64(12, c.r64(1).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000934e7e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000934e7f); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000934e8c, 0x7000934e87);
        }
        102 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000934e8c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000934e8e); }
            blk = 72; continue;
        }
        103 => {
            c.w32(2, c.r32(2));
            let r196 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r196);
            c.w64(3, 0xfffffffffffc0000u64);
            let r197 = c.r64(3) & c.r64(2);
            c.w64(3, r197);
            c.logic(r197, 64);
            let Some(t198) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000934ea3); };
            c.logic(t198 & 0x2u64, 8);
            if c.cc_e() { blk = 48; continue; }
            blk = 104; continue;
        }
        104 => {
            c.w64(1, c.r64(11).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000934eb1); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000934eb2); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(1));
            return c.call(0x70007f13b4, 0x7000934ebf, 0x7000934eba);
        }
        105 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000934ebf); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000934ec1); }
            blk = 48; continue;
        }
        106 => {
            return RegfileExit::jump(0x7000934ec8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000934ed4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000934ed4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000934ed8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000934ede); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000934ee2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000934ee6); };
            c.w64(11, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000934eeb); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000934ef2); };
            c.w64(11, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000934ef6); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000934efd); };
            c.w32(0, t9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x6)) else { return RegfileExit::fault(0x7000934f03); };
            c.w32(3, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000934f08); };
            c.w32(2, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000934f0c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000934f10); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000934f14); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000934f18); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000934f21); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t14) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000934f2c); };
            c.w64(1, t14);
            let r15 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r15);
            let r16 = c.r64(0) & 0x1fu64;
            c.w64(0, r16);
            c.logic(r16, 64);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let r17 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r17);
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000934f3e); }
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(3));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(2, c.r64(8));
            return c.call(0x70008425f4, 0x7000934f56, 0x7000934f51);
        }
        3 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000934f56); };
            c.w64(7, t18);
            let Some(t19) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000934f5a); };
            c.w64(7, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000934f5e); };
            c.w64(8, t20);
            c.w64(9, c.r64(8).wrapping_add(0xa));
            let Some(t21) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000934f66); };
            c.w32(8, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000934f6b); };
            c.w64(15, t22);
            let Some(t23) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000934f6f); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000934f73); };
            c.w64(5, t24);
            c.w64(12, c.r64(7));
            let r25 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r25);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000934f84);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000934f94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000934f94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000934f98); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000934f9e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000934fa2); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000934fa6); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000934faa); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000934fb1); };
            c.w32(0, t6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000934fb6); };
            c.w32(3, t7);
            let r8 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r8);
            let Some(t9) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000934fbe); };
            c.w64(2, t9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000934fc2); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            let Some(t12) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000934fc9); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(3).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000934fce); };
            c.w32(3, t13);
            let r14 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r14);
            let Some(t15) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000934fd5); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            let Some(t17) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000934fdc); };
            c.w32(11, t17);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000934fe0); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000934fe4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000934fe8); }
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000934fec); };
            c.cmp(t18, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000934ff5); };
            c.w64(8, t19);
            blk = 2; continue;
        }
        2 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r20 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r20);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000935003); }
            c.w64(6, c.r64(2));
            c.w64(2, c.r64(8));
            return c.call(0x70008a8734, 0x7000935012, 0x700093500d);
        }
        3 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000935012); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000935016); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700093501a); };
            c.w64(8, t23);
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t24) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000935022); };
            c.w32(8, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935027); };
            c.w64(15, t25);
            let Some(t26) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700093502b); };
            c.w64(1, t26);
            let Some(t27) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700093502f); };
            c.w64(5, t27);
            c.w64(12, c.r64(7));
            let r28 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r28);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000935040);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935054(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000935054); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000935058); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700093505e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935062); };
            c.w64(7, t2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935066); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700093506c); };
            c.w32(8, t5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935071); };
            c.w32(11, t6);
            let r7 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r7);
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000935079); };
            c.w64(0, t8);
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700093507d); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000935083); };
            c.w32(3, t11);
            let Some(t12) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000935089); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093508e); };
            c.w32(8, t13);
            let r14 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r14);
            let Some(t15) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000935096); };
            c.w32(11, t15);
            let r16 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r16);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x700093509d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70009350a1); }
            let Some(t17) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009350a5); };
            c.cmp(t17, 0x81u64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70009350ae); };
            c.w32(7, t18);
            let r19 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r19);
            blk = 2; continue;
        }
        2 => {
            let Some(t20) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009350b4); };
            c.w32(11, t20);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(1), c.r64(11), 64);
            if c.cc_ae() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r21 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r21);
            let Some(t22) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009350ca); };
            c.w32(7, t22);
            let r23 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r23);
            let r24 = c.add(c.r64(9), 0xau64, 64);
            c.w64(9, r24);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70009350d5); }
            c.logic(c.r8(3) & 0x2u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009350de); }
            c.w64(6, c.r64(0));
            c.w64(0, c.r64(8));
            c.w64(3, c.r64(7));
            return c.call(0x70007f9f34, 0x70009350f0, 0x70009350eb);
        }
        5 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009350f0); };
            c.w64(2, t25);
            let Some(t26) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009350f4); };
            c.w64(2, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009350f8); };
            c.w64(9, t27);
            let Some(t28) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70009350fc); };
            c.w32(7, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000935101); };
            c.w64(15, t29);
            let Some(t30) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935105); };
            c.w64(1, t30);
            let Some(t31) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000935109); };
            c.w64(5, t31);
            c.w64(12, c.r64(2));
            let r32 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r32);
            return RegfileExit::jump(c.r64(1));
        }
        6 => {
            return RegfileExit::fault(0x7000935116);
        }
        7 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700093511c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000935120); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000935122); }
            c.w64(6, c.r64(0));
            c.w32(0, 0x2u64);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x26d8)) else { return RegfileExit::fault(0x700093512b); };
            c.w64(3, t33);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700093513a, 0x7000935135);
        }
        9 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700093513a); };
            c.w64(2, t34);
            let Some(t35) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700093513e); };
            c.w64(2, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935142); };
            c.w64(9, t36);
            let Some(t37) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000935146); };
            c.w32(7, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700093514b); };
            c.w64(15, t38);
            let Some(t39) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700093514f); };
            c.w64(1, t39);
            let Some(t40) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000935153); };
            c.w64(5, t40);
            c.w64(12, c.r64(2));
            let r41 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r41);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000935160); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000935164); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000935166); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x26e0)) else { return RegfileExit::fault(0x7000935167); };
            c.w64(3, t42);
            c.w64(6, c.r64(0));
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700093517b, 0x7000935176);
        }
        11 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700093517b); };
            c.w64(2, t43);
            let Some(t44) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700093517f); };
            c.w64(2, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935183); };
            c.w64(9, t45);
            let Some(t46) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000935187); };
            c.w32(7, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700093518c); };
            c.w64(15, t47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935190); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000935194); };
            c.w64(5, t49);
            c.w64(12, c.r64(2));
            let r50 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r50);
            return RegfileExit::jump(c.r64(1));
        }
        12 => {
            return RegfileExit::jump(0x70009351a4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009351b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009351b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009351b8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009351c2); };
            c.w64(7, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009351c8); }
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009351cc); };
            c.w32(2, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009351d1); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009351d9); };
            c.w64(11, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009351dd); };
            c.w32(2, t7);
            let r8 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r8);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009351e5); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70009351e6); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2708)) else { return RegfileExit::fault(0x70009351ea); };
            c.w64(3, t9);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x70009351fe, 0x70009351f9);
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009351fe); };
            c.w64(2, t10);
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000935202); };
            c.w64(7, t11);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000935209); };
            c.w64(2, t12);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7).wrapping_add(0x6));
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000935214); };
            c.w32(7, t13);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x7000935219); };
            c.w64(15, t14);
            let Some(t15) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935220); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000935224); };
            c.w64(5, t16);
            c.w64(12, c.r64(2));
            let r17 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r17);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000935234);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935254(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000935254); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000935258); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000935262); };
            c.w64(7, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000935268); }
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700093526c); };
            c.w64(2, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000935271); };
            c.w32(8, t4);
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935276); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700093527e); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000935282); };
            c.w64(0, t8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935286); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700093528e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700093528f); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000935291); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2710)) else { return RegfileExit::fault(0x7000935295); };
            c.w64(3, t11);
            c.w64(6, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70009352ac, 0x70009352a7);
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009352ac); };
            c.w64(2, t12);
            let Some(t13) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009352b0); };
            c.w64(7, t13);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009352b7); };
            c.w64(2, t14);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7).wrapping_add(0xa));
            let Some(t15) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70009352c2); };
            c.w32(7, t15);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x70009352c7); };
            c.w64(15, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70009352ce); };
            c.w64(1, t17);
            let Some(t18) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009352d2); };
            c.w64(5, t18);
            c.w64(12, c.r64(2));
            let r19 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r19);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70009352e0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009352f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009352f4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009352f9); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            c.w64(7, c.r64(5));
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000935304); };
            c.w32(8, t3);
            let Some(t4) = c.ld32(c.r64(6).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935309); };
            c.w32(3, t4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700093530d); };
            c.w64(6, t5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000935311); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000935318); };
            c.w64(11, t7);
            c.w64(2, c.r64(8).wrapping_mul(4).wrapping_add(0x10));
            c.w64(0, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000935327); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700093532e); };
            c.cmp(t9, c.r64(0), 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.st64(c.r64(7), c.r64(0)) { return RegfileExit::fault(0x7000935333); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 2; continue;
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093533a); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935341); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000935348); };
            c.w32(0, t14);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x79u64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x7000935358);
        }
        4 => {
            let r15 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r15);
            let r16 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r16);
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x7000935360); };
            c.w32(11, t17);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000935367); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700093536b); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x700093536f); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x7000935372); }
            c.cmp(c.r64(2), 0x10u64, 64);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(3, 0x10u64);
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000935380); };
            c.w64(6, t18);
            blk = 6; continue;
        }
        6 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000935387); }
            let r19 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r19);
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_ne() { blk = 6; continue; }
            blk = 7; continue;
        }
        7 => {
            let r20 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r20);
            let Some(t21) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935398); };
            c.w32(3, t21);
            let Some(t22) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700093539d); };
            c.w64(1, t22);
            c.w64(0, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009353a6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009353aa); }
            let r23 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x70009353b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70009353b4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70009353b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009353bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70009353c0); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70009353c4); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70009353c8); }
            return c.call(0x70007febf4, 0x70009353d1, 0x70009353cc);
        }
        9 => {
            c.w64(7, c.r64(0));
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009353d4); };
            c.w64(12, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009353d8); };
            c.w64(15, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009353dc); };
            c.w64(9, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009353e0); };
            c.w64(2, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009353e4); };
            c.w64(6, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009353e8); };
            c.w64(3, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009353ec); };
            c.w64(8, t30);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70009353f3); }
            blk = 2; continue;
        }
        10 => {
            return RegfileExit::jump(0x70009353fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935414(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000935414); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935419); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            c.w64(7, c.r64(5));
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000935424); };
            c.w32(8, t3);
            let Some(t4) = c.ld32(c.r64(6).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935429); };
            c.w32(3, t4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700093542d); };
            c.w64(6, t5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000935431); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000935438); };
            c.w64(11, t7);
            c.w64(2, c.r64(8).wrapping_mul(4).wrapping_add(0x10));
            c.w64(0, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000935447); };
            c.w64(1, t8);
            let Some(t9) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700093544e); };
            c.cmp(t9, c.r64(0), 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.st64(c.r64(7), c.r64(0)) { return RegfileExit::fault(0x7000935453); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 2; continue;
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093545a); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935461); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000935468); };
            c.w32(0, t14);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x7bu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x7000935478);
        }
        4 => {
            let r15 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r15);
            let r16 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r16);
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0x1f3)) else { return RegfileExit::fault(0x7000935480); };
            c.w32(11, t17);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000935487); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700093548b); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x700093548f); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x7000935492); }
            c.cmp(c.r64(2), 0x10u64, 64);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(3, 0x10u64);
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009354a0); };
            c.w64(6, t18);
            blk = 6; continue;
        }
        6 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70009354a7); }
            let r19 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r19);
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_ne() { blk = 6; continue; }
            blk = 7; continue;
        }
        7 => {
            let r20 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r20);
            let Some(t21) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009354b8); };
            c.w32(3, t21);
            let Some(t22) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009354bd); };
            c.w64(1, t22);
            c.w64(0, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009354c6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009354ca); }
            let r23 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x70009354d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70009354d4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70009354d8); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009354dc); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70009354e0); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70009354e4); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70009354e8); }
            return c.call(0x70007febf4, 0x70009354f1, 0x70009354ec);
        }
        9 => {
            c.w64(7, c.r64(0));
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009354f4); };
            c.w64(12, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009354f8); };
            c.w64(15, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009354fc); };
            c.w64(9, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000935500); };
            c.w64(2, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000935504); };
            c.w64(6, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000935508); };
            c.w64(3, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700093550c); };
            c.w64(8, t30);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000935513); }
            blk = 2; continue;
        }
        10 => {
            return RegfileExit::jump(0x700093551c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935534(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000935534); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000935538); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000935542); };
            c.w64(7, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000935548); }
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700093554c); };
            c.w64(2, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000935551); };
            c.w32(8, t4);
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935556); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700093555e); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000935562); };
            c.w64(0, t8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935566); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700093556e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700093556f); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000935571); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2718)) else { return RegfileExit::fault(0x7000935575); };
            c.w64(3, t11);
            c.w64(6, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x700093558c, 0x7000935587);
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093558c); };
            c.w64(2, t12);
            let Some(t13) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000935590); };
            c.w64(7, t13);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000935597); };
            c.w64(2, t14);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7).wrapping_add(0xa));
            let Some(t15) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70009355a2); };
            c.w32(7, t15);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x70009355a7); };
            c.w64(15, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70009355ae); };
            c.w64(1, t17);
            let Some(t18) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009355b2); };
            c.w64(5, t18);
            c.w64(12, c.r64(2));
            let r19 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r19);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70009355c0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009355d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009355d7); };
            c.w64(7, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009355db); };
            c.w32(8, t1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009355e0); };
            c.w32(11, (t2 as u8 as i8 as i64 as u64));
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009355e6); };
            c.cmp(t3, c.r32(11), 32);
            if c.cc_l() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009355ed); };
            c.w64(7, t4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009355f1); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            c.w32(3, c.r32(8));
            let Some(t7) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009355fb); };
            c.w32(1, t7);
            let r8 = c.sub(c.r32(1), 0x9u64, 32);
            c.w32(1, r8);
            let r9 = c.sub(c.r32(1), c.r32(3), 32);
            c.w32(1, r9);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_l() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700093560d); }
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000935611); };
            c.w64(2, t10);
            let r11 = c.sub(c.r64(9), c.r64(8), 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000935618); };
            c.w32(7, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700093561d); };
            c.w64(1, t13);
            c.w64(12, c.r64(2));
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000935626); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700093562a); }
            let r14 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r14);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r15 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r15);
            let Some(t16) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935637); };
            c.w64(3, t16);
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700093563b); };
            c.w32(3, t17);
            let r18 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r18);
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000935641); };
            c.w32(3, t19);
            let r20 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r20);
            let Some(t21) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000935647); };
            c.w32(3, t21);
            let r22 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r22);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700093564d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000935651); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x7000935655); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000935659); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700093565d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000935661); }
            let Some(t23) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935665); };
            c.cmp(t23, 0x5eu64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(3, c.r32(8));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000935673); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000935677); }
            c.w64(6, c.r64(7));
            return c.call(0x70007fcf54, 0x7000935683, 0x700093567e);
        }
        6 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000935683); };
            c.w64(2, t24);
            let Some(t25) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935687); };
            c.w64(7, t25);
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700093568b); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935692); };
            c.w32(11, t28);
            let r29 = c.sub(c.r32(11), 0x9u64, 32);
            c.w32(11, r29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700093569a); };
            c.w64(9, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700093569e); };
            let r32 = c.sub(c.r64(9), t31, 64);
            c.w64(9, r32);
            let Some(t33) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009356a2); };
            let r34 = c.sub(c.r32(11), t33, 32);
            c.w32(11, r34);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70009356a6); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ge() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t35) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009356b0); };
            c.w64(8, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009356b4); };
            c.w64(11, t36);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009356b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009356bc); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2148)) else { return RegfileExit::fault(0x70009356bd); };
            c.w64(3, t37);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086ed74, 0x70009356d1, 0x70009356cc);
        }
        8 => {
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009356d1); };
            c.w64(2, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009356d5); };
            c.w64(9, t39);
            blk = 10; continue;
        }
        9 => {
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70009356db); }
            blk = 10; continue;
        }
        10 => {
            let Some(t40) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009356df); };
            c.w64(2, t40);
            let Some(t41) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70009356e3); };
            c.w32(7, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009356e8); };
            c.w64(15, t42);
            let Some(t43) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70009356ec); };
            c.w64(1, t43);
            let Some(t44) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009356f0); };
            c.w64(5, t44);
            c.w64(12, c.r64(2));
            let Some(t45) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70009356f7); };
            c.w64(0, t45);
            let r46 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r46);
            return RegfileExit::jump(c.r64(1));
        }
        11 => {
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000935702); }
            let Some(t47) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000935706); };
            c.w64(5, t47);
            c.w64(6, c.r64(7));
            let r48 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r48);
            return RegfileExit::jump(0x70007fd494);
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000935716); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700093571a); }
            let r49 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r49);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r50 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r50);
            let Some(t51) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000935727); };
            c.w64(3, t51);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700093572b); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700093572f); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000935730); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000935734); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x7000935738); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700093573c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000935740); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x1u64);
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2148)) else { return RegfileExit::fault(0x700093574c); };
            c.w64(3, t52);
            c.w64(7, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700093575e, 0x7000935759);
        }
        13 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093575e); };
            c.w64(0, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000935762); };
            c.w64(15, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000935766); };
            c.w64(2, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700093576a); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700093576e); };
            c.w64(8, t57);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000935775); }
            blk = 3; continue;
        }
        14 => {
            return RegfileExit::jump(0x700093577c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935794(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935797); };
            c.w64(3, t0);
            let Some(t1) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700093579b); };
            c.w32(3, t1);
            let r2 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009357a1); };
            c.w32(2, t3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009357a6); };
            c.w32(6, t4);
            c.w32(6, c.r64(2).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70009357ad); }
            let r5 = c.add(c.r64(9), c.r64(2), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009357b3); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009357b8); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x70009357c0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009357d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009357d7); };
            c.w32(2, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009357dc); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let Some(t3) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009357e4); };
            c.w64(3, t3);
            let Some(t4) = c.ld32(c.r64(6).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009357e8); };
            c.w32(2, t4);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009357ef); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009357f8); };
            c.w32(6, t7);
            c.w32(6, c.r64(2).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70009357ff); }
            let r8 = c.add(c.r64(9), c.r64(2), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935805); };
            c.w32(3, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700093580a); };
            c.w64(1, t10);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000935814);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935834(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000935834); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935839); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935841); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000935845); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935852); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935857); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935860); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935867); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935870); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000935877); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700093587d); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935882); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x700093588c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935894(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000935894); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935899); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009358a1); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70009358a5); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009358b2); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009358b7); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009358c0); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009358c7); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009358d0); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70009358d7); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009358dd); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009358e2); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x70009358ec);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009358f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009358f4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009358f9); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935901); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000935905); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935912); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935917); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935920); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935927); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935930); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000935937); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700093593d); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935942); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x700093594c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935954(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000935954); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935959); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935961); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000935965); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935972); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935977); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935980); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935987); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935990); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000935997); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700093599d); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009359a2); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x70009359ac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009359b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009359b4); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70009359bd); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let r2 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009359ca); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009359cf); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            c.w64(3, c.r64(5));
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009359d8); };
            c.w32(2, t5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009359dd); };
            c.w32(6, t6);
            let r7 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r7);
            let Some(t8) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009359e5); };
            c.w64(3, t8);
            let Some(t9) = c.ld32(c.r64(6).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009359e9); };
            c.w32(2, t9);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t10) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009359f0); };
            c.w32(3, t10);
            let r11 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r11);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let Some(t12) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009359f9); };
            c.w32(6, t12);
            c.w32(6, c.r64(2).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000935a00); }
            let r13 = c.add(c.r64(9), c.r64(2), 64);
            c.w64(9, r13);
            let Some(t14) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935a06); };
            c.w32(3, t14);
            let Some(t15) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935a0b); };
            c.w64(1, t15);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000935a14);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935a34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000935a34); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935a39); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935a41); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000935a45); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935a52); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935a57); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935a60); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935a67); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935a70); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000935a77); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935a7d); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935a82); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000935a8c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935a94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000935a94); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935a99); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935aa1); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000935aa5); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r5 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935ab2); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935ab7); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935ac0); };
            c.w64(2, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935ac7); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935ad0); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000935ad7); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935add); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935ae2); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000935aec);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935af4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000935af4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935af9); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935b01); };
            c.w32(3, t3);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000935b09); };
            c.w32(2, t4);
            let r5 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r5);
            let Some(t6) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935b0f); };
            c.cmp(t6, 0xbau64, 16);
            if c.cc_b() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t7) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935b1a); };
            c.w64(2, t7);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935b21); };
            c.w32(2, t8);
            let r9 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r9);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935b2a); };
            c.w32(6, t10);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000935b31); }
            let r11 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935b37); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935b3c); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            let r14 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r14);
            let Some(t15) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935b46); };
            c.w32(3, t15);
            let Some(t16) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935b4b); };
            c.w64(1, t16);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000935b54);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935b74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000935b74); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935b79); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935b81); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000935b85); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000935b94); };
            c.cmp(t5, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000935b9d); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000935ba3); };
            c.logic(t8 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000935ba9); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935bb2); };
            c.cmp(t10, 0x41u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 8; continue;
        }
        7 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(2, c.r64(5));
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935bc3); };
            c.w64(2, t11);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935bca); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935bd3); };
            c.w32(6, t14);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000935bda); }
            let r15 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935be0); };
            c.w32(3, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935be5); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            let r18 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r18);
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935bef); };
            c.w32(3, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935bf4); };
            c.w64(1, t20);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000935bfa); };
            c.logic(t21 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 8; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000935c05); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            return RegfileExit::jump(0x7000935c18);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935c34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000935c34); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935c39); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935c41); };
            c.w32(3, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000935c45); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000935c54); };
            c.cmp(t5, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000935c5d); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000935c63); };
            c.logic(t8 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000935c69); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935c72); };
            c.cmp(t10, 0x41u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 8; continue;
        }
        7 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let r11 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935c84); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935c89); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            c.w64(2, c.r64(5));
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935c92); };
            c.w64(2, t14);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t15) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935c99); };
            c.w32(2, t15);
            let r16 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r16);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935ca2); };
            c.w32(6, t17);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000935ca9); }
            let r18 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r18);
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935caf); };
            c.w32(3, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935cb4); };
            c.w64(1, t20);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000935cba); };
            c.logic(t21 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 8; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000935cc5); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            return RegfileExit::jump(0x7000935cd8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935cf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000935cf4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000935cf9); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000935d08); };
            c.cmp(t2, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000935d11); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000935d17); };
            c.logic(t5 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000935d1d); };
            c.cmp(t6, c.r32(2), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935d26); };
            c.cmp(t7, 0x41u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 8; continue;
        }
        7 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(2, c.r64(5));
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935d37); };
            c.w64(2, t8);
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935d3b); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935d41); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000935d48); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935d4e); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935d53); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            let r15 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r15);
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935d5d); };
            c.w32(3, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935d62); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000935d68); };
            c.logic(t18 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 8; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000935d73); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            return RegfileExit::jump(0x7000935d88);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935d94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000935d94); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000935d99); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000935da8); };
            c.cmp(t2, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000935db1); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000935db7); };
            c.logic(t5 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000935dbd); };
            c.cmp(t6, c.r32(2), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935dc6); };
            c.cmp(t7, 0x41u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 8; continue;
        }
        7 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let r8 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r8);
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935dd8); };
            c.w32(3, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935ddd); };
            c.w64(1, t10);
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            c.w64(2, c.r64(5));
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935de6); };
            c.w64(2, t11);
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935dea); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935df0); };
            c.w32(6, t14);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000935df7); }
            let r15 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935dfd); };
            c.w32(3, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935e02); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000935e08); };
            c.logic(t18 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 8; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000935e13); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            return RegfileExit::jump(0x7000935e28);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935e34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000935e34); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000935e39); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935e46); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935e4b); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935e54); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935e58); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935e5e); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000935e65); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935e6b); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935e70); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000935e78);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935e94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000935e94); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000935e99); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935ea6); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935eab); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935eb4); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935eb8); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935ebe); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000935ec5); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935ecb); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935ed0); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000935ed8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935ef4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000935ef4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000935ef9); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935f06); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935f0b); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935f14); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935f18); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935f1e); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000935f25); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935f2b); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935f30); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000935f38);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935f54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000935f54); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000935f59); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935f66); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935f6b); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935f74); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935f78); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935f7e); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000935f85); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935f8b); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935f90); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000935f98);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000935fb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000935fb4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000935fb9); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935fc6); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935fcb); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000935fd4); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000935fd8); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000935fde); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000935fe5); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000935feb); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000935ff0); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000935ff8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000936014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000936014); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000936019); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r2 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000936026); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700093602b); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000936034); };
            c.w64(2, t5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000936038); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093603e); };
            c.w32(6, t8);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000936045); }
            let r9 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700093604b); };
            c.w32(3, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936050); };
            c.w64(1, t11);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000936058);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000936074(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000936074); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700093607d); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let r2 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700093608a); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700093608f); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            c.w64(3, c.r64(5));
            let Some(t5) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000936098); };
            c.w64(3, t5);
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700093609c); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009360a2); };
            c.w32(2, t8);
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009360a7); };
            c.w32(6, t9);
            c.w32(6, c.r64(2).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70009360ae); }
            let r10 = c.add(c.r64(9), c.r64(2), 64);
            c.w64(9, r10);
            let Some(t11) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009360b4); };
            c.w32(3, t11);
            let Some(t12) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009360b9); };
            c.w64(1, t12);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70009360c0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009360d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009360d4); };
            c.w32(3, t0);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009360dd); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009360e3); };
            c.cmp(t3, 0xbau64, 16);
            if c.cc_b() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(2, c.r64(5));
            let Some(t4) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009360ee); };
            c.w64(2, t4);
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009360f2); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009360f8); };
            c.w32(6, t7);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x5));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70009360ff); }
            let r8 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000936105); };
            c.w32(3, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700093610a); };
            c.w64(1, t10);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            let r11 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000936114); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936119); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000936120);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000936134(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000936137); };
            c.w64(2, (t0 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700093613f); };
            c.w32(6, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000936144); };
            c.w32(7, t2);
            let r3 = c.sub(c.r64(3), c.r64(2), 64);
            c.w64(3, r3);
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_l() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_le() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let r4 = c.add(c.r64(3), c.r64(6), 64);
            c.w64(3, r4);
            c.w64(2, c.r64(5));
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093615d); };
            c.w32(6, t5);
            let r6 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000936165); };
            c.w64(2, t7);
            let Some(t8) = c.ld32(c.r64(6).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000936169); };
            c.w32(3, t8);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000936170); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000936179); };
            c.w32(6, t11);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0xd));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000936180); }
            let r12 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000936186); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700093618b); };
            c.w64(1, t14);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            let r15 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r15);
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000936195); };
            c.w32(3, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700093619a); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70009361a4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009361b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009361b4); };
            c.w64(2, (t0 as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(5));
            let Some(t1) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70009361bc); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009361c0); };
            c.w32(0, t2);
            let r3 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r3);
            let Some(t4) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009361c6); };
            c.w64(8, t4);
            let Some(t5) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009361ca); };
            c.w32(11, t5);
            c.w64(3, c.r64(11));
            let r6 = c.r64(3) & 0x3ffu64;
            c.w64(3, r6);
            c.logic(r6, 64);
            c.cmp(c.r64(3), 0x3ffu64, 64);
            if c.cc_e() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(11, c.r64(0));
            c.w64(3, c.r64(2));
            blk = 2; continue;
        }
        2 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009361eb); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70009361ee); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x3f8)) else { return RegfileExit::fault(0x70009361f7); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000936200); };
            c.cmp(t10, 0x834u64, 16);
            if c.cc_ne() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t11) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700093620b); };
            c.cmp(t11, c.r32(1), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000936210); };
            c.w32(3, t12);
            let r13 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000936217); };
            c.cmp(t14, c.r32(3), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t15) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000936220); };
            c.w32(11, t15);
            let r16 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r16);
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000936227); };
            c.w32(1, t17);
            c.logic(c.r16(1) & 0x3ffu64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 12; continue;
        }
        9 => {
            let r18 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r18);
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000936238); };
            c.w32(2, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700093623d); };
            c.w64(1, t20);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000936243); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000936247); }
            let r21 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r21);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700093624d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000936251); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000936255); }
            blk = 26; continue;
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700093625e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000936262); }
            let r22 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r22);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000936268); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700093626c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000936270); }
            blk = 26; continue;
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000936279); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700093627d); }
            let r23 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000936283); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000936287); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700093628b); }
            blk = 26; continue;
        }
        13 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000936294); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000936298); }
            let r24 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r24);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700093629e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009362a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009362a6); }
            c.logic(c.r32(11) & 0x200000u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009362b7); };
            c.w32(11, t25);
            let r26 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r26);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70009362c4); };
            c.w64(11, t27);
            blk = 16; continue;
        }
        16 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009362cb); };
            c.w32(3, t28);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x70009362db);
        }
        18 => {
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009362de); };
            c.cmp(t29, c.r32(3), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, c.r64(2));
            blk = 20; continue;
        }
        20 => {
            let Some(t30) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009362e7); };
            c.w32(3, t30);
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70009362eb); };
            c.cmp(t31, c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x3f8)) else { return RegfileExit::fault(0x70009362f4); };
            c.cmp(t32, c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t33) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009362fd); };
            c.cmp(t33, 0x834u64, 16);
            if c.cc_ne() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t34) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000936307); };
            c.cmp(t34, c.r32(3), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t35) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700093630d); };
            c.w32(11, t35);
            let r36 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r36);
            let Some(t37) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000936314); };
            c.cmp(t37, c.r32(11), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t38) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093631d); };
            c.w32(0, t38);
            let r39 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r39);
            let Some(t40) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000936324); };
            c.w32(3, t40);
            c.logic(c.r16(3) & 0x3ffu64, 16);
            if c.cc_e() { blk = 20; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r41 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r41);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000936335); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000936339); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1f50)) else { return RegfileExit::fault(0x700093633a); };
            c.w64(3, t42);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086ed74, 0x700093634e, 0x7000936349);
        }
        27 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700093634e); };
            c.w64(7, t43);
            let Some(t44) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000936352); };
            c.w64(2, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000936356); };
            c.w64(9, t45);
            let r46 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r46);
            let Some(t47) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700093635e); };
            c.w32(7, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000936363); };
            c.w64(15, t48);
            let Some(t49) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936367); };
            c.w64(1, t49);
            let Some(t50) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700093636b); };
            c.w64(5, t50);
            c.w64(12, c.r64(2));
            let r51 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r51);
            return RegfileExit::jump(c.r64(1));
        }
        28 => {
            let r52 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r52);
            let Some(t53) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700093637c); };
            c.w32(2, t53);
            let Some(t54) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936381); };
            c.w64(1, t54);
            let Some(t55) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000936385); };
            c.w64(5, t55);
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000936389); };
            c.w64(0, t56);
            let r57 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r57);
            return RegfileExit::jump(c.r64(1));
        }
        29 => {
            return RegfileExit::jump(0x7000936398);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009363b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009363b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009363b8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009363be); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009363c2); };
            c.w64(2, t2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009363c6); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009363cc); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009363d2); };
            c.w32(6, t7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009363d7); };
            c.w32(7, t8);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009363da); }
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009363de); };
            c.cmp(t9, c.r32(7), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009363e7); };
            c.w64(2, t10);
            blk = 2; continue;
        }
        2 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009363ee); };
            c.w32(7, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009363f1); };
            c.cmp(t12, c.r32(2), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000936400); };
            c.cmp(t13, c.r32(7), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x700093640d); };
            c.w32(7, t14);
            let r15 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r15);
            let Some(t16) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000936413); };
            c.w32(7, t16);
            let r17 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r17);
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000936419); };
            c.w32(11, t18);
            let r19 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r19);
            let Some(t20) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000936420); };
            c.w32(11, t20);
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000936424); };
            c.w32(1, t21);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t22) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700093642a); };
            c.w32(7, t22);
            let r23 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r23);
            let r24 = c.r64(1) & 0x3ffu64;
            c.w64(1, r24);
            c.logic(r24, 64);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(1), c.r64(11), 64);
            if c.cc_le() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(11, 0x6u64);
            blk = 6; continue;
        }
        5 => {
            c.w32(11, 0x2u64);
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000936453); };
            c.w32(8, t25);
            c.w32(11, c.r32(11));
            c.w32(15, c.r32(8));
            let r26 = c.r32(11) | c.r32(15);
            c.w32(11, r26);
            c.logic(r26, 32);
            c.cmp(c.r32(8), c.r32(11), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x7000936469); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700093646e); }
            blk = 9; continue;
        }
        9 => {
            c.w32(2, c.r64(1).wrapping_add(c.r64(1)));
            blk = 10; continue;
        }
        10 => {
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000936478); };
            c.w64(6, (t27 as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x700093647d); }
            let r28 = c.sub(c.r64(6), 0x1u64, 64);
            c.w64(6, r28);
            if !c.st64(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(8)), c.r64(7)) { return RegfileExit::fault(0x7000936485); }
            let r29 = c.sub(c.r64(6), 0x1u64, 64);
            c.w64(6, r29);
            if !c.st64(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(8)), c.r64(2)) { return RegfileExit::fault(0x700093648d); }
            let r30 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r30);
            let Some(t31) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000936495); };
            c.w32(3, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093649a); };
            c.w64(15, t32);
            let Some(t33) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700093649e); };
            c.w64(1, t33);
            let Some(t34) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009364a2); };
            c.w64(5, t34);
            let r35 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r35);
            return RegfileExit::jump(c.r64(1));
        }
        11 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t36) = c.ld32(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009364b2); };
            c.w32(7, t36);
            c.w32(8, c.r32(7));
            let r37 = c.r32(8) | 0xeu64;
            c.w32(8, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x70009364c5); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009364c9); }
            blk = 14; continue;
        }
        14 => {
            let Some(t38) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009364d0); };
            c.w32(2, t38);
            c.w64(7, c.r64(0));
            blk = 10; continue;
        }
        15 => {
            return RegfileExit::jump(0x70009364dc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009364f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009364f4); };
            c.w64(3, (t0 as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009364fc); };
            c.w64(6, (t1 as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936501); };
            c.w64(3, t2);
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936505); };
            c.w64(2, t3);
            c.cmp(c.r32(2), c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700093650d); };
            c.w64(3, t4);
            blk = 3; continue;
        }
        2 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000936516); };
            c.w64(3, t5);
            blk = 3; continue;
        }
        3 => {
            let r6 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r6);
            let Some(t7) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000936521); };
            c.w32(2, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936526); };
            c.w64(1, t8);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000936530);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000936534(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000936534); };
            c.w64(2, (t0 as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(5));
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700093653c); };
            c.w64(8, (t1 as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000936541); };
            c.w64(11, (t2 as u32 as i32 as i64 as u64));
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936546); };
            c.w64(2, t3);
            let Some(t4) = c.ld64(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700093654a); };
            c.w64(8, t4);
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000936552); };
            c.w64(3, t5);
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000936556); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld64(c.r64(7).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700093655c); };
            c.w64(0, t8);
            let Some(t9) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936560); };
            c.w64(11, t9);
            let Some(t10) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000936564); };
            c.w32(3, t10);
            let r11 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r11);
            let Some(t12) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0xd)) else { return RegfileExit::fault(0x700093656a); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700093656f); };
            c.w32(6, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000936572); };
            c.cmp(t14, c.r32(6), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700093657b); };
            c.w64(3, t15);
            blk = 2; continue;
        }
        2 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000936582); };
            c.w32(6, t16);
            c.cmp(c.r32(8), c.r32(6), 32);
            if c.cc_ae() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t17) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093658d); };
            c.w32(0, t17);
            let r18 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r18);
            c.w64(8, c.r64(9).wrapping_add(0x11));
            let Some(t19) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000936599); };
            c.cmp(t19, c.r32(11), 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t20) = c.ld8(c.r64(8).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700093659f); };
            c.w32(2, t20);
            let Some(t21) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70009365a4); };
            c.w64(1, t21);
            c.w64(9, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        5 => {
            return RegfileExit::fault(0x70009365ad);
        }
        6 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009365ae); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009365b2); }
            let r22 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r22);
            let Some(t23) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009365b8); };
            c.w64(11, t23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70009365bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70009365c0); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009365c4); }
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009365c8); };
            c.cmp(t24, c.r32(3), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t25) = c.ld32(c.r64(3).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009365d1); };
            c.w32(6, t25);
            c.w32(12, c.r32(6));
            let r26 = c.r32(12) | 0xeu64;
            c.w32(12, r26);
            c.logic(r26, 32);
            c.cmp(c.r32(6), c.r32(12), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(6, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(6)) { return RegfileExit::fault(0x70009365e4); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009365e8); }
            blk = 9; continue;
        }
        9 => {
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let r27 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r27);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70009365f5); }
            c.w64(6, c.r64(11));
            c.w64(3, c.r64(2));
            return c.call(0x7000857fb4, 0x7000936604, 0x70009365ff);
        }
        10 => {
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000936604); };
            c.w64(7, t28);
            let Some(t29) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000936608); };
            c.w64(2, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093660c); };
            c.w64(8, t30);
            let Some(t31) = c.ld8(c.r64(8).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000936610); };
            c.w32(7, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000936615); };
            c.w64(15, t32);
            let Some(t33) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936619); };
            c.w64(1, t33);
            let Some(t34) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700093661d); };
            c.w64(5, t34);
            c.w64(12, c.r64(2));
            c.w64(9, c.r64(8));
            let r35 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r35);
            return RegfileExit::jump(c.r64(1));
        }
        11 => {
            return RegfileExit::jump(0x7000936630);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000936634(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000936634); };
            c.w64(3, (t0 as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700093663c); };
            c.w64(3, t1);
            let r2 = c.add(c.r32(3), 0x2u64, 32);
            c.w32(3, r2);
            c.w64(0, (c.r32(3) as u32 as i32 as i64 as u64));
            let r3 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r3);
            let Some(t4) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700093664a); };
            c.w32(3, t4);
            let Some(t5) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700093664f); };
            c.w64(1, t5);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000936658);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000936674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000936674); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r1 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r1);
            let Some(t2) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000936681); };
            c.w32(2, t2);
            let Some(t3) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936686); };
            c.w64(1, t3);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700093668c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000936690); }
            let r4 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r4);
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000936696); };
            c.w32(2, t5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093669b); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009366a3); };
            c.w64(8, t8);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009366ab); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            let Some(t11) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009366b2); };
            c.w64(7, t11);
            let r12 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r12);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009366b9); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009366bd); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70009366be); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70009366c2); }
            c.w32(0, 0x1u64);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2228)) else { return RegfileExit::fault(0x70009366cb); };
            c.w64(3, t13);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70009366da, 0x70009366d5);
        }
        3 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009366da); };
            c.w64(2, t14);
            let Some(t15) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009366de); };
            c.w64(7, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009366e2); };
            c.w64(8, t16);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009366e6); }
            c.w32(2, 0x62u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009366ef); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x70009366f0); };
            c.w64(3, t17);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000936704, 0x70009366ff);
        }
        4 => {
            return RegfileExit::fault(0x7000936704);
        }
        5 => {
            return RegfileExit::jump(0x7000936708);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000936714(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000936714); };
            c.w64(2, (t0 as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(5));
            let Some(t1) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700093671c); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000936720); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000936724); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700093672b); };
            c.logic(t5 & 0x40u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let r6 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r6);
            let Some(t7) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000936736); };
            c.w32(2, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700093673b); };
            c.w64(1, t8);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000936741); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000936745); }
            let r9 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r9);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t10) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093674f); };
            c.w64(0, t10);
            let r11 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r11);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000936756); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700093675a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700093675b); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x700093675c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x7000936760); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x1da8)) else { return RegfileExit::fault(0x7000936764); };
            c.w64(3, t12);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086ed74, 0x7000936778, 0x7000936773);
        }
        3 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000936778); };
            c.w64(7, t13);
            let Some(t14) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700093677c); };
            c.w64(2, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000936780); };
            c.w64(8, t15);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000936784); }
            c.w32(7, 0x62u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700093678d); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x700093678e); };
            c.w64(3, t16);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70009367a2, 0x700093679d);
        }
        4 => {
            return RegfileExit::fault(0x70009367a2);
        }
        5 => {
            return RegfileExit::jump(0x70009367a4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009367b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009367b4); };
            c.w64(3, (t0 as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009367bc); };
            c.w64(3, t1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009367c0); };
            c.cmp(t2, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009367c9); };
            c.w32(6, t3);
            c.w64(7, 0xfffffffffffffffcu64);
            if !c.st32(c.r64(3).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x70009367d3); }
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009367d6); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            if !c.st64(c.r64(2).wrapping_sub(0x8), c.r64(3)) { return RegfileExit::fault(0x70009367dc); }
            c.w64(3, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009367e3); };
            c.w32(6, t6);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r7 = c.add(c.r64(3), c.r64(6), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009367ee); };
            c.w32(6, t8);
            let r9 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r9);
            let Some(t10) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009367f6); };
            c.w64(2, t10);
            let Some(t11) = c.ld32(c.r64(6).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009367fa); };
            c.w32(3, t11);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000936801); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700093680a); };
            c.w32(6, t14);
            c.w32(6, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0xd));
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x7000936811); }
            let r15 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000936817); };
            c.w32(3, t16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700093681c); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            let r18 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r18);
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000936826); };
            c.w32(3, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700093682b); };
            c.w64(1, t20);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x7000936834);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000936854(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000936854); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000936858); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000936862); };
            c.w64(1, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000936868); }
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700093686c); };
            c.w64(7, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000936871); };
            c.w64(8, t4);
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936875); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000936879); };
            c.w64(11, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700093687d); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000936884); };
            c.w32(15, t9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0xd)) else { return RegfileExit::fault(0x7000936889); };
            c.w32(12, t10);
            let Some(t11) = c.ld32(c.r64(7).wrapping_add(0x23)) else { return RegfileExit::fault(0x700093688e); };
            c.w32(3, t11);
            let r12 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r12);
            let Some(t13) = c.ld16(c.r64(8).wrapping_add(0x15)) else { return RegfileExit::fault(0x7000936894); };
            c.w32(8, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000936899); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700093689d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70009368a1); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009368a5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009368a9); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(0));
            blk = 4; continue;
        }
        1 => {
            c.w64(7, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(0).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(c.r64(0)), c.r32(2)) { return RegfileExit::fault(0x70009368c0); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r14 = c.r64(11) & c.r64(3);
            c.w64(11, r14);
            c.logic(r14, 64);
            let Some(t15) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009368d2); };
            c.logic(t15 & 0x4u64, 8);
            if c.cc_ne() { blk = 27; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(0, c.r64(7));
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(0), c.r64(8), 64);
            if c.cc_ae() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(2, c.r64(0).wrapping_add(0x3));
            let Some(t16) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009368e9); };
            c.w32(7, t16);
            let Some(t17) = c.ld64(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70009368ec); };
            c.w64(2, t17);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(0), c.r64(7), 64);
            if c.cc_b() { blk = 1; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 20; continue;
        }
        7 => {
            let r18 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r18);
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(c.r64(7)), c.r32(2)) { return RegfileExit::fault(0x700093690c); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r19 = c.r64(9) & c.r64(3);
            c.w64(9, r19);
            c.logic(r19, 64);
            let Some(t20) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700093691e); };
            c.logic(t20 & 0x4u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(6), c.r64(15), 64);
            if c.cc_ae() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(2, 0xfffffffffffffffau64);
            let r21 = c.sub(c.r64(2), c.r64(6), 64);
            c.w64(2, r21);
            let Some(t22) = c.ld64(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936938); };
            c.w64(2, t22);
            c.w64(7, c.r64(8).wrapping_add(c.r64(6)));
            let Some(t23) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000936940); };
            c.w32(9, t23);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_b() { blk = 7; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 19; continue;
        }
        12 => {
            c.w32(2, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000936958); };
            c.w64(7, t24);
            c.w32(8, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000936960); };
            c.w64(9, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000936964); };
            c.w64(11, t26);
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x7000936968); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r27 = c.r64(12) & c.r64(9);
            c.w64(12, r27);
            c.logic(r27, 64);
            let Some(t28) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700093697c); };
            c.logic(t28 & 0x4u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st32(c.r64(9).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000936984); }
            if !c.st32(c.r64(9).wrapping_add(0x17), c.r32(8)) { return RegfileExit::fault(0x7000936988); }
            let Some(t29) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700093698c); };
            c.w64(2, t29);
            let Some(t30) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000936990); };
            c.w32(8, t30);
            let r31 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000936997); };
            c.w32(9, t32);
            let r33 = c.sub(c.r32(7), 0x21u64, 32);
            c.w32(7, r33);
            let r34 = c.sub(c.r32(9), 0x11u64, 32);
            c.w32(9, r34);
            let r35 = c.sub(c.r32(9), c.r32(7), 32);
            c.w32(9, r35);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70009369ab); }
            blk = 16; continue;
        }
        16 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009369af); };
            c.w64(0, t36);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70009369b6); }
            return c.ret(0x0, 0x70009369b7);
        }
        17 => {
            let Some(t37) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009369b8); };
            c.w64(7, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009369bc); };
            c.w64(8, t38);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009369c0); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009369c4); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2140)) else { return RegfileExit::fault(0x70009369c5); };
            c.w64(3, t39);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70009369d9, 0x70009369d4);
        }
        18 => {
            blk = 16; continue;
        }
        19 => {
            return RegfileExit::fault(0x70009369db);
        }
        20 => {
            return RegfileExit::fault(0x70009369dc);
        }
        21 => {
            c.w32(11, c.r32(11));
            let r40 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r40);
            c.w64(12, 0xfffffffffffc0000u64);
            let r41 = c.r64(12) & c.r64(11);
            c.w64(12, r41);
            c.logic(r41, 64);
            let Some(t42) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009369ed); };
            c.logic(t42 & 0x2u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(15, c.r64(9).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009369f9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009369fa); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000936a07, 0x7000936a02);
        }
        23 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000936a07); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000936a09); }
            blk = 14; continue;
        }
        24 => {
            c.w32(2, c.r32(2));
            let r43 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r43);
            c.w64(9, 0xfffffffffffc0000u64);
            let r44 = c.r64(9) & c.r64(2);
            c.w64(9, r44);
            c.logic(r44, 64);
            let Some(t45) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000936a1e); };
            c.logic(t45 & 0x2u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(3).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000936a2d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000936a2e); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000936a3b, 0x7000936a36);
        }
        26 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000936a3b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000936a3d); }
            blk = 9; continue;
        }
        27 => {
            c.w32(2, c.r32(2));
            let r46 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r46);
            c.w64(11, 0xfffffffffffc0000u64);
            let r47 = c.r64(11) & c.r64(2);
            c.w64(11, r47);
            c.logic(r47, 64);
            let Some(t48) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000936a52); };
            c.logic(t48 & 0x2u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(9, c.r64(3).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000936a61); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000936a62); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000936a6f, 0x7000936a6a);
        }
        29 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000936a6f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000936a71); }
            blk = 3; continue;
        }
        30 => {
            return RegfileExit::jump(0x7000936a78);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000936a94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000936a94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000936a98); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000936a9e); };
            c.w64(3, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000936aa2); };
            c.w64(2, (t2 as u32 as i32 as i64 as u64));
            let Some(t3) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000936aa7); };
            c.w64(6, t3);
            let Some(t4) = c.ld64(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936aab); };
            c.w64(2, t4);
            let Some(t5) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000936aaf); };
            c.w32(6, t5);
            let r6 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000936ab5); };
            c.w32(7, t7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000936aba); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(6).wrapping_add(0x15)) else { return RegfileExit::fault(0x7000936ac1); };
            c.w32(6, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000936ac5); }
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            let r11 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x198)) else { return RegfileExit::fault(0x7000936ad2); };
            c.w64(1, t12);
            if !c.st32(c.r64(8).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000936ad9); }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(0, c.r64(6).wrapping_add(c.r64(11)));
            let Some(t13) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000936ae7); };
            c.w32(1, t13);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(0), c.r64(1), 64);
            if c.cc_ae() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000936af6); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            c.w64(15, 0xfffffffffffffffau64);
            let r16 = c.sub(c.r64(15), c.r64(11), 64);
            c.w64(15, r16);
            if !c.st64(c.r64(3).wrapping_add(c.r64(15).wrapping_mul(8)), c.r64(1)) { return RegfileExit::fault(0x7000936b08); }
            let Some(t17) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000936b0c); };
            c.w32(1, t17);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(0), c.r64(1), 64);
            if c.cc_b() { blk = 1; continue; }
            blk = 5; continue;
        }
        5 => {
            blk = 8; continue;
        }
        6 => {
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000936b1d); };
            c.w32(0, t18);
            let r19 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r19);
            let r20 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r20);
            let Some(t21) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000936b27); };
            c.w32(3, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000936b2c); };
            c.w64(15, t22);
            let Some(t23) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936b30); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000936b34); };
            c.w64(5, t24);
            let r25 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r25);
            return RegfileExit::jump(c.r64(1));
        }
        7 => {
            return RegfileExit::fault(0x7000936b3e);
        }
        8 => {
            return RegfileExit::fault(0x7000936b3f);
        }
        9 => {
            return RegfileExit::jump(0x7000936b44);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000936b54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000936b54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000936b58); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(7, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000936b62); };
            c.w64(8, t1);
            let r2 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r2);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000936b68); }
            let Some(t3) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000936b6c); };
            c.w64(7, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000936b70); };
            c.w64(11, (t4 as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000936b75); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000936b7b); };
            c.w64(0, t7);
            let Some(t8) = c.ld64(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936b7f); };
            c.w64(11, t8);
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000936b83); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000936b89); };
            c.w32(3, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000936b8c); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000936b90); };
            c.cmp(t12, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000936b99); };
            c.w64(7, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000936ba0); };
            c.w64(3, (t14 as u32 as i32 as i64 as u64));
            let Some(t15) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000936ba5); };
            c.w64(2, (t15 as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(3).wrapping_add(c.r64(3)));
            c.w64(3, c.r64(2).wrapping_add(c.r64(2)));
            c.w64(6, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(2, c.r64(7));
            return c.call(0x70008a8954, 0x7000936bc0, 0x7000936bbb);
        }
        3 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000936bc0); };
            c.w64(7, t16);
            let Some(t17) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000936bc4); };
            c.w64(8, t17);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t18) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000936bcb); };
            c.w64(7, t18);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8).wrapping_add(0xe));
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000936bd6); };
            c.w32(8, t19);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2d70)) else { return RegfileExit::fault(0x7000936bdb); };
            c.w64(15, t20);
            let Some(t21) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936be2); };
            c.w64(1, t21);
            let Some(t22) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000936be6); };
            c.w64(5, t22);
            c.w64(12, c.r64(7));
            let r23 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r23);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000936bf4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000936c14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000936c14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000936c18); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000936c1e); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000936c22); };
            c.w32(7, t2);
            let Some(t3) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000936c27); };
            c.w64(8, t3);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t4) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000936c2f); };
            c.w64(3, t4);
            let r5 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r5);
            let r6 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r6);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000936c38); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000936c3c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000936c40); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000936c44); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000936c48); }
            c.w64(0, c.r64(8));
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(7));
            return c.call(0x70008a86d4, 0x7000936c5a, 0x7000936c55);
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000936c5a); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000936c5e); };
            c.w64(2, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000936c62); };
            c.w64(7, t9);
            c.w64(9, c.r64(7).wrapping_add(0x5));
            let Some(t10) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000936c6a); };
            c.w32(7, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000936c6f); };
            c.w64(15, t11);
            let Some(t12) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000936c73); };
            c.w64(1, t12);
            let Some(t13) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000936c77); };
            c.w64(5, t13);
            c.w64(12, c.r64(2));
            let Some(t14) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000936c7e); };
            c.w64(0, t14);
            let r15 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r15);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000936c8c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000936c94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000936c94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000936c98); }
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000936c9a); };
            c.w32(2, t0);
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r1 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r1);
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000936ca5); };
            c.w64(7, t2);
            let r3 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r3);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000936cab); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000936caf); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000936cb0); };
            c.w64(3, t4);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000936cc3, 0x7000936cbe);
        }
        1 => {
            return RegfileExit::fault(0x7000936cc3);
        }
        2 => {
            return RegfileExit::jump(0x7000936cc8);
        }
            _ => unreachable!(),
        }
    }
}
