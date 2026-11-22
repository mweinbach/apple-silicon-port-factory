#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_700090f194(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700090f194); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700090f198); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090f19e); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090f1a2); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090f1a6); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090f1ad); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f1b4); };
            c.w32(0, t7);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700090f1b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700090f1bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700090f1c0); }
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700090f1c4); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(0, c.r64(11));
            blk = 3; continue;
        }
        2 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090f1d2); };
            c.w64(0, t9);
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700090f1d9); };
            c.w64(3, (t10 as u16 as i16 as i64 as u64));
            let Some(t11) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700090f1df); };
            c.w64(3, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090f1e3); };
            c.w64(11, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090f1ea); };
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
            let Some(t14) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f1fc); };
            c.w32(2, t14);
            let r15 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r15);
            blk = 7; continue;
        }
        6 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090f204); };
            c.w64(2, t16);
            blk = 7; continue;
        }
        7 => {
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090f20b); };
            c.logic(t17 & 0x1000000u64, 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t18) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700090f218); };
            c.w32(1, t18);
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w32(6, c.r32(2));
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700090f226); };
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
            let Some(t23) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f242); };
            c.w32(1, t23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x2c8)) else { return RegfileExit::fault(0x700090f245); };
            c.cmp(t24, c.r32(1), 32);
            if c.cc_ne() { blk = 82; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t25) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090f252); };
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
            let Some(t27) = c.ld32(c.r64(7).wrapping_add(c.r64(6)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f26a); };
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
            let Some(t30) = c.ld32(c.r64(7).wrapping_add(c.r64(6)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090f287); };
            c.w32(11, t30);
            let r31 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r31);
            c.w64(2, c.r64(11));
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090f292); };
            c.w64(15, t32);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090f296); };
            c.w64(11, t33);
            blk = 17; continue;
        }
        16 => {
            let Some(t34) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700090f29f); };
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
            let Some(t38) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090f30e); };
            c.w32(1, t38);
            let r39 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r39);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700090f319); };
            c.w64(1, t40);
            blk = 28; continue;
        }
        28 => {
            let Some(t41) = c.ld32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f320); };
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
            let Some(t43) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f33b); };
            c.w32(2, t43);
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090f33f); };
            c.cmp(t44, c.r32(2), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700090f34c); }
            blk = 74; continue;
        }
        32 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t45) = c.ld32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f35c); };
            c.w32(0, t45);
            let r46 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r46);
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        34 => {
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f36c); };
            c.w32(11, t47);
            let r48 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r48);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 78; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t49) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f37e); };
            c.w32(2, t49);
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090f382); };
            c.cmp(t50, c.r32(2), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700090f38f); }
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
            let Some(t54) = c.ld8(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f42e); };
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
            let Some(t56) = c.ld16(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f44b); };
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
            let Some(t58) = c.ld32(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f468); };
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
            let Some(t61) = c.ld32(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f48f); };
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
            let Some(t63) = c.ld64(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f4b8); };
            c.w64(11, t63);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 68; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090f4c3); };
            c.w64(0, t64);
            let Some(t65) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700090f4ca); };
            c.w64(3, t65);
            c.w64(2, c.r64(3).wrapping_add(0x10));
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090f4d1); };
            c.w64(1, t66);
            let Some(t67) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700090f4d8); };
            c.cmp(t67, c.r64(2), 64);
            if c.cc_be() { blk = 334; continue; }
            blk = 64; continue;
        }
        64 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x700090f4e1); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 65; continue;
        }
        65 => {
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700090f4e8); };
            c.w64(3, t68);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700090f4ef); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_g() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700090f4f8); }
            let r69 = c.neg(c.r64(11), 64);
            c.w64(11, r69);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(11)) { return RegfileExit::fault(0x700090f502); }
            blk = 71; continue;
        }
        67 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700090f508); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(11)) { return RegfileExit::fault(0x700090f50f); }
            blk = 71; continue;
        }
        68 => {
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090f515); };
            c.w64(11, t70);
            let Some(t71) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090f51c); };
            c.w64(0, t71);
            c.w64(2, c.r64(0).wrapping_add(0x8));
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090f523); };
            c.w64(3, t72);
            let Some(t73) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700090f52a); };
            c.cmp(t73, c.r64(2), 64);
            if c.cc_be() { blk = 336; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x700090f533); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 70; continue;
        }
        70 => {
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700090f53a); };
            c.w64(0, t74);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700090f541); }
            if !c.st32(c.r64(11).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700090f545); }
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
            if !c.movss_rm(0, c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x700090f563); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 74; continue;
        }
        73 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r76 = c.r64(6) & 0xfffffu64;
            c.w64(6, r76);
            c.logic(r76, 64);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x700090f57a); }
            blk = 74; continue;
        }
        74 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700090f580); };
            c.w64(11, t77);
            let Some(t78) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700090f587); };
            c.w64(0, t78);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700090f58e); };
            c.w64(2, t79);
            let Some(t80) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700090f595); };
            c.cmp(t80, c.r64(3), 64);
            if c.cc_be() { blk = 338; continue; }
            blk = 75; continue;
        }
        75 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700090f59e); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 76; continue;
        }
        76 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700090f5a5); };
            c.w64(0, t81);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700090f5ac); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700090f5b0); }
            blk = 80; continue;
        }
        77 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r82 = c.r64(6) & 0xfffffu64;
            c.w64(6, r82);
            c.logic(r82, 64);
            let Some(t83) = c.ld32(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f5c3); };
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
            let Some(t86) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090f5d6); };
            c.w64(2, t86);
            let Some(t87) = c.ld16(c.r64(2).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090f5da); };
            c.w32(1, t87);
            let Some(t88) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f5e0); };
            c.w32(6, t88);
            let r89 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r89);
            let Some(t90) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090f5e6); };
            c.w64(7, t90);
            let Some(t91) = c.ld16(c.r64(2).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700090f5ea); };
            c.w32(2, t91);
            let Some(t92) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f5f0); };
            c.w32(1, t92);
            let r93 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r93);
            if let Some(sr) = c.shl(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090f5fa); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090f5fe); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700090f5ff); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700090f600); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090f601); }
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x2050)) else { return RegfileExit::fault(0x700090f602); };
            c.w64(3, t94);
            c.w32(0, 0x4u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700090f616, 0x700090f611);
        }
        79 => {
            c.w64(11, c.r64(0));
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090f619); };
            c.w64(15, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090f61d); };
            c.w64(9, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090f621); };
            c.w64(8, t97);
            blk = 80; continue;
        }
        80 => {
            let Some(t98) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090f625); };
            c.w64(8, t98);
            c.w64(0, c.r64(9));
            c.w64(9, c.r64(0).wrapping_add(0x7));
            let Some(t99) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700090f630); };
            c.w32(0, t99);
            let Some(t100) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700090f635); };
            c.w64(1, t100);
            let Some(t101) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700090f639); };
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
            let Some(t104) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090f652); };
            c.w64(2, t104);
            let Some(t105) = c.ld16(c.r64(2).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090f656); };
            c.w32(1, t105);
            let Some(t106) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f65c); };
            c.w32(6, t106);
            let r107 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r107);
            let Some(t108) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090f662); };
            c.w64(7, t108);
            let Some(t109) = c.ld16(c.r64(2).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700090f666); };
            c.w32(2, t109);
            let Some(t110) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f66c); };
            c.w32(1, t110);
            let r111 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r111);
            if let Some(sr) = c.shl(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700090f676); }
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(10));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(6, c.r64(7));
            return c.call(0x7000832fd4, 0x700090f694, 0x700090f68f);
        }
        83 => {
            c.w64(11, c.r64(0));
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090f697); };
            c.w64(15, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090f69b); };
            c.w64(9, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090f69f); };
            c.w64(8, t114);
            blk = 80; continue;
        }
        84 => {
            let Some(t115) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f6a5); };
            c.w32(1, t115);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700090f6a8); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700090f6ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700090f6b0); }
            let Some(t116) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x700090f6b4); };
            c.cmp(t116, c.r32(1), 32);
            if c.cc_e() { blk = 166; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t117) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f6c1); };
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
            let Some(t119) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090f6cd); };
            c.cmp(t119, c.r32(6), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t120) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090f6d6); };
            c.w32(1, t120);
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700090f6df); }
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
            let Some(t121) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090f6f7); };
            c.w32(7, t121);
            let r122 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r122);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700090f703); };
            c.w64(7, t123);
            blk = 91; continue;
        }
        91 => {
            let Some(t124) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090f70a); };
            c.w32(0, t124);
            let Some(t125) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090f70d); };
            c.w32(1, t125);
            let Some(t126) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f713); };
            c.w32(2, t126);
            let r127 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r127);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t128) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f71e); };
            c.w32(1, t128);
            let r129 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r129);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r130 = c.sub(c.r64(0), 0x1u64, 64);
            c.w64(0, r130);
            let Some(t131) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090f72c); };
            c.w32(15, t131);
            let r132 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r132);
            let r133 = c.r64(15) & c.r64(0);
            c.w64(15, r133);
            c.logic(r133, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x700090f737); }
            blk = 92; continue;
        }
        92 => {
            c.w64(3, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(11, c.r64(3).wrapping_mul(4));
            let Some(t134) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f748); };
            c.w32(2, t134);
            let Some(t135) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090f74d); };
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
            let Some(t138) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090f771); };
            c.w32(0, t138);
            let Some(t139) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090f774); };
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
            return RegfileExit::fault(0x700090f788);
        }
        97 => {
            let Some(t141) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090f789); };
            c.w32(0, t141);
            let Some(t142) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090f78f); };
            c.w64(3, t142);
            let Some(t143) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090f793); };
            c.w32(11, t143);
            let r144 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700090f79b); };
            c.w64(1, t145);
            let Some(t146) = c.ld32(c.r64(1).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f79f); };
            c.w32(0, t146);
            let r147 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r147);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 149; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t148) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f7af); };
            c.w32(2, t148);
            let Some(t149) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700090f7b3); };
            c.cmp(t149, c.r32(2), 32);
            if c.cc_ne() { blk = 123; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t150) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090f7c0); };
            c.w32(2, t150);
            let r151 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r151);
            let Some(t152) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f7c7); };
            c.w32(11, t152);
            let r153 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r153);
            let Some(t154) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090f7ce); };
            c.logic(t154 & 0x2u64, 8);
            if c.cc_ne() { blk = 121; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t155) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090f7d9); };
            c.cmp(t155, c.r32(11), 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t156) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090f7e2); };
            c.w64(11, t156);
            blk = 149; continue;
        }
        102 => {
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090f7ee); };
            c.w64(11, t157);
            let Some(t158) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x700090f7f5); };
            c.cmp(t158, 0x20u64, 8);
            if c.cc_e() { blk = 119; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t159) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x700090f7ff); };
            c.w32(11, t159);
            let Some(t160) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090f803); };
            c.cmp(t160, c.r32(11), 32);
            if c.cc_e() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090f80c); };
            c.w64(3, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090f810); };
            c.w64(0, t162);
            blk = 78; continue;
        }
        105 => {
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090f819); };
            c.w64(1, t163);
            let Some(t164) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f81d); };
            c.w32(11, t164);
            let r165 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r165);
            let Some(t166) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f824); };
            c.w32(0, t166);
            let Some(t167) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090f828); };
            c.cmp(t167, c.r32(0), 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t168) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090f831); };
            c.w32(11, t168);
            let r169 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r169);
            let Some(t170) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f838); };
            c.w32(0, t170);
            let Some(t171) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090f83c); };
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
            let Some(t172) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f84f); };
            c.w32(0, t172);
            let r173 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r173);
            let Some(t174) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f856); };
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
            let Some(t175) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090f872); };
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
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090f883); };
            c.w64(0, t176);
            blk = 78; continue;
        }
        113 => {
            let Some(t177) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090f88c); };
            c.w32(11, t177);
            let r178 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r178);
            blk = 114; continue;
        }
        114 => {
            let Some(t179) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090f893); };
            c.w32(11, t179);
            let r180 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r180);
            let Some(t181) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f89a); };
            c.w32(11, t181);
            let r182 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r182);
            let Some(t183) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f8a1); };
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
            let Some(t186) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090f8b6); };
            c.w32(6, t186);
            let r187 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r187);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090f8bd); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700090f8c1); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x700090f8c9, 0x700090f8c4);
        }
        116 => {
            c.w64(11, c.r64(0));
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090f8cc); };
            c.w64(9, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090f8d0); };
            c.w64(8, t189);
            blk = 149; continue;
        }
        117 => {
            c.w64(3, c.r64(1));
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090f8dc); };
            c.w64(0, t190);
            blk = 78; continue;
        }
        118 => {
            c.w64(3, c.r64(1));
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090f8e8); };
            c.w64(0, t191);
            blk = 78; continue;
        }
        119 => {
            c.w64(11, c.r64(9));
            c.w64(2, c.r64(11).wrapping_sub(0x1));
            let r192 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r192);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700090f8fa); }
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090f8fe); };
            if !c.push64(t193) { return RegfileExit::fault(0x700090f8fe); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090f901); }
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090f902); };
            if !c.push64(t194) { return RegfileExit::fault(0x700090f902); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700090f90d); };
            c.w64(3, t195);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x700090f91c, 0x700090f917);
        }
        120 => {
            c.w64(11, c.r64(0));
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090f91f); };
            c.w64(9, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090f923); };
            c.w64(8, t197);
            blk = 149; continue;
        }
        121 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r198 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r198);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090f935); }
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090f939); };
            if !c.push64(t199) { return RegfileExit::fault(0x700090f939); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70007f2274, 0x700090f949, 0x700090f944);
        }
        122 => {
            c.w64(11, c.r64(0));
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090f94c); };
            c.w64(9, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090f950); };
            c.w64(8, t201);
            blk = 149; continue;
        }
        123 => {
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090f959); };
            c.w64(2, t202);
            let Some(t203) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f95d); };
            c.w32(0, t203);
            let r204 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r204);
            let Some(t205) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f963); };
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
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090f987); };
            c.w64(0, t206);
            blk = 78; continue;
        }
        127 => {
            let Some(t207) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090f990); };
            c.w32(11, t207);
            let Some(t208) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090f994); };
            c.cmp(t208, c.r32(11), 32);
            if c.cc_e() { blk = 129; continue; }
            blk = 128; continue;
        }
        128 => {
            c.w64(3, c.r64(2));
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090f9a0); };
            c.w64(0, t209);
            blk = 78; continue;
        }
        129 => {
            let Some(t210) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090f9a9); };
            c.w32(11, t210);
            let r211 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r211);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(3, c.r64(2));
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090f9b9); };
            c.w64(0, t212);
            blk = 78; continue;
        }
        131 => {
            let Some(t213) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090f9c2); };
            c.w32(0, t213);
            let r214 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r214);
            let Some(t215) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f9c9); };
            c.cmp(t215, 0x40u64, 16);
            if c.cc_b() { blk = 133; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w64(3, c.r64(2));
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090f9d3); };
            c.w64(0, t216);
            blk = 78; continue;
        }
        133 => {
            let Some(t217) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090f9dc); };
            c.w32(11, t217);
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(11, c.r64(0));
            blk = 149; continue;
        }
        134 => {
            let Some(t218) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090f9ec); };
            c.w32(11, t218);
            let Some(t219) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700090f9f0); };
            c.cmp(t219, c.r32(11), 32);
            if c.cc_e() { blk = 136; continue; }
            blk = 135; continue;
        }
        135 => {
            c.w64(3, c.r64(2));
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090f9fc); };
            c.w64(0, t220);
            blk = 78; continue;
        }
        136 => {
            let Some(t221) = c.ld8(c.r64(0).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090fa05); };
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
            let Some(t223) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090fa19); };
            c.w32(0, t223);
            let r224 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r224);
            let Some(t225) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090fa1f); };
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
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090fa30); };
            c.w64(0, t228);
            blk = 78; continue;
        }
        139 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 145; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t229) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090fa3f); };
            c.w32(0, t229);
            let r230 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r230);
            let Some(t231) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090fa45); };
            c.cmp(t231, c.r32(0), 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t232) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090fa4e); };
            c.w32(11, t232);
            let Some(t233) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090fa52); };
            c.cmp(t233, c.r32(11), 32);
            if c.cc_ne() { blk = 143; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t234) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090fa5b); };
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
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090fa69); };
            c.w64(0, t236);
            blk = 78; continue;
        }
        145 => {
            c.w64(3, c.r64(2));
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090fa75); };
            c.w64(0, t237);
            blk = 78; continue;
        }
        146 => {
            let Some(t238) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090fa7e); };
            c.w32(11, t238);
            let Some(t239) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090fa82); };
            c.cmp(t239, c.r32(11), 32);
            if c.cc_e() { blk = 148; continue; }
            blk = 147; continue;
        }
        147 => {
            c.w64(3, c.r64(2));
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090fa8e); };
            c.w64(0, t240);
            blk = 78; continue;
        }
        148 => {
            let Some(t241) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090fa97); };
            c.w32(0, t241);
            let r242 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r242);
            c.w64(11, c.r64(0));
            blk = 149; continue;
        }
        149 => {
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090faa0); };
            c.w64(15, t243);
            blk = 80; continue;
        }
        150 => {
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090faa9); };
            c.w64(15, t244);
            let Some(t245) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700090faad); };
            c.w64(11, t245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700090fab4); };
            c.w64(3, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090fab8); };
            c.w64(0, t247);
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700090fabc); };
            c.w64(2, t248);
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700090fac0); };
            c.w64(1, t249);
            blk = 159; continue;
        }
        151 => {
            let Some(t250) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090fac9); };
            c.w32(6, t250);
            let r251 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r251);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 152; continue;
        }
        152 => {
            let Some(t252) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090fad8); };
            c.w64(7, t252);
            let r253 = c.r64(6) & 0xfffffffffffffffdu64;
            c.w64(6, r253);
            c.logic(r253, 64);
            let Some(t254) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090fae0); };
            c.w32(7, t254);
            let r255 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r255);
            let Some(t256) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090fae6); };
            c.w32(7, t256);
            let r257 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r257);
            c.cmp(c.r32(6), c.r32(7), 32);
            if c.cc_e() { blk = 159; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t258) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090faf0); };
            c.w32(12, t258);
            let r259 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r259);
            let Some(t260) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090faf7); };
            c.cmp(t260, 0xbeu64, 16);
            if c.cc_ne() { blk = 78; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t261) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090fb05); };
            c.w32(12, t261);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xa9u64, 64);
            if c.cc_a() { blk = 156; continue; }
            blk = 155; continue;
        }
        155 => {
            return RegfileExit::fault(0x700090fb18);
        }
        156 => {
            let Some(t262) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090fb19); };
            c.w32(12, t262);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t263) = c.ld32(c.r64(6).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x700090fb20); };
            c.w32(6, t263);
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xa9u64, 64);
            if c.cc_a() { blk = 158; continue; }
            blk = 157; continue;
        }
        157 => {
            return RegfileExit::fault(0x700090fb32);
        }
        158 => {
            let Some(t264) = c.ld32(c.r64(7).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x700090fb33); };
            c.cmp(t264, c.r32(6), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t265) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090fb3f); };
            c.w32(6, t265);
            let r266 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r266);
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 165; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t267) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700090fb4b); };
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
            let Some(t270) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090fb85); };
            c.w64(6, t270);
            let Some(t271) = c.ld16(c.r64(6).wrapping_add(c.r64(11)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090fb89); };
            c.w32(7, t271);
            let Some(t272) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090fb8f); };
            c.w32(9, t272);
            let r273 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r273);
            let Some(t274) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090fb96); };
            c.w64(12, t274);
            let Some(t275) = c.ld16(c.r64(6).wrapping_add(c.r64(11)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700090fb9a); };
            c.w32(6, t275);
            let Some(t276) = c.ld32(c.r64(9).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090fba0); };
            c.w32(7, t276);
            let r277 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r277);
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700090fbab); }
            c.w64(8, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(1, c.r64(7));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(6));
            c.w64(6, c.r64(12));
            let Some(t278) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700090fbc1); };
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
            let Some(t280) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700090fbd1); };
            c.w32(8, t280);
            let Some(t281) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x700090fbd5); };
            c.w64(8, t281);
            blk = 169; continue;
        }
        169 => {
            return c.call(c.r64(8), 0x700090fbe0, 0x700090fbdd);
        }
        170 => {
            c.w64(11, c.r64(0));
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090fbe3); };
            c.w64(15, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090fbe7); };
            c.w64(9, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090fbeb); };
            c.w64(8, t284);
            blk = 80; continue;
        }
        171 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700090fbf4); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700090fbf8); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700090fbfc); }
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
            let Some(t286) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090fc6b); };
            c.w64(6, t286);
            let Some(t287) = c.ld16(c.r64(6).wrapping_add(c.r64(11)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090fc6f); };
            c.w32(7, t287);
            let Some(t288) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090fc75); };
            c.w32(9, t288);
            let r289 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r289);
            let Some(t290) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090fc7c); };
            c.w64(12, t290);
            let Some(t291) = c.ld16(c.r64(6).wrapping_add(c.r64(11)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700090fc80); };
            c.w32(6, t291);
            let Some(t292) = c.ld32(c.r64(9).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090fc86); };
            c.w32(7, t292);
            let r293 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r293);
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700090fc91); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700090fc95); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090fc96); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700090fc97); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700090fc98); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090fc99); }
            let Some(t294) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x700090fc9a); };
            c.w64(3, t294);
            c.w32(0, 0x5u64);
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x700090fcae, 0x700090fca9);
        }
        181 => {
            c.w64(11, c.r64(0));
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090fcb1); };
            c.w64(15, t295);
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090fcb5); };
            c.w64(9, t296);
            let Some(t297) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090fcb9); };
            c.w64(8, t297);
            blk = 80; continue;
        }
        182 => {
            let Some(t298) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090fcc2); };
            c.w32(11, t298);
            let r299 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r299);
            let Some(t300) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090fcc9); };
            c.w32(11, t300);
            let r301 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r301);
            let Some(t302) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090fcd0); };
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
            return RegfileExit::fault(0x700090fcea);
        }
        184 => {
            let Some(t304) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090fceb); };
            c.w32(11, t304);
            let r305 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r305);
            let Some(t306) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090fcf3); };
            c.w32(0, t306);
            let r307 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r307);
            let Some(t308) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090fcfa); };
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
            let Some(t310) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090fd14); };
            c.w64(3, t310);
            let Some(t311) = c.ld16(c.r64(3).wrapping_add(c.r64(11)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090fd18); };
            c.w32(2, t311);
            let Some(t312) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090fd1e); };
            c.w32(3, t312);
            let r313 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r313);
            let Some(t314) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090fd24); };
            c.w64(1, t314);
            let Some(t315) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090fd28); };
            c.w32(3, t315);
            let r316 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r316);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090fd2f); }
            c.w32(0, 0x174u64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090fd38); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090fd39); }
            let Some(t317) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x700090fd3a); };
            c.w64(3, t317);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x700090fd4e, 0x700090fd49);
        }
        187 => {
            c.w64(11, c.r64(0));
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090fd51); };
            c.w64(15, t318);
            let Some(t319) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090fd55); };
            c.w64(9, t319);
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090fd59); };
            c.w64(8, t320);
            blk = 80; continue;
        }
        188 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r321 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r321);
            let Some(t322) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090fd6b); };
            c.w64(2, t322);
            let Some(t323) = c.ld16(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090fd6f); };
            c.w32(6, t323);
            let Some(t324) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090fd75); };
            c.w32(2, t324);
            let r325 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r325);
            let Some(t326) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090fd7b); };
            c.w64(7, t326);
            let Some(t327) = c.ld32(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090fd7f); };
            c.w32(2, t327);
            let r328 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r328);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090fd86); }
            c.w64(6, c.r64(7));
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(3));
            c.w64(3, c.r64(2));
            c.w32(2, 0x2u64);
            return c.call(0x70008bc174, 0x700090fda0, 0x700090fd9b);
        }
        189 => {
            c.w64(11, c.r64(0));
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090fda3); };
            c.w64(15, t329);
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090fda7); };
            c.w64(9, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090fdab); };
            c.w64(8, t331);
            blk = 80; continue;
        }
        190 => {
            let Some(t332) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700090fdb4); };
            c.w32(2, t332);
            let r333 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r333);
            let Some(t334) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090fdba); };
            c.w32(6, t334);
            let Some(t335) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090fdbd); };
            c.cmp(t335, c.r32(2), 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 191; continue;
        }
        191 => {
            let Some(t336) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090fdca); };
            c.w64(7, t336);
            let Some(t337) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700090fdce); };
            c.w64(12, t337);
            let Some(t338) = c.ld16(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090fdd2); };
            c.w32(0, t338);
            let Some(t339) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090fdd8); };
            c.w32(7, t339);
            let r340 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r340);
            let Some(t341) = c.ld32(c.r64(7).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090fdde); };
            c.w32(0, t341);
            let r342 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r342);
            c.logic(c.r8(6) & 0x2u64, 8);
            if c.cc_e() { blk = 242; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t343) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090fdef); };
            c.w32(6, t343);
            let r344 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r344);
            let Some(t345) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700090fdf5); };
            c.cmp(t345, c.r32(6), 32);
            if c.cc_ne() { blk = 217; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t346) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090fe02); };
            c.w32(6, t346);
            let r347 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r347);
            let Some(t348) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090fe08); };
            c.w32(7, t348);
            let r349 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r349);
            let Some(t350) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x700090fe0e); };
            c.logic(t350 & 0x2u64, 8);
            if c.cc_ne() { blk = 215; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t351) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090fe18); };
            c.cmp(t351, c.r32(7), 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 195; continue;
        }
        195 => {
            c.w64(2, c.r64(11));
            blk = 242; continue;
        }
        196 => {
            let Some(t352) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700090fe29); };
            c.w64(11, t352);
            let Some(t353) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x700090fe30); };
            c.cmp(t353, 0x20u64, 8);
            if c.cc_e() { blk = 213; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t354) = c.ld32(c.r64(6).wrapping_add(0x33)) else { return RegfileExit::fault(0x700090fe3a); };
            c.w32(11, t354);
            let Some(t355) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700090fe3e); };
            c.cmp(t355, c.r32(11), 32);
            if c.cc_e() { blk = 199; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t356) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090fe47); };
            c.w64(0, t356);
            blk = 78; continue;
        }
        199 => {
            let Some(t357) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090fe50); };
            c.w32(11, t357);
            let r358 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r358);
            let Some(t359) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090fe57); };
            c.w32(0, t359);
            let Some(t360) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090fe5b); };
            c.cmp(t360, c.r32(0), 32);
            if c.cc_ne() { blk = 201; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t361) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090fe64); };
            c.w32(11, t361);
            let r362 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r362);
            let Some(t363) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090fe6b); };
            c.w32(0, t363);
            let Some(t364) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700090fe6f); };
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
            let Some(t365) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090fe82); };
            c.w32(0, t365);
            let r366 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r366);
            let Some(t367) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090fe89); };
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
            let Some(t368) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700090fea7); };
            c.cmp(t368, c.r32(0), 32);
            if c.cc_ne() { blk = 208; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090feb0); };
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
            let Some(t370) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700090fec1); };
            c.w32(11, t370);
            let r371 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r371);
            let Some(t372) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090fec8); };
            c.w32(11, t372);
            let r373 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r373);
            let Some(t374) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090fecf); };
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
            let Some(t377) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700090fee4); };
            c.w32(11, t377);
            let r378 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r378);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090feeb); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090feef); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(6));
            c.w64(6, c.r64(11));
            return c.call(0x70007f6fb4, 0x700090fefd, 0x700090fef8);
        }
        210 => {
            c.w64(2, c.r64(0));
            let Some(t379) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090ff00); };
            c.w64(15, t379);
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090ff04); };
            c.w64(9, t380);
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ff08); };
            c.w64(8, t381);
            blk = 242; continue;
        }
        211 => {
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090ff11); };
            c.w64(0, t382);
            blk = 78; continue;
        }
        212 => {
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090ff1a); };
            c.w64(0, t383);
            blk = 78; continue;
        }
        213 => {
            c.w64(11, c.r64(9));
            c.w64(2, c.r64(11).wrapping_sub(0x1));
            let r384 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r384);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700090ff2c); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700090ff30); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700090ff31); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090ff32); }
            let Some(t385) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700090ff33); };
            c.w64(3, t385);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x700090ff47, 0x700090ff42);
        }
        214 => {
            c.w64(2, c.r64(0));
            let Some(t386) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090ff4a); };
            c.w64(15, t386);
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090ff4e); };
            c.w64(9, t387);
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ff52); };
            c.w64(8, t388);
            blk = 242; continue;
        }
        215 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r389 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r389);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700090ff64); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700090ff68); }
            c.w64(7, c.r64(6));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(12));
            return c.call(0x70007f2274, 0x700090ff76, 0x700090ff71);
        }
        216 => {
            c.w64(2, c.r64(0));
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700090ff79); };
            c.w64(15, t390);
            let Some(t391) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700090ff7d); };
            c.w64(9, t391);
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700090ff81); };
            c.w64(8, t392);
            blk = 242; continue;
        }
        217 => {
            let Some(t393) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700090ff8a); };
            c.w32(11, t393);
            let r394 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r394);
            let Some(t395) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090ff91); };
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
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090ffb1); };
            c.w64(0, t396);
            blk = 78; continue;
        }
        221 => {
            let Some(t397) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090ffba); };
            c.w32(11, t397);
            let Some(t398) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700090ffbe); };
            c.cmp(t398, c.r32(11), 32);
            if c.cc_e() { blk = 223; continue; }
            blk = 222; continue;
        }
        222 => {
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090ffc7); };
            c.w64(0, t399);
            blk = 78; continue;
        }
        223 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 225; continue; }
            blk = 224; continue;
        }
        224 => {
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090ffd5); };
            c.w64(0, t400);
            blk = 78; continue;
        }
        225 => {
            let Some(t401) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090ffde); };
            c.cmp(t401, 0x40u64, 16);
            if c.cc_b() { blk = 227; continue; }
            blk = 226; continue;
        }
        226 => {
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700090ffe5); };
            c.w64(0, t402);
            blk = 78; continue;
        }
        227 => {
            let Some(t403) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700090ffee); };
            c.w32(11, t403);
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(2, c.r64(0));
            blk = 242; continue;
        }
        228 => {
            let Some(t404) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700090fffe); };
            c.w32(0, t404);
            let Some(t405) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000910001); };
            c.cmp(t405, c.r32(0), 32);
            if c.cc_e() { blk = 230; continue; }
            blk = 229; continue;
        }
        229 => {
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091000a); };
            c.w64(0, t406);
            blk = 78; continue;
        }
        230 => {
            let Some(t407) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000910013); };
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
            let Some(t409) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000910027); };
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
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000910035); };
            c.w64(0, t412);
            blk = 78; continue;
        }
        233 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 239; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t413) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000910044); };
            c.w32(0, t413);
            let r414 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r414);
            let Some(t415) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700091004a); };
            c.cmp(t415, c.r32(0), 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 235; continue;
        }
        235 => {
            let Some(t416) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000910053); };
            c.w32(11, t416);
            let Some(t417) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000910057); };
            c.cmp(t417, c.r32(11), 32);
            if c.cc_ne() { blk = 237; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t418) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000910060); };
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
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091006b); };
            c.w64(0, t420);
            blk = 78; continue;
        }
        239 => {
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000910074); };
            c.w64(0, t421);
            blk = 78; continue;
        }
        240 => {
            let Some(t422) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091007d); };
            c.w32(11, t422);
            let Some(t423) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000910081); };
            c.cmp(t423, c.r32(11), 32);
            if c.cc_e() { blk = 242; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t424) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091008a); };
            c.w64(0, t424);
            blk = 78; continue;
        }
        242 => {
            c.w64(11, c.r64(2));
            blk = 80; continue;
        }
        243 => {
            let Some(t425) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700091009b); };
            c.w64(11, t425);
            let Some(t426) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70009100a2); };
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
            let Some(t427) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009100b2); };
            c.w32(11, t427);
            let r428 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r428);
            blk = 247; continue;
        }
        246 => {
            let Some(t429) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009100bb); };
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
            return RegfileExit::fault(0x70009100c8);
        }
        249 => {
            let r431 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r431);
            c.logic(r431, 64);
            let Some(t432) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009100cd); };
            c.w32(0, t432);
            let r433 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r433);
            let Some(t434) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009100d3); };
            c.w32(2, t434);
            let r435 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r435);
            let Some(t436) = c.ld64(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009100d9); };
            c.w64(0, t436);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 250; continue;
        }
        250 => {
            let Some(t437) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009100e3); };
            c.w32(1, t437);
            let r438 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r438);
            let Some(t439) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009100e9); };
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
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70009100fd); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000910101); }
            c.w64(6, c.r64(11));
            c.w64(3, c.r64(2));
            c.w64(2, c.r64(0));
            c.w64(7, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007fe8d4, 0x7000910115, 0x7000910110);
        }
        253 => {
            c.w64(11, c.r64(0));
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910118); };
            c.w64(15, t442);
            let Some(t443) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091011c); };
            c.w64(9, t443);
            let Some(t444) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910120); };
            c.w64(8, t444);
            blk = 80; continue;
        }
        254 => {
            let Some(t445) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000910129); };
            c.w64(11, t445);
            let Some(t446) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000910130); };
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
            let Some(t450) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091014e); };
            c.w32(6, t450);
            let r451 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r451);
            let Some(t452) = c.ld32(c.r64(6).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000910154); };
            c.w32(6, t452);
            let r453 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r453);
            let Some(t454) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700091015a); };
            c.w64(7, t454);
            let Some(t455) = c.ld32(c.r64(6).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700091015e); };
            c.w32(2, t455);
            let r456 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r456);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000910165); }
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(10));
            c.w64(6, c.r64(7));
            return c.call(0x70007fea14, 0x700091017a, 0x7000910175);
        }
        256 => {
            c.w64(11, c.r64(0));
            let Some(t457) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091017d); };
            c.w64(15, t457);
            let Some(t458) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000910181); };
            c.w64(9, t458);
            let Some(t459) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910185); };
            c.w64(8, t459);
            blk = 80; continue;
        }
        257 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r460 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r460);
            let Some(t461) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910197); };
            c.w64(2, t461);
            let Some(t462) = c.ld16(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091019b); };
            c.w32(1, t462);
            let Some(t463) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009101a1); };
            c.w32(2, t463);
            let r464 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r464);
            let Some(t465) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009101a7); };
            c.w64(6, t465);
            let Some(t466) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009101ab); };
            c.w32(2, t466);
            let r467 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r467);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009101b2); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70009101b6); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009101b7); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70009101b8); }
            let Some(t468) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70009101b9); };
            c.w64(3, t468);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70009101ca, 0x70009101c5);
        }
        258 => {
            c.w64(11, c.r64(0));
            let Some(t469) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009101cd); };
            c.w64(15, t469);
            let Some(t470) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009101d1); };
            c.w64(9, t470);
            let Some(t471) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009101d5); };
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
            let Some(t475) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009101f6); };
            c.w32(1, t475);
            let r476 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r476);
            let Some(t477) = c.ld32(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x70009101fc); };
            c.w32(1, t477);
            let r478 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r478);
            let Some(t479) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000910202); };
            c.w32(2, t479);
            let r480 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r480);
            let Some(t481) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000910209); };
            c.w64(1, t481);
            let Some(t482) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091020d); };
            c.w32(7, t482);
            let r483 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r483);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000910213); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000910217); }
            c.w64(6, c.r64(1));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000910222, 0x700091021d);
        }
        260 => {
            c.w64(11, c.r64(0));
            let Some(t484) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910225); };
            c.w64(15, t484);
            let Some(t485) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000910229); };
            c.w64(9, t485);
            let Some(t486) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091022d); };
            c.w64(8, t486);
            blk = 80; continue;
        }
        261 => {
            let Some(t487) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910236); };
            c.w32(2, t487);
            let r488 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r488);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 263; continue; }
            blk = 262; continue;
        }
        262 => {
            let Some(t489) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000910241); };
            c.w64(2, t489);
            blk = 263; continue;
        }
        263 => {
            let Some(t490) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910248); };
            c.w64(6, t490);
            let Some(t491) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091024c); };
            c.w32(7, t491);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t492) = c.ld16(c.r64(6).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910252); };
            c.w32(12, t492);
            let Some(t493) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000910258); };
            c.w32(3, t493);
            let r494 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r494);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t495) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000910261); };
            c.w32(12, t495);
            let r496 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r496);
            let r497 = c.sub(c.r64(7), 0x1u64, 64);
            c.w64(7, r497);
            let Some(t498) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091026d); };
            c.w32(0, t498);
            let r499 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r499);
            let r500 = c.r64(0) & c.r64(7);
            c.w64(0, r500);
            c.logic(r500, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000910278); }
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 264; continue;
        }
        264 => {
            c.w64(1, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(11, c.r64(1).wrapping_mul(4));
            let Some(t501) = c.ld32(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091028c); };
            c.w32(3, t501);
            let Some(t502) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000910291); };
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
            let Some(t505) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009102b3); };
            c.w32(0, t505);
            let Some(t506) = c.ld32(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009102b6); };
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
            return RegfileExit::fault(0x70009102ca);
        }
        269 => {
            let Some(t508) = c.ld16(c.r64(6).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009102cb); };
            c.w32(0, t508);
            let Some(t509) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009102d1); };
            c.w64(1, t509);
            let Some(t510) = c.ld32(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009102d5); };
            c.w32(11, t510);
            let r511 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r511);
            let Some(t512) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009102dd); };
            c.w64(2, t512);
            let Some(t513) = c.ld32(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009102e1); };
            c.w32(0, t513);
            let r514 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r514);
            c.logic(c.r8(3) & 0x2u64, 8);
            if c.cc_e() { blk = 321; continue; }
            blk = 270; continue;
        }
        270 => {
            let Some(t515) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009102f1); };
            c.w32(3, t515);
            let Some(t516) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70009102f5); };
            c.cmp(t516, c.r32(3), 32);
            if c.cc_ne() { blk = 295; continue; }
            blk = 271; continue;
        }
        271 => {
            let Some(t517) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910302); };
            c.w32(2, t517);
            let r518 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r518);
            let Some(t519) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000910309); };
            c.w32(11, t519);
            let r520 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r520);
            let Some(t521) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000910310); };
            c.logic(t521 & 0x2u64, 8);
            if c.cc_ne() { blk = 293; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t522) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700091031b); };
            c.cmp(t522, c.r32(11), 32);
            if c.cc_e() { blk = 274; continue; }
            blk = 273; continue;
        }
        273 => {
            let Some(t523) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000910324); };
            c.w64(11, t523);
            blk = 321; continue;
        }
        274 => {
            let Some(t524) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000910330); };
            c.w64(11, t524);
            let Some(t525) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000910337); };
            c.cmp(t525, 0x20u64, 8);
            if c.cc_e() { blk = 291; continue; }
            blk = 275; continue;
        }
        275 => {
            let Some(t526) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000910341); };
            c.w32(11, t526);
            let Some(t527) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000910345); };
            c.cmp(t527, c.r32(11), 32);
            if c.cc_e() { blk = 277; continue; }
            blk = 276; continue;
        }
        276 => {
            let Some(t528) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091034e); };
            c.w64(3, t528);
            let Some(t529) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000910352); };
            c.w64(0, t529);
            blk = 78; continue;
        }
        277 => {
            let Some(t530) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091035b); };
            c.w64(3, t530);
            let Some(t531) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091035f); };
            c.w32(11, t531);
            let r532 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r532);
            let Some(t533) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000910366); };
            c.w32(0, t533);
            let Some(t534) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700091036a); };
            c.cmp(t534, c.r32(0), 32);
            if c.cc_ne() { blk = 279; continue; }
            blk = 278; continue;
        }
        278 => {
            let Some(t535) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000910373); };
            c.w32(11, t535);
            let r536 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r536);
            let Some(t537) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091037a); };
            c.w32(0, t537);
            let Some(t538) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700091037e); };
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
            let Some(t539) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000910391); };
            c.w32(0, t539);
            let r540 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r540);
            let Some(t541) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000910398); };
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
            let Some(t542) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70009103b4); };
            c.cmp(t542, c.r32(0), 32);
            if c.cc_ne() { blk = 286; continue; }
            blk = 283; continue;
        }
        283 => {
            let Some(t543) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009103bd); };
            c.w64(3, t543);
            let Some(t544) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009103c1); };
            c.w64(0, t544);
            blk = 78; continue;
        }
        284 => {
            let Some(t545) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009103ca); };
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
            let Some(t547) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009103d8); };
            c.w32(11, t547);
            let r548 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r548);
            let Some(t549) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009103df); };
            c.w32(11, t549);
            let r550 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r550);
            let Some(t551) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009103e6); };
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
            let Some(t554) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009103fb); };
            c.w32(6, t554);
            let r555 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r555);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000910402); }
            let Some(t556) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000910406); };
            if !c.push64(t556) { return RegfileExit::fault(0x7000910406); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000910410, 0x700091040b);
        }
        288 => {
            c.w64(11, c.r64(0));
            let Some(t557) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000910413); };
            c.w64(9, t557);
            let Some(t558) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910417); };
            c.w64(8, t558);
            blk = 321; continue;
        }
        289 => {
            let Some(t559) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000910420); };
            c.w64(3, t559);
            let Some(t560) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000910424); };
            c.w64(0, t560);
            blk = 78; continue;
        }
        290 => {
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091042d); };
            c.w64(3, t561);
            let Some(t562) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000910431); };
            c.w64(0, t562);
            blk = 78; continue;
        }
        291 => {
            c.w64(11, c.r64(9));
            c.w64(3, c.r64(11).wrapping_sub(0x1));
            let r563 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r563);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000910443); }
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000910447); };
            if !c.push64(t564) { return RegfileExit::fault(0x7000910447); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091044a); }
            let Some(t565) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091044b); };
            if !c.push64(t565) { return RegfileExit::fault(0x700091044b); }
            let Some(t566) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700091044e); };
            c.w64(3, t566);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x7000910462, 0x700091045d);
        }
        292 => {
            c.w64(11, c.r64(0));
            let Some(t567) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000910465); };
            c.w64(9, t567);
            let Some(t568) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910469); };
            c.w64(8, t568);
            blk = 321; continue;
        }
        293 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r569 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r569);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700091047b); }
            let Some(t570) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091047f); };
            if !c.push64(t570) { return RegfileExit::fault(0x700091047f); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            return c.call(0x70007f2274, 0x700091048f, 0x700091048a);
        }
        294 => {
            c.w64(11, c.r64(0));
            let Some(t571) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000910492); };
            c.w64(9, t571);
            let Some(t572) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910496); };
            c.w64(8, t572);
            blk = 321; continue;
        }
        295 => {
            let Some(t573) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091049f); };
            c.w64(0, t573);
            let Some(t574) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009104a3); };
            c.w32(3, t574);
            let r575 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r575);
            let Some(t576) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009104a9); };
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
            let Some(t577) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009104ce); };
            c.w64(3, t577);
            let Some(t578) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009104d2); };
            c.w64(0, t578);
            blk = 78; continue;
        }
        299 => {
            let Some(t579) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009104db); };
            c.w32(11, t579);
            let Some(t580) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70009104df); };
            c.cmp(t580, c.r32(11), 32);
            if c.cc_e() { blk = 301; continue; }
            blk = 300; continue;
        }
        300 => {
            let Some(t581) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009104e8); };
            c.w64(3, t581);
            let Some(t582) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009104ec); };
            c.w64(0, t582);
            blk = 78; continue;
        }
        301 => {
            let Some(t583) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009104f5); };
            c.w32(11, t583);
            let r584 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r584);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 303; continue; }
            blk = 302; continue;
        }
        302 => {
            let Some(t585) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000910502); };
            c.w64(3, t585);
            let Some(t586) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000910506); };
            c.w64(0, t586);
            blk = 78; continue;
        }
        303 => {
            let Some(t587) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091050f); };
            c.w32(0, t587);
            let r588 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r588);
            let Some(t589) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000910516); };
            c.cmp(t589, 0x40u64, 16);
            if c.cc_b() { blk = 305; continue; }
            blk = 304; continue;
        }
        304 => {
            let Some(t590) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091051d); };
            c.w64(3, t590);
            let Some(t591) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000910521); };
            c.w64(0, t591);
            blk = 78; continue;
        }
        305 => {
            let Some(t592) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091052a); };
            c.w32(11, t592);
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(11, c.r64(0));
            blk = 321; continue;
        }
        306 => {
            let Some(t593) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091053a); };
            c.w32(11, t593);
            let Some(t594) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700091053e); };
            c.cmp(t594, c.r32(11), 32);
            if c.cc_e() { blk = 308; continue; }
            blk = 307; continue;
        }
        307 => {
            let Some(t595) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000910547); };
            c.w64(3, t595);
            let Some(t596) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091054b); };
            c.w64(0, t596);
            blk = 78; continue;
        }
        308 => {
            let Some(t597) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000910554); };
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
            let Some(t599) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091056a); };
            c.w32(3, t599);
            let r600 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r600);
            let Some(t601) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000910570); };
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
            let Some(t604) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091057e); };
            c.w64(3, t604);
            let Some(t605) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000910582); };
            c.w64(0, t605);
            blk = 78; continue;
        }
        311 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 317; continue; }
            blk = 312; continue;
        }
        312 => {
            let Some(t606) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000910591); };
            c.w32(0, t606);
            let r607 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r607);
            let Some(t608) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000910597); };
            c.cmp(t608, c.r32(0), 32);
            if c.cc_e() { blk = 316; continue; }
            blk = 313; continue;
        }
        313 => {
            let Some(t609) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009105a0); };
            c.w32(11, t609);
            let Some(t610) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70009105a4); };
            c.cmp(t610, c.r32(11), 32);
            if c.cc_ne() { blk = 315; continue; }
            blk = 314; continue;
        }
        314 => {
            let Some(t611) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009105ad); };
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
            let Some(t613) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009105b8); };
            c.w64(3, t613);
            let Some(t614) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009105bc); };
            c.w64(0, t614);
            blk = 78; continue;
        }
        317 => {
            let Some(t615) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009105c5); };
            c.w64(3, t615);
            let Some(t616) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009105c9); };
            c.w64(0, t616);
            blk = 78; continue;
        }
        318 => {
            let Some(t617) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009105d2); };
            c.w32(11, t617);
            let Some(t618) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70009105d6); };
            c.cmp(t618, c.r32(11), 32);
            if c.cc_e() { blk = 320; continue; }
            blk = 319; continue;
        }
        319 => {
            let Some(t619) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009105df); };
            c.w64(3, t619);
            let Some(t620) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009105e3); };
            c.w64(0, t620);
            blk = 78; continue;
        }
        320 => {
            let Some(t621) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009105ec); };
            c.w32(0, t621);
            let r622 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r622);
            c.w64(11, c.r64(0));
            blk = 321; continue;
        }
        321 => {
            let Some(t623) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009105f5); };
            c.w64(15, t623);
            blk = 80; continue;
        }
        322 => {
            let Some(t624) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009105fe); };
            c.w64(3, t624);
            let Some(t625) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000910602); };
            c.w64(0, t625);
            blk = 78; continue;
        }
        323 => {
            return RegfileExit::fault(0x700091060b);
        }
        324 => {
            let Some(t626) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091060c); };
            c.w64(0, t626);
            let Some(t627) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000910613); };
            c.w64(3, t627);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091061a); }
            let Some(t628) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091061e); };
            c.w64(1, t628);
            let Some(t629) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000910625); };
            c.cmp(t629, c.r64(2), 64);
            if c.cc_a() { blk = 327; continue; }
            blk = 325; continue;
        }
        325 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000910634, 0x700091062f);
        }
        326 => {
            let Some(t630) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000910634); };
            c.w64(11, t630);
            let Some(t631) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910638); };
            c.w64(15, t631);
            let Some(t632) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091063c); };
            c.w64(9, t632);
            let Some(t633) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910640); };
            c.w64(8, t633);
            blk = 328; continue;
        }
        327 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x7000910646); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 328; continue;
        }
        328 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            let Some(t634) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000910656); };
            c.w64(11, t634);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700091065d); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000910661); }
            blk = 58; continue;
        }
        329 => {
            let Some(t635) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091066b); };
            c.w64(0, t635);
            let Some(t636) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000910672); };
            c.w64(3, t636);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000910679); }
            let Some(t637) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091067d); };
            c.w64(1, t637);
            let Some(t638) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000910684); };
            c.cmp(t638, c.r64(2), 64);
            if c.cc_a() { blk = 332; continue; }
            blk = 330; continue;
        }
        330 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000910693, 0x700091068e);
        }
        331 => {
            let Some(t639) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000910693); };
            c.w64(11, t639);
            let Some(t640) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910697); };
            c.w64(15, t640);
            let Some(t641) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091069b); };
            c.w64(9, t641);
            let Some(t642) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091069f); };
            c.w64(8, t642);
            blk = 333; continue;
        }
        332 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70009106a5); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 333; continue;
        }
        333 => {
            c.w32(10, c.r32(11));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t643) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009106b8); };
            c.w64(11, t643);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70009106bf); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70009106c3); }
            blk = 61; continue;
        }
        334 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009106d2); }
            return c.call(0x70007febb4, 0x70009106db, 0x70009106d6);
        }
        335 => {
            let Some(t644) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009106db); };
            c.w64(11, t644);
            let Some(t645) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009106df); };
            c.w64(15, t645);
            let Some(t646) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009106e3); };
            c.w64(9, t646);
            let Some(t647) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009106e7); };
            c.w64(8, t647);
            blk = 65; continue;
        }
        336 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70009106fa, 0x70009106f5);
        }
        337 => {
            c.w64(11, c.r64(0));
            let Some(t648) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009106fd); };
            c.w64(15, t648);
            let Some(t649) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000910701); };
            c.w64(9, t649);
            let Some(t650) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910705); };
            c.w64(8, t650);
            blk = 70; continue;
        }
        338 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000910713); }
            return c.call(0x70007febf4, 0x700091071d, 0x7000910718);
        }
        339 => {
            c.w64(11, c.r64(0));
            let Some(t651) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910720); };
            c.w64(15, t651);
            let Some(t652) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000910724); };
            c.w64(9, t652);
            let Some(t653) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910728); };
            c.w64(8, t653);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700091072c); }
            blk = 76; continue;
        }
        340 => {
            c.w64(11, c.r64(9));
            let r654 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r654);
            let r655 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r655);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000910740); }
            let Some(t656) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000910744); };
            c.w64(8, t656);
            let Some(t657) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091074b); };
            c.w32(8, t657);
            let r658 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r658);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000910752); }
            let Some(t659) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x7000910754); };
            c.w64(3, t659);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000910767, 0x7000910762);
        }
        341 => {
            return RegfileExit::fault(0x7000910767);
        }
        342 => {
            blk = 78; continue;
        }
        343 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r660 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r660);
            let Some(t661) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910776); };
            c.w64(2, t661);
            let Some(t662) = c.ld16(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091077a); };
            c.w32(1, t662);
            let Some(t663) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000910780); };
            c.w32(2, t663);
            let r664 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r664);
            let Some(t665) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000910786); };
            c.w64(6, t665);
            let Some(t666) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091078a); };
            c.w32(1, t666);
            let r667 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r667);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000910791); }
            c.w32(0, 0xau64);
            c.w64(2, c.r64(3));
            return c.call(0x7000804ff4, 0x70009107a2, 0x700091079d);
        }
        344 => {
            c.w64(11, c.r64(0));
            let Some(t668) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009107a5); };
            c.w64(15, t668);
            let Some(t669) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009107a9); };
            c.w64(9, t669);
            let Some(t670) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009107ad); };
            c.w64(8, t670);
            blk = 80; continue;
        }
        345 => {
            return RegfileExit::jump(0x70009107b8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009107d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009107d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009107d8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009107de); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009107e2); };
            c.w64(11, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009107e6); };
            c.w64(3, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009107ec); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009107f3); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009107f9); };
            c.w32(2, t8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009107ff); };
            c.w32(1, t9);
            let r10 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r10);
            let Some(t11) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910807); };
            c.w64(3, t11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091080b); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000910812); };
            c.w32(6, t14);
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000910818); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            let Some(t17) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091081e); };
            c.w32(1, t17);
            let r18 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r18);
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000910825); };
            c.w32(0, t19);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000910829); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091082d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000910831); }
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000910835); };
            c.cmp(t20, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091083e); };
            c.w64(11, t21);
            blk = 2; continue;
        }
        2 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let Some(t22) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000910849); };
            c.w64(2, t22);
            let r23 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r23);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000910855); }
            c.w64(0, c.r64(6));
            c.w64(6, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(11));
            return c.call(0x7000834434, 0x700091086a, 0x7000910865);
        }
        3 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091086a); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091086e); };
            c.w64(8, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000910872); };
            c.w64(11, t26);
            c.w64(9, c.r64(11).wrapping_add(0x7));
            let Some(t27) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700091087a); };
            c.w32(11, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091087f); };
            c.w64(15, t28);
            let Some(t29) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910883); };
            c.w64(1, t29);
            let Some(t30) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000910887); };
            c.w64(5, t30);
            c.w64(12, c.r64(8));
            let r31 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r31);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000910898);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009108b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009108b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009108b8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009108be); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009108c2); };
            c.w64(8, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009108c6); };
            c.w64(11, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009108cc); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009108d3); };
            c.w64(11, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009108d7); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009108de); };
            c.w32(3, t9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009108e4); };
            c.w32(2, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009108e8); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009108ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009108f0); }
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009108f4); };
            c.cmp(t11, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009108fd); };
            c.w64(8, t12);
            blk = 2; continue;
        }
        2 => {
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t13) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000910908); };
            c.w64(1, t13);
            let r14 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r14);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000910914); }
            c.w64(2, c.r64(11));
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(8));
            return c.call(0x7000836434, 0x700091092c, 0x7000910927);
        }
        3 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091092c); };
            c.w64(7, t15);
            let Some(t16) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910930); };
            c.w64(7, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000910934); };
            c.w64(8, t17);
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091093c); };
            c.w32(8, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910941); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910945); };
            c.w64(1, t20);
            let Some(t21) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000910949); };
            c.w64(5, t21);
            c.w64(12, c.r64(7));
            let r22 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r22);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000910958);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000910974(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000910977); };
            c.w64(3, t0);
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091097b); };
            c.w32(2, (t1 as u16 as i16 as i64 as u64));
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910981); };
            c.w32(6, t2);
            c.cmp(c.r16(6), 0x0u64, 16);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091098d); };
            c.w32(7, t3);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091099c); };
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
            return RegfileExit::fault(0x70009109a9);
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009109aa); };
            c.w32(6, t7);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x2u64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70009109b9);
        }
        7 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009109ba); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_g() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t10) = c.ld32(c.r64(3).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009109c9); };
            c.w32(3, t10);
            let r11 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r11);
            let Some(t12) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009109cf); };
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
            let Some(t14) = c.ld32(c.r64(3).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009109e7); };
            c.w32(3, t14);
            let r15 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r15);
            let Some(t16) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009109ee); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            blk = 12; continue;
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(3).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009109f6); };
            c.w32(3, t18);
            let r19 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r19);
            let Some(t20) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009109fc); };
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
            let Some(t22) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000910a0e); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910a15); };
            c.w32(3, t24);
            let r25 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r25);
            blk = 12; continue;
        }
        12 => {
            let r26 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r26);
            let Some(t27) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000910a1f); };
            c.w32(2, t27);
            let Some(t28) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910a24); };
            c.w64(1, t28);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        13 => {
            return RegfileExit::fault(0x7000910a2d);
        }
        14 => {
            return RegfileExit::fault(0x7000910a2e);
        }
        15 => {
            return RegfileExit::jump(0x7000910a30);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000910a34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000910a34); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000910a38); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000910a3e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000910a42); };
            c.w64(7, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000910a46); };
            c.w32(8, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910a4c); };
            c.w32(11, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000910a52); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000910a56); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000910a5a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000910a5e); }
            c.cmp(c.r16(11), 0x0u64, 16);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910a69); };
            c.w32(3, t5);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000910a78); };
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
            return RegfileExit::fault(0x7000910a86);
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910a87); };
            c.w32(11, t9);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x7000910a97);
        }
        7 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000910a9b); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(7, c.r64(9).wrapping_sub(0x1));
            let Some(t12) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000910aab); };
            c.w64(8, t12);
            let r13 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r13);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000910ab1); }
            c.w32(7, 0x6au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000910aba); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000910abb); };
            c.w64(3, t14);
            c.w64(6, c.r64(8));
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x7000910ad2, 0x7000910acd);
        }
        9 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910ad2); };
            c.w64(0, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000910ad6); };
            c.w64(15, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910ada); };
            c.w64(9, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000910ade); };
            c.w64(2, t18);
            blk = 13; continue;
        }
        10 => {
            let r19 = c.sub(c.r64(8), 0x1u64, 64);
            c.w64(8, r19);
            let Some(t20) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000910ae8); };
            c.w32(7, t20);
            let r21 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r21);
            let Some(t22) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910aee); };
            c.w32(11, t22);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ae() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t23) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000910afd); };
            c.w32(7, t23);
            let r24 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r24);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x7000910b05); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r25 = c.r64(8) & c.r64(7);
            c.w64(8, r25);
            c.logic(r25, 64);
            let Some(t26) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000910b16); };
            c.logic(t26 & 0x4u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t27) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910b1d); };
            c.w64(2, t27);
            c.w64(7, c.r64(9));
            c.w64(9, c.r64(7).wrapping_add(0x5));
            let Some(t28) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000910b28); };
            c.w32(7, t28);
            let Some(t29) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910b2d); };
            c.w64(1, t29);
            let Some(t30) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000910b31); };
            c.w64(5, t30);
            c.w64(12, c.r64(2));
            let r31 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r31);
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            return RegfileExit::fault(0x7000910b3e);
        }
        15 => {
            c.w32(0, c.r32(0));
            let r32 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r32);
            c.w64(8, 0xfffffffffffc0000u64);
            let r33 = c.r64(8) & c.r64(0);
            c.w64(8, r33);
            c.logic(r33, 64);
            let Some(t34) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000910b4e); };
            c.logic(t34 & 0x2u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(7).wrapping_add(0x3));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000910b59); }
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000910b63, 0x7000910b5e);
        }
        17 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000910b63); }
            blk = 13; continue;
        }
        18 => {
            return RegfileExit::jump(0x7000910b68);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000910b74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000910b74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000910b78); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000910b7e); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910b82); };
            c.w64(11, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000910b86); };
            c.w64(3, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000910b8c); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910b93); };
            c.w32(2, t6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000910b99); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910ba1); };
            c.w64(3, t9);
            let Some(t10) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910ba5); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000910bac); };
            c.w32(6, t12);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000910bb2); };
            c.w32(1, t13);
            let r14 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r14);
            let Some(t15) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000910bb9); };
            c.w32(2, t15);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000910bbd); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000910bc1); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000910bc5); }
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000910bc9); };
            c.cmp(t16, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000910bd2); };
            c.w64(11, t17);
            blk = 2; continue;
        }
        2 => {
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t18) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000910bdd); };
            c.w64(7, t18);
            let r19 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r19);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000910be9); }
            c.w64(2, c.r64(3));
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(7));
            c.w64(7, c.r64(10));
            c.w64(3, c.r64(11));
            return c.call(0x700083d494, 0x7000910c01, 0x7000910bfc);
        }
        3 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910c01); };
            c.w64(8, t20);
            let Some(t21) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910c05); };
            c.w64(8, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000910c09); };
            c.w64(11, t22);
            c.w64(9, c.r64(11).wrapping_add(0x7));
            let Some(t23) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000910c11); };
            c.w32(11, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910c16); };
            c.w64(15, t24);
            let Some(t25) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910c1a); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000910c1e); };
            c.w64(5, t26);
            c.w64(12, c.r64(8));
            let r27 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r27);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000910c2c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000910c34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000910c34); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000910c38); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000910c3e); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910c42); };
            c.w64(11, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000910c46); };
            c.w64(3, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000910c4c); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910c53); };
            c.w32(2, t6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000910c59); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910c61); };
            c.w64(3, t9);
            let Some(t10) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910c65); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000910c6c); };
            c.w32(6, t12);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000910c72); };
            c.w32(1, t13);
            let r14 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r14);
            let Some(t15) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000910c79); };
            c.w32(2, t15);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000910c7d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000910c81); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000910c85); }
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000910c89); };
            c.cmp(t16, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000910c92); };
            c.w64(11, t17);
            blk = 2; continue;
        }
        2 => {
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t18) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000910c9d); };
            c.w64(7, t18);
            let r19 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r19);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000910ca9); }
            c.w64(2, c.r64(3));
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(7));
            c.w64(7, c.r64(10));
            c.w64(3, c.r64(11));
            return c.call(0x700083d494, 0x7000910cc1, 0x7000910cbc);
        }
        3 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910cc1); };
            c.w64(8, t20);
            let Some(t21) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910cc5); };
            c.w64(8, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000910cc9); };
            c.w64(11, t22);
            c.w64(9, c.r64(11).wrapping_add(0x7));
            let Some(t23) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000910cd1); };
            c.w32(11, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910cd6); };
            c.w64(15, t24);
            let Some(t25) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910cda); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000910cde); };
            c.w64(5, t26);
            c.w64(12, c.r64(8));
            let r27 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r27);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000910cec);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000910cf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000910cf4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000910cf8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000910cfe); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910d02); };
            c.w64(11, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000910d06); };
            c.w64(3, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910d0c); };
            c.w64(2, (t4 as u16 as i16 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000910d12); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910d19); };
            c.w64(3, t7);
            let Some(t8) = c.ld64(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910d1d); };
            c.w64(2, t8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910d21); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000910d28); };
            c.w32(1, t11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000910d2e); };
            c.w32(6, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000910d32); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000910d36); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000910d3a); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000910d3e); };
            c.cmp(t13, c.r32(6), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000910d47); };
            c.w64(11, t14);
            blk = 2; continue;
        }
        2 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let Some(t15) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000910d52); };
            c.w64(7, t15);
            let r16 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r16);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000910d5e); }
            c.w64(6, c.r64(7));
            c.w64(7, c.r64(1));
            c.w64(1, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(11));
            return c.call(0x700083eb74, 0x7000910d76, 0x7000910d71);
        }
        3 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910d76); };
            c.w64(8, t17);
            let Some(t18) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910d7a); };
            c.w64(8, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000910d7e); };
            c.w64(11, t19);
            c.w64(9, c.r64(11).wrapping_add(0x7));
            let Some(t20) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000910d86); };
            c.w32(11, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910d8b); };
            c.w64(15, t21);
            let Some(t22) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910d8f); };
            c.w64(1, t22);
            let Some(t23) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000910d93); };
            c.w64(5, t23);
            c.w64(12, c.r64(8));
            let r24 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r24);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000910da4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000910db4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000910db4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000910db8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000910dbe); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910dc2); };
            c.w64(11, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000910dc6); };
            c.w64(3, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910dcc); };
            c.w64(2, (t4 as u16 as i16 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000910dd2); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910dd9); };
            c.w64(3, t7);
            let Some(t8) = c.ld64(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910ddd); };
            c.w64(2, t8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910de1); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000910de8); };
            c.w32(1, t11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000910dee); };
            c.w32(6, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000910df2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000910df6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000910dfa); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000910dfe); };
            c.cmp(t13, c.r32(6), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000910e07); };
            c.w64(11, t14);
            blk = 2; continue;
        }
        2 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let Some(t15) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000910e12); };
            c.w64(7, t15);
            let r16 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r16);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000910e1e); }
            c.w64(6, c.r64(7));
            c.w64(7, c.r64(1));
            c.w64(1, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(11));
            return c.call(0x7000840434, 0x7000910e36, 0x7000910e31);
        }
        3 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910e36); };
            c.w64(8, t17);
            let Some(t18) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910e3a); };
            c.w64(8, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000910e3e); };
            c.w64(11, t19);
            c.w64(9, c.r64(11).wrapping_add(0x7));
            let Some(t20) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000910e46); };
            c.w32(11, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910e4b); };
            c.w64(15, t21);
            let Some(t22) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910e4f); };
            c.w64(1, t22);
            let Some(t23) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000910e53); };
            c.w64(5, t23);
            c.w64(12, c.r64(8));
            let r24 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r24);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000910e64);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000910e74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000910e74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000910e78); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000910e7e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910e82); };
            c.w64(7, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000910e86); };
            c.w64(8, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910e8c); };
            c.w64(11, (t4 as u16 as i16 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000910e92); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910e98); };
            c.w64(8, t7);
            let Some(t8) = c.ld64(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910e9c); };
            c.w64(11, t8);
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910ea0); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000910ea6); };
            c.w32(3, t11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x6)) else { return RegfileExit::fault(0x7000910eac); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000910eb2); };
            c.w32(6, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000910eb5); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000910eb9); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000910ebd); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000910ec1); }
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000910ec5); };
            c.cmp(t14, c.r32(6), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000910ece); };
            c.w64(7, t15);
            blk = 2; continue;
        }
        2 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let Some(t16) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000910ed9); };
            c.w64(12, t16);
            let r17 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r17);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let r18 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r18);
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000910ee7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000910eeb); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000910eed); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000910eef); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000910ef0); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000910ef1); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000910ef2); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2360)) else { return RegfileExit::fault(0x7000910ef3); };
            c.w64(3, t19);
            c.w64(6, c.r64(12));
            c.w32(0, 0x6u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x7000910f0a, 0x7000910f05);
        }
        3 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000910f0a); };
            c.w64(2, t20);
            let Some(t21) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910f0e); };
            c.w64(2, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910f12); };
            c.w64(7, t22);
            c.w64(9, c.r64(7).wrapping_add(0x8));
            let Some(t23) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000910f1a); };
            c.w32(7, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000910f1f); };
            c.w64(15, t24);
            let Some(t25) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910f23); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000910f27); };
            c.w64(5, t26);
            c.w64(12, c.r64(2));
            let Some(t27) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000910f2e); };
            c.w64(0, t27);
            let r28 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r28);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000910f3c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000910f54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000910f54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000910f58); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000910f5e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000910f62); };
            c.w64(7, t2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000910f66); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000910f6c); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000910f72); };
            c.w32(8, (t7 as u16 as i16 as i64 as u64));
            let Some(t8) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000910f78); };
            c.w32(11, t8);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000910f7c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000910f80); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000910f84); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000910f88); }
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000910f8c); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000910f95); };
            c.w64(7, t10);
            blk = 2; continue;
        }
        2 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000910fa0); };
            c.w64(3, t11);
            let r12 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r12);
            let r13 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r13);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000910faa); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000910fae); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000910fb0); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000910fb1); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2320)) else { return RegfileExit::fault(0x7000910fba); };
            c.w64(3, t14);
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000910fcc, 0x7000910fc7);
        }
        3 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000910fcc); };
            c.w64(2, t15);
            let Some(t16) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910fd0); };
            c.w64(2, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000910fd4); };
            c.w64(7, t17);
            c.w64(9, c.r64(7).wrapping_add(0x3));
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000910fdc); };
            c.w32(7, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000910fe1); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000910fe5); };
            c.w64(1, t20);
            let Some(t21) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000910fe9); };
            c.w64(5, t21);
            c.w64(12, c.r64(2));
            let Some(t22) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000910ff0); };
            c.w64(0, t22);
            let r23 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r23);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000910ffc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000911014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000911014); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000911018); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091101e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000911022); };
            c.w64(8, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000911026); };
            c.w64(11, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091102c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000911033); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000911037); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091103b); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000911042); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911048); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091104c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000911050); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000911054); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000911058); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091105c); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000911065); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w32(1, c.r32(0));
            c.w32(6, c.r32(11));
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091107b); };
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
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911098); };
            c.w32(1, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091109b); };
            c.cmp(t16, c.r32(1), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009110ab); }
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
            let Some(t19) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009110c7); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70009110da); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009110df); }
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
            let Some(t23) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911107); };
            c.w32(1, t23);
            let r24 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r24);
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091110e); };
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
            let Some(t26) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911121); };
            c.w32(1, t26);
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000911124); };
            c.cmp(t27, c.r32(1), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000911131); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000911137); }
            blk = 17; continue;
        }
        16 => {
            let r28 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r28);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000911140); }
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
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000911153); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000911166); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091116b); }
            blk = 20; continue;
        }
        20 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000911173); };
            c.w64(8, t31);
            let Some(t32) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700091117a); };
            c.w64(11, t32);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000911181); };
            c.w64(3, t33);
            let Some(t34) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000911188); };
            c.cmp(t34, c.r64(0), 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x7000911191); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 22; continue;
        }
        22 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091119c); };
            c.w64(11, t35);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70009111a3); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70009111a7); }
            blk = 45; continue;
        }
        23 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r36 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r36);
            let Some(t37) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009111b8); };
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
            let Some(t38) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009111e0); };
            c.w32(1, t38);
            let r39 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r39);
            let Some(t40) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009111e6); };
            c.cmp(t40, 0x41u64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 38; continue;
        }
        29 => {
            let Some(t41) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009111f6); };
            c.w32(1, t41);
            let r42 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r42);
            let Some(t43) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009111fc); };
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
            let Some(t44) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000911208); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(3)) { return RegfileExit::fault(0x7000911219); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091121e); }
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000911226); }
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000911230); };
            c.w64(6, t46);
            return c.call(0x700086f9d4, 0x7000911239, 0x7000911234);
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
            let Some(t47) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911247); };
            c.w32(1, t47);
            let Some(t48) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091124a); };
            c.cmp(t48, c.r32(1), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911253); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000911259); };
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
            let Some(t53) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091127b); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(3)) { return RegfileExit::fault(0x700091128f); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000911294); }
            blk = 43; continue;
        }
        43 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700091129c); }
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009112a6); };
            c.w64(6, t55);
            return c.call(0x70008ad794, 0x70009112af, 0x70009112aa);
        }
        44 => {
            c.w64(8, c.r64(0));
            blk = 45; continue;
        }
        45 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009112b2); };
            c.w64(7, t56);
            let Some(t57) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009112b6); };
            c.w64(7, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009112ba); };
            c.w64(11, t58);
            c.w64(9, c.r64(11).wrapping_add(0x5));
            let Some(t59) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009112c2); };
            c.w32(11, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009112c7); };
            c.w64(15, t60);
            let Some(t61) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009112cb); };
            c.w64(1, t61);
            let Some(t62) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009112cf); };
            c.w64(5, t62);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(8));
            let r63 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r63);
            return RegfileExit::jump(c.r64(1));
        }
        46 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70009112e4); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 1) { return RegfileExit::fault(0x70009112e9); }
            return c.call(0x70007febf4, 0x70009112f3, 0x70009112ee);
        }
        47 => {
            c.w64(8, c.r64(0));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009112f6); };
            c.w64(15, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009112fa); };
            c.w64(9, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009112fe); };
            c.w64(7, t66);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000911302); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x7000911307); }
            blk = 22; continue;
        }
        48 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000911311); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000911315); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000911319); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700091131d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000911321); }
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091132b); };
            c.w64(6, t67);
            return c.call(0x700089b334, 0x7000911334, 0x700091132f);
        }
        49 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t68) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000911338); };
            c.cmp(t68, 0x0u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091133e); };
            c.w64(8, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000911342); };
            c.w64(2, t70);
            let Some(t71) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000911346); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700091135a); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091135f); }
            blk = 53; continue;
        }
        53 => {
            c.w64(8, c.r64(0));
            blk = 45; continue;
        }
        54 => {
            let Some(t73) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091136f); };
            c.cmp(t73, 0x0u64, 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000911375); };
            c.w64(8, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000911379); };
            c.w64(7, t75);
            let Some(t76) = c.ld32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091137d); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000911392); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000911397); }
            blk = 57; continue;
        }
        57 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091139f); };
            c.w64(3, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009113a3); };
            c.w64(6, t79);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70009113a7); }
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009113b0); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70009113b1); };
            c.w64(3, t80);
            c.w32(0, 0x1u64);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009113bd); };
            c.w64(6, t81);
            return c.call(0x700086ed74, 0x70009113c6, 0x70009113c1);
        }
        58 => {
            return RegfileExit::fault(0x70009113c6);
        }
        59 => {
            return RegfileExit::jump(0x70009113c8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009113d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009113d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009113d8); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009113de); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009113e2); };
            c.w64(8, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009113e6); };
            c.w64(11, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009113ec); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009113f3); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009113f7); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009113fb); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000911402); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911408); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091140c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000911410); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000911414); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000911418); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091141c); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000911425); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w32(1, c.r32(11));
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091142f); };
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
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091144f); };
            c.w32(1, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000911452); };
            c.cmp(t16, c.r32(1), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000911462); }
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
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000911484); };
            c.w64(3, t19);
            let Some(t20) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700091148b); };
            c.w64(1, t20);
            let r21 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r21);
            let r22 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r22);
            c.w64(12, c.r64(1).wrapping_add(0xc));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000911497); };
            c.w64(15, t23);
            let Some(t24) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700091149e); };
            c.cmp(t24, c.r64(12), 64);
            if c.cc_be() { blk = 42; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(3), c.r64(12)) { return RegfileExit::fault(0x70009114a7); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 10; continue;
        }
        10 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            c.arith_sd(1, 1, c.xmm_u64(0));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009114c3); };
            c.w64(11, t25);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70009114ca); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70009114ce); }
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
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009114e4); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70009114fa); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009114ff); }
            blk = 41; continue;
        }
        14 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091150c); };
            c.w32(1, t28);
            let r29 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r29);
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000911513); };
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
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911524); };
            c.w32(1, t31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000911527); };
            c.cmp(t32, c.r32(1), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000911534); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091153a); }
            blk = 19; continue;
        }
        18 => {
            let r33 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r33);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000911543); }
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
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000911556); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000911569); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091156e); }
            blk = 22; continue;
        }
        22 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000911576); };
            c.w64(8, t36);
            let Some(t37) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700091157d); };
            c.w64(11, t37);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000911584); };
            c.w64(3, t38);
            let Some(t39) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700091158b); };
            c.cmp(t39, c.r64(0), 64);
            if c.cc_be() { blk = 44; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x7000911594); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 24; continue;
        }
        24 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091159f); };
            c.w64(11, t40);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70009115a6); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70009115aa); }
            c.w64(11, c.r64(8));
            blk = 41; continue;
        }
        25 => {
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r41 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r41);
            let Some(t42) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009115bf); };
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
            let Some(t43) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009115d3); };
            c.w32(1, t43);
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009115d6); };
            c.cmp(t44, c.r32(1), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(1, 0x1eu64);
            blk = 36; continue;
        }
        30 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009115e6); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            let Some(t47) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009115ec); };
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
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911605); };
            c.w32(1, t48);
            let r49 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r49);
            let Some(t50) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091160b); };
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
            let Some(t52) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000911627); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700091163b); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000911640); }
            blk = 39; continue;
        }
        39 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000911648); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000911652); };
            c.w64(0, t54);
            return c.call(0x70008adb14, 0x700091165b, 0x7000911656);
        }
        40 => {
            c.w64(11, c.r64(0));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091165e); };
            c.w64(9, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000911662); };
            c.w64(7, t56);
            blk = 41; continue;
        }
        41 => {
            let Some(t57) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000911666); };
            c.w64(7, t57);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t58) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000911671); };
            c.w32(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000911676); };
            c.w64(15, t59);
            let Some(t60) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091167a); };
            c.w64(1, t60);
            let Some(t61) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091167e); };
            c.w64(5, t61);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r62 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r62);
            return RegfileExit::jump(c.r64(1));
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700091168e); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000911697); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700091169b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091169f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70009116a3); }
            c.w64(3, c.r64(2));
            return c.call(0x70007febf4, 0x70009116af, 0x70009116aa);
        }
        43 => {
            c.w64(3, c.r64(0));
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009116b2); };
            c.w64(9, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009116b6); };
            c.w64(7, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009116ba); };
            c.w64(8, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009116be); };
            c.w64(2, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009116c2); };
            c.w64(6, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009116c6); };
            c.w64(11, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009116ca); };
            c.w64(0, t69);
            blk = 10; continue;
        }
        44 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x70009116d8); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x70009116dd); }
            return c.call(0x70007febf4, 0x70009116e7, 0x70009116e2);
        }
        45 => {
            c.w64(8, c.r64(0));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009116ea); };
            c.w64(15, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009116ee); };
            c.w64(9, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009116f2); };
            c.w64(7, t72);
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70009116f6); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x70009116fb); }
            blk = 24; continue;
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000911705); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000911709); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700091170d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000911711); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000911715); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t73) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091171e); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000911732); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000911737); }
            blk = 49; continue;
        }
        49 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700091173f); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000911749); };
            c.w64(0, t75);
            return c.call(0x700089bcd4, 0x7000911752, 0x700091174d);
        }
        50 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(11, c.r64(0));
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000911759); };
            c.w64(9, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091175d); };
            c.w64(7, t77);
            blk = 41; continue;
        }
        52 => {
            let Some(t78) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000911766); };
            c.cmp(t78, 0x0u64, 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091176c); };
            c.w64(8, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000911770); };
            c.w64(2, t80);
            let Some(t81) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000911774); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700091178b); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000911790); }
            blk = 55; continue;
        }
        55 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000911798); };
            c.w64(7, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700091179c); };
            c.w64(12, t84);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70009117a0); }
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009117a9); }
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70009117aa); };
            c.w64(3, t85);
            c.w32(0, 0x1u64);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009117b6); };
            c.w64(6, t86);
            return c.call(0x700086ed74, 0x70009117bf, 0x70009117ba);
        }
        56 => {
            return RegfileExit::fault(0x70009117bf);
        }
        57 => {
            return RegfileExit::jump(0x70009117c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009117d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009117d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009117d8); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009117de); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009117e2); };
            c.w64(8, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009117e6); };
            c.w64(11, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009117ec); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009117f3); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009117f7); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009117fb); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000911802); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911808); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091180c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000911810); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000911814); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000911818); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091181c); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000911825); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091182c); };
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
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911847); };
            c.w32(6, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091184a); };
            c.cmp(t16, c.r32(6), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091185a); }
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
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x70009118ad); };
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
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009118cf); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70009118e5); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009118ea); }
            blk = 46; continue;
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009118f7); };
            c.w32(6, t28);
            let r29 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r29);
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009118fe); };
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
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091190f); };
            c.w32(6, t31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000911912); };
            c.cmp(t32, c.r32(6), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700091191f); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000911925); }
            blk = 24; continue;
        }
        23 => {
            let r33 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r33);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700091192e); }
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
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000911941); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000911954); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000911959); }
            blk = 27; continue;
        }
        27 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000911961); };
            c.w64(8, t36);
            let Some(t37) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000911968); };
            c.w64(11, t37);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091196f); };
            c.w64(3, t38);
            let Some(t39) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000911976); };
            c.cmp(t39, c.r64(0), 64);
            if c.cc_be() { blk = 57; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700091197f); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 29; continue;
        }
        29 => {
            c.arith_sd(2, 0, c.xmm_u64(1));
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091198a); };
            c.w64(11, t40);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000911991); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000911995); }
            c.w64(11, c.r64(8));
            blk = 46; continue;
        }
        30 => {
            let Some(t41) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009119a3); };
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
            let Some(t43) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009119be); };
            c.w32(6, t43);
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009119c1); };
            c.cmp(t44, c.r32(6), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(6, 0x1eu64);
            blk = 41; continue;
        }
        35 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009119d1); };
            c.w32(6, t45);
            let r46 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r46);
            let Some(t47) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009119d7); };
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
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009119f0); };
            c.w32(6, t48);
            let r49 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r49);
            let Some(t50) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009119f6); };
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
            let Some(t52) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000911a12); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000911a26); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000911a2b); }
            blk = 44; continue;
        }
        44 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000911a33); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000911a3d); };
            c.w64(0, t54);
            return c.call(0x70008adcd4, 0x7000911a46, 0x7000911a41);
        }
        45 => {
            c.w64(11, c.r64(0));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000911a49); };
            c.w64(15, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000911a4d); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000911a51); };
            c.w64(7, t57);
            blk = 46; continue;
        }
        46 => {
            let Some(t58) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000911a55); };
            c.w64(7, t58);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t59) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000911a60); };
            c.w32(8, t59);
            let Some(t60) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000911a65); };
            c.w64(1, t60);
            let Some(t61) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000911a69); };
            c.w64(5, t61);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r62 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r62);
            return RegfileExit::jump(c.r64(1));
        }
        47 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000911a79); };
            c.w64(11, t63);
            let Some(t64) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000911a80); };
            c.w64(0, t64);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000911a87); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000911a8b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000911a8f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000911a93); }
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000911a97); };
            c.w64(6, t65);
            let Some(t66) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000911a9e); };
            c.cmp(t66, c.r64(3), 64);
            if c.cc_a() { blk = 50; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x7000911ab0, 0x7000911aab);
        }
        49 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000911ab0); };
            c.w64(12, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000911ab4); };
            c.w64(15, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000911ab8); };
            c.w64(9, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000911abc); };
            c.w64(7, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000911ac0); };
            c.w64(8, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000911ac4); };
            c.w64(2, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000911ac8); };
            c.w64(1, t73);
            blk = 51; continue;
        }
        50 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x7000911ace); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 51; continue;
        }
        51 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000911ae1); };
            c.w64(11, t74);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000911ae8); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000911aec); }
            c.w64(11, c.r64(0));
            blk = 13; continue;
        }
        52 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000911af9); };
            c.w64(11, t75);
            let Some(t76) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000911b00); };
            c.w64(0, t76);
            c.w64(12, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000911b07); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000911b0b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000911b0f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000911b13); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000911b17); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000911b1b); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000911b1f); };
            c.w64(0, t77);
            let Some(t78) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000911b26); };
            c.cmp(t78, c.r64(12), 64);
            if c.cc_a() { blk = 55; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x7000911b38, 0x7000911b33);
        }
        54 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000911b38); };
            c.w64(3, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000911b3c); };
            c.w64(15, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000911b40); };
            c.w64(9, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000911b44); };
            c.w64(7, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000911b48); };
            c.w64(8, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000911b4c); };
            c.w64(2, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000911b50); };
            c.w64(1, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000911b54); };
            c.w64(6, t86);
            blk = 56; continue;
        }
        55 => {
            if !c.st64(c.r64(11), c.r64(12)) { return RegfileExit::fault(0x7000911b5a); }
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000911b5d); };
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
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000911b7c); };
            c.w64(11, t89);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000911b83); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000911b87); }
            c.w64(11, c.r64(0));
            blk = 13; continue;
        }
        57 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000911b99); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 1) { return RegfileExit::fault(0x7000911b9e); }
            return c.call(0x70007febf4, 0x7000911ba8, 0x7000911ba3);
        }
        58 => {
            c.w64(8, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000911bab); };
            c.w64(15, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000911baf); };
            c.w64(9, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000911bb3); };
            c.w64(7, t92);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000911bb7); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x7000911bbc); }
            blk = 29; continue;
        }
        59 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 62; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t93) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000911bcb); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x7000911bdc); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000911be1); }
            blk = 62; continue;
        }
        62 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000911be9); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000911bed); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000911bef); }
            c.w32(8, 0x4u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000911bf6); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x7000911c00); };
            c.w64(3, t95);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000911c0f, 0x7000911c0a);
        }
        63 => {
            c.w64(11, c.r64(0));
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000911c12); };
            c.w64(15, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000911c16); };
            c.w64(9, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000911c1a); };
            c.w64(7, t98);
            blk = 46; continue;
        }
        64 => {
            return RegfileExit::jump(0x7000911c24);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000911c34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000911c34); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000911c38); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000911c3e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000911c42); };
            c.w64(8, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000911c46); };
            c.w64(11, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000911c4c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000911c53); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000911c57); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000911c5b); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000911c62); };
            c.w32(1, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911c68); };
            c.w32(2, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000911c6c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000911c70); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000911c74); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000911c78); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000911c7c); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000911c85); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000911c8c); };
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
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911ca9); };
            c.w32(2, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000911cac); };
            c.cmp(t16, c.r32(2), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000911cbc); }
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
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000911d15); };
            c.w64(11, t21);
            let Some(t22) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000911d1c); };
            c.w64(0, t22);
            c.w64(2, c.r64(0).wrapping_add(0xc));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000911d23); };
            c.w64(15, t23);
            let Some(t24) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000911d2a); };
            c.cmp(t24, c.r64(2), 64);
            if c.cc_be() { blk = 47; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x7000911d33); }
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
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000911d52); };
            c.w64(0, t25);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000911d59); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x7000911d5d); }
            c.w32(0, 0x6u64);
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000911d71); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000911d87); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000911d8c); }
            blk = 46; continue;
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911d99); };
            c.w32(2, t28);
            let r29 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r29);
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000911da0); };
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
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911db1); };
            c.w32(2, t31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000911db4); };
            c.cmp(t32, c.r32(2), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000911dc1); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000911dc7); }
            blk = 24; continue;
        }
        23 => {
            let r33 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r33);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000911dd0); }
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
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000911de3); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000911df6); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000911dfb); }
            blk = 27; continue;
        }
        27 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000911e03); };
            c.w64(8, t36);
            let Some(t37) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000911e0a); };
            c.w64(11, t37);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000911e11); };
            c.w64(3, t38);
            let Some(t39) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000911e18); };
            c.cmp(t39, c.r64(0), 64);
            if c.cc_be() { blk = 49; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x7000911e21); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 29; continue;
        }
        29 => {
            c.arith_sd(3, 1, c.xmm_u64(0));
            c.mov128_rr(1, 1);
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000911e2f); };
            c.w64(11, t40);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000911e36); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x7000911e3a); }
            c.w64(11, c.r64(8));
            blk = 46; continue;
        }
        30 => {
            let Some(t41) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000911e48); };
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
            let Some(t43) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911e63); };
            c.w32(2, t43);
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000911e66); };
            c.cmp(t44, c.r32(2), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(2, 0x1eu64);
            blk = 41; continue;
        }
        35 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911e76); };
            c.w32(2, t45);
            let r46 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r46);
            let Some(t47) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000911e7c); };
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
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000911e95); };
            c.w32(2, t48);
            let r49 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r49);
            let Some(t50) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000911e9b); };
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
            let Some(t52) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000911eb7); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000911ecb); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000911ed0); }
            blk = 44; continue;
        }
        44 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000911ed8); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000911ee2); };
            c.w64(0, t54);
            return c.call(0x70008adf94, 0x7000911eeb, 0x7000911ee6);
        }
        45 => {
            c.w64(11, c.r64(0));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000911eee); };
            c.w64(9, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000911ef2); };
            c.w64(7, t56);
            blk = 46; continue;
        }
        46 => {
            let Some(t57) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000911ef6); };
            c.w64(7, t57);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t58) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000911f01); };
            c.w32(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000911f06); };
            c.w64(15, t59);
            let Some(t60) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000911f0a); };
            c.w64(1, t60);
            let Some(t61) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000911f0e); };
            c.w64(5, t61);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r62 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r62);
            return RegfileExit::jump(c.r64(1));
        }
        47 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000911f23); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000911f27); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x7000911f2b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000911f2f); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000911f33); }
            return c.call(0x70007febf4, 0x7000911f3c, 0x7000911f37);
        }
        48 => {
            c.w64(11, c.r64(0));
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000911f3f); };
            c.w64(9, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000911f43); };
            c.w64(7, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000911f47); };
            c.w64(8, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000911f4b); };
            c.w64(1, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000911f4f); };
            c.w64(6, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000911f53); };
            c.w64(12, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000911f57); };
            c.w64(3, t69);
            blk = 15; continue;
        }
        49 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x7000911f65); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x7000911f6a); }
            return c.call(0x70007febf4, 0x7000911f74, 0x7000911f6f);
        }
        50 => {
            c.w64(8, c.r64(0));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000911f77); };
            c.w64(15, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000911f7b); };
            c.w64(9, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000911f7f); };
            c.w64(7, t72);
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000911f83); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x7000911f88); }
            blk = 29; continue;
        }
        51 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t73) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000911f97); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000911fa8); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000911fad); }
            blk = 54; continue;
        }
        54 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000911fb5); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000911fb9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000911fbb); }
            c.w32(8, 0x6u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000911fc2); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x7000911fcc); };
            c.w64(3, t75);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000911fdb, 0x7000911fd6);
        }
        55 => {
            c.w64(11, c.r64(0));
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000911fde); };
            c.w64(9, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000911fe2); };
            c.w64(7, t77);
            blk = 46; continue;
        }
        56 => {
            return RegfileExit::jump(0x7000911fec);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000911ff4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000911ff4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000911ff8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000911ffe); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000912002); };
            c.w64(8, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000912006); };
            c.w64(11, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091200c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000912013); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000912017); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091201b); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000912022); };
            c.w32(1, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000912028); };
            c.w32(2, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091202c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000912030); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000912034); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000912038); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091203c); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000912045); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091204c); };
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
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000912069); };
            c.w32(2, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091206c); };
            c.cmp(t16, c.r32(2), 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091207c); }
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
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009120cf); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70009120e5); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009120ea); }
            blk = 44; continue;
        }
        15 => {
            let Some(t22) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009120f7); };
            c.w32(2, t22);
            let r23 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009120fe); };
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
            let Some(t25) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091210f); };
            c.w32(2, t25);
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000912112); };
            c.cmp(t26, c.r32(2), 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700091211f); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000912125); }
            blk = 20; continue;
        }
        19 => {
            let r27 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r27);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700091212e); }
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
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000912141); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000912154); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000912159); }
            blk = 23; continue;
        }
        23 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000912161); };
            c.w64(8, t30);
            let Some(t31) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000912168); };
            c.w64(11, t31);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091216f); };
            c.w64(3, t32);
            let Some(t33) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000912176); };
            c.cmp(t33, c.r64(0), 64);
            if c.cc_be() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700091217f); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 25; continue;
        }
        25 => {
            let r34 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r34);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700091218a); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x700091218f); }
            if !c.movsd_mr(c.r64(4), 1) { return RegfileExit::fault(0x7000912192); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x7000912197); }
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
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009121a6); }
            if !c.popfq() { return RegfileExit::fault(0x70009121a7); }
            if c.cc_p() { blk = 26; continue; }
            blk = 27; continue;
        }
        27 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x70009121ac); }
            if !c.movsd_rm(1, c.r64(4)) { return RegfileExit::fault(0x70009121af); }
            let r37 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r37);
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009121b8); };
            c.w64(11, t38);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70009121bf); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70009121c3); }
            c.w64(11, c.r64(8));
            blk = 44; continue;
        }
        28 => {
            let Some(t39) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009121d1); };
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
            let Some(t41) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009121ec); };
            c.w32(2, t41);
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009121ef); };
            c.cmp(t42, c.r32(2), 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(2, 0x1eu64);
            blk = 39; continue;
        }
        33 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009121ff); };
            c.w32(2, t43);
            let r44 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r44);
            let Some(t45) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000912205); };
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
            let Some(t46) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091221e); };
            c.w32(2, t46);
            let r47 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r47);
            let Some(t48) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000912224); };
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
            let Some(t50) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000912240); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000912254); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000912259); }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000912261); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091226b); };
            c.w64(0, t52);
            return c.call(0x70008ae1b4, 0x7000912274, 0x700091226f);
        }
        43 => {
            c.w64(11, c.r64(0));
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000912277); };
            c.w64(15, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091227b); };
            c.w64(9, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091227f); };
            c.w64(7, t55);
            blk = 44; continue;
        }
        44 => {
            let Some(t56) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000912283); };
            c.w64(7, t56);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t57) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091228e); };
            c.w32(8, t57);
            let Some(t58) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000912293); };
            c.w64(1, t58);
            let Some(t59) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000912297); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70009122ad); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70009122b1); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70009122b5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009122b9); }
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
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009122d6); };
            c.w64(11, t62);
            let Some(t63) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009122dd); };
            c.w64(0, t63);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009122e4); };
            c.w64(12, t64);
            let Some(t65) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70009122eb); };
            c.cmp(t65, c.r64(3), 64);
            if c.cc_a() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70009122fe, 0x70009122f9);
        }
        50 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009122fe); };
            c.w64(2, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000912302); };
            c.w64(15, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000912306); };
            c.w64(9, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091230a); };
            c.w64(7, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091230e); };
            c.w64(8, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000912312); };
            c.w64(1, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000912316); };
            c.w64(6, t72);
            blk = 52; continue;
        }
        51 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700091231c); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 52; continue;
        }
        52 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091232e); };
            c.w64(11, t73);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000912335); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000912339); }
            blk = 53; continue;
        }
        53 => {
            c.w64(11, c.r64(0));
            blk = 9; continue;
        }
        54 => {
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x7000912346); };
            c.w64(11, t74);
            blk = 9; continue;
        }
        55 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x7000912352); };
            c.w64(11, t75);
            blk = 9; continue;
        }
        56 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000912363); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 1) { return RegfileExit::fault(0x7000912368); }
            return c.call(0x70007febf4, 0x7000912372, 0x700091236d);
        }
        57 => {
            c.w64(8, c.r64(0));
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000912375); };
            c.w64(15, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000912379); };
            c.w64(9, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091237d); };
            c.w64(7, t78);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000912381); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x7000912386); }
            blk = 25; continue;
        }
        58 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t79) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000912395); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x70009123a6); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009123ab); }
            blk = 61; continue;
        }
        61 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70009123b3); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70009123b7); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009123b9); }
            c.w32(8, 0x8u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009123c0); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70009123ca); };
            c.w64(3, t81);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70009123d9, 0x70009123d4);
        }
        62 => {
            c.w64(11, c.r64(0));
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009123dc); };
            c.w64(15, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009123e0); };
            c.w64(9, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009123e4); };
            c.w64(7, t84);
            blk = 44; continue;
        }
        63 => {
            return RegfileExit::jump(0x70009123f0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009123f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009123f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009123f8); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009123fe); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000912402); };
            c.w64(8, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000912406); };
            c.w64(11, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091240c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000912413); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000912417); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091241b); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000912422); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000912428); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091242c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000912430); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000912434); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000912438); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091243c); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000912440); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000912449); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000912450); };
            c.cmp(t14, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700091245d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000912461); }
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
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000912473); };
            c.w32(6, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000912476); };
            c.cmp(t16, c.r32(6), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(2, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000912486); }
            c.xorpd_rr(3, 3);
            c.cvtsi2sd(3, (c.r32(11) as u32 as i32 as i64));
            blk = 17; continue;
        }
        6 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000912499); };
            c.w64(3, t18);
            let Some(t19) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70009124a0); };
            c.w64(6, t19);
            let r20 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r20);
            let r21 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r21);
            c.w64(12, c.r64(6).wrapping_add(0xc));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009124ac); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70009124b3); };
            c.cmp(t23, c.r64(12), 64);
            if c.cc_be() { blk = 40; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(3), c.r64(12)) { return RegfileExit::fault(0x70009124bc); }
            c.w64(7, c.r64(6).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009124d4); };
            c.w64(6, t24);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70009124db); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70009124de); }
            c.w64(10, c.r64(4));
            let r25 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r25);
            let r26 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r26);
            c.logic(r26, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70009124ed); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x70009124f2); };
            c.w64(0, t27);
            c.w64(10, 0x7000912500u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000912500); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000912504); }
            return c.call(c.r64(0), 0x700091250a, 0x7000912508);
        }
        9 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700091250a); }
            let Some(t28) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000912512); };
            c.w64(4, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000912517); };
            c.w64(0, t29);
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700091251b); }
            let Some(t30) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000912520); };
            c.cmp(t30, 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091252a); };
            c.w64(8, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091252e); };
            c.w64(7, t32);
            let Some(t33) = c.ld32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000912532); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000912549); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091254e); }
            blk = 39; continue;
        }
        12 => {
            let Some(t35) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091255b); };
            c.w32(6, t35);
            let r36 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r36);
            let Some(t37) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000912562); };
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
            let Some(t38) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000912573); };
            c.w32(6, t38);
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000912576); };
            c.cmp(t39, c.r32(6), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.movsd_rm(3, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000912583); }
            if !c.movsd_rm(2, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000912589); }
            blk = 17; continue;
        }
        16 => {
            let r40 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r40);
            if !c.movsd_rm(3, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000912592); }
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
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009125a5); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70009125b8); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009125bd); }
            blk = 20; continue;
        }
        20 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009125c5); };
            c.w64(8, t43);
            let Some(t44) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70009125cc); };
            c.w64(11, t44);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009125d3); };
            c.w64(3, t45);
            let Some(t46) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70009125da); };
            c.cmp(t46, c.r64(0), 64);
            if c.cc_be() { blk = 42; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70009125e3); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 22; continue;
        }
        22 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009125ea); };
            c.w64(11, t47);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70009125f1); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009125f5); }
            c.mov128_rr(0, 3);
            c.mov128_rr(1, 2);
            c.w64(10, c.r64(4));
            let r48 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r48);
            let r49 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r49);
            c.logic(r49, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700091260a); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x700091260f); };
            c.w64(0, t50);
            c.w64(10, 0x700091261du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700091261d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000912621); }
            return c.call(c.r64(0), 0x7000912627, 0x7000912625);
        }
        23 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000912627); }
            let Some(t51) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700091262f); };
            c.w64(4, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000912634); };
            c.w64(0, t52);
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000912638); }
            blk = 39; continue;
        }
        24 => {
            let Some(t53) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000912642); };
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
            let Some(t55) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091265d); };
            c.w32(6, t55);
            let Some(t56) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000912660); };
            c.cmp(t56, c.r32(6), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(6, 0x1eu64);
            blk = 35; continue;
        }
        29 => {
            let Some(t57) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000912670); };
            c.w32(6, t57);
            let r58 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r58);
            let Some(t59) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000912676); };
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
            let Some(t60) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091268f); };
            c.w32(6, t60);
            let r61 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r61);
            let Some(t62) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000912695); };
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
            let Some(t64) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009126b1); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70009126c5); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009126ca); }
            blk = 38; continue;
        }
        38 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70009126d2); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009126dc); };
            c.w64(0, t66);
            return c.call(0x70008ae454, 0x70009126e5, 0x70009126e0);
        }
        39 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009126e5); };
            c.w64(7, t67);
            let Some(t68) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009126e9); };
            c.w64(7, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009126ed); };
            c.w64(8, t69);
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t70) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009126f5); };
            c.w32(8, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009126fa); };
            c.w64(15, t71);
            let Some(t72) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009126fe); };
            c.w64(1, t72);
            let Some(t73) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000912702); };
            c.w64(5, t73);
            c.w64(12, c.r64(7));
            let r74 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r74);
            return RegfileExit::jump(c.r64(1));
        }
        40 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000912714); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000912718); }
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x7000912724, 0x700091271f);
        }
        41 => {
            c.w64(7, c.r64(0));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000912727); };
            c.w64(11, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700091272b); };
            c.w64(0, t76);
            blk = 8; continue;
        }
        42 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 2) { return RegfileExit::fault(0x7000912739); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 3) { return RegfileExit::fault(0x700091273e); }
            return c.call(0x70007febf4, 0x7000912748, 0x7000912743);
        }
        43 => {
            c.w64(8, c.r64(0));
            if !c.movsd_rm(2, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x700091274b); }
            if !c.movsd_rm(3, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x7000912750); }
            blk = 22; continue;
        }
        44 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t77) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091275f); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x7000912770); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000912775); }
            blk = 47; continue;
        }
        47 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700091277d); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000912781); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000912783); }
            c.w32(8, 0xau64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091278a); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x7000912794); };
            c.w64(3, t79);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70009127a3, 0x700091279e);
        }
        48 => {
            blk = 39; continue;
        }
        49 => {
            return RegfileExit::jump(0x70009127ac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009127b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009127b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009127b8); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009127be); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009127c2); };
            c.w64(8, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009127c6); };
            c.w64(11, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009127cc); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009127d3); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009127d7); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009127db); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009127e2); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009127e8); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009127ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009127f0); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70009127f4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009127f8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70009127fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000912800); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000912804); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000912808); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000912811); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r14 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r14);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091281e); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x7000912824); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x7000912828); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700091282e); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091283b); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000912843); };
            c.cmp(t17, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t18) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000912850); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000912863); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000912872, 0x700091286d);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091287b); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091287f); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000912883); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000912887); };
            c.w64(2, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091288b); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700091288f); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000912893); };
            c.w64(1, t25);
            blk = 3; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000912899); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009128ac); };
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
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009128bf); };
            c.w32(8, t31);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70009128c3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009128cf); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009128d6); };
            c.cmp(t34, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009128df); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009128f5); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70009128f9); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000912905, 0x7000912900);
        }
        17 => {
            c.w32(11, 0xfeu64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700091290b); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091290f); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000912913); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000912917); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091291b); };
            c.w64(2, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091291f); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000912923); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000912927); };
            c.w64(8, t43);
            blk = 22; continue;
        }
        18 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091292d); };
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
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000912963); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700091297c); };
            c.w32(8, t47);
            let r48 = c.r32(8) | 0xeu64;
            c.w32(8, r48);
            c.logic(r48, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000912987); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700091298b); }
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
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000912998); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091299e); };
            c.cmp(t51, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t52) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009129ab); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009129bb); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70009129c7, 0x70009129c2);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70009129cc); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009129d0); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009129d4); };
            c.w64(3, t55);
            blk = 26; continue;
        }
        32 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009129da); };
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
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000912a02); };
            c.w64(8, t61);
            c.w64(1, c.r64(11));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000912a09); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000912a18); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000912a1c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000912a1e); }
            c.w32(11, 0xeu64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000912a25); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000912a27); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x7000912a33); };
            c.w64(3, t64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000912a42, 0x7000912a3d);
        }
        36 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000912a42); };
            c.w64(8, t65);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000912a48); }
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
            let Some(t67) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000912a68); };
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
            let Some(t70) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000912a8f); };
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
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000912a9e); };
            c.w64(7, t73);
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000912aa2); };
            c.cmp(t74, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000912aab); };
            c.w64(11, t75);
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000912aaf); };
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
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x7000912ac4); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000912ac9); }
            blk = 48; continue;
        }
        48 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000912ad0); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000912ad4); };
            c.w64(7, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000912ad8); };
            c.w64(8, t80);
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t81) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000912ae0); };
            c.w32(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000912ae5); };
            c.w64(15, t82);
            let Some(t83) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000912ae9); };
            c.w64(1, t83);
            let Some(t84) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000912aed); };
            c.w64(5, t84);
            c.w64(12, c.r64(7));
            let r85 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r85);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000912afa); };
            c.w64(11, t86);
            let Some(t87) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000912b01); };
            c.w64(3, t87);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000912b08); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x7000912b0c); }
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000912b10); };
            c.w64(1, t88);
            let Some(t89) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000912b17); };
            c.cmp(t89, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000912b26, 0x7000912b21);
        }
        51 => {
            c.w64(11, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000912b29); };
            c.w64(7, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000912b2d); };
            c.w64(8, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000912b31); };
            c.w64(0, t92);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x7000912b37); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000912b3e); };
            c.w64(3, t93);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000912b45); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000912b51); }
            c.w64(0, c.r64(11));
            blk = 41; continue;
        }
        54 => {
            let r94 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r94);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000912b63); }
            return c.call(0x70008708b4, 0x7000912b6d, 0x7000912b68);
        }
        55 => {
            let Some(t95) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000912b6d); };
            c.w32(0, t95);
            let r96 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            blk = 38; continue;
        }
        56 => {
            let r97 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r97);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000912b7d); }
            return c.call(0x70008708b4, 0x7000912b87, 0x7000912b82);
        }
        57 => {
            let Some(t98) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000912b87); };
            c.w32(12, t98);
            let r99 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r99);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x7000912b98);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000912bb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000912bb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000912bb8); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000912bbe); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000912bc2); };
            c.w64(8, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000912bc6); };
            c.w64(11, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000912bcc); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000912bd3); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000912bd7); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000912bdb); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000912be2); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000912be8); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000912bec); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000912bf0); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000912bf4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000912bf8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000912bfc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000912c00); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000912c04); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000912c08); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000912c11); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r14 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r14);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000912c1e); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x7000912c24); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x7000912c28); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x7000912c2e); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000912c3b); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000912c43); };
            c.cmp(t17, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t18) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000912c50); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000912c63); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000912c72, 0x7000912c6d);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000912c7b); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000912c7f); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000912c83); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000912c87); };
            c.w64(2, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000912c8b); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000912c8f); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000912c93); };
            c.w64(1, t25);
            blk = 3; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000912c99); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000912cac); };
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
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000912cbf); };
            c.w32(8, t31);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000912cc3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000912ccf); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000912cd6); };
            c.cmp(t34, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000912cdf); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000912cf5); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000912cf9); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000912d05, 0x7000912d00);
        }
        17 => {
            c.w32(6, 0xfeu64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000912d0a); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000912d0e); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000912d12); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000912d16); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000912d1a); };
            c.w64(2, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000912d1e); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000912d22); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000912d26); };
            c.w64(8, t43);
            blk = 22; continue;
        }
        18 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000912d2c); };
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
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000912d61); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000912d7a); };
            c.w32(8, t47);
            let r48 = c.r32(8) | 0xeu64;
            c.w32(8, r48);
            c.logic(r48, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000912d85); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000912d89); }
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
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000912d96); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000912d9c); };
            c.cmp(t51, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t52) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000912da9); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000912db9); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000912dc5, 0x7000912dc0);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000912dca); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000912dce); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000912dd2); };
            c.w64(3, t55);
            blk = 26; continue;
        }
        32 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000912dd8); };
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
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000912e00); };
            c.w64(8, t61);
            c.w64(1, c.r64(11));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000912e07); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000912e16); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000912e1a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000912e1c); }
            c.w32(11, 0x10u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000912e23); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000912e25); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x7000912e31); };
            c.w64(3, t64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000912e40, 0x7000912e3b);
        }
        36 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000912e40); };
            c.w64(8, t65);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000912e46); }
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
            let Some(t67) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000912e66); };
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
            let Some(t70) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000912e8b); };
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
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000912e9a); };
            c.w64(7, t73);
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000912e9e); };
            c.cmp(t74, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000912ea7); };
            c.w64(11, t75);
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000912eab); };
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
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x7000912ec0); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000912ec5); }
            blk = 48; continue;
        }
        48 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000912ecc); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000912ed0); };
            c.w64(7, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000912ed4); };
            c.w64(8, t80);
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t81) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000912edc); };
            c.w32(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000912ee1); };
            c.w64(15, t82);
            let Some(t83) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000912ee5); };
            c.w64(1, t83);
            let Some(t84) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000912ee9); };
            c.w64(5, t84);
            c.w64(12, c.r64(7));
            let r85 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r85);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000912ef6); };
            c.w64(11, t86);
            let Some(t87) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000912efd); };
            c.w64(3, t87);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000912f04); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x7000912f08); }
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000912f0c); };
            c.w64(1, t88);
            let Some(t89) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000912f13); };
            c.cmp(t89, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000912f22, 0x7000912f1d);
        }
        51 => {
            c.w64(11, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000912f25); };
            c.w64(7, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000912f29); };
            c.w64(8, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000912f2d); };
            c.w64(0, t92);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x7000912f33); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000912f3a); };
            c.w64(3, t93);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000912f41); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000912f4d); }
            c.w64(0, c.r64(11));
            blk = 41; continue;
        }
        54 => {
            let r94 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r94);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000912f5f); }
            return c.call(0x70008708b4, 0x7000912f69, 0x7000912f64);
        }
        55 => {
            let Some(t95) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000912f69); };
            c.w32(0, t95);
            let r96 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            blk = 38; continue;
        }
        56 => {
            let r97 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r97);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000912f79); }
            return c.call(0x70008708b4, 0x7000912f83, 0x7000912f7e);
        }
        57 => {
            let Some(t98) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000912f83); };
            c.w32(12, t98);
            let r99 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r99);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x7000912f94);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000912fb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000912fb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000912fb8); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000912fbe); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000912fc2); };
            c.w64(8, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000912fc6); };
            c.w64(11, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000912fcc); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000912fd3); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000912fd7); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000912fdb); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000912fe2); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000912fe8); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000912fec); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000912ff0); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000912ff4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000912ff8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000912ffc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000913000); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000913004); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000913008); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000913011); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r14 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r14);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091301e); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x7000913024); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x7000913028); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700091302e); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091303b); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000913043); };
            c.cmp(t17, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t18) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000913050); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000913063); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000913072, 0x700091306d);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091307b); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091307f); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000913083); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000913087); };
            c.w64(2, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091308b); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700091308f); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000913093); };
            c.w64(1, t25);
            blk = 3; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000913099); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009130ac); };
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
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009130bf); };
            c.w32(8, t31);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70009130c3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009130cf); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009130d6); };
            c.cmp(t34, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009130df); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009130f5); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70009130f9); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000913105, 0x7000913100);
        }
        17 => {
            c.w32(6, 0xfeu64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700091310a); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091310e); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000913112); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000913116); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091311a); };
            c.w64(2, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091311e); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000913122); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000913126); };
            c.w64(8, t43);
            blk = 22; continue;
        }
        18 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091312c); };
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
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000913161); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700091317a); };
            c.w32(8, t47);
            let r48 = c.r32(8) | 0xeu64;
            c.w32(8, r48);
            c.logic(r48, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000913185); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000913189); }
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
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000913196); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091319c); };
            c.cmp(t51, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t52) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009131a9); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009131b9); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70009131c5, 0x70009131c0);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70009131ca); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009131ce); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009131d2); };
            c.w64(3, t55);
            blk = 26; continue;
        }
        32 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009131d8); };
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
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000913200); };
            c.w64(8, t61);
            c.w64(1, c.r64(11));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000913207); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000913216); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700091321a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091321c); }
            c.w32(11, 0xcu64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000913223); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000913225); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x7000913231); };
            c.w64(3, t64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000913240, 0x700091323b);
        }
        36 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000913240); };
            c.w64(8, t65);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000913246); }
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
            let Some(t67) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000913266); };
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
            let Some(t70) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091328b); };
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
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091329a); };
            c.w64(7, t73);
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091329e); };
            c.cmp(t74, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009132a7); };
            c.w64(11, t75);
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009132ab); };
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
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70009132c0); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009132c5); }
            blk = 48; continue;
        }
        48 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009132cc); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009132d0); };
            c.w64(7, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009132d4); };
            c.w64(8, t80);
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t81) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009132dc); };
            c.w32(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009132e1); };
            c.w64(15, t82);
            let Some(t83) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009132e5); };
            c.w64(1, t83);
            let Some(t84) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009132e9); };
            c.w64(5, t84);
            c.w64(12, c.r64(7));
            let r85 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r85);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009132f6); };
            c.w64(11, t86);
            let Some(t87) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009132fd); };
            c.w64(3, t87);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000913304); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x7000913308); }
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091330c); };
            c.w64(1, t88);
            let Some(t89) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000913313); };
            c.cmp(t89, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000913322, 0x700091331d);
        }
        51 => {
            c.w64(11, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000913325); };
            c.w64(7, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000913329); };
            c.w64(8, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091332d); };
            c.w64(0, t92);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x7000913333); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091333a); };
            c.w64(3, t93);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000913341); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700091334d); }
            c.w64(0, c.r64(11));
            blk = 41; continue;
        }
        54 => {
            let r94 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r94);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700091335f); }
            return c.call(0x70008708b4, 0x7000913369, 0x7000913364);
        }
        55 => {
            let Some(t95) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000913369); };
            c.w32(0, t95);
            let r96 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            blk = 38; continue;
        }
        56 => {
            let r97 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r97);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000913379); }
            return c.call(0x70008708b4, 0x7000913383, 0x700091337e);
        }
        57 => {
            let Some(t98) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000913383); };
            c.w32(12, t98);
            let r99 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r99);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x7000913394);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009133b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009133b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009133b8); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009133be); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009133c2); };
            c.w64(8, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009133c6); };
            c.w64(11, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009133cc); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009133d3); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009133d7); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009133db); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009133e2); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009133e8); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009133ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009133f0); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70009133f4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009133f8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70009133fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000913400); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x7000913404); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000913408); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000913411); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r14 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r14);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091341e); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x7000913424); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000913428); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700091342e); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091343b); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000913443); };
            c.cmp(t17, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t18) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000913450); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000913463); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000913472, 0x700091346d);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091347b); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091347f); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000913483); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000913487); };
            c.w64(2, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091348b); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700091348f); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000913493); };
            c.w64(1, t25);
            blk = 3; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000913499); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009134ac); };
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
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009134bf); };
            c.w32(8, t31);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70009134c3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009134cf); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009134d6); };
            c.cmp(t34, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009134df); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009134f5); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70009134f9); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000913505, 0x7000913500);
        }
        17 => {
            c.w32(6, 0xfeu64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700091350a); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091350e); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000913512); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000913516); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091351a); };
            c.w64(2, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091351e); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000913522); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000913526); };
            c.w64(8, t43);
            blk = 22; continue;
        }
        18 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091352c); };
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
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000913561); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091357a); };
            c.w32(8, t47);
            let r48 = c.r32(8) | 0xeu64;
            c.w32(8, r48);
            c.logic(r48, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000913585); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000913589); }
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
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000913596); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091359c); };
            c.cmp(t51, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t52) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009135a9); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009135b9); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70009135c5, 0x70009135c0);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70009135ca); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009135ce); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009135d2); };
            c.w64(3, t55);
            blk = 26; continue;
        }
        32 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009135d8); };
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
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000913600); };
            c.w64(8, t61);
            c.w64(1, c.r64(11));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000913607); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000913616); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700091361a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091361c); }
            c.w32(11, 0x12u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000913623); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000913625); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x7000913631); };
            c.w64(3, t64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000913640, 0x700091363b);
        }
        36 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000913640); };
            c.w64(8, t65);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000913646); }
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
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000913669); };
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
            let Some(t70) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000913692); };
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
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009136a1); };
            c.w64(7, t73);
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009136a5); };
            c.cmp(t74, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009136ae); };
            c.w64(11, t75);
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009136b2); };
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
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70009136c7); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009136cc); }
            blk = 48; continue;
        }
        48 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009136d3); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009136d7); };
            c.w64(7, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009136db); };
            c.w64(8, t80);
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t81) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009136e3); };
            c.w32(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009136e8); };
            c.w64(15, t82);
            let Some(t83) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009136ec); };
            c.w64(1, t83);
            let Some(t84) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009136f0); };
            c.w64(5, t84);
            c.w64(12, c.r64(7));
            let r85 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r85);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009136fd); };
            c.w64(0, t86);
            let Some(t87) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000913704); };
            c.w64(3, t87);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700091370b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700091370f); }
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000913713); };
            c.w64(1, t88);
            let Some(t89) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700091371a); };
            c.cmp(t89, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000913729, 0x7000913724);
        }
        51 => {
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000913729); };
            c.w64(11, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091372d); };
            c.w64(7, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000913731); };
            c.w64(8, t92);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x7000913737); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091373e); };
            c.w64(3, t93);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000913745); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000913751); }
            blk = 41; continue;
        }
        54 => {
            let r94 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r94);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700091375f); }
            return c.call(0x70008708b4, 0x7000913769, 0x7000913764);
        }
        55 => {
            let Some(t95) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000913769); };
            c.w32(0, t95);
            let r96 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            blk = 38; continue;
        }
        56 => {
            let r97 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r97);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000913779); }
            return c.call(0x70008708b4, 0x7000913783, 0x700091377e);
        }
        57 => {
            let Some(t98) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000913783); };
            c.w32(12, t98);
            let r99 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r99);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x7000913794);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009137b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009137b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009137b8); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009137be); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009137c2); };
            c.w64(8, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009137c6); };
            c.w64(11, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009137cc); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009137d3); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009137d7); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009137db); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009137e2); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009137e8); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009137ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009137f0); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70009137f4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009137f8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70009137fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000913800); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x7000913804); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000913808); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000913811); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r14 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r14);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091381e); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x7000913824); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000913828); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700091382e); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091383b); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000913843); };
            c.cmp(t17, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t18) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000913850); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000913863); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000913872, 0x700091386d);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091387b); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091387f); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000913883); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000913887); };
            c.w64(2, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091388b); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700091388f); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000913893); };
            c.w64(1, t25);
            blk = 3; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000913899); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009138ac); };
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
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009138bf); };
            c.w32(8, t31);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70009138c3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009138cf); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009138d6); };
            c.cmp(t34, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009138df); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009138f5); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70009138f9); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000913905, 0x7000913900);
        }
        17 => {
            c.w32(6, 0xfeu64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700091390a); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091390e); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000913912); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000913916); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091391a); };
            c.w64(2, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091391e); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000913922); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000913926); };
            c.w64(8, t43);
            blk = 22; continue;
        }
        18 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700091392c); };
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
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000913961); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091397a); };
            c.w32(8, t47);
            let r48 = c.r32(8) | 0xeu64;
            c.w32(8, r48);
            c.logic(r48, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000913985); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000913989); }
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
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000913996); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091399c); };
            c.cmp(t51, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t52) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009139a9); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009139b9); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70009139c5, 0x70009139c0);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70009139ca); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009139ce); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009139d2); };
            c.w64(3, t55);
            blk = 26; continue;
        }
        32 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009139d8); };
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
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000913a00); };
            c.w64(8, t61);
            c.w64(1, c.r64(11));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000913a07); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000913a16); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000913a1a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000913a1c); }
            c.w32(11, 0x14u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000913a23); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000913a25); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x7000913a31); };
            c.w64(3, t64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000913a40, 0x7000913a3b);
        }
        36 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000913a40); };
            c.w64(8, t65);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000913a46); }
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
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000913a69); };
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
            let Some(t70) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000913a92); };
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
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000913aa1); };
            c.w64(7, t73);
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000913aa5); };
            c.cmp(t74, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000913aae); };
            c.w64(11, t75);
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000913ab2); };
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
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x7000913ac7); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000913acc); }
            blk = 48; continue;
        }
        48 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000913ad3); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000913ad7); };
            c.w64(7, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000913adb); };
            c.w64(8, t80);
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t81) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000913ae3); };
            c.w32(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000913ae8); };
            c.w64(15, t82);
            let Some(t83) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000913aec); };
            c.w64(1, t83);
            let Some(t84) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000913af0); };
            c.w64(5, t84);
            c.w64(12, c.r64(7));
            let r85 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r85);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000913afd); };
            c.w64(0, t86);
            let Some(t87) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000913b04); };
            c.w64(3, t87);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000913b0b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000913b0f); }
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000913b13); };
            c.w64(1, t88);
            let Some(t89) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000913b1a); };
            c.cmp(t89, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000913b29, 0x7000913b24);
        }
        51 => {
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000913b29); };
            c.w64(11, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000913b2d); };
            c.w64(7, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000913b31); };
            c.w64(8, t92);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x7000913b37); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000913b3e); };
            c.w64(3, t93);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000913b45); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000913b51); }
            blk = 41; continue;
        }
        54 => {
            let r94 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r94);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000913b5f); }
            return c.call(0x70008708b4, 0x7000913b69, 0x7000913b64);
        }
        55 => {
            let Some(t95) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000913b69); };
            c.w32(0, t95);
            let r96 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            blk = 38; continue;
        }
        56 => {
            let r97 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r97);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000913b79); }
            return c.call(0x70008708b4, 0x7000913b83, 0x7000913b7e);
        }
        57 => {
            let Some(t98) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000913b83); };
            c.w32(12, t98);
            let r99 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r99);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x7000913b94);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000913bb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000913bb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000913bb8); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000913bbe); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000913bc2); };
            c.w64(8, t2);
            let Some(t3) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000913bc6); };
            c.w64(11, (t3 as u16 as i16 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000913bcc); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000913bd3); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000913bd7); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000913bdb); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000913be2); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000913be8); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000913bec); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000913bf0); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000913bf4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000913bf8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000913bfc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000913c00); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x7000913c04); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000913c08); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000913c11); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r14 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r14);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000913c1e); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x7000913c24); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000913c28); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000913c2e); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000913c3b); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000913c43); };
            c.cmp(t17, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t18) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000913c50); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000913c63); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000913c72, 0x7000913c6d);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000913c7b); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000913c7f); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000913c83); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000913c87); };
            c.w64(2, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000913c8b); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000913c8f); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000913c93); };
            c.w64(1, t25);
            blk = 3; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000913c99); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000913cac); };
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
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000913cbf); };
            c.w32(8, t31);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000913cc3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000913ccf); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000913cd6); };
            c.cmp(t34, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000913cdf); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000913cf5); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000913cf9); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000913d05, 0x7000913d00);
        }
        17 => {
            c.w32(6, 0xfeu64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000913d0a); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000913d0e); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000913d12); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000913d16); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000913d1a); };
            c.w64(2, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000913d1e); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000913d22); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000913d26); };
            c.w64(8, t43);
            blk = 22; continue;
        }
        18 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000913d2c); };
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
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000913d61); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000913d7a); };
            c.w32(8, t47);
            let r48 = c.r32(8) | 0xeu64;
            c.w32(8, r48);
            c.logic(r48, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000913d85); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000913d89); }
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
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000913d96); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000913d9c); };
            c.cmp(t51, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t52) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000913da9); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000913db9); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000913dc5, 0x7000913dc0);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000913dca); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000913dce); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000913dd2); };
            c.w64(3, t55);
            blk = 26; continue;
        }
        32 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000913dd8); };
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
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000913e00); };
            c.w64(8, t61);
            c.w64(1, c.r64(11));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000913e07); };
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
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000913e16); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000913e1a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000913e1c); }
            c.w32(11, 0x16u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000913e23); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000913e25); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x7000913e31); };
            c.w64(3, t64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000913e40, 0x7000913e3b);
        }
        36 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000913e40); };
            c.w64(8, t65);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000913e46); }
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
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000913e69); };
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
            let Some(t69) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000913e94); };
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
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000913ea3); };
            c.w64(7, t72);
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000913ea7); };
            c.cmp(t73, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000913eb0); };
            c.w64(11, t74);
            let Some(t75) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000913eb4); };
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
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x7000913ec9); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000913ece); }
            blk = 48; continue;
        }
        48 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000913ed5); };
            c.w64(7, t77);
            let Some(t78) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000913ed9); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000913edd); };
            c.w64(8, t79);
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t80) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000913ee5); };
            c.w32(8, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000913eea); };
            c.w64(15, t81);
            let Some(t82) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000913eee); };
            c.w64(1, t82);
            let Some(t83) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000913ef2); };
            c.w64(5, t83);
            c.w64(12, c.r64(7));
            let r84 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r84);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000913eff); };
            c.w64(0, t85);
            let Some(t86) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000913f06); };
            c.w64(3, t86);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000913f0d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000913f11); }
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000913f15); };
            c.w64(1, t87);
            let Some(t88) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000913f1c); };
            c.cmp(t88, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000913f2b, 0x7000913f26);
        }
        51 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000913f2b); };
            c.w64(11, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000913f2f); };
            c.w64(7, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000913f33); };
            c.w64(8, t91);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x7000913f39); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000913f40); };
            c.w64(3, t92);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000913f47); }
            c.w32(10, c.r32(11));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000913f56); }
            blk = 41; continue;
        }
        54 => {
            let r93 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r93);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000913f64); }
            return c.call(0x70008708b4, 0x7000913f6e, 0x7000913f69);
        }
        55 => {
            let Some(t94) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000913f6e); };
            c.w32(0, t94);
            let r95 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r95);
            blk = 38; continue;
        }
        56 => {
            let r96 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000913f7e); }
            return c.call(0x70008708b4, 0x7000913f88, 0x7000913f83);
        }
        57 => {
            let Some(t97) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000913f88); };
            c.w32(12, t97);
            let r98 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r98);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x7000913f98);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000913fb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000913fb7); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000913fbb); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000913fc2); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000913fc6); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000913fcd); };
            c.w32(3, (t6 as u16 as i16 as i64 as u64));
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000913fd3); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000913fd9); };
            c.w32(1, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000913fdd); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000913fe6); };
            c.w64(8, t10);
            blk = 2; continue;
        }
        2 => {
            let r11 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r11);
            c.w32(1, c.r32(0));
            c.w32(6, c.r32(3));
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000913ff3); };
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
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000914010); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000914023); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000914028); }
            blk = 7; continue;
        }
        7 => {
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            blk = 8; continue;
        }
        8 => {
            let Some(t16) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000914033); };
            c.w64(7, t16);
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x5));
            let Some(t17) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700091403e); };
            c.w32(11, t17);
            let Some(t18) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000914043); };
            c.w64(1, t18);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091404f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000914053); }
            let r19 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r19);
            let r20 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r20);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000914063); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000914067); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091406b); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x700091406f); }
            blk = 12; continue;
        }
        10 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000914075); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000914079); }
            let r21 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r21);
            let Some(t22) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091407f); };
            c.w32(1, t22);
            let r23 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000914085); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000914089); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091408d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000914091); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000914095); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000914099); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700091409d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70009140a1); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x70009140a5); }
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009140a9); };
            c.cmp(t24, c.r32(1), 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009140b6); }
            blk = 12; continue;
        }
        12 => {
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70009140bb); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009140c6); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70009140d9); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009140de); }
            blk = 15; continue;
        }
        15 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009140e6); };
            c.w64(8, t27);
            let Some(t28) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70009140ed); };
            c.w64(11, t28);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009140f4); };
            c.w64(2, t29);
            let Some(t30) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70009140fb); };
            c.cmp(t30, c.r64(0), 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700091410a, 0x7000914105);
        }
        17 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700091410a); };
            c.w64(3, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091410e); };
            c.w64(15, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000914112); };
            c.w64(9, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000914116); };
            c.w64(7, t34);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700091411a); }
            blk = 19; continue;
        }
        18 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x7000914121); }
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
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000914139); };
            c.w64(8, t36);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000914140); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 1) { return RegfileExit::fault(0x7000914144); }
            c.w64(8, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091414f); }
            blk = 8; continue;
        }
        20 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let Some(t37) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000914159); };
            c.w32(1, t37);
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            let r38 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r38);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x7000914168); }
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
            let Some(t39) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000914193); };
            c.w32(0, t39);
            let r40 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r40);
            let Some(t41) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000914199); };
            c.cmp(t41, 0x41u64, 16);
            if c.cc_ne() { blk = 45; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70009141a4); }
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009141a8); };
            c.w64(0, t42);
            c.w64(6, c.r64(11));
            return c.call(0x700089b334, 0x70009141b4, 0x70009141af);
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t43) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009141b8); };
            c.cmp(t43, 0x0u64, 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009141be); };
            c.w64(8, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009141c2); };
            c.w64(2, t45);
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009141c6); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x70009141da); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009141df); }
            blk = 30; continue;
        }
        30 => {
            c.w64(8, c.r64(0));
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009141ea); };
            c.w64(15, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009141ee); };
            c.w64(9, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009141f2); };
            c.w64(7, t50);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70009141f9); }
            blk = 8; continue;
        }
        31 => {
            let Some(t51) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009141ff); };
            c.cmp(t51, 0x0u64, 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000914205); };
            c.w64(8, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000914209); };
            c.w64(7, t53);
            let Some(t54) = c.ld32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091420d); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000914222); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000914227); }
            blk = 34; continue;
        }
        34 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091422f); };
            c.w64(11, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000914233); };
            c.w64(2, t57);
            if !c.st64(c.r64(11).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000914237); }
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000914240); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x7000914241); };
            c.w64(3, t58);
            c.w32(0, 0x1u64);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091424d); };
            c.w64(6, t59);
            return c.call(0x700086ed74, 0x7000914256, 0x7000914251);
        }
        35 => {
            return RegfileExit::fault(0x7000914256);
        }
        36 => {
            let Some(t60) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000914257); };
            c.w32(0, t60);
            let r61 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r61);
            let Some(t62) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091425d); };
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
            let Some(t63) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091426a); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700091427b); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000914280); }
            blk = 40; continue;
        }
        40 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000914288); }
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091428c); };
            c.w64(0, t65);
            c.w64(6, c.r64(11));
            return c.call(0x700086f9d4, 0x7000914298, 0x7000914293);
        }
        41 => {
            c.w64(8, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091429b); };
            c.w64(15, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091429f); };
            c.w64(9, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009142a3); };
            c.w64(7, t68);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70009142aa); }
            blk = 8; continue;
        }
        42 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t69) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009142b5); };
            c.w32(0, t69);
            let r70 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r70);
            let Some(t71) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009142bb); };
            c.cmp(t71, c.r32(0), 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t72) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009142c4); };
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
            let Some(t73) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009142dd); };
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
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70009142f1); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009142f6); }
            blk = 50; continue;
        }
        50 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70009142fe); }
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000914302); };
            c.w64(0, t75);
            c.w64(6, c.r64(11));
            return c.call(0x70008ad794, 0x700091430e, 0x7000914309);
        }
        51 => {
            c.w64(8, c.r64(0));
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000914311); };
            c.w64(15, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000914315); };
            c.w64(9, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000914319); };
            c.w64(7, t78);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000914320); }
            blk = 8; continue;
        }
        52 => {
            return RegfileExit::jump(0x7000914328);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000914334(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000914337); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091433b); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000914342); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000914346); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091434d); };
            c.w32(3, (t6 as u16 as i16 as i64 as u64));
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000914353); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000914359); };
            c.w32(1, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091435d); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000914366); };
            c.w64(8, t10);
            blk = 2; continue;
        }
        2 => {
            let r11 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r11);
            c.w32(1, c.r32(0));
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000914371); };
            c.cmp(t12, c.r32(8), 32);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(11, c.r32(3));
            let r13 = c.sub(c.r32(1), c.r32(11), 32);
            c.w32(1, r13);
            if c.cc_o() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w32(0, 0x2u64);
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091439d); };
            c.w32(3, t14);
            c.w32(0, c.r32(0));
            c.w32(1, c.r32(3));
            let r15 = c.r32(0) | c.r32(1);
            c.w32(0, r15);
            c.logic(r15, 32);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70009143af); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009143b4); }
            blk = 8; continue;
        }
        8 => {
            let Some(t16) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009143bc); };
            c.w64(7, t16);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t17) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009143c7); };
            c.w32(8, t17);
            let Some(t18) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009143cc); };
            c.w64(1, t18);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009143d8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009143dc); }
            let r19 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r19);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009143e2); };
            c.w64(11, t20);
            let Some(t21) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009143e9); };
            c.w64(1, t21);
            let r22 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r22);
            c.w64(12, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009143f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009143f6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009143fa); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009143fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000914402); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x7000914406); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700091440a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x700091440e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000914412); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000914416); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700091441d); };
            c.cmp(t24, c.r64(12), 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700091442f, 0x700091442a);
        }
        11 => {
            c.w64(11, c.r64(0));
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000914432); };
            c.w64(15, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000914436); };
            c.w64(9, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091443a); };
            c.w64(7, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091443e); };
            c.w64(0, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000914442); };
            c.w64(8, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000914446); };
            c.w64(2, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091444a); };
            c.w64(6, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091444e); };
            c.w64(3, t32);
            blk = 13; continue;
        }
        12 => {
            if !c.st64(c.r64(11), c.r64(12)) { return RegfileExit::fault(0x7000914454); }
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000914457); };
            c.w64(11, t33);
            let r34 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r34);
            blk = 13; continue;
        }
        13 => {
            let r35 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r35);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            c.arith_sd(1, 1, c.xmm_u64(0));
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000914475); };
            c.w64(0, t36);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700091447c); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x7000914480); }
            c.w32(0, 0xeu64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091448e); }
            blk = 5; continue;
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000914494); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000914498); }
            let r37 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r37);
            let Some(t38) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091449e); };
            c.w32(1, t38);
            let r39 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r39);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009144a4); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009144a8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009144ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009144b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70009144b4); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70009144b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70009144bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70009144c0); }
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009144c4); };
            c.cmp(t40, c.r32(1), 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 15; continue;
        }
        15 => {
            let r41 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r41);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009144d3); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70009144e0); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 1) { return RegfileExit::fault(0x70009144e5); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009144ef); };
            c.w32(11, t42);
            c.w32(0, c.r32(11));
            let r43 = c.r32(0) | 0xeu64;
            c.w32(0, r43);
            c.logic(r43, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000914502); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000914507); }
            blk = 18; continue;
        }
        18 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091450f); };
            c.w64(8, t44);
            let Some(t45) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000914516); };
            c.w64(11, t45);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091451d); };
            c.w64(3, t46);
            let Some(t47) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000914524); };
            c.cmp(t47, c.r64(0), 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000914533, 0x700091452e);
        }
        20 => {
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x7000914533); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000914538); }
            blk = 22; continue;
        }
        21 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700091453f); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 22; continue;
        }
        22 => {
            c.arith_sd(1, 0, c.xmm_u64(1));
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091454d); };
            c.w64(7, t48);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000914554); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000914557); }
            c.w64(11, c.r64(0));
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091455f); };
            c.w64(15, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000914563); };
            c.w64(9, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000914567); };
            c.w64(7, t51);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091456e); }
            blk = 8; continue;
        }
        23 => {
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r52 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r52);
            let Some(t53) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091457b); };
            c.w32(1, t53);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700091457f); }
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 31; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_ne() { blk = 44; continue; }
            blk = 25; continue;
        }
        25 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t54) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000914598); };
            c.w32(1, t54);
            let r55 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r55);
            let Some(t56) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091459e); };
            c.cmp(t56, c.r32(1), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(1, 0x1eu64);
            blk = 45; continue;
        }
        28 => {
            let Some(t57) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009145b1); };
            c.cmp(t57, 0x43u64, 16);
            if c.cc_ne() { blk = 44; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(1, 0x1eu64);
            blk = 45; continue;
        }
        30 => {
            c.w32(1, 0x1eu64);
            blk = 45; continue;
        }
        31 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t58) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009145d9); };
            c.w32(1, t58);
            let r59 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r59);
            let Some(t60) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009145df); };
            c.cmp(t60, 0x41u64, 16);
            if c.cc_ne() { blk = 44; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t61) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009145ef); };
            c.w32(1, t61);
            c.w32(0, c.r32(1));
            let r62 = c.r32(0) | 0x40u64;
            c.w32(0, r62);
            c.logic(r62, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000914600); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000914605); }
            blk = 36; continue;
        }
        36 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700091460d); }
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000914611); };
            c.w64(0, t63);
            c.w64(6, c.r64(11));
            return c.call(0x700089bcd4, 0x700091461d, 0x7000914618);
        }
        37 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(11, c.r64(0));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000914624); };
            c.w64(15, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000914628); };
            c.w64(9, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091462c); };
            c.w64(7, t66);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000914633); }
            blk = 8; continue;
        }
        39 => {
            let Some(t67) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000914639); };
            c.cmp(t67, 0x0u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091463f); };
            c.w64(8, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000914643); };
            c.w64(2, t69);
            let Some(t70) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000914647); };
            c.w32(7, t70);
            c.w32(11, c.r32(7));
            let r71 = c.r32(11) | 0xfeu64;
            c.w32(11, r71);
            c.logic(r71, 32);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700091465e); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000914663); }
            blk = 42; continue;
        }
        42 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091466b); };
            c.w64(7, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700091466f); };
            c.w64(11, t73);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000914673); }
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091467c); }
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700091467d); };
            c.w64(3, t74);
            c.w32(0, 0x1u64);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000914689); };
            c.w64(6, t75);
            return c.call(0x700086ed74, 0x7000914692, 0x700091468d);
        }
        43 => {
            return RegfileExit::fault(0x7000914692);
        }
        44 => {
            c.w32(1, 0xfeu64);
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t76) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091469d); };
            c.w32(6, t76);
            c.w32(1, c.r32(1));
            c.w32(11, c.r32(6));
            let r77 = c.r32(1) | c.r32(11);
            c.w32(1, r77);
            c.logic(r77, 32);
            c.cmp(c.r32(6), c.r32(1), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70009146b1); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009146b6); }
            blk = 48; continue;
        }
        48 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70009146be); }
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009146c2); };
            c.w64(6, t78);
            return c.call(0x70008adb14, 0x70009146cb, 0x70009146c6);
        }
        49 => {
            c.w64(11, c.r64(0));
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009146ce); };
            c.w64(15, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009146d2); };
            c.w64(9, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009146d6); };
            c.w64(7, t81);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70009146dd); }
            blk = 8; continue;
        }
        50 => {
            return RegfileExit::jump(0x70009146e4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009146f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009146f7); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009146fb); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000914702); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000914706); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091470d); };
            c.w32(3, (t6 as u16 as i16 as i64 as u64));
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000914713); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000914719); };
            c.w32(1, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091471d); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000914726); };
            c.w64(8, t10);
            blk = 2; continue;
        }
        2 => {
            let r11 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091472f); };
            c.cmp(t12, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 27; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(11, c.r64(3));
            let r13 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r13);
            let r14 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r14);
            c.w64(6, c.r64(0));
            let r15 = c.imul2(c.r32(6), c.r32(11), 32);
            c.w32(6, r15);
            if c.cc_o() { blk = 22; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(11, c.r64(6));
            let r16 = c.add(c.r32(11), c.r32(6), 32);
            c.w32(11, r16);
            if c.cc_o() { blk = 17; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 10; continue;
        }
        7 => {
            let r17 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r17);
            let r18 = c.r32(3) | c.r32(0);
            c.w32(3, r18);
            c.logic(r18, 32);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 10; continue;
        }
        9 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x700091477d); };
            c.w64(11, t19);
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(0, 0xeu64);
            blk = 13; continue;
        }
        12 => {
            c.w32(0, 0x2u64);
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091479b); };
            c.w32(3, t20);
            c.w32(0, c.r32(0));
            c.w32(1, c.r32(3));
            let r21 = c.r32(0) | c.r32(1);
            c.w32(0, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70009147ad); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009147b2); }
            blk = 16; continue;
        }
        16 => {
            let Some(t22) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009147ba); };
            c.w64(7, t22);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t23) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009147c5); };
            c.w32(8, t23);
            let Some(t24) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009147ca); };
            c.w64(1, t24);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009147d6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009147da); }
            let r25 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r25);
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009147e0); };
            c.w64(11, t26);
            let Some(t27) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009147e7); };
            c.w64(0, t27);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009147ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009147f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009147f6); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70009147fa); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70009147fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000914802); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000914806); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091480a); };
            c.w64(12, t28);
            let Some(t29) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000914811); };
            c.cmp(t29, c.r64(3), 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x7000914824, 0x700091481f);
        }
        19 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000914824); };
            c.w64(6, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000914828); };
            c.w64(15, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091482c); };
            c.w64(9, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000914830); };
            c.w64(7, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000914834); };
            c.w64(8, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000914838); };
            c.w64(2, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091483c); };
            c.w64(1, t36);
            blk = 21; continue;
        }
        20 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x7000914842); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 21; continue;
        }
        21 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(6) as u32 as i32 as i64));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000914854); };
            c.w64(11, t37);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700091485b); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700091485f); }
            c.w64(11, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091486a); }
            blk = 10; continue;
        }
        22 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000914870); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000914874); }
            let r38 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r38);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091487a); };
            c.w64(3, t39);
            let Some(t40) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000914881); };
            c.w64(6, t40);
            c.w64(12, c.r64(6).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000914888); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091488c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000914890); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000914894); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000914898); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700091489c); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009148a0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70009148a4); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x70009148a8); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009148ac); };
            c.w64(6, t41);
            let Some(t42) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70009148b3); };
            c.cmp(t42, c.r64(12), 64);
            if c.cc_a() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(2, 0xcu64);
            c.w64(3, c.r64(2));
            return c.call(0x70007febf4, 0x70009148c5, 0x70009148c0);
        }
        24 => {
            c.w64(3, c.r64(0));
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009148c8); };
            c.w64(15, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009148cc); };
            c.w64(9, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009148d0); };
            c.w64(7, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009148d4); };
            c.w64(8, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009148d8); };
            c.w64(2, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009148dc); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009148e0); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009148e4); };
            c.w64(11, t50);
            blk = 26; continue;
        }
        25 => {
            if !c.st64(c.r64(3), c.r64(12)) { return RegfileExit::fault(0x70009148ea); }
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009148ed); };
            c.w64(3, t51);
            let r52 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r52);
            blk = 26; continue;
        }
        26 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            c.arith_sd(2, 0, c.xmm_u64(1));
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091490a); };
            c.w64(11, t53);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000914911); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000914915); }
            c.w64(11, c.r64(3));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000914920); }
            blk = 10; continue;
        }
        27 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000914926); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091492a); }
            let r54 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r54);
            let Some(t55) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000914930); };
            c.w32(6, t55);
            let r56 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r56);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000914936); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700091493a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091493e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000914942); }
            let Some(t57) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000914946); };
            c.cmp(t57, c.r32(6), 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 28; continue;
        }
        28 => {
            let r58 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r58);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000914955); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x7000914962); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x7000914967); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t59) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000914971); };
            c.w32(11, t59);
            c.w32(0, c.r32(11));
            let r60 = c.r32(0) | 0xeu64;
            c.w32(0, r60);
            c.logic(r60, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000914984); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000914989); }
            blk = 31; continue;
        }
        31 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000914991); };
            c.w64(8, t61);
            let Some(t62) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000914998); };
            c.w64(11, t62);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091499f); };
            c.w64(3, t63);
            let Some(t64) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70009149a6); };
            c.cmp(t64, c.r64(0), 64);
            if c.cc_a() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70009149b5, 0x70009149b0);
        }
        33 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70009149b5); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70009149ba); }
            blk = 35; continue;
        }
        34 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70009149c1); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 35; continue;
        }
        35 => {
            c.arith_sd(2, 1, c.xmm_u64(0));
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009149cf); };
            c.w64(7, t65);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70009149d6); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70009149d9); }
            c.w64(11, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009149e1); };
            c.w64(15, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009149e5); };
            c.w64(9, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009149e9); };
            c.w64(7, t68);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70009149f0); }
            blk = 16; continue;
        }
        36 => {
            let Some(t69) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009149f6); };
            c.w32(6, t69);
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r70 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r70);
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_ne() { blk = 51; continue; }
            blk = 38; continue;
        }
        38 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t71) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000914a16); };
            c.w32(6, t71);
            let r72 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r72);
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000914a1c); };
            c.cmp(t73, c.r32(6), 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(6, 0x1eu64);
            blk = 52; continue;
        }
        41 => {
            let Some(t74) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000914a2f); };
            c.cmp(t74, 0x43u64, 16);
            if c.cc_ne() { blk = 51; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(6, 0x1eu64);
            blk = 52; continue;
        }
        43 => {
            c.w32(6, 0x1eu64);
            blk = 52; continue;
        }
        44 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t75) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000914a53); };
            c.w32(6, t75);
            let r76 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r76);
            let Some(t77) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000914a59); };
            c.cmp(t77, 0x41u64, 16);
            if c.cc_ne() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t78) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000914a65); };
            c.w32(1, t78);
            c.w32(6, c.r32(1));
            let r79 = c.r32(6) | 0x40u64;
            c.w32(6, r79);
            c.logic(r79, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(1, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x7000914a76); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000914a7b); }
            blk = 49; continue;
        }
        49 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000914a83); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000914a87); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000914a88); }
            c.w32(8, 0x4u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000914a8f); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x7000914a91); };
            c.w64(3, t80);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x7000914aa5, 0x7000914aa0);
        }
        50 => {
            c.w64(11, c.r64(0));
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000914aa8); };
            c.w64(15, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000914aac); };
            c.w64(9, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000914ab0); };
            c.w64(7, t83);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000914ab7); }
            blk = 16; continue;
        }
        51 => {
            c.w32(6, 0xfeu64);
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t84) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000914ac7); };
            c.w32(1, t84);
            c.w32(6, c.r32(6));
            c.w32(11, c.r32(1));
            let r85 = c.r32(6) | c.r32(11);
            c.w32(6, r85);
            c.logic(r85, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(11, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000914adb); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000914ae0); }
            blk = 55; continue;
        }
        55 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000914ae8); }
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000914aec); };
            c.w64(6, t86);
            return c.call(0x70008adcd4, 0x7000914af5, 0x7000914af0);
        }
        56 => {
            c.w64(11, c.r64(0));
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000914af8); };
            c.w64(15, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000914afc); };
            c.w64(9, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000914b00); };
            c.w64(7, t89);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000914b07); }
            blk = 16; continue;
        }
        57 => {
            return RegfileExit::jump(0x7000914b10);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000914b14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000914b17); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000914b1b); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000914b22); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000914b26); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000914b2d); };
            c.w32(3, (t6 as u16 as i16 as i64 as u64));
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000914b33); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000914b39); };
            c.w32(2, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000914b3d); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000914b46); };
            c.w64(8, t10);
            blk = 2; continue;
        }
        2 => {
            let r11 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000914b4f); };
            c.cmp(t12, c.r32(8), 32);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 23; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(11, c.r64(3));
            let r13 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r13);
            c.w64(12, c.r64(0));
            let r14 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r14);
            c.w32(2, 0);
            c.logic(0, 32);
            c.cmp(c.r32(3), c.r32(2), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 16; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(11), 0xffffffffffffffffu64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(12), 0xc0000000u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(0, c.r64(12));
            c.cdq();
            c.idiv(c.r32(11), 32);
            c.w64(3, c.r64(0));
            let r15 = c.imul2(c.r32(3), c.r32(11), 32);
            c.w32(3, r15);
            c.cmp(c.r32(3), c.r32(12), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(11, c.r64(0).wrapping_add(c.r64(0)));
            c.w32(0, 0x2u64);
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000914bb9); };
            c.w32(3, t16);
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(3));
            let r17 = c.r32(0) | c.r32(2);
            c.w32(0, r17);
            c.logic(r17, 32);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000914bcb); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000914bd0); }
            blk = 13; continue;
        }
        13 => {
            let Some(t18) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000914bd8); };
            c.w64(7, t18);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000914be3); };
            c.w32(8, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000914be8); };
            c.w64(1, t20);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000914bf4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000914bf8); }
            let r21 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r21);
            blk = 18; continue;
        }
        15 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000914c00); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000914c04); }
            let r22 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r22);
            blk = 18; continue;
        }
        16 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000914c0c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000914c10); }
            let r23 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r23);
            blk = 18; continue;
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000914c18); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000914c1c); }
            let r24 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r24);
            blk = 18; continue;
        }
        18 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000914c22); };
            c.w64(0, t25);
            let Some(t26) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000914c29); };
            c.w64(3, t26);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000914c30); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000914c34); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000914c38); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000914c3c); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000914c40); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x7000914c44); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000914c48); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000914c4c); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000914c50); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000914c54); };
            c.w64(3, t27);
            let Some(t28) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000914c5b); };
            c.cmp(t28, c.r64(2), 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000914c6a, 0x7000914c65);
        }
        20 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000914c6a); };
            c.w64(11, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000914c6e); };
            c.w64(15, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000914c72); };
            c.w64(9, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000914c76); };
            c.w64(7, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000914c7a); };
            c.w64(8, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000914c7e); };
            c.w64(1, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000914c82); };
            c.w64(6, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000914c86); };
            c.w64(12, t36);
            blk = 22; continue;
        }
        21 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x7000914c8c); }
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000914c8f); };
            c.w64(0, t37);
            let r38 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r38);
            blk = 22; continue;
        }
        22 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            c.arith_sd(3, 1, c.xmm_u64(0));
            c.mov128_rr(1, 1);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000914cb0); };
            c.w64(11, t39);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000914cb7); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 1) { return RegfileExit::fault(0x7000914cbb); }
            c.w64(11, c.r64(0));
            c.w32(0, 0x6u64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000914ccb); }
            blk = 10; continue;
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000914cd1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000914cd5); }
            let r40 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r40);
            let Some(t41) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000914cdb); };
            c.w32(2, t41);
            let r42 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r42);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000914ce1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000914ce5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000914ce9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000914ced); }
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000914cf1); };
            c.cmp(t43, c.r32(2), 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 24; continue;
        }
        24 => {
            let r44 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r44);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000914d00); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000914d0d); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 1) { return RegfileExit::fault(0x7000914d12); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t45) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000914d1c); };
            c.w32(11, t45);
            c.w32(0, c.r32(11));
            let r46 = c.r32(0) | 0xeu64;
            c.w32(0, r46);
            c.logic(r46, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000914d2f); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000914d34); }
            blk = 27; continue;
        }
        27 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000914d3c); };
            c.w64(8, t47);
            let Some(t48) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000914d43); };
            c.w64(11, t48);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000914d4a); };
            c.w64(3, t49);
            let Some(t50) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000914d51); };
            c.cmp(t50, c.r64(0), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000914d60, 0x7000914d5b);
        }
        29 => {
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x7000914d60); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000914d65); }
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x7000914d6c); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 31; continue;
        }
        31 => {
            c.arith_sd(3, 0, c.xmm_u64(1));
            c.mov128_rr(0, 0);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000914d7d); };
            c.w64(7, t51);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000914d84); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000914d87); }
            c.w64(11, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000914d8f); };
            c.w64(15, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000914d93); };
            c.w64(9, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000914d97); };
            c.w64(7, t54);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000914d9e); }
            blk = 13; continue;
        }
        32 => {
            let Some(t55) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000914da4); };
            c.w32(2, t55);
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r56 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r56);
            c.cmp(c.r16(2), 0x41u64, 16);
            if c.cc_e() { blk = 40; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r16(2), 0x43u64, 16);
            if c.cc_ne() { blk = 47; continue; }
            blk = 34; continue;
        }
        34 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t57) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000914dc4); };
            c.w32(2, t57);
            let r58 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r58);
            let Some(t59) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000914dca); };
            c.cmp(t59, c.r32(2), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(2, 0x1eu64);
            blk = 48; continue;
        }
        37 => {
            let Some(t60) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000914ddd); };
            c.cmp(t60, 0x43u64, 16);
            if c.cc_ne() { blk = 47; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(2, 0x1eu64);
            blk = 48; continue;
        }
        39 => {
            c.w32(2, 0x1eu64);
            blk = 48; continue;
        }
        40 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 47; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t61) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000914e01); };
            c.w32(2, t61);
            let r62 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r62);
            let Some(t63) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000914e07); };
            c.cmp(t63, 0x41u64, 16);
            if c.cc_ne() { blk = 47; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t64) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000914e13); };
            c.w32(2, t64);
            c.w32(6, c.r32(2));
            let r65 = c.r32(6) | 0x40u64;
            c.w32(6, r65);
            c.logic(r65, 32);
            c.cmp(c.r32(2), c.r32(6), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(2, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000914e24); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000914e29); }
            blk = 45; continue;
        }
        45 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000914e31); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000914e35); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000914e36); }
            c.w32(8, 0x6u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000914e3d); }
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x7000914e3f); };
            c.w64(3, t66);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x7000914e53, 0x7000914e4e);
        }
        46 => {
            c.w64(11, c.r64(0));
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000914e56); };
            c.w64(15, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000914e5a); };
            c.w64(9, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000914e5e); };
            c.w64(7, t69);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000914e65); }
            blk = 13; continue;
        }
        47 => {
            c.w32(2, 0xfeu64);
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t70) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000914e75); };
            c.w32(6, t70);
            c.w32(2, c.r32(2));
            c.w32(11, c.r32(6));
            let r71 = c.r32(2) | c.r32(11);
            c.w32(2, r71);
            c.logic(r71, 32);
            c.cmp(c.r32(6), c.r32(2), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000914e89); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000914e8e); }
            blk = 51; continue;
        }
        51 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000914e96); }
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000914e9a); };
            c.w64(6, t72);
            return c.call(0x70008adf94, 0x7000914ea3, 0x7000914e9e);
        }
        52 => {
            c.w64(11, c.r64(0));
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000914ea6); };
            c.w64(15, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000914eaa); };
            c.w64(9, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000914eae); };
            c.w64(7, t75);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000914eb5); }
            blk = 13; continue;
        }
        53 => {
            return RegfileExit::jump(0x7000914ebc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000914ed4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000914ed7); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000914edb); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000914ee2); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000914ee6); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000914eed); };
            c.w32(3, (t6 as u16 as i16 as i64 as u64));
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000914ef3); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000914ef9); };
            c.w32(2, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000914efd); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000914f06); };
            c.w64(8, t10);
            blk = 2; continue;
        }
        2 => {
            let r11 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000914f0f); };
            c.cmp(t12, c.r32(8), 32);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 3; continue;
        }
        3 => {
            let r13 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r13);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 4; continue;
        }
        4 => {
            let r14 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r14);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 13; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cdq();
            c.idiv(c.r32(3), 32);
            c.w32(11, c.r64(2).wrapping_add(c.r64(2)));
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(0, 0xeu64);
            blk = 9; continue;
        }
        8 => {
            c.w32(0, 0x2u64);
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000914f56); };
            c.w32(3, t15);
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(3));
            let r16 = c.r32(0) | c.r32(2);
            c.w32(0, r16);
            c.logic(r16, 32);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000914f68); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000914f6d); }
            blk = 12; continue;
        }
        12 => {
            let Some(t17) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000914f75); };
            c.w64(7, t17);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000914f80); };
            c.w32(8, t18);
            let Some(t19) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000914f85); };
            c.w64(1, t19);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        13 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000914f91); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000914f95); }
            let r20 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r20);
            c.cdq();
            c.idiv(c.r32(3), 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000914f9e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000914fa2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000914fa6); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000914faa); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000914fae); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x7000914fb2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000914fb6); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(11, c.r64(2));
            let r21 = c.add(c.r32(11), c.r32(2), 32);
            c.w32(11, r21);
            if c.cc_o() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            blk = 21; continue;
        }
        16 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000914fd3); };
            c.w64(11, t22);
            let Some(t23) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000914fda); };
            c.w64(0, t23);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000914fe1); };
            c.w64(12, t24);
            let Some(t25) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000914fe8); };
            c.cmp(t25, c.r64(3), 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x7000914ffb, 0x7000914ff6);
        }
        18 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000914ffb); };
            c.w64(2, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000914fff); };
            c.w64(15, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000915003); };
            c.w64(9, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000915007); };
            c.w64(7, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091500b); };
            c.w64(8, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091500f); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000915013); };
            c.w64(6, t32);
            blk = 20; continue;
        }
        19 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x7000915019); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 20; continue;
        }
        20 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700091502b); };
            c.w64(11, t33);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000915032); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000915036); }
            blk = 21; continue;
        }
        21 => {
            c.w64(11, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000915041); }
            blk = 6; continue;
        }
        22 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x7000915047); };
            c.w64(11, t34);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000915051); }
            blk = 6; continue;
        }
        23 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x7000915057); };
            c.w64(11, t35);
            blk = 6; continue;
        }
        24 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000915063); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000915067); }
            let r36 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r36);
            let Some(t37) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091506d); };
            c.w32(2, t37);
            let r38 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r38);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000915073); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000915077); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091507b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700091507f); }
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000915083); };
            c.cmp(t39, c.r32(2), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 25; continue;
        }
        25 => {
            let r40 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r40);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000915092); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000915097); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700091509b); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009150a5); };
            c.w32(11, t41);
            c.w32(0, c.r32(11));
            let r42 = c.r32(0) | 0xeu64;
            c.w32(0, r42);
            c.logic(r42, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70009150b8); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009150bd); }
            blk = 28; continue;
        }
        28 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009150c5); };
            c.w64(8, t43);
            let Some(t44) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70009150cc); };
            c.w64(11, t44);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009150d3); };
            c.w64(2, t45);
            let Some(t46) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70009150da); };
            c.cmp(t46, c.r64(0), 64);
            if c.cc_a() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70009150e9, 0x70009150e4);
        }
        30 => {
            c.w64(8, c.r64(0));
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009150ec); };
            c.w64(15, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009150f0); };
            c.w64(9, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009150f4); };
            c.w64(7, t49);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70009150f8); }
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009150fd); };
            c.w64(3, t50);
            blk = 32; continue;
        }
        31 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x7000915103); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 32; continue;
        }
        32 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            let r51 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r51);
            if !c.movsd_mr(c.r64(4), 1) { return RegfileExit::fault(0x7000915116); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x700091511b); }
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700091511e); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x7000915123); }
            blk = 33; continue;
        }
        33 => {
            c.fprem();
            c.fnstsw_ax();
            let r52 = c.shr(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r52);
            let r53 = c.r32(0) & 0xffu64;
            c.w32(0, r53);
            c.logic(r53, 32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000915132); }
            if !c.popfq() { return RegfileExit::fault(0x7000915133); }
            if c.cc_p() { blk = 33; continue; }
            blk = 34; continue;
        }
        34 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x7000915138); }
            if !c.movsd_rm(0, c.r64(4)) { return RegfileExit::fault(0x700091513b); }
            let r54 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r54);
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000915144); };
            c.w64(11, t55);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700091514b); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700091514f); }
            c.w64(11, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700091515b); }
            blk = 12; continue;
        }
        35 => {
            let Some(t56) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000915161); };
            c.w32(2, t56);
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r57 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r57);
            c.cmp(c.r16(2), 0x41u64, 16);
            if c.cc_e() { blk = 43; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r16(2), 0x43u64, 16);
            if c.cc_ne() { blk = 50; continue; }
            blk = 37; continue;
        }
        37 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 42; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t58) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000915181); };
            c.w32(2, t58);
            let r59 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r59);
            let Some(t60) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000915187); };
            c.cmp(t60, c.r32(2), 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(2, 0x1eu64);
            blk = 51; continue;
        }
        40 => {
            let Some(t61) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091519a); };
            c.cmp(t61, 0x43u64, 16);
            if c.cc_ne() { blk = 50; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w32(2, 0x1eu64);
            blk = 51; continue;
        }
        42 => {
            c.w32(2, 0x1eu64);
            blk = 51; continue;
        }
        43 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t62) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009151be); };
            c.w32(2, t62);
            let r63 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r63);
            let Some(t64) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009151c4); };
            c.cmp(t64, 0x41u64, 16);
            if c.cc_ne() { blk = 50; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t65) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009151d0); };
            c.w32(2, t65);
            c.w32(6, c.r32(2));
            let r66 = c.r32(6) | 0x40u64;
            c.w32(6, r66);
            c.logic(r66, 32);
            c.cmp(c.r32(2), c.r32(6), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(2, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x70009151e1); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009151e6); }
            blk = 48; continue;
        }
        48 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70009151ee); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009151f2); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70009151f3); }
            c.w32(8, 0x8u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009151fa); }
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70009151fc); };
            c.w64(3, t67);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x7000915210, 0x700091520b);
        }
        49 => {
            c.w64(11, c.r64(0));
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000915213); };
            c.w64(15, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000915217); };
            c.w64(9, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091521b); };
            c.w64(7, t70);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000915222); }
            blk = 12; continue;
        }
        50 => {
            c.w32(2, 0xfeu64);
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t71) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000915232); };
            c.w32(6, t71);
            c.w32(2, c.r32(2));
            c.w32(11, c.r32(6));
            let r72 = c.r32(2) | c.r32(11);
            c.w32(2, r72);
            c.logic(r72, 32);
            c.cmp(c.r32(6), c.r32(2), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000915246); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091524b); }
            blk = 54; continue;
        }
        54 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000915253); }
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000915257); };
            c.w64(6, t73);
            return c.call(0x70008ae1b4, 0x7000915260, 0x700091525b);
        }
        55 => {
            c.w64(11, c.r64(0));
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000915263); };
            c.w64(15, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000915267); };
            c.w64(9, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091526b); };
            c.w64(7, t76);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000915272); }
            blk = 12; continue;
        }
        56 => {
            return RegfileExit::jump(0x700091527c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000915294(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000915294); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000915298); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091529e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009152a2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009152a6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009152ad); };
            c.w64(11, t5);
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009152b1); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009152b8); };
            c.w32(3, (t8 as u16 as i16 as i64 as u64));
            let Some(t9) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009152be); };
            c.w32(2, t9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009152c4); };
            c.w32(1, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009152c8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70009152cc); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009152d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009152d4); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70009152d8); }
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009152dc); };
            c.cmp(t11, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009152e5); };
            c.w64(8, t12);
            blk = 2; continue;
        }
        2 => {
            let r13 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009152ee); };
            c.cmp(t14, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70009152fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70009152ff); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091530b); };
            c.w64(11, t15);
            let Some(t16) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000915312); };
            c.w64(6, t16);
            let r17 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r17);
            c.w64(12, c.r64(6).wrapping_add(0xc));
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091531b); };
            c.w64(0, t18);
            let Some(t19) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000915322); };
            c.cmp(t19, c.r64(12), 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(11), c.r64(12)) { return RegfileExit::fault(0x700091532b); }
            c.w64(11, c.r64(6).wrapping_add(0x1));
            blk = 5; continue;
        }
        5 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000915332); };
            c.w64(0, t20);
            let r21 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r21);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000915348); };
            c.w64(0, t22);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700091534f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000915353); }
            c.w64(10, c.r64(4));
            let r23 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r23);
            let r24 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r24);
            c.logic(r24, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000915362); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x7000915367); };
            c.w64(0, t25);
            c.w64(10, 0x7000915375u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000915375); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000915379); }
            return c.call(c.r64(0), 0x700091537f, 0x700091537d);
        }
        6 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700091537f); }
            let Some(t26) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000915387); };
            c.w64(4, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091538c); };
            c.w64(0, t27);
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000915390); }
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000915395); };
            c.cmp(t28, 0x0u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091539b); };
            c.w64(8, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091539f); };
            c.w64(7, t30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009153a3); };
            c.w32(11, t31);
            c.w32(3, c.r32(11));
            let r32 = c.r32(3) | 0xeu64;
            c.w32(3, r32);
            c.logic(r32, 32);
            c.cmp(c.r32(11), c.r32(3), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(11, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70009153b6); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009153bb); }
            blk = 9; continue;
        }
        9 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009153c3); };
            c.w64(7, t33);
            let Some(t34) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009153c7); };
            c.w64(7, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009153cb); };
            c.w64(8, t35);
            c.w64(9, c.r64(8).wrapping_add(0x5));
            let Some(t36) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009153d3); };
            c.w32(8, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009153d8); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009153dc); };
            c.w64(1, t38);
            let Some(t39) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009153e0); };
            c.w64(5, t39);
            c.w64(12, c.r64(7));
            let r40 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r40);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70009153f2); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70009153fe, 0x70009153f9);
        }
        11 => {
            c.w64(11, c.r64(0));
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000915401); };
            c.w64(3, t41);
            blk = 5; continue;
        }
        12 => {
            let Some(t42) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091540a); };
            c.w32(6, t42);
            let r43 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r43);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000915410); }
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000915414); };
            c.cmp(t44, c.r32(6), 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 13; continue;
        }
        13 => {
            let r45 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r45);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000915423); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 0) { return RegfileExit::fault(0x7000915430); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 1) { return RegfileExit::fault(0x7000915435); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091543f); };
            c.w32(11, t46);
            c.w32(0, c.r32(11));
            let r47 = c.r32(0) | 0xeu64;
            c.w32(0, r47);
            c.logic(r47, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000915452); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000915457); }
            blk = 16; continue;
        }
        16 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091545f); };
            c.w64(8, t48);
            let Some(t49) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000915466); };
            c.w64(11, t49);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700091546d); };
            c.w64(3, t50);
            let Some(t51) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000915474); };
            c.cmp(t51, c.r64(0), 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000915483, 0x700091547e);
        }
        18 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x7000915483); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x7000915488); }
            blk = 20; continue;
        }
        19 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700091548f); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 20; continue;
        }
        20 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000915499); };
            c.w64(7, t52);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70009154a0); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009154a3); }
            c.w64(10, c.r64(4));
            let r53 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r53);
            let r54 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r54);
            c.logic(r54, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70009154b2); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x70009154b7); };
            c.w64(0, t55);
            c.w64(10, 0x70009154c5u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70009154c5); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70009154c9); }
            return c.call(c.r64(0), 0x70009154cf, 0x70009154cd);
        }
        21 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70009154cf); }
            let Some(t56) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70009154d7); };
            c.w64(4, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009154dc); };
            c.w64(0, t57);
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70009154e0); }
            blk = 9; continue;
        }
        22 => {
            let Some(t58) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009154ea); };
            c.w32(6, t58);
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r59 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r59);
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 30; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_ne() { blk = 37; continue; }
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t60) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091550a); };
            c.w32(6, t60);
            let r61 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r61);
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000915510); };
            c.cmp(t62, c.r32(6), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(6, 0x1eu64);
            blk = 38; continue;
        }
        27 => {
            let Some(t63) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000915523); };
            c.cmp(t63, 0x43u64, 16);
            if c.cc_ne() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(6, 0x1eu64);
            blk = 38; continue;
        }
        29 => {
            c.w32(6, 0x1eu64);
            blk = 38; continue;
        }
        30 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t64) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000915541); };
            c.w32(6, t64);
            let r65 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r65);
            let Some(t66) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000915547); };
            c.cmp(t66, 0x41u64, 16);
            if c.cc_ne() { blk = 37; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000915553); };
            c.w32(1, t67);
            c.w32(6, c.r32(1));
            let r68 = c.r32(6) | 0x40u64;
            c.w32(6, r68);
            c.logic(r68, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(1, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x7000915564); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000915569); }
            blk = 35; continue;
        }
        35 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000915571); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000915575); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000915576); }
            c.w32(8, 0xau64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091557d); }
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700091557f); };
            c.w64(3, t69);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x7000915593, 0x700091558e);
        }
        36 => {
            blk = 9; continue;
        }
        37 => {
            c.w32(6, 0xfeu64);
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t70) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009155a2); };
            c.w32(1, t70);
            c.w32(6, c.r32(6));
            c.w32(11, c.r32(1));
            let r71 = c.r32(6) | c.r32(11);
            c.w32(6, r71);
            c.logic(r71, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(11, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70009155b6); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009155bb); }
            blk = 41; continue;
        }
        41 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70009155c3); }
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009155c7); };
            c.w64(6, t72);
            return c.call(0x70008ae454, 0x70009155d0, 0x70009155cb);
        }
        42 => {
            blk = 9; continue;
        }
        43 => {
            return RegfileExit::jump(0x70009155d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009155f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009155f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009155f8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009155fe); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000915602); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000915606); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091560d); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000915614); };
            c.w32(11, (t7 as u16 as i16 as i64 as u64));
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091561a); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000915620); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000915624); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000915628); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091562c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000915630); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000915634); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000915638); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000915641); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000915648); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091564c); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000915659); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091565d); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000915663); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091566b); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000915672); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091567f); };
            c.w32(12, t17);
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r18 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r18);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000915699); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x70009156a5, 0x70009156a0);
        }
        8 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009156a5); };
            c.w64(3, t19);
            c.w32(6, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009156ae); };
            c.w64(15, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009156b2); };
            c.w64(9, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009156b6); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009156ba); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009156be); };
            c.w64(11, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009156c2); };
            c.w64(2, t25);
            blk = 3; continue;
        }
        9 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009156c8); };
            c.w32(0, t26);
            let r27 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r27);
            c.w32(6, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(2, c.r32(6));
            let r28 = c.r32(2) | 0x2u64;
            c.w32(2, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r29);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        11 => {
            c.w64(1, c.r64(9));
            c.w64(11, c.r64(1).wrapping_sub(0x1));
            let r30 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r30);
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009156eb); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009156f3); };
            c.w32(1, t32);
            let r33 = c.r32(0) | 0x40u64;
            c.w32(0, r33);
            c.logic(r33, 32);
            c.w32(6, c.r32(1));
            let r34 = c.r32(0) | c.r32(6);
            c.w32(0, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000915706); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091570b); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000915713); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091571c); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700091571d); };
            c.w64(3, t35);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x7000915731, 0x700091572c);
        }
        15 => {
            return RegfileExit::fault(0x7000915731);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000915732); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(6));
            let r36 = c.r32(2) | 0xeu64;
            c.w32(2, r36);
            c.logic(r36, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        18 => {
            let r37 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r37);
            let r38 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r38);
            let r39 = c.r32(11) | c.r32(0);
            c.w32(11, r39);
            c.logic(r39, 32);
            c.w64(0, c.r64(11));
            let r40 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r40);
            if c.cc_o() { blk = 27; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, 0xeu64);
            blk = 23; continue;
        }
        22 => {
            c.w32(0, 0x2u64);
            blk = 23; continue;
        }
        23 => {
            let Some(t41) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000915776); };
            c.cmp(t41, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091577c); };
            c.w32(1, t42);
            c.w32(2, c.r32(2));
            c.w32(0, c.r32(0));
            c.w32(6, c.r32(1));
            let r43 = c.r32(2) | c.r32(0);
            c.w32(2, r43);
            c.logic(r43, 32);
            let r44 = c.r32(2) | c.r32(6);
            c.w32(2, r44);
            c.logic(r44, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000915792); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000915797); }
            blk = 26; continue;
        }
        26 => {
            let Some(t45) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091579f); };
            c.w64(7, t45);
            let r46 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r46);
            let Some(t47) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009157a7); };
            c.w32(8, t47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009157ac); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009157b0); };
            c.w64(5, t49);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r50 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r50);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009157c0); };
            c.w64(0, t51);
            let Some(t52) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70009157c7); };
            c.w64(1, t52);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70009157ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70009157d2); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009157d6); };
            c.w64(12, t53);
            let Some(t54) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70009157dd); };
            c.cmp(t54, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(0, 0xcu64);
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x70009157f0, 0x70009157eb);
        }
        29 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009157f0); };
            c.w64(3, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009157f4); };
            c.w64(15, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009157f8); };
            c.w64(9, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009157fc); };
            c.w64(7, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000915800); };
            c.w64(8, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000915804); };
            c.w64(2, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000915808); };
            c.w64(11, t61);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x700091580e); }
            c.w64(0, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000915815); };
            c.w64(1, t62);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700091581c); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000915828); }
            c.w64(11, c.r64(0));
            blk = 20; continue;
        }
        32 => {
            let r63 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r63);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000915839); }
            return c.call(0x70008708b4, 0x7000915843, 0x700091583e);
        }
        33 => {
            let Some(t64) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000915843); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r65);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x7000915850);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000915854(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000915854); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000915858); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091585e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000915862); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000915866); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091586d); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000915874); };
            c.w32(11, (t7 as u16 as i16 as i64 as u64));
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091587a); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000915880); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000915884); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000915888); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091588c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000915890); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000915894); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000915898); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009158a1); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009158a8); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009158ac); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009158b9); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70009158bd); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70009158c3); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009158cb); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009158d2); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009158df); };
            c.w32(12, t17);
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r18 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r18);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70009158f9); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x7000915905, 0x7000915900);
        }
        8 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000915905); };
            c.w64(3, t19);
            c.w32(6, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091590e); };
            c.w64(15, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000915912); };
            c.w64(9, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000915916); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091591a); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091591e); };
            c.w64(11, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000915922); };
            c.w64(2, t25);
            blk = 3; continue;
        }
        9 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000915928); };
            c.w32(0, t26);
            let r27 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r27);
            c.w32(6, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(2, c.r32(6));
            let r28 = c.r32(2) | 0x2u64;
            c.w32(2, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r29);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        11 => {
            c.w64(1, c.r64(9));
            c.w64(11, c.r64(1).wrapping_sub(0x1));
            let r30 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r30);
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700091594b); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000915953); };
            c.w32(1, t32);
            let r33 = c.r32(0) | 0x40u64;
            c.w32(0, r33);
            c.logic(r33, 32);
            c.w32(6, c.r32(1));
            let r34 = c.r32(0) | c.r32(6);
            c.w32(0, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000915966); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091596b); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000915973); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091597c); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700091597d); };
            c.w64(3, t35);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x7000915991, 0x700091598c);
        }
        15 => {
            return RegfileExit::fault(0x7000915991);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000915992); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(6));
            let r36 = c.r32(2) | 0xeu64;
            c.w32(2, r36);
            c.logic(r36, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        18 => {
            let r37 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r37);
            let r38 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r38);
            let r39 = c.r32(11) ^ c.r32(0);
            c.w32(11, r39);
            c.logic(r39, 32);
            c.w64(0, c.r64(11));
            let r40 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r40);
            if c.cc_o() { blk = 27; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, 0xeu64);
            blk = 23; continue;
        }
        22 => {
            c.w32(0, 0x2u64);
            blk = 23; continue;
        }
        23 => {
            let Some(t41) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009159d6); };
            c.cmp(t41, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009159dc); };
            c.w32(1, t42);
            c.w32(2, c.r32(2));
            c.w32(0, c.r32(0));
            c.w32(6, c.r32(1));
            let r43 = c.r32(2) | c.r32(0);
            c.w32(2, r43);
            c.logic(r43, 32);
            let r44 = c.r32(2) | c.r32(6);
            c.w32(2, r44);
            c.logic(r44, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70009159f2); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009159f7); }
            blk = 26; continue;
        }
        26 => {
            let Some(t45) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009159ff); };
            c.w64(7, t45);
            let r46 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r46);
            let Some(t47) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000915a07); };
            c.w32(8, t47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000915a0c); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000915a10); };
            c.w64(5, t49);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r50 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r50);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000915a20); };
            c.w64(0, t51);
            let Some(t52) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000915a27); };
            c.w64(1, t52);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000915a2e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000915a32); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000915a36); };
            c.w64(12, t53);
            let Some(t54) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000915a3d); };
            c.cmp(t54, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(0, 0xcu64);
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x7000915a50, 0x7000915a4b);
        }
        29 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000915a50); };
            c.w64(3, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000915a54); };
            c.w64(15, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000915a58); };
            c.w64(9, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000915a5c); };
            c.w64(7, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000915a60); };
            c.w64(8, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000915a64); };
            c.w64(2, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000915a68); };
            c.w64(11, t61);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x7000915a6e); }
            c.w64(0, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000915a75); };
            c.w64(1, t62);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000915a7c); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000915a88); }
            c.w64(11, c.r64(0));
            blk = 20; continue;
        }
        32 => {
            let r63 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r63);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000915a99); }
            return c.call(0x70008708b4, 0x7000915aa3, 0x7000915a9e);
        }
        33 => {
            let Some(t64) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000915aa3); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r65);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x7000915ab0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000915ab4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000915ab4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000915ab8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000915abe); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000915ac2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000915ac6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000915acd); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000915ad4); };
            c.w32(11, (t7 as u16 as i16 as i64 as u64));
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000915ada); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000915ae0); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000915ae4); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000915ae8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000915aec); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000915af0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000915af4); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000915af8); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000915b01); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000915b08); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000915b0c); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000915b19); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000915b1d); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000915b23); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000915b2b); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000915b32); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000915b3f); };
            c.w32(12, t17);
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r18 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r18);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000915b59); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x7000915b65, 0x7000915b60);
        }
        8 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000915b65); };
            c.w64(3, t19);
            c.w32(6, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000915b6e); };
            c.w64(15, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000915b72); };
            c.w64(9, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000915b76); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000915b7a); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000915b7e); };
            c.w64(11, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000915b82); };
            c.w64(2, t25);
            blk = 3; continue;
        }
        9 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000915b88); };
            c.w32(0, t26);
            let r27 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r27);
            c.w32(6, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(2, c.r32(6));
            let r28 = c.r32(2) | 0x2u64;
            c.w32(2, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r29);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        11 => {
            c.w64(1, c.r64(9));
            c.w64(11, c.r64(1).wrapping_sub(0x1));
            let r30 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r30);
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000915bab); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000915bb3); };
            c.w32(1, t32);
            let r33 = c.r32(0) | 0x40u64;
            c.w32(0, r33);
            c.logic(r33, 32);
            c.w32(6, c.r32(1));
            let r34 = c.r32(0) | c.r32(6);
            c.w32(0, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000915bc6); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000915bcb); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000915bd3); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000915bdc); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000915bdd); };
            c.w64(3, t35);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x7000915bf1, 0x7000915bec);
        }
        15 => {
            return RegfileExit::fault(0x7000915bf1);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000915bf2); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(6));
            let r36 = c.r32(2) | 0xeu64;
            c.w32(2, r36);
            c.logic(r36, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        18 => {
            let r37 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r37);
            let r38 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r38);
            let r39 = c.r32(11) & c.r32(0);
            c.w32(11, r39);
            c.logic(r39, 32);
            c.w64(0, c.r64(11));
            let r40 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r40);
            if c.cc_o() { blk = 27; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, 0xeu64);
            blk = 23; continue;
        }
        22 => {
            c.w32(0, 0x2u64);
            blk = 23; continue;
        }
        23 => {
            let Some(t41) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000915c36); };
            c.cmp(t41, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000915c3c); };
            c.w32(1, t42);
            c.w32(2, c.r32(2));
            c.w32(0, c.r32(0));
            c.w32(6, c.r32(1));
            let r43 = c.r32(2) | c.r32(0);
            c.w32(2, r43);
            c.logic(r43, 32);
            let r44 = c.r32(2) | c.r32(6);
            c.w32(2, r44);
            c.logic(r44, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000915c52); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000915c57); }
            blk = 26; continue;
        }
        26 => {
            let Some(t45) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000915c5f); };
            c.w64(7, t45);
            let r46 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r46);
            let Some(t47) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000915c67); };
            c.w32(8, t47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000915c6c); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000915c70); };
            c.w64(5, t49);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r50 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r50);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000915c80); };
            c.w64(0, t51);
            let Some(t52) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000915c87); };
            c.w64(1, t52);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000915c8e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000915c92); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000915c96); };
            c.w64(12, t53);
            let Some(t54) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000915c9d); };
            c.cmp(t54, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(0, 0xcu64);
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x7000915cb0, 0x7000915cab);
        }
        29 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000915cb0); };
            c.w64(3, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000915cb4); };
            c.w64(15, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000915cb8); };
            c.w64(9, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000915cbc); };
            c.w64(7, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000915cc0); };
            c.w64(8, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000915cc4); };
            c.w64(2, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000915cc8); };
            c.w64(11, t61);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x7000915cce); }
            c.w64(0, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000915cd5); };
            c.w64(1, t62);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000915cdc); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000915ce8); }
            c.w64(11, c.r64(0));
            blk = 20; continue;
        }
        32 => {
            let r63 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r63);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000915cf9); }
            return c.call(0x70008708b4, 0x7000915d03, 0x7000915cfe);
        }
        33 => {
            let Some(t64) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000915d03); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r65);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x7000915d10);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000915d14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000915d14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000915d18); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000915d1e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000915d22); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000915d26); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000915d2d); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000915d34); };
            c.w32(11, (t7 as u16 as i16 as i64 as u64));
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000915d3a); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000915d40); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000915d44); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000915d48); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000915d4c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000915d50); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000915d54); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000915d58); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000915d61); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000915d68); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000915d6c); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000915d79); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000915d7d); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000915d83); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000915d8b); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000915d92); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000915d9f); };
            c.w32(12, t17);
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r18 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r18);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000915db9); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x7000915dc5, 0x7000915dc0);
        }
        8 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000915dc5); };
            c.w64(3, t19);
            c.w32(6, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000915dce); };
            c.w64(15, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000915dd2); };
            c.w64(9, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000915dd6); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000915dda); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000915dde); };
            c.w64(11, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000915de2); };
            c.w64(2, t25);
            blk = 3; continue;
        }
        9 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000915de8); };
            c.w32(0, t26);
            let r27 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r27);
            c.w32(6, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(2, c.r32(6));
            let r28 = c.r32(2) | 0x2u64;
            c.w32(2, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r29);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        11 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r30 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r30);
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000915e08); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000915e10); };
            c.w32(1, t32);
            let r33 = c.r32(0) | 0x40u64;
            c.w32(0, r33);
            c.logic(r33, 32);
            c.w32(6, c.r32(1));
            let r34 = c.r32(0) | c.r32(6);
            c.w32(0, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000915e23); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000915e28); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000915e30); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000915e39); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000915e3a); };
            c.w64(3, t35);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x7000915e4e, 0x7000915e49);
        }
        15 => {
            return RegfileExit::fault(0x7000915e4e);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000915e4f); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(6));
            let r36 = c.r32(2) | 0xeu64;
            c.w32(2, r36);
            c.logic(r36, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        18 => {
            let r37 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r37);
            let r38 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r38);
            c.w64(1, c.r64(11));
            let r39 = c.shl(c.r32(0), c.r8(1), 32).unwrap_or(c.r32(0));
            c.w32(0, r39);
            c.w64(11, c.r64(0));
            let r40 = c.add(c.r32(11), c.r32(0), 32);
            c.w32(11, r40);
            if c.cc_o() { blk = 27; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, 0xeu64);
            blk = 23; continue;
        }
        22 => {
            c.w32(0, 0x2u64);
            blk = 23; continue;
        }
        23 => {
            let Some(t41) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000915e95); };
            c.cmp(t41, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000915e9b); };
            c.w32(1, t42);
            c.w32(2, c.r32(2));
            c.w32(0, c.r32(0));
            c.w32(6, c.r32(1));
            let r43 = c.r32(2) | c.r32(0);
            c.w32(2, r43);
            c.logic(r43, 32);
            let r44 = c.r32(2) | c.r32(6);
            c.w32(2, r44);
            c.logic(r44, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000915eb1); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000915eb6); }
            blk = 26; continue;
        }
        26 => {
            let Some(t45) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000915ebe); };
            c.w64(7, t45);
            let r46 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r46);
            let Some(t47) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000915ec6); };
            c.w32(8, t47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000915ecb); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000915ecf); };
            c.w64(5, t49);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r50 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r50);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000915edf); };
            c.w64(11, t51);
            let Some(t52) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000915ee6); };
            c.w64(1, t52);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000915eed); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x7000915ef1); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000915ef5); };
            c.w64(12, t53);
            let Some(t54) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000915efc); };
            c.cmp(t54, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x7000915f0f, 0x7000915f0a);
        }
        29 => {
            c.w64(11, c.r64(0));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000915f12); };
            c.w64(15, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000915f16); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000915f1a); };
            c.w64(7, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000915f1e); };
            c.w64(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000915f22); };
            c.w64(2, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000915f26); };
            c.w64(0, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000915f2a); };
            c.w64(3, t61);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(11), c.r64(6)) { return RegfileExit::fault(0x7000915f30); }
            c.w64(11, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000915f37); };
            c.w64(1, t62);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000915f3e); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000915f4a); }
            blk = 20; continue;
        }
        32 => {
            let r63 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r63);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000915f59); }
            return c.call(0x70008708b4, 0x7000915f63, 0x7000915f5e);
        }
        33 => {
            let Some(t64) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000915f63); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r65);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x7000915f70);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000915f74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000915f74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000915f78); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000915f7e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000915f82); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000915f86); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000915f8d); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000915f94); };
            c.w32(11, (t7 as u16 as i16 as i64 as u64));
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000915f9a); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000915fa0); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000915fa4); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000915fa8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000915fac); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000915fb0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000915fb4); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000915fb8); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000915fc1); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000915fc8); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000915fcc); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000915fd9); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000915fdd); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000915fe3); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000915feb); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000915ff2); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000915fff); };
            c.w32(12, t17);
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r18 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r18);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000916019); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x7000916025, 0x7000916020);
        }
        8 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000916025); };
            c.w64(3, t19);
            c.w32(6, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091602e); };
            c.w64(15, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000916032); };
            c.w64(9, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000916036); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091603a); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091603e); };
            c.w64(11, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000916042); };
            c.w64(2, t25);
            blk = 3; continue;
        }
        9 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000916048); };
            c.w32(0, t26);
            let r27 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r27);
            c.w32(6, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(2, c.r32(6));
            let r28 = c.r32(2) | 0x2u64;
            c.w32(2, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r29);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        11 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r30 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r30);
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000916068); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000916070); };
            c.w32(1, t32);
            let r33 = c.r32(0) | 0x40u64;
            c.w32(0, r33);
            c.logic(r33, 32);
            c.w32(6, c.r32(1));
            let r34 = c.r32(0) | c.r32(6);
            c.w32(0, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000916083); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000916088); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000916090); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000916099); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700091609a); };
            c.w64(3, t35);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70009160ae, 0x70009160a9);
        }
        15 => {
            return RegfileExit::fault(0x70009160ae);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009160af); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(6));
            let r36 = c.r32(2) | 0xeu64;
            c.w32(2, r36);
            c.logic(r36, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        18 => {
            let r37 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r37);
            let r38 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r38);
            c.w64(1, c.r64(11));
            let r39 = c.sar(c.r32(0), c.r8(1), 32).unwrap_or(c.r32(0));
            c.w32(0, r39);
            c.w64(11, c.r64(0));
            let r40 = c.add(c.r32(11), c.r32(0), 32);
            c.w32(11, r40);
            if c.cc_o() { blk = 27; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, 0xeu64);
            blk = 23; continue;
        }
        22 => {
            c.w32(0, 0x2u64);
            blk = 23; continue;
        }
        23 => {
            let Some(t41) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009160f5); };
            c.cmp(t41, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009160fb); };
            c.w32(1, t42);
            c.w32(2, c.r32(2));
            c.w32(0, c.r32(0));
            c.w32(6, c.r32(1));
            let r43 = c.r32(2) | c.r32(0);
            c.w32(2, r43);
            c.logic(r43, 32);
            let r44 = c.r32(2) | c.r32(6);
            c.w32(2, r44);
            c.logic(r44, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000916111); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000916116); }
            blk = 26; continue;
        }
        26 => {
            let Some(t45) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091611e); };
            c.w64(7, t45);
            let r46 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r46);
            let Some(t47) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000916126); };
            c.w32(8, t47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091612b); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091612f); };
            c.w64(5, t49);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r50 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r50);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091613f); };
            c.w64(11, t51);
            let Some(t52) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000916146); };
            c.w64(1, t52);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091614d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x7000916151); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000916155); };
            c.w64(12, t53);
            let Some(t54) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700091615c); };
            c.cmp(t54, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700091616f, 0x700091616a);
        }
        29 => {
            c.w64(11, c.r64(0));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000916172); };
            c.w64(15, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000916176); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091617a); };
            c.w64(7, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091617e); };
            c.w64(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000916182); };
            c.w64(2, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000916186); };
            c.w64(0, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700091618a); };
            c.w64(3, t61);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(11), c.r64(6)) { return RegfileExit::fault(0x7000916190); }
            c.w64(11, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000916197); };
            c.w64(1, t62);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700091619e); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70009161aa); }
            blk = 20; continue;
        }
        32 => {
            let r63 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r63);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70009161b9); }
            return c.call(0x70008708b4, 0x70009161c3, 0x70009161be);
        }
        33 => {
            let Some(t64) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70009161c3); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r65);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x70009161d0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009161d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009161d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009161d8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009161de); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009161e2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009161e6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009161ed); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009161f4); };
            c.w32(11, (t7 as u16 as i16 as i64 as u64));
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009161fa); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000916200); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000916204); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000916208); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091620c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000916210); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000916214); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000916218); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000916221); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000916228); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091622c); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000916239); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700091623d); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000916243); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091624b); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000916252); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t17) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091625f); };
            c.w32(12, t17);
            c.cmp(c.r16(12), 0x41u64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r18 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r18);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000916279); }
            c.w64(6, c.r64(2));
            return c.call(0x70008a10d4, 0x7000916285, 0x7000916280);
        }
        8 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000916285); };
            c.w64(3, t19);
            c.w32(6, 0xfeu64);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091628e); };
            c.w64(15, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000916292); };
            c.w64(9, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000916296); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091629a); };
            c.w64(8, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091629e); };
            c.w64(11, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009162a2); };
            c.w64(2, t25);
            blk = 3; continue;
        }
        9 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009162a8); };
            c.w32(0, t26);
            let r27 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r27);
            c.w32(6, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(2, c.r32(6));
            let r28 = c.r32(2) | 0x2u64;
            c.w32(2, r28);
            c.logic(r28, 32);
            let r29 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r29);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        11 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r30 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r30);
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009162c8); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(6));
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009162d0); };
            c.w32(1, t32);
            let r33 = c.r32(0) | 0x40u64;
            c.w32(0, r33);
            c.logic(r33, 32);
            c.w32(6, c.r32(1));
            let r34 = c.r32(0) | c.r32(6);
            c.w32(0, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x70009162e3); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009162e8); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009162f0); }
            c.w32(7, 0x2au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009162f9); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70009162fa); };
            c.w64(3, t35);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x700091630e, 0x7000916309);
        }
        15 => {
            return RegfileExit::fault(0x700091630e);
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700091630f); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(6));
            let r36 = c.r32(2) | 0xeu64;
            c.w32(2, r36);
            c.logic(r36, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 18; continue;
        }
        18 => {
            let r37 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r37);
            let r38 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r38);
            c.w64(1, c.r64(11));
            let r39 = c.shr(c.r32(0), c.r8(1), 32).unwrap_or(c.r32(0));
            c.w32(0, r39);
            c.cmp(c.r32(0), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 27; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(11, c.r64(0).wrapping_add(c.r64(0)));
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, 0xeu64);
            blk = 23; continue;
        }
        22 => {
            c.w32(0, 0x2u64);
            blk = 23; continue;
        }
        23 => {
            let Some(t40) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000916355); };
            c.cmp(t40, 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700091635b); };
            c.w32(1, t41);
            c.w32(2, c.r32(2));
            c.w32(0, c.r32(0));
            c.w32(6, c.r32(1));
            let r42 = c.r32(2) | c.r32(0);
            c.w32(2, r42);
            c.logic(r42, 32);
            let r43 = c.r32(2) | c.r32(6);
            c.w32(2, r43);
            c.logic(r43, 32);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000916371); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000916376); }
            blk = 26; continue;
        }
        26 => {
            let Some(t44) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091637e); };
            c.w64(7, t44);
            let r45 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r45);
            let Some(t46) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000916386); };
            c.w32(8, t46);
            let Some(t47) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091638b); };
            c.w64(1, t47);
            let Some(t48) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700091638f); };
            c.w64(5, t48);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r49 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r49);
            return RegfileExit::jump(c.r64(1));
        }
        27 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700091639f); };
            c.w64(11, t50);
            let Some(t51) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009163a6); };
            c.w64(1, t51);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70009163ad); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70009163b1); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009163b5); };
            c.w64(12, t52);
            let Some(t53) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70009163bc); };
            c.cmp(t53, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70009163cf, 0x70009163ca);
        }
        29 => {
            c.w64(11, c.r64(0));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009163d2); };
            c.w64(15, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009163d6); };
            c.w64(9, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009163da); };
            c.w64(7, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009163de); };
            c.w64(8, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009163e2); };
            c.w64(2, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009163e6); };
            c.w64(0, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009163ea); };
            c.w64(3, t60);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(11), c.r64(6)) { return RegfileExit::fault(0x70009163f0); }
            c.w64(11, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009163f7); };
            c.w64(1, t61);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70009163fe); }
            c.w32(10, c.r32(0));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700091640e); }
            blk = 20; continue;
        }
        32 => {
            let r62 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r62);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700091641d); }
            return c.call(0x70008708b4, 0x7000916427, 0x7000916422);
        }
        33 => {
            let Some(t63) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000916427); };
            c.w32(0, t63);
            let r64 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r64);
            blk = 17; continue;
        }
        34 => {
            return RegfileExit::jump(0x7000916434);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000916454(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000916457); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091645b); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000916462); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000916466); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091646d); };
            c.w32(3, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000916473); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000916477); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000916480); };
            c.w64(8, t9);
            blk = 2; continue;
        }
        2 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 3; continue;
        }
        3 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091648f); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000916495); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t13) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091649e); };
            c.w32(1, t13);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 21; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_ne() { blk = 19; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009164b6); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            c.w32(2, 0x1eu64);
            blk = 3; continue;
        }
        8 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009164c3); }
            blk = 12; continue;
        }
        9 => {
            let r16 = c.add(c.r32(1), 0x2u64, 32);
            c.w32(1, r16);
            if c.cc_o() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(11, c.r32(2));
            let r17 = c.r32(11) | 0x2u64;
            c.w32(11, r17);
            c.logic(r17, 32);
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 15; continue;
        }
        11 => {
            let r18 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r18);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 12; continue;
        }
        12 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009164e8); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009164ef); };
            c.w64(0, t20);
            c.w64(1, c.r64(0).wrapping_add(0xc));
            c.w32(2, c.r32(2));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009164f8); };
            c.w64(6, t21);
            let Some(t22) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70009164ff); };
            c.cmp(t22, c.r64(1), 64);
            if c.cc_be() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(11), c.r64(1)) { return RegfileExit::fault(0x7000916508); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 14; continue;
        }
        14 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000916521); };
            c.w64(0, t23);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000916528); }
            let r24 = c.r32(2) | 0xeu64;
            c.w32(2, r24);
            c.logic(r24, 32);
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700091652f); }
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            blk = 15; continue;
        }
        15 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091653b); };
            c.cmp(t25, c.r32(8), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000916544); };
            c.w32(11, t26);
            c.w32(2, c.r32(2));
            c.w32(1, c.r32(11));
            let r27 = c.r32(2) | c.r32(1);
            c.w32(2, r27);
            c.logic(r27, 32);
            c.cmp(c.r32(11), c.r32(2), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000916558); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091655d); }
            blk = 18; continue;
        }
        18 => {
            let Some(t28) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000916565); };
            c.w64(7, t28);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t29) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000916570); };
            c.w32(8, t29);
            let Some(t30) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000916575); };
            c.w64(1, t30);
            c.w64(12, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091657e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000916582); }
            let r31 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r31);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let r32 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r32);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700091658e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000916592); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000916596); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700091659a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091659e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70009165a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70009165a6); }
            c.w64(6, c.r64(11));
            return c.call(0x70008a10d4, 0x70009165b2, 0x70009165ad);
        }
        20 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009165b2); };
            c.w64(11, t33);
            c.w32(2, 0xfeu64);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009165bb); };
            c.w64(15, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009165bf); };
            c.w64(9, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009165c3); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009165c7); };
            c.w64(8, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009165cb); };
            c.w64(3, t38);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70009165d2); }
            blk = 3; continue;
        }
        21 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009165d8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009165dc); }
            let r39 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r39);
            c.w32(2, c.r32(2));
            let r40 = c.r32(2) | 0x40u64;
            c.w32(2, r40);
            c.logic(r40, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r41 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r41);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70009165f0); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009165f4); }
            c.w32(0, 0x1cu64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009165fa); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009165fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009165ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000916603); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000916607); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700091660b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700091660f); }
            c.w64(6, c.r64(11));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x7000916616); };
            c.w64(3, t42);
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x700091662a, 0x7000916625);
        }
        22 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091662a); };
            c.w64(2, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091662e); };
            c.w64(3, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000916632); };
            c.w64(15, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000916636); };
            c.w64(9, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091663a); };
            c.w64(7, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091663e); };
            c.w64(8, t48);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000916645); }
            blk = 15; continue;
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700091664b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091664f); }
            let r49 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r49);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000916655); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091665e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000916662); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000916666); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700091666a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700091666e); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x7000916672); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700091667f, 0x700091667a);
        }
        24 => {
            c.w64(11, c.r64(0));
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000916682); };
            c.w64(15, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000916686); };
            c.w64(9, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091668a); };
            c.w64(7, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091668e); };
            c.w64(8, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000916692); };
            c.w64(3, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000916696); };
            c.w64(2, t55);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700091669a); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70009166a2); }
            blk = 14; continue;
        }
        25 => {
            return RegfileExit::jump(0x70009166ac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009166b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009166b7); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009166bb); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009166c2); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009166c6); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009166cd); };
            c.w32(3, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009166d3); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009166d7); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009166e0); };
            c.w64(8, t9);
            blk = 2; continue;
        }
        2 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 3; continue;
        }
        3 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009166ef); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009166f5); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t13) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009166fe); };
            c.w32(1, t13);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 21; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_ne() { blk = 19; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000916716); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            c.w32(2, 0x1eu64);
            blk = 3; continue;
        }
        8 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000916723); }
            blk = 12; continue;
        }
        9 => {
            let r16 = c.add(c.r32(1), 0xfffffffffffffffeu64, 32);
            c.w32(1, r16);
            if c.cc_o() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(11, c.r32(2));
            let r17 = c.r32(11) | 0x2u64;
            c.w32(11, r17);
            c.logic(r17, 32);
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 15; continue;
        }
        11 => {
            let r18 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r18);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 12; continue;
        }
        12 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000916748); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700091674f); };
            c.w64(0, t20);
            c.w64(1, c.r64(0).wrapping_add(0xc));
            c.w32(2, c.r32(2));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000916758); };
            c.w64(6, t21);
            let Some(t22) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700091675f); };
            c.cmp(t22, c.r64(1), 64);
            if c.cc_be() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(11), c.r64(1)) { return RegfileExit::fault(0x7000916768); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 14; continue;
        }
        14 => {
            c.w64(10, 0xbff0000000000000u64);
            c.movq_xr(1, 10);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000916782); };
            c.w64(0, t23);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000916789); }
            let r24 = c.r32(2) | 0xeu64;
            c.w32(2, r24);
            c.logic(r24, 32);
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x7000916790); }
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            blk = 15; continue;
        }
        15 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091679c); };
            c.cmp(t25, c.r32(8), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009167a5); };
            c.w32(11, t26);
            c.w32(2, c.r32(2));
            c.w32(1, c.r32(11));
            let r27 = c.r32(2) | c.r32(1);
            c.w32(2, r27);
            c.logic(r27, 32);
            c.cmp(c.r32(11), c.r32(2), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70009167b9); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009167be); }
            blk = 18; continue;
        }
        18 => {
            let Some(t28) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009167c6); };
            c.w64(7, t28);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t29) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009167d1); };
            c.w32(8, t29);
            let Some(t30) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009167d6); };
            c.w64(1, t30);
            c.w64(12, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009167df); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009167e3); }
            let r31 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r31);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let r32 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r32);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009167ef); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009167f3); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009167f7); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009167fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009167ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000916803); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000916807); }
            c.w64(6, c.r64(11));
            return c.call(0x70008a10d4, 0x7000916813, 0x700091680e);
        }
        20 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000916813); };
            c.w64(11, t33);
            c.w32(2, 0xfeu64);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091681c); };
            c.w64(15, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000916820); };
            c.w64(9, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000916824); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000916828); };
            c.w64(8, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091682c); };
            c.w64(3, t38);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000916833); }
            blk = 3; continue;
        }
        21 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000916839); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700091683d); }
            let r39 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r39);
            c.w32(2, c.r32(2));
            let r40 = c.r32(2) | 0x40u64;
            c.w32(2, r40);
            c.logic(r40, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r41 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r41);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000916851); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000916855); }
            c.w32(0, 0x1eu64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700091685b); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091685c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000916860); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000916864); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000916868); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700091686c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x7000916870); }
            c.w64(6, c.r64(11));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x7000916877); };
            c.w64(3, t42);
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x700091688b, 0x7000916886);
        }
        22 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700091688b); };
            c.w64(2, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700091688f); };
            c.w64(3, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000916893); };
            c.w64(15, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000916897); };
            c.w64(9, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091689b); };
            c.w64(7, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700091689f); };
            c.w64(8, t48);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70009168a6); }
            blk = 15; continue;
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009168ac); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009168b0); }
            let r49 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r49);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70009168b6); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009168bf); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009168c3); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009168c7); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009168cb); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70009168cf); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x70009168d3); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70009168e0, 0x70009168db);
        }
        24 => {
            c.w64(11, c.r64(0));
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009168e3); };
            c.w64(15, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009168e7); };
            c.w64(9, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009168eb); };
            c.w64(7, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009168ef); };
            c.w64(8, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009168f3); };
            c.w64(3, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009168f7); };
            c.w64(2, t55);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x70009168fb); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000916903); }
            blk = 14; continue;
        }
        25 => {
            return RegfileExit::jump(0x700091690c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000916914(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000916917); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091691b); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000916922); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000916926); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700091692d); };
            c.w32(3, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000916933); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000916937); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000916940); };
            c.w64(8, t9);
            blk = 2; continue;
        }
        2 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 3; continue;
        }
        3 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091694f); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000916955); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t13) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091695e); };
            c.w32(1, t13);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_ne() { blk = 21; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000916976); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            c.w32(2, 0x1eu64);
            blk = 3; continue;
        }
        8 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000916983); }
            blk = 13; continue;
        }
        9 => {
            c.w32(11, c.r32(2));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(6, 0x80000000u64);
            c.cmp(c.r32(6), c.r32(0), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            let r16 = c.neg(c.r32(1), 32);
            c.w32(1, r16);
            let r17 = c.r32(11) | 0x2u64;
            c.w32(11, r17);
            c.logic(r17, 32);
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 17; continue;
        }
        12 => {
            let r18 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r18);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 13; continue;
        }
        13 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009169b3); };
            c.w64(11, t19);
            let Some(t20) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009169ba); };
            c.w64(0, t20);
            c.w64(1, c.r64(0).wrapping_add(0xc));
            c.w32(2, c.r32(2));
            let r21 = c.r32(2) | 0xeu64;
            c.w32(2, r21);
            c.logic(r21, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009169c9); };
            c.w64(6, t22);
            let Some(t23) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70009169d0); };
            c.cmp(t23, c.r64(1), 64);
            if c.cc_be() { blk = 25; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(11), c.r64(1)) { return RegfileExit::fault(0x70009169d9); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 15; continue;
        }
        15 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70009169e0); };
            c.w64(10, t24);
            if !c.xorpd_rm(0, c.r64(10)) { return RegfileExit::fault(0x70009169e7); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009169eb); };
            c.w64(0, t25);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70009169f2); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70009169f6); }
            c.w64(0, c.r64(11));
            blk = 17; continue;
        }
        16 => {
            let r26 = c.r32(11) | 0xeu64;
            c.w32(11, r26);
            c.logic(r26, 32);
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x7000916a08); };
            c.w64(0, t27);
            blk = 17; continue;
        }
        17 => {
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000916a0f); };
            c.cmp(t28, c.r32(8), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000916a18); };
            c.w32(11, t29);
            c.w32(2, c.r32(2));
            c.w32(1, c.r32(11));
            let r30 = c.r32(2) | c.r32(1);
            c.w32(2, r30);
            c.logic(r30, 32);
            c.cmp(c.r32(11), c.r32(2), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000916a2c); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000916a31); }
            blk = 20; continue;
        }
        20 => {
            let Some(t31) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000916a39); };
            c.w64(7, t31);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t32) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000916a44); };
            c.w32(8, t32);
            let Some(t33) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000916a49); };
            c.w64(1, t33);
            c.w64(12, c.r64(7));
            return RegfileExit::jump(c.r64(1));
        }
        21 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000916a52); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000916a56); }
            let r34 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r34);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let r35 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r35);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000916a62); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000916a66); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000916a6a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000916a6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000916a72); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000916a76); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000916a7a); }
            c.w64(6, c.r64(11));
            return c.call(0x70008a10d4, 0x7000916a86, 0x7000916a81);
        }
        22 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000916a86); };
            c.w64(11, t36);
            c.w32(2, 0xfeu64);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000916a8f); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000916a93); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000916a97); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000916a9b); };
            c.w64(8, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000916a9f); };
            c.w64(3, t41);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000916aa6); }
            blk = 3; continue;
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000916aac); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000916ab0); }
            let r42 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r42);
            c.w32(2, c.r32(2));
            let r43 = c.r32(2) | 0x40u64;
            c.w32(2, r43);
            c.logic(r43, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r44 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r44);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000916ac4); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000916ac8); }
            c.w32(0, 0x1au64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000916ace); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000916acf); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000916ad3); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000916ad7); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000916adb); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000916adf); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x7000916ae3); }
            c.w64(6, c.r64(11));
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x7000916aea); };
            c.w64(3, t45);
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x7000916afe, 0x7000916af9);
        }
        24 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000916afe); };
            c.w64(2, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000916b02); };
            c.w64(3, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000916b06); };
            c.w64(15, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000916b0a); };
            c.w64(9, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000916b0e); };
            c.w64(7, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000916b12); };
            c.w64(8, t51);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000916b19); }
            blk = 17; continue;
        }
        25 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000916b1f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000916b23); }
            let r52 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r52);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x7000916b29); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000916b32); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000916b36); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000916b3a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000916b3e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000916b42); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x7000916b46); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x7000916b53, 0x7000916b4e);
        }
        26 => {
            c.w64(11, c.r64(0));
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000916b56); };
            c.w64(15, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000916b5a); };
            c.w64(9, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000916b5e); };
            c.w64(7, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000916b62); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000916b66); };
            c.w64(3, t57);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x7000916b6a); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000916b6f); };
            c.w64(2, t58);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000916b76); }
            blk = 15; continue;
        }
        27 => {
            return RegfileExit::jump(0x7000916b80);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000916b94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000916b94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000916b98); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000916b9e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000916ba2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000916ba6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000916bad); };
            c.w64(11, t5);
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000916bb1); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000916bb8); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000916bbe); };
            c.w32(2, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000916bc2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000916bc6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000916bca); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000916bce); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000916bd2); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000916bd6); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000916bdf); };
            c.w64(8, t11);
            blk = 2; continue;
        }
        2 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000916be6); };
            c.cmp(t12, c.r32(8), 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000916bf3); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000916bf9); }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000916c01); };
            c.w32(6, t13);
            let r14 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000916c07); };
            c.cmp(t15, c.r32(6), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t16) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000916c10); };
            c.w32(1, t16);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 22; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r17 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r17);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000916c2a); }
            c.w64(6, c.r64(11));
            return c.call(0x70008a10d4, 0x7000916c36, 0x7000916c31);
        }
        8 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000916c36); };
            c.w64(3, t18);
            c.w32(1, 0xfeu64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000916c3f); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000916c43); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000916c47); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000916c4b); };
            c.w64(8, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000916c4f); };
            c.w64(11, t23);
            blk = 3; continue;
        }
        9 => {
            let Some(t24) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000916c55); };
            c.w32(0, t24);
            let r25 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r25);
            c.w32(1, 0x1eu64);
            blk = 3; continue;
        }
        10 => {
            c.w32(11, c.r32(1));
            let r26 = c.r32(11) | 0x2u64;
            c.w32(11, r26);
            c.logic(r26, 32);
            let r27 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r27);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 13; continue;
        }
        11 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000916c70); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 32; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(0, c.r32(0));
            c.w32(11, c.r32(1));
            let r28 = c.r32(11) | 0xeu64;
            c.w32(11, r28);
            c.logic(r28, 32);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 13; continue;
        }
        13 => {
            let r29 = !c.r32(0);
            c.w32(0, r29);
            c.w64(2, c.r64(0));
            let r30 = c.add(c.r32(2), c.r32(0), 32);
            c.w32(2, r30);
            if c.cc_o() { blk = 27; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(0, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w32(2, 0xeu64);
            blk = 18; continue;
        }
        17 => {
            c.w32(2, 0x2u64);
            blk = 18; continue;
        }
        18 => {
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000916cb0); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000916cb6); };
            c.w32(1, t32);
            c.w32(11, c.r32(11));
            c.w32(2, c.r32(2));
            c.w32(6, c.r32(1));
            let r33 = c.r32(11) | c.r32(2);
            c.w32(11, r33);
            c.logic(r33, 32);
            let r34 = c.r32(11) | c.r32(6);
            c.w32(11, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000916cd0); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000916cd5); }
            blk = 21; continue;
        }
        21 => {
            let Some(t35) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000916cdd); };
            c.w64(7, t35);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t36) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000916ce8); };
            c.w32(8, t36);
            let Some(t37) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000916ced); };
            c.w64(1, t37);
            let Some(t38) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000916cf1); };
            c.w64(5, t38);
            c.w64(12, c.r64(7));
            let r39 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r39);
            return RegfileExit::jump(c.r64(1));
        }
        22 => {
            c.w64(2, c.r64(9));
            c.w64(1, c.r64(2).wrapping_sub(0x1));
            let r40 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r40);
            let Some(t41) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000916d07); };
            c.cmp(t41, 0x0u64, 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000916d0d); };
            c.w32(6, t42);
            c.w32(9, c.r32(6));
            let r43 = c.r32(9) | 0x40u64;
            c.w32(9, r43);
            c.logic(r43, 32);
            c.cmp(c.r32(6), c.r32(9), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(6, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f), c.r32(6)) { return RegfileExit::fault(0x7000916d21); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000916d26); }
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000916d2e); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000916d32); }
            c.w32(8, 0x18u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000916d39); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1d68)) else { return RegfileExit::fault(0x7000916d3b); };
            c.w64(3, t44);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x7000916d4f, 0x7000916d4a);
        }
        26 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000916d4f); };
            c.w64(7, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000916d53); };
            c.w64(15, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000916d57); };
            c.w64(9, t47);
            blk = 21; continue;
        }
        27 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000916d5d); };
            c.w64(2, t48);
            let Some(t49) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000916d64); };
            c.w64(1, t49);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000916d6b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x7000916d6f); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000916d73); };
            c.w64(12, t50);
            let Some(t51) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000916d7a); };
            c.cmp(t51, c.r64(6), 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000916d8a, 0x7000916d85);
        }
        29 => {
            c.w64(2, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000916d8d); };
            c.w64(15, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000916d91); };
            c.w64(9, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000916d95); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000916d99); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000916d9d); };
            c.w64(11, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000916da1); };
            c.w64(0, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000916da5); };
            c.w64(3, t58);
            blk = 31; continue;
        }
        30 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x7000916dab); }
            c.w64(2, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000916db2); };
            c.w64(1, t59);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000916db9); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000916dc4); }
            c.w64(0, c.r64(2));
            blk = 15; continue;
        }
        32 => {
            let r60 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r60);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000916dd5); }
            return c.call(0x70008708b4, 0x7000916ddf, 0x7000916dda);
        }
        33 => {
            let Some(t61) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000916ddf); };
            c.w32(0, t61);
            let r62 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r62);
            blk = 12; continue;
        }
        34 => {
            return RegfileExit::jump(0x7000916dec);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000916df4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000916df4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000916df8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000916dfe); };
            c.w64(2, (t1 as u16 as i16 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000916e04); };
            c.w64(7, t2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000916e08); };
            c.w64(2, t3);
            c.w64(8, c.r64(9).wrapping_sub(0x1));
            let Some(t4) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000916e10); };
            c.w64(11, t4);
            let r5 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r5);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000916e17); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000916e1b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000916e1f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000916e23); }
            c.w32(1, 0x2u64);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(11));
            return c.call(0x7000820e54, 0x7000916e3a, 0x7000916e35);
        }
        1 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000916e3a); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000916e3e); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000916e42); };
            c.w64(7, t8);
            c.w64(9, c.r64(7).wrapping_add(0x3));
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000916e4a); };
            c.w32(7, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000916e4f); };
            c.w64(15, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000916e53); };
            c.w64(1, t11);
            let Some(t12) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000916e57); };
            c.w64(5, t12);
            c.w64(12, c.r64(2));
            let r13 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r13);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000916e68);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000916e74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000916e74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000916e78); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000916e7e); };
            c.w64(2, (t1 as u16 as i16 as i64 as u64));
            let Some(t2) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000916e84); };
            c.w64(7, t2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000916e88); };
            c.w64(2, t3);
            c.w64(8, c.r64(9).wrapping_sub(0x1));
            let Some(t4) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000916e90); };
            c.w64(11, t4);
            let r5 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r5);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000916e97); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000916e9b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000916e9f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000916ea3); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(6, c.r64(11));
            return c.call(0x7000820e54, 0x7000916eb7, 0x7000916eb2);
        }
        1 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000916eb7); };
            c.w64(2, t6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000916ebb); };
            c.w64(2, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000916ebf); };
            c.w64(7, t8);
            c.w64(9, c.r64(7).wrapping_add(0x3));
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000916ec7); };
            c.w32(7, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000916ecc); };
            c.w64(15, t10);
            let Some(t11) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000916ed0); };
            c.w64(1, t11);
            let Some(t12) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000916ed4); };
            c.w64(5, t12);
            c.w64(12, c.r64(2));
            let r13 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r13);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000916ee4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000916ef4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000916ef4); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000916efa); };
            c.w64(2, (t2 as u16 as i16 as i64 as u64));
            c.w64(6, c.r64(5));
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000916f03); };
            c.w32(3, t3);
            let r4 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r4);
            if !c.st64(c.r64(6).wrapping_add(c.r64(2).wrapping_mul(8)), c.r64(3)) { return RegfileExit::fault(0x7000916f09); }
            let r5 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000916f11); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000916f16); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x7000916f20);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000916f34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000916f34); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000916f38); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000916f42); };
            c.w64(1, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000916f48); }
            let Some(t3) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000916f4c); };
            c.w64(2, t3);
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000916f50); };
            c.w64(8, (t4 as u16 as i16 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000916f56); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000916f5c); };
            c.w64(8, t7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000916f60); };
            c.w32(2, t8);
            let r9 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000916f66); };
            c.w32(11, (t10 as u16 as i16 as i64 as u64));
            let Some(t11) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000916f6c); };
            c.w32(15, t11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000916f72); };
            c.w32(0, t12);
            let Some(t13) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000916f78); };
            c.w32(3, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x7000916f7b); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000916f7f); }
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000916f83); };
            c.cmp(t14, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000916f8c); };
            c.w64(2, t15);
            blk = 2; continue;
        }
        2 => {
            let Some(t16) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000916f93); };
            c.w64(3, t16);
            let Some(t17) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000916f97); };
            c.w32(6, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000916f9a); };
            c.cmp(t18, c.r32(6), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(6, c.r64(0).wrapping_mul(4));
            c.w64(0, c.r64(0).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t19) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000916fb7); };
            c.w32(7, t19);
            let r20 = c.add(c.r32(7), 0x8u64, 32);
            c.w32(7, r20);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(7)) { return RegfileExit::fault(0x7000916fc1); }
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000916fc5); };
            c.w32(7, t21);
            let r22 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r22);
            c.w32(15, c.r32(7));
            let r23 = c.r32(15) & 0xfffffffffffffffdu64;
            c.w32(15, r23);
            c.logic(r23, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000916fd3); }
            c.cmp(c.r32(8), c.r32(15), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000916fe0); };
            c.cmp(t24, c.r32(7), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000916fed); };
            c.cmp(t25, c.r32(7), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000917003); };
            c.w32(15, t26);
            c.w64(11, c.r64(7));
            let r27 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r27);
            c.logic(r27, 64);
            c.w32(0, c.r32(15));
            let r28 = c.r32(0) & 0x3u64;
            c.w32(0, r28);
            c.logic(r28, 32);
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 27; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.logic(c.r8(15) & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t29) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917038); };
            c.w32(15, t29);
            let r30 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r30);
            let Some(t31) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091703f); };
            c.w32(15, t31);
            let r32 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r32);
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000917046); };
            c.w32(0, t33);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x7000917056);
        }
        12 => {
            let Some(t34) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000917057); };
            c.cmp(t34, c.r32(8), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t35) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000917061); };
            c.w64(9, (t35 as u16 as i16 as i64 as u64));
            c.w32(11, c.r32(7));
            let Some(t36) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x700091706a); };
            c.w64(9, t36);
            let r37 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t38) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700091707b); };
            c.w32(9, t38);
            c.w32(11, c.r32(8));
            c.w32(12, c.r32(9));
            let r39 = c.r32(11) & 0x1u64;
            c.w32(11, r39);
            c.logic(r39, 32);
            let r40 = c.r32(12) & 0x3u64;
            c.w32(12, r40);
            c.logic(r40, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r41 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r41);
            c.logic(r41, 64);
            let r42 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r42);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70009170a7); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009170ac); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(9, c.r64(8));
            blk = 20; continue;
        }
        19 => {
            let Some(t43) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009170c2); };
            c.w32(9, t43);
            let r44 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r44);
            blk = 20; continue;
        }
        20 => {
            let Some(t45) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009170c9); };
            c.w32(11, t45);
            let r46 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r46);
            let Some(t47) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009170d0); };
            c.w32(11, t47);
            c.cmp(c.r16(11), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(11, c.r16(11));
            let r48 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r48);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t49) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009170f2); };
            c.w32(9, t49);
            let r50 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r50);
            let Some(t51) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009170f9); };
            c.w32(11, t51);
            let r52 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r52);
            let Some(t53) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917100); };
            c.w32(9, t53);
            let r54 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r54);
            let Some(t55) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917107); };
            c.w32(11, t55);
            let Some(t56) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091710b); };
            c.cmp(t56, c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(8));
            let r57 = c.r64(9) | 0x3u64;
            c.w64(9, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091711c); };
            c.w64(15, t58);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(9)) { return RegfileExit::fault(0x7000917120); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r59 = c.r64(11) & c.r64(2);
            c.w64(11, r59);
            c.logic(r59, 64);
            let Some(t60) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000917134); };
            c.logic(t60 & 0x4u64, 8);
            if c.cc_ne() { blk = 63; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091713f); }
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000917146); };
            c.w64(11, t61);
            blk = 56; continue;
        }
        26 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091714f); };
            c.w64(11, t62);
            blk = 56; continue;
        }
        27 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t63) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917162); };
            c.w32(9, t63);
            let r64 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r64);
            let Some(t65) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000917169); };
            c.w32(9, t65);
            let r66 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r66);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091717f); };
            c.w32(9, t67);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917188); };
            c.w32(12, t68);
            let r69 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r69);
            let Some(t70) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091718f); };
            c.w32(12, t70);
            let r71 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r71);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t72) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009171a6); };
            c.w32(11, t72);
            let r73 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r73);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 32; continue;
        }
        32 => {
            let r74 = c.r64(11) | 0x3u64;
            c.w64(11, r74);
            c.logic(r74, 64);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009171ba); };
            c.w64(15, t75);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(11)) { return RegfileExit::fault(0x70009171be); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r76 = c.r64(9) & c.r64(2);
            c.w64(9, r76);
            c.logic(r76, 64);
            let Some(t77) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009171d2); };
            c.logic(t77 & 0x4u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009171dd); }
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009171e4); };
            c.w64(11, t78);
            blk = 56; continue;
        }
        35 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009171ed); };
            c.w64(11, t79);
            blk = 56; continue;
        }
        36 => {
            let Some(t80) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009171f6); };
            c.w32(15, t80);
            c.w32(7, c.r32(15));
            let r81 = c.r32(7) & 0x3u64;
            c.w32(7, r81);
            c.logic(r81, 32);
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r82 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r82);
            c.logic(r82, 64);
            let r83 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r83);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x7000917219); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091721e); }
            blk = 39; continue;
        }
        39 => {
            let Some(t84) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917225); };
            c.w32(15, t84);
            let r85 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r85);
            let Some(t86) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091722c); };
            c.w32(15, t86);
            let r87 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r87);
            let Some(t88) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000917233); };
            c.w32(7, t88);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xfu64, 64);
            if c.cc_a() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            return RegfileExit::fault(0x7000917243);
        }
        41 => {
            let Some(t89) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000917244); };
            c.cmp(t89, c.r32(8), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(9, c.r64(8));
            blk = 46; continue;
        }
        43 => {
            let Some(t90) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091724f); };
            c.w64(9, (t90 as u16 as i16 as i64 as u64));
            let Some(t91) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000917255); };
            c.w32(12, t91);
            let Some(t92) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x700091725a); };
            c.w64(9, t92);
            c.w32(15, c.r32(12));
            let r93 = c.r32(15) & 0x3u64;
            c.w32(15, r93);
            c.logic(r93, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r94 = c.r64(12) | 0x2u64;
            c.w64(12, r94);
            c.logic(r94, 64);
            let r95 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r95);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x700091727e); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000917283); }
            blk = 46; continue;
        }
        46 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(12, c.r64(9));
            blk = 49; continue;
        }
        48 => {
            let Some(t96) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000917299); };
            c.w32(12, t96);
            let r97 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r97);
            blk = 49; continue;
        }
        49 => {
            let Some(t98) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009172a1); };
            c.w32(15, t98);
            let r99 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r99);
            let Some(t100) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009172a9); };
            c.w32(15, t100);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(15, c.r16(15));
            let r101 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r101);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t102) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009172c7); };
            c.w32(12, t102);
            let r103 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r103);
            let Some(t104) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009172cf); };
            c.w32(15, t104);
            let r105 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r105);
            let Some(t106) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009172d6); };
            c.w32(12, t106);
            let r107 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r107);
            let Some(t108) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009172de); };
            c.w32(15, t108);
            let Some(t109) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009172e2); };
            c.cmp(t109, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            let r110 = c.r64(9) | 0x3u64;
            c.w64(9, r110);
            c.logic(r110, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(0)), c.r32(9)) { return RegfileExit::fault(0x70009172ed); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r111 = c.r64(12) & c.r64(2);
            c.w64(12, r111);
            c.logic(r111, 64);
            let Some(t112) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000917301); };
            c.logic(t112 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000917309); }
            blk = 56; continue;
        }
        55 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000917312); };
            c.w64(9, t113);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x7000917319); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091731e); }
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000917325); };
            c.w64(11, t114);
            blk = 56; continue;
        }
        56 => {
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8)));
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000917330); };
            c.w64(1, t115);
            c.w32(0, c.r64(1).wrapping_sub(0x1));
            let Some(t116) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000917337); };
            c.w64(5, t116);
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(2));
            c.w64(7, c.r64(8));
            let r117 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r117);
            return RegfileExit::jump(0x70007fcbb4);
        }
        57 => {
            c.w32(9, c.r32(9));
            let r118 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r118);
            c.w64(12, 0xfffffffffffc0000u64);
            let r119 = c.r64(12) & c.r64(9);
            c.w64(12, r119);
            c.logic(r119, 64);
            let Some(t120) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091735d); };
            c.logic(t120 & 0x2u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(15, c.r64(2).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000917369); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091736a); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(15));
            return c.call(0x70007f1594, 0x7000917377, 0x7000917372);
        }
        59 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000917377); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000917379); }
            blk = 54; continue;
        }
        60 => {
            c.w32(11, c.r32(11));
            let r121 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r121);
            c.w64(9, 0xfffffffffffc0000u64);
            let r122 = c.r64(9) & c.r64(11);
            c.w64(9, r122);
            c.logic(r122, 64);
            let Some(t123) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091738c); };
            c.logic(t123 & 0x2u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091739b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091739c); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70009173a9, 0x70009173a4);
        }
        62 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009173a9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009173ab); }
            blk = 34; continue;
        }
        63 => {
            c.w32(9, c.r32(9));
            let r124 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r124);
            c.w64(11, 0xfffffffffffc0000u64);
            let r125 = c.r64(11) & c.r64(9);
            c.w64(11, r125);
            c.logic(r125, 64);
            let Some(t126) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009173c1); };
            c.logic(t126 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009173d0); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009173d1); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70009173de, 0x70009173d9);
        }
        65 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009173de); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009173e0); }
            blk = 25; continue;
        }
        66 => {
            return RegfileExit::jump(0x70009173e8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009173f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009173f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009173f8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000917402); };
            c.w64(1, t1);
            let r2 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r2);
            if !c.st64(c.r64(1).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000917408); }
            let Some(t3) = c.ld64(c.r64(1).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091740c); };
            c.w64(2, t3);
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000917410); };
            c.w64(8, (t4 as u16 as i16 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917416); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091741c); };
            c.w64(8, t7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000917420); };
            c.w32(2, t8);
            let r9 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000917426); };
            c.w32(11, (t10 as u16 as i16 as i64 as u64));
            let Some(t11) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700091742c); };
            c.w32(15, t11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000917432); };
            c.w32(0, t12);
            let Some(t13) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917438); };
            c.w32(3, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700091743b); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700091743f); }
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000917443); };
            c.cmp(t14, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700091744c); };
            c.w64(2, t15);
            blk = 2; continue;
        }
        2 => {
            let Some(t16) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000917453); };
            c.w64(3, t16);
            let Some(t17) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917457); };
            c.w32(6, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700091745a); };
            c.cmp(t18, c.r32(6), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(6, c.r64(0).wrapping_mul(4));
            c.w64(0, c.r64(0).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t19) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000917477); };
            c.w32(7, t19);
            let r20 = c.add(c.r32(7), 0x8u64, 32);
            c.w32(7, r20);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(7)) { return RegfileExit::fault(0x7000917481); }
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000917485); };
            c.w32(7, t21);
            let r22 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r22);
            c.w32(15, c.r32(7));
            let r23 = c.r32(15) & 0xfffffffffffffffdu64;
            c.w32(15, r23);
            c.logic(r23, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000917493); }
            c.cmp(c.r32(8), c.r32(15), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70009174a0); };
            c.cmp(t24, c.r32(7), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70009174ad); };
            c.cmp(t25, c.r32(7), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009174c3); };
            c.w32(15, t26);
            c.w64(11, c.r64(7));
            let r27 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r27);
            c.logic(r27, 64);
            c.w32(0, c.r32(15));
            let r28 = c.r32(0) & 0x3u64;
            c.w32(0, r28);
            c.logic(r28, 32);
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 27; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.logic(c.r8(15) & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t29) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009174f8); };
            c.w32(15, t29);
            let r30 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r30);
            let Some(t31) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009174ff); };
            c.w32(15, t31);
            let r32 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r32);
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000917506); };
            c.w32(0, t33);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x7000917516);
        }
        12 => {
            let Some(t34) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000917517); };
            c.cmp(t34, c.r32(8), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t35) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000917521); };
            c.w64(9, (t35 as u16 as i16 as i64 as u64));
            c.w32(11, c.r32(7));
            let Some(t36) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x700091752a); };
            c.w64(9, t36);
            let r37 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t38) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700091753b); };
            c.w32(9, t38);
            c.w32(11, c.r32(8));
            c.w32(12, c.r32(9));
            let r39 = c.r32(11) & 0x1u64;
            c.w32(11, r39);
            c.logic(r39, 32);
            let r40 = c.r32(12) & 0x3u64;
            c.w32(12, r40);
            c.logic(r40, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r41 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r41);
            c.logic(r41, 64);
            let r42 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r42);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x7000917567); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091756c); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(9, c.r64(8));
            blk = 20; continue;
        }
        19 => {
            let Some(t43) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000917582); };
            c.w32(9, t43);
            let r44 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r44);
            blk = 20; continue;
        }
        20 => {
            let Some(t45) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917589); };
            c.w32(11, t45);
            let r46 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r46);
            let Some(t47) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000917590); };
            c.w32(11, t47);
            c.cmp(c.r16(11), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(11, c.r16(11));
            let r48 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r48);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t49) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009175b2); };
            c.w32(9, t49);
            let r50 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r50);
            let Some(t51) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009175b9); };
            c.w32(11, t51);
            let r52 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r52);
            let Some(t53) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009175c0); };
            c.w32(9, t53);
            let r54 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r54);
            let Some(t55) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009175c7); };
            c.w32(11, t55);
            let Some(t56) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009175cb); };
            c.cmp(t56, c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(8));
            let r57 = c.r64(9) | 0x3u64;
            c.w64(9, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009175dc); };
            c.w64(15, t58);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(9)) { return RegfileExit::fault(0x70009175e0); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r59 = c.r64(11) & c.r64(2);
            c.w64(11, r59);
            c.logic(r59, 64);
            let Some(t60) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009175f4); };
            c.logic(t60 & 0x4u64, 8);
            if c.cc_ne() { blk = 63; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009175ff); }
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000917606); };
            c.w64(11, t61);
            blk = 56; continue;
        }
        26 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700091760f); };
            c.w64(11, t62);
            blk = 56; continue;
        }
        27 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t63) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917622); };
            c.w32(9, t63);
            let r64 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r64);
            let Some(t65) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000917629); };
            c.w32(9, t65);
            let r66 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r66);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091763f); };
            c.w32(9, t67);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917648); };
            c.w32(12, t68);
            let r69 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r69);
            let Some(t70) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091764f); };
            c.w32(12, t70);
            let r71 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r71);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t72) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917666); };
            c.w32(11, t72);
            let r73 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r73);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 32; continue;
        }
        32 => {
            let r74 = c.r64(11) | 0x3u64;
            c.w64(11, r74);
            c.logic(r74, 64);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700091767a); };
            c.w64(15, t75);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(11)) { return RegfileExit::fault(0x700091767e); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r76 = c.r64(9) & c.r64(2);
            c.w64(9, r76);
            c.logic(r76, 64);
            let Some(t77) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000917692); };
            c.logic(t77 & 0x4u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091769d); }
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009176a4); };
            c.w64(11, t78);
            blk = 56; continue;
        }
        35 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009176ad); };
            c.w64(11, t79);
            blk = 56; continue;
        }
        36 => {
            let Some(t80) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009176b6); };
            c.w32(15, t80);
            c.w32(7, c.r32(15));
            let r81 = c.r32(7) & 0x3u64;
            c.w32(7, r81);
            c.logic(r81, 32);
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r82 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r82);
            c.logic(r82, 64);
            let r83 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r83);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x70009176d9); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009176de); }
            blk = 39; continue;
        }
        39 => {
            let Some(t84) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009176e5); };
            c.w32(15, t84);
            let r85 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r85);
            let Some(t86) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009176ec); };
            c.w32(15, t86);
            let r87 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r87);
            let Some(t88) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009176f3); };
            c.w32(7, t88);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xfu64, 64);
            if c.cc_a() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            return RegfileExit::fault(0x7000917703);
        }
        41 => {
            let Some(t89) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000917704); };
            c.cmp(t89, c.r32(8), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(9, c.r64(8));
            blk = 46; continue;
        }
        43 => {
            let Some(t90) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091770f); };
            c.w64(9, (t90 as u16 as i16 as i64 as u64));
            let Some(t91) = c.ld32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000917715); };
            c.w32(12, t91);
            let Some(t92) = c.ld64(c.r64(1).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x700091771a); };
            c.w64(9, t92);
            c.w32(15, c.r32(12));
            let r93 = c.r32(15) & 0x3u64;
            c.w32(15, r93);
            c.logic(r93, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r94 = c.r64(12) | 0x2u64;
            c.w64(12, r94);
            c.logic(r94, 64);
            let r95 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r95);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x700091773e); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000917743); }
            blk = 46; continue;
        }
        46 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(12, c.r64(9));
            blk = 49; continue;
        }
        48 => {
            let Some(t96) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000917759); };
            c.w32(12, t96);
            let r97 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r97);
            blk = 49; continue;
        }
        49 => {
            let Some(t98) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917761); };
            c.w32(15, t98);
            let r99 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r99);
            let Some(t100) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000917769); };
            c.w32(15, t100);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(15, c.r16(15));
            let r101 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r101);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t102) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000917787); };
            c.w32(12, t102);
            let r103 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r103);
            let Some(t104) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091778f); };
            c.w32(15, t104);
            let r105 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r105);
            let Some(t106) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917796); };
            c.w32(12, t106);
            let r107 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r107);
            let Some(t108) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091779e); };
            c.w32(15, t108);
            let Some(t109) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009177a2); };
            c.cmp(t109, c.r32(15), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            let r110 = c.r64(9) | 0x3u64;
            c.w64(9, r110);
            c.logic(r110, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(0)), c.r32(9)) { return RegfileExit::fault(0x70009177ad); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r111 = c.r64(12) & c.r64(2);
            c.w64(12, r111);
            c.logic(r111, 64);
            let Some(t112) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009177c1); };
            c.logic(t112 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009177c9); }
            blk = 56; continue;
        }
        55 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70009177d2); };
            c.w64(9, t113);
            if !c.st32(c.r64(2).wrapping_add(c.r64(6)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x70009177d9); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009177de); }
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009177e5); };
            c.w64(11, t114);
            blk = 56; continue;
        }
        56 => {
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(1).wrapping_add(c.r64(2).wrapping_mul(8)));
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009177f0); };
            c.w64(1, t115);
            c.w32(0, c.r64(1).wrapping_sub(0x1));
            let Some(t116) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009177f7); };
            c.w64(5, t116);
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(2));
            c.w64(7, c.r64(8));
            let r117 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r117);
            return RegfileExit::jump(0x70007fcbb4);
        }
        57 => {
            c.w32(9, c.r32(9));
            let r118 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r118);
            c.w64(12, 0xfffffffffffc0000u64);
            let r119 = c.r64(12) & c.r64(9);
            c.w64(12, r119);
            c.logic(r119, 64);
            let Some(t120) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091781d); };
            c.logic(t120 & 0x2u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(15, c.r64(2).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000917829); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091782a); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(15));
            return c.call(0x70007f1594, 0x7000917837, 0x7000917832);
        }
        59 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000917837); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000917839); }
            blk = 54; continue;
        }
        60 => {
            c.w32(11, c.r32(11));
            let r121 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r121);
            c.w64(9, 0xfffffffffffc0000u64);
            let r122 = c.r64(9) & c.r64(11);
            c.w64(9, r122);
            c.logic(r122, 64);
            let Some(t123) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091784c); };
            c.logic(t123 & 0x2u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091785b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091785c); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x7000917869, 0x7000917864);
        }
        62 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000917869); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700091786b); }
            blk = 34; continue;
        }
        63 => {
            c.w32(9, c.r32(9));
            let r124 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r124);
            c.w64(11, 0xfffffffffffc0000u64);
            let r125 = c.r64(11) & c.r64(9);
            c.w64(11, r125);
            c.logic(r125, 64);
            let Some(t126) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000917881); };
            c.logic(t126 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000917890); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000917891); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x700091789e, 0x7000917899);
        }
        65 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700091789e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009178a0); }
            blk = 25; continue;
        }
        66 => {
            return RegfileExit::jump(0x70009178a8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009178b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009178b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009178b8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009178c2); };
            c.w64(2, t1);
            let r2 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r2);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70009178c8); }
            let Some(t3) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009178cc); };
            c.w64(3, t3);
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009178d0); };
            c.w64(1, (t4 as u16 as i16 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009178d6); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x70009178dc); };
            c.w64(1, t7);
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009178e0); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009178e6); };
            c.w32(8, t10);
            let Some(t11) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009178ec); };
            c.w32(11, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009178f0); };
            c.cmp(t12, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009178f9); };
            c.w64(3, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000917900); };
            c.w64(11, t14);
            let Some(t15) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917904); };
            c.w32(15, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000917908); };
            c.cmp(t16, c.r32(15), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000917925); };
            c.w32(0, t17);
            let r18 = c.add(c.r32(0), 0x8u64, 32);
            c.w32(0, r18);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x7000917930); }
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000917935); };
            c.w32(0, t19);
            let r20 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r20);
            c.w32(6, c.r32(0));
            let r21 = c.r32(6) & 0xfffffffffffffffdu64;
            c.w32(6, r21);
            c.logic(r21, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000917942); }
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700091794e); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700091795b); };
            c.cmp(t23, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000917971); };
            c.w32(6, t24);
            c.w64(7, c.r64(0));
            let r25 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r25);
            c.logic(r25, 64);
            c.w32(8, c.r32(6));
            let r26 = c.r32(8) & 0x3u64;
            c.w32(8, r26);
            c.logic(r26, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t27) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009179a8); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009179af); };
            c.w32(8, t29);
            let r30 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009179b6); };
            c.w32(6, t31);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70009179c6);
        }
        12 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x43)) else { return RegfileExit::fault(0x70009179c7); };
            c.cmp(t32, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t33) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009179d1); };
            c.w64(8, (t33 as u16 as i16 as i64 as u64));
            c.w32(0, c.r32(0));
            let Some(t34) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009179d9); };
            c.w64(8, t34);
            let r35 = c.r32(0) & 0xfffffffffffffffdu64;
            c.w32(0, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t36) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009179e9); };
            c.w32(8, t36);
            c.w32(0, c.r32(1));
            c.w32(6, c.r32(8));
            let r37 = c.r32(0) & 0x1u64;
            c.w32(0, r37);
            c.logic(r37, 32);
            let r38 = c.r32(6) & 0x3u64;
            c.w32(6, r38);
            c.logic(r38, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r39 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r39);
            c.logic(r39, 64);
            let r40 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r40);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(8)) { return RegfileExit::fault(0x7000917a11); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000917a16); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(8, c.r64(1));
            blk = 20; continue;
        }
        19 => {
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000917a2b); };
            c.w32(8, t41);
            let r42 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r42);
            blk = 20; continue;
        }
        20 => {
            let Some(t43) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917a32); };
            c.w32(0, t43);
            let r44 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r44);
            let Some(t45) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000917a39); };
            c.w32(0, t45);
            c.cmp(c.r16(0), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, c.r16(0));
            let r46 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r46);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t47) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000917a54); };
            c.w32(8, t47);
            let r48 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r48);
            let Some(t49) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917a5b); };
            c.w32(0, t49);
            let r50 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r50);
            let Some(t51) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917a62); };
            c.w32(8, t51);
            let r52 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r52);
            let Some(t53) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917a69); };
            c.w32(0, t53);
            let Some(t54) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917a6c); };
            c.cmp(t54, c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, c.r64(1));
            let r55 = c.r64(8) | 0x3u64;
            c.w64(8, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000917a7d); };
            c.w64(0, t56);
            if !c.st32(c.r64(3).wrapping_add(c.r64(0)), c.r32(8)) { return RegfileExit::fault(0x7000917a81); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r57 = c.r64(15) & c.r64(3);
            c.w64(15, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000917a95); };
            c.logic(t58 & 0x4u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000917aa0); }
            blk = 54; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t59) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917ab5); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000917abc); };
            c.w32(8, t61);
            let r62 = c.sub(c.r32(8), 0x80cu64, 32);
            c.w32(8, r62);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t63) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917ad2); };
            c.w32(8, t63);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t64) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917adf); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r65);
            let Some(t66) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000917ae5); };
            c.w32(0, t66);
            let r67 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r67);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t68) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917af7); };
            c.w32(0, t68);
            let r69 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r69);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 31; continue;
        }
        31 => {
            let r70 = c.r64(0) | 0x3u64;
            c.w64(0, r70);
            c.logic(r70, 64);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000917b0a); };
            c.w64(8, t71);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000917b0e); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r72 = c.r64(15) & c.r64(3);
            c.w64(15, r72);
            c.logic(r72, 64);
            let Some(t73) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000917b20); };
            c.logic(t73 & 0x4u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000917b2b); }
            blk = 54; continue;
        }
        34 => {
            let Some(t74) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000917b37); };
            c.w32(0, t74);
            c.w32(6, c.r32(0));
            let r75 = c.r32(6) & 0x3u64;
            c.w32(6, r75);
            c.logic(r75, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r76 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r76);
            c.logic(r76, 64);
            let r77 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r77);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x7000917b56); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000917b5b); }
            blk = 37; continue;
        }
        37 => {
            let Some(t78) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917b62); };
            c.w32(0, t78);
            let r79 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r79);
            let Some(t80) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917b69); };
            c.w32(0, t80);
            let r81 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r81);
            let Some(t82) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000917b6f); };
            c.w32(6, t82);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x7000917b7e);
        }
        39 => {
            let Some(t83) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000917b7f); };
            c.cmp(t83, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            blk = 44; continue;
        }
        41 => {
            let Some(t84) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000917b89); };
            c.w64(0, (t84 as u16 as i16 as i64 as u64));
            let Some(t85) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000917b8f); };
            c.w32(6, t85);
            let Some(t86) = c.ld64(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x7000917b94); };
            c.w64(0, t86);
            c.w32(7, c.r32(6));
            let r87 = c.r32(7) & 0x3u64;
            c.w32(7, r87);
            c.logic(r87, 32);
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r88 = c.r64(6) | 0x2u64;
            c.w64(6, r88);
            c.logic(r88, 64);
            let r89 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r89);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(6)) { return RegfileExit::fault(0x7000917bb4); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000917bb9); }
            blk = 44; continue;
        }
        44 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(6, c.r64(0));
            blk = 47; continue;
        }
        46 => {
            let Some(t90) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000917bcd); };
            c.w32(6, t90);
            let r91 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r91);
            blk = 47; continue;
        }
        47 => {
            let Some(t92) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917bd3); };
            c.w32(7, t92);
            let r93 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r93);
            let Some(t94) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000917bd9); };
            c.w32(7, t94);
            c.cmp(c.r16(7), 0x80bu64, 16);
            if c.cc_e() { blk = 46; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(7, c.r16(7));
            let r95 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r95);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t96) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000917bf2); };
            c.w32(6, t96);
            let r97 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r97);
            let Some(t98) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917bf8); };
            c.w32(7, t98);
            let r99 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r99);
            let Some(t100) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917bff); };
            c.w32(6, t100);
            let r101 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r101);
            let Some(t102) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917c05); };
            c.w32(7, t102);
            let Some(t103) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917c08); };
            c.cmp(t103, c.r32(7), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            let r104 = c.r64(0) | 0x3u64;
            c.w64(0, r104);
            c.logic(r104, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000917c11); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r105 = c.r64(15) & c.r64(3);
            c.w64(15, r105);
            c.logic(r105, 64);
            let Some(t106) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000917c23); };
            c.logic(t106 & 0x4u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000917c2a); }
            blk = 54; continue;
        }
        53 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000917c33); };
            c.w64(8, t107);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x7000917c3a); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000917c3f); }
            blk = 54; continue;
        }
        54 => {
            let Some(t108) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000917c46); };
            c.w64(3, (t108 as u16 as i16 as i64 as u64));
            let Some(t109) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000917c4c); };
            c.w64(3, t109);
            let Some(t110) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000917c50); };
            c.w64(5, t110);
            let r111 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r111);
            let Some(t112) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x7000917c58); };
            if !c.push64(t112) { return RegfileExit::fault(0x7000917c58); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(3)) { return RegfileExit::fault(0x7000917c60); }
            c.w64(6, c.r64(11));
            return RegfileExit::jump(0x70007f2274);
        }
        55 => {
            c.w32(0, c.r32(0));
            let r113 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r113);
            c.w64(15, 0xfffffffffffc0000u64);
            let r114 = c.r64(15) & c.r64(0);
            c.w64(15, r114);
            c.logic(r114, 64);
            let Some(t115) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000917c7c); };
            c.logic(t115 & 0x2u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000917c87); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000917c88); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x7000917c95, 0x7000917c90);
        }
        57 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000917c95); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000917c97); }
            blk = 52; continue;
        }
        58 => {
            c.w32(0, c.r32(0));
            let r116 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r116);
            c.w64(15, 0xfffffffffffc0000u64);
            let r117 = c.r64(15) & c.r64(0);
            c.w64(15, r117);
            c.logic(r117, 64);
            let Some(t118) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000917ca9); };
            c.logic(t118 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000917cb8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000917cb9); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x7000917cc6, 0x7000917cc1);
        }
        60 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000917cc6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000917cc8); }
            blk = 33; continue;
        }
        61 => {
            c.w32(8, c.r32(8));
            let r119 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r119);
            c.w64(15, 0xfffffffffffc0000u64);
            let r120 = c.r64(15) & c.r64(8);
            c.w64(15, r120);
            c.logic(r120, 64);
            let Some(t121) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000917cde); };
            c.logic(t121 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000917ced); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000917cee); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x7000917cfb, 0x7000917cf6);
        }
        63 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000917cfb); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000917cfd); }
            blk = 25; continue;
        }
        64 => {
            return RegfileExit::jump(0x7000917d04);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000917d14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000917d14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000917d18); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000917d22); };
            c.w64(2, t1);
            let r2 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r2);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000917d28); }
            let Some(t3) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000917d2c); };
            c.w64(3, t3);
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000917d30); };
            c.w64(1, (t4 as u16 as i16 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917d36); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x7000917d3c); };
            c.w64(1, t7);
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000917d40); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000917d46); };
            c.w32(8, t10);
            let Some(t11) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917d4c); };
            c.w32(11, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000917d50); };
            c.cmp(t12, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000917d59); };
            c.w64(3, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000917d60); };
            c.w64(11, t14);
            let Some(t15) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917d64); };
            c.w32(15, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000917d68); };
            c.cmp(t16, c.r32(15), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000917d85); };
            c.w32(0, t17);
            let r18 = c.add(c.r32(0), 0x8u64, 32);
            c.w32(0, r18);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x7000917d90); }
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000917d95); };
            c.w32(0, t19);
            let r20 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r20);
            c.w32(6, c.r32(0));
            let r21 = c.r32(6) & 0xfffffffffffffffdu64;
            c.w32(6, r21);
            c.logic(r21, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000917da2); }
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000917dae); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000917dbb); };
            c.cmp(t23, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000917dd1); };
            c.w32(6, t24);
            c.w64(7, c.r64(0));
            let r25 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r25);
            c.logic(r25, 64);
            c.w32(8, c.r32(6));
            let r26 = c.r32(8) & 0x3u64;
            c.w32(8, r26);
            c.logic(r26, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t27) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917e08); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917e0f); };
            c.w32(8, t29);
            let r30 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000917e16); };
            c.w32(6, t31);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x7000917e26);
        }
        12 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000917e27); };
            c.cmp(t32, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t33) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000917e31); };
            c.w64(8, (t33 as u16 as i16 as i64 as u64));
            c.w32(0, c.r32(0));
            let Some(t34) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000917e39); };
            c.w64(8, t34);
            let r35 = c.r32(0) & 0xfffffffffffffffdu64;
            c.w32(0, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t36) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000917e49); };
            c.w32(8, t36);
            c.w32(0, c.r32(1));
            c.w32(6, c.r32(8));
            let r37 = c.r32(0) & 0x1u64;
            c.w32(0, r37);
            c.logic(r37, 32);
            let r38 = c.r32(6) & 0x3u64;
            c.w32(6, r38);
            c.logic(r38, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r39 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r39);
            c.logic(r39, 64);
            let r40 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r40);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(8)) { return RegfileExit::fault(0x7000917e71); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000917e76); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(8, c.r64(1));
            blk = 20; continue;
        }
        19 => {
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000917e8b); };
            c.w32(8, t41);
            let r42 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r42);
            blk = 20; continue;
        }
        20 => {
            let Some(t43) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917e92); };
            c.w32(0, t43);
            let r44 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r44);
            let Some(t45) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000917e99); };
            c.w32(0, t45);
            c.cmp(c.r16(0), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, c.r16(0));
            let r46 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r46);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t47) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000917eb4); };
            c.w32(8, t47);
            let r48 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r48);
            let Some(t49) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917ebb); };
            c.w32(0, t49);
            let r50 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r50);
            let Some(t51) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917ec2); };
            c.w32(8, t51);
            let r52 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r52);
            let Some(t53) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917ec9); };
            c.w32(0, t53);
            let Some(t54) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917ecc); };
            c.cmp(t54, c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, c.r64(1));
            let r55 = c.r64(8) | 0x3u64;
            c.w64(8, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000917edd); };
            c.w64(0, t56);
            if !c.st32(c.r64(3).wrapping_add(c.r64(0)), c.r32(8)) { return RegfileExit::fault(0x7000917ee1); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r57 = c.r64(15) & c.r64(3);
            c.w64(15, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000917ef5); };
            c.logic(t58 & 0x4u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000917f00); }
            blk = 54; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t59) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917f15); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000917f1c); };
            c.w32(8, t61);
            let r62 = c.sub(c.r32(8), 0x80cu64, 32);
            c.w32(8, r62);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t63) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917f32); };
            c.w32(8, t63);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t64) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917f3f); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r65);
            let Some(t66) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000917f45); };
            c.w32(0, t66);
            let r67 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r67);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t68) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917f57); };
            c.w32(0, t68);
            let r69 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r69);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 31; continue;
        }
        31 => {
            let r70 = c.r64(0) | 0x3u64;
            c.w64(0, r70);
            c.logic(r70, 64);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000917f6a); };
            c.w64(8, t71);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000917f6e); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r72 = c.r64(15) & c.r64(3);
            c.w64(15, r72);
            c.logic(r72, 64);
            let Some(t73) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000917f80); };
            c.logic(t73 & 0x4u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000917f8b); }
            blk = 54; continue;
        }
        34 => {
            let Some(t74) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000917f97); };
            c.w32(0, t74);
            c.w32(6, c.r32(0));
            let r75 = c.r32(6) & 0x3u64;
            c.w32(6, r75);
            c.logic(r75, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r76 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r76);
            c.logic(r76, 64);
            let r77 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r77);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x7000917fb6); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000917fbb); }
            blk = 37; continue;
        }
        37 => {
            let Some(t78) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000917fc2); };
            c.w32(0, t78);
            let r79 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r79);
            let Some(t80) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000917fc9); };
            c.w32(0, t80);
            let r81 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r81);
            let Some(t82) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000917fcf); };
            c.w32(6, t82);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x7000917fde);
        }
        39 => {
            let Some(t83) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x7000917fdf); };
            c.cmp(t83, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            blk = 44; continue;
        }
        41 => {
            let Some(t84) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000917fe9); };
            c.w64(0, (t84 as u16 as i16 as i64 as u64));
            let Some(t85) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000917fef); };
            c.w32(6, t85);
            let Some(t86) = c.ld64(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x7000917ff4); };
            c.w64(0, t86);
            c.w32(7, c.r32(6));
            let r87 = c.r32(7) & 0x3u64;
            c.w32(7, r87);
            c.logic(r87, 32);
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r88 = c.r64(6) | 0x2u64;
            c.w64(6, r88);
            c.logic(r88, 64);
            let r89 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r89);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(6)) { return RegfileExit::fault(0x7000918014); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000918019); }
            blk = 44; continue;
        }
        44 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(6, c.r64(0));
            blk = 47; continue;
        }
        46 => {
            let Some(t90) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091802d); };
            c.w32(6, t90);
            let r91 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r91);
            blk = 47; continue;
        }
        47 => {
            let Some(t92) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918033); };
            c.w32(7, t92);
            let r93 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r93);
            let Some(t94) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000918039); };
            c.w32(7, t94);
            c.cmp(c.r16(7), 0x80bu64, 16);
            if c.cc_e() { blk = 46; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(7, c.r16(7));
            let r95 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r95);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t96) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000918052); };
            c.w32(6, t96);
            let r97 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r97);
            let Some(t98) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918058); };
            c.w32(7, t98);
            let r99 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r99);
            let Some(t100) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091805f); };
            c.w32(6, t100);
            let r101 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r101);
            let Some(t102) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918065); };
            c.w32(7, t102);
            let Some(t103) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918068); };
            c.cmp(t103, c.r32(7), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            let r104 = c.r64(0) | 0x3u64;
            c.w64(0, r104);
            c.logic(r104, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000918071); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r105 = c.r64(15) & c.r64(3);
            c.w64(15, r105);
            c.logic(r105, 64);
            let Some(t106) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000918083); };
            c.logic(t106 & 0x4u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091808a); }
            blk = 54; continue;
        }
        53 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000918093); };
            c.w64(8, t107);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700091809a); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091809f); }
            blk = 54; continue;
        }
        54 => {
            let Some(t108) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009180a6); };
            c.w64(3, (t108 as u16 as i16 as i64 as u64));
            let Some(t109) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009180ac); };
            c.w64(8, (t109 as u16 as i16 as i64 as u64));
            let Some(t110) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009180b2); };
            c.w64(3, t110);
            let Some(t111) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009180b6); };
            c.w64(2, t111);
            let Some(t112) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009180ba); };
            c.w64(5, t112);
            let r113 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r113);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70009180c2); }
            let Some(t114) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009180c3); };
            if !c.push64(t114) { return RegfileExit::fault(0x70009180c3); }
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x70009180cf); }
            c.w64(6, c.r64(11));
            return RegfileExit::jump(0x70007f2274);
        }
        55 => {
            c.w32(0, c.r32(0));
            let r115 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r115);
            c.w64(15, 0xfffffffffffc0000u64);
            let r116 = c.r64(15) & c.r64(0);
            c.w64(15, r116);
            c.logic(r116, 64);
            let Some(t117) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009180eb); };
            c.logic(t117 & 0x2u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009180f6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009180f7); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x7000918104, 0x70009180ff);
        }
        57 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000918104); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000918106); }
            blk = 52; continue;
        }
        58 => {
            c.w32(0, c.r32(0));
            let r118 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r118);
            c.w64(15, 0xfffffffffffc0000u64);
            let r119 = c.r64(15) & c.r64(0);
            c.w64(15, r119);
            c.logic(r119, 64);
            let Some(t120) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000918118); };
            c.logic(t120 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000918127); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000918128); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x7000918135, 0x7000918130);
        }
        60 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000918135); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000918137); }
            blk = 33; continue;
        }
        61 => {
            c.w32(8, c.r32(8));
            let r121 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r121);
            c.w64(15, 0xfffffffffffc0000u64);
            let r122 = c.r64(15) & c.r64(8);
            c.w64(15, r122);
            c.logic(r122, 64);
            let Some(t123) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700091814d); };
            c.logic(t123 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700091815c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091815d); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x700091816a, 0x7000918165);
        }
        63 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700091816a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700091816c); }
            blk = 25; continue;
        }
        64 => {
            return RegfileExit::jump(0x7000918174);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000918194(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000918194); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000918198); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009181a2); };
            c.w64(2, t1);
            let r2 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r2);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70009181a8); }
            let Some(t3) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009181ac); };
            c.w64(3, t3);
            let Some(t4) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009181b0); };
            c.w64(1, (t4 as u16 as i16 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009181b6); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x70009181bc); };
            c.w64(1, t7);
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009181c0); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009181c6); };
            c.w32(8, t10);
            let Some(t11) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009181cc); };
            c.w32(11, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009181d0); };
            c.cmp(t12, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009181d9); };
            c.w64(3, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009181e0); };
            c.w64(11, t14);
            let Some(t15) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009181e4); };
            c.w32(15, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009181e8); };
            c.cmp(t16, c.r32(15), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(15, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000918205); };
            c.w32(0, t17);
            let r18 = c.add(c.r32(0), 0x8u64, 32);
            c.w32(0, r18);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x7000918210); }
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000918215); };
            c.w32(0, t19);
            let r20 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r20);
            c.w32(6, c.r32(0));
            let r21 = c.r32(6) & 0xfffffffffffffffdu64;
            c.w32(6, r21);
            c.logic(r21, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000918222); }
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700091822e); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700091823b); };
            c.cmp(t23, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000918251); };
            c.w32(6, t24);
            c.w64(7, c.r64(0));
            let r25 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r25);
            c.logic(r25, 64);
            c.w32(8, c.r32(6));
            let r26 = c.r32(8) & 0x3u64;
            c.w32(8, r26);
            c.logic(r26, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t27) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918288); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091828f); };
            c.w32(8, t29);
            let r30 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000918296); };
            c.w32(6, t31);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70009182a6);
        }
        12 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x43)) else { return RegfileExit::fault(0x70009182a7); };
            c.cmp(t32, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t33) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009182b1); };
            c.w64(8, (t33 as u16 as i16 as i64 as u64));
            c.w32(0, c.r32(0));
            let Some(t34) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009182b9); };
            c.w64(8, t34);
            let r35 = c.r32(0) & 0xfffffffffffffffdu64;
            c.w32(0, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t36) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70009182c9); };
            c.w32(8, t36);
            c.w32(0, c.r32(1));
            c.w32(6, c.r32(8));
            let r37 = c.r32(0) & 0x1u64;
            c.w32(0, r37);
            c.logic(r37, 32);
            let r38 = c.r32(6) & 0x3u64;
            c.w32(6, r38);
            c.logic(r38, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r39 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r39);
            c.logic(r39, 64);
            let r40 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r40);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(8)) { return RegfileExit::fault(0x70009182f1); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009182f6); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(8, c.r64(1));
            blk = 20; continue;
        }
        19 => {
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700091830b); };
            c.w32(8, t41);
            let r42 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r42);
            blk = 20; continue;
        }
        20 => {
            let Some(t43) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918312); };
            c.w32(0, t43);
            let r44 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r44);
            let Some(t45) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000918319); };
            c.w32(0, t45);
            c.cmp(c.r16(0), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(0, c.r16(0));
            let r46 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r46);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t47) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000918334); };
            c.w32(8, t47);
            let r48 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r48);
            let Some(t49) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700091833b); };
            c.w32(0, t49);
            let r50 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r50);
            let Some(t51) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918342); };
            c.w32(8, t51);
            let r52 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r52);
            let Some(t53) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918349); };
            c.w32(0, t53);
            let Some(t54) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700091834c); };
            c.cmp(t54, c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, c.r64(1));
            let r55 = c.r64(8) | 0x3u64;
            c.w64(8, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700091835d); };
            c.w64(0, t56);
            if !c.st32(c.r64(3).wrapping_add(c.r64(0)), c.r32(8)) { return RegfileExit::fault(0x7000918361); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r57 = c.r64(15) & c.r64(3);
            c.w64(15, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000918375); };
            c.logic(t58 & 0x4u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000918380); }
            blk = 54; continue;
        }
        26 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t59) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918395); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700091839c); };
            c.w32(8, t61);
            let r62 = c.sub(c.r32(8), 0x80cu64, 32);
            c.w32(8, r62);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t63) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009183b2); };
            c.w32(8, t63);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t64) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009183bf); };
            c.w32(0, t64);
            let r65 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r65);
            let Some(t66) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009183c5); };
            c.w32(0, t66);
            let r67 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r67);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t68) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009183d7); };
            c.w32(0, t68);
            let r69 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r69);
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 31; continue;
        }
        31 => {
            let r70 = c.r64(0) | 0x3u64;
            c.w64(0, r70);
            c.logic(r70, 64);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009183ea); };
            c.w64(8, t71);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x70009183ee); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r72 = c.r64(15) & c.r64(3);
            c.w64(15, r72);
            c.logic(r72, 64);
            let Some(t73) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000918400); };
            c.logic(t73 & 0x4u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091840b); }
            blk = 54; continue;
        }
        34 => {
            let Some(t74) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000918417); };
            c.w32(0, t74);
            c.w32(6, c.r32(0));
            let r75 = c.r32(6) & 0x3u64;
            c.w32(6, r75);
            c.logic(r75, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r76 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r76);
            c.logic(r76, 64);
            let r77 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r77);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(0)) { return RegfileExit::fault(0x7000918436); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091843b); }
            blk = 37; continue;
        }
        37 => {
            let Some(t78) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000918442); };
            c.w32(0, t78);
            let r79 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r79);
            let Some(t80) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000918449); };
            c.w32(0, t80);
            let r81 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r81);
            let Some(t82) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700091844f); };
            c.w32(6, t82);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x700091845e);
        }
        39 => {
            let Some(t83) = c.ld32(c.r64(0).wrapping_add(0x43)) else { return RegfileExit::fault(0x700091845f); };
            c.cmp(t83, c.r32(1), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            blk = 44; continue;
        }
        41 => {
            let Some(t84) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000918469); };
            c.w64(0, (t84 as u16 as i16 as i64 as u64));
            let Some(t85) = c.ld32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700091846f); };
            c.w32(6, t85);
            let Some(t86) = c.ld64(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x7000918474); };
            c.w64(0, t86);
            c.w32(7, c.r32(6));
            let r87 = c.r32(7) & 0x3u64;
            c.w32(7, r87);
            c.logic(r87, 32);
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r88 = c.r64(6) | 0x2u64;
            c.w64(6, r88);
            c.logic(r88, 64);
            let r89 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r89);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x23), c.r32(6)) { return RegfileExit::fault(0x7000918494); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000918499); }
            blk = 44; continue;
        }
        44 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(6, c.r64(0));
            blk = 47; continue;
        }
        46 => {
            let Some(t90) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009184ad); };
            c.w32(6, t90);
            let r91 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r91);
            blk = 47; continue;
        }
        47 => {
            let Some(t92) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009184b3); };
            c.w32(7, t92);
            let r93 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r93);
            let Some(t94) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009184b9); };
            c.w32(7, t94);
            c.cmp(c.r16(7), 0x80bu64, 16);
            if c.cc_e() { blk = 46; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(7, c.r16(7));
            let r95 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r95);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t96) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009184d2); };
            c.w32(6, t96);
            let r97 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r97);
            let Some(t98) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009184d8); };
            c.w32(7, t98);
            let r99 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r99);
            let Some(t100) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009184df); };
            c.w32(6, t100);
            let r101 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r101);
            let Some(t102) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009184e5); };
            c.w32(7, t102);
            let Some(t103) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009184e8); };
            c.cmp(t103, c.r32(7), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            let r104 = c.r64(0) | 0x3u64;
            c.w64(0, r104);
            c.logic(r104, 64);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x70009184f1); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r105 = c.r64(15) & c.r64(3);
            c.w64(15, r105);
            c.logic(r105, 64);
            let Some(t106) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000918503); };
            c.logic(t106 & 0x4u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 52; continue;
        }
        52 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091850a); }
            blk = 54; continue;
        }
        53 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x7000918513); };
            c.w64(8, t107);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700091851a); }
            if !c.st32(c.r64(3).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700091851f); }
            blk = 54; continue;
        }
        54 => {
            let Some(t108) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000918526); };
            c.w64(3, (t108 as u16 as i16 as i64 as u64));
            let Some(t109) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700091852c); };
            c.w64(8, (t109 as u16 as i16 as i64 as u64));
            let Some(t110) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000918532); };
            c.w64(9, (t110 as u16 as i16 as i64 as u64));
            let Some(t111) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000918538); };
            c.w64(3, t111);
            let Some(t112) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700091853c); };
            c.w64(8, t112);
            let Some(t113) = c.ld64(c.r64(2).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x7000918540); };
            c.w64(2, t113);
            let Some(t114) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000918544); };
            c.w64(5, t114);
            let r115 = c.add(c.r64(4), 0x18u64, 64);
            c.w64(4, r115);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700091854c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700091854e); }
            let Some(t116) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700091854f); };
            if !c.push64(t116) { return RegfileExit::fault(0x700091854f); }
            c.w32(0, 0x2u64);
            c.w64(7, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x700091855b); }
            c.w64(6, c.r64(11));
            return RegfileExit::jump(0x70007f2274);
        }
        55 => {
            c.w32(0, c.r32(0));
            let r117 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r117);
            c.w64(15, 0xfffffffffffc0000u64);
            let r118 = c.r64(15) & c.r64(0);
            c.w64(15, r118);
            c.logic(r118, 64);
            let Some(t119) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000918577); };
            c.logic(t119 & 0x2u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000918582); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000918583); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x7000918590, 0x700091858b);
        }
        57 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000918590); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000918592); }
            blk = 52; continue;
        }
        58 => {
            c.w32(0, c.r32(0));
            let r120 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r120);
            c.w64(15, 0xfffffffffffc0000u64);
            let r121 = c.r64(15) & c.r64(0);
            c.w64(15, r121);
            c.logic(r121, 64);
            let Some(t122) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009185a7); };
            c.logic(t122 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009185b6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009185b7); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x70009185c4, 0x70009185bf);
        }
        60 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009185c4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009185c6); }
            blk = 33; continue;
        }
        61 => {
            c.w32(8, c.r32(8));
            let r123 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r123);
            c.w64(15, 0xfffffffffffc0000u64);
            let r124 = c.r64(15) & c.r64(8);
            c.w64(15, r124);
            c.logic(r124, 64);
            let Some(t125) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009185dc); };
            c.logic(t125 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(6, c.r64(3).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009185eb); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009185ec); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f1594, 0x70009185f9, 0x70009185f4);
        }
        63 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70009185f9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70009185fb); }
            blk = 25; continue;
        }
        64 => {
            return RegfileExit::jump(0x7000918604);
        }
            _ => unreachable!(),
        }
    }
}
