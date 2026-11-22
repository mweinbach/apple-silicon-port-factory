#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_700087df94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087df94); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700087df98); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087df99); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087df9a); }
            let r0 = c.sub(c.r64(4), 0xa8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700087dfa2); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700087dfa6); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700087dfaa); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 217; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087dfc2); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700087dfc7); };
            c.cmp(t3, c.r32(11), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087dfd6); };
            c.cmp(t4, c.r32(11), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700087dfe5); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x700087dfe9); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700087dfed); }
            let r5 = c.r32(12) | c.r32(15);
            c.w32(12, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 215; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 219; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e004); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087e00b); };
            c.cmp(t8, 0xbau64, 16);
            if c.cc_b() { blk = 219; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            blk = 5; continue;
        }
        5 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x700087e028); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e032); };
            c.w32(3, t9);
            let r10 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r10);
            let Some(t11) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087e03a); };
            c.cmp(t11, 0x834u64, 16);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t12) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087e042); };
            c.w32(15, t12);
            let r13 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r13);
            c.w64(3, c.r64(15));
            blk = 30; continue;
        }
        8 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 10; continue;
        }
        9 => {
            c.w32(3, 0x1u64);
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e060); };
            c.w32(3, t14);
            let r15 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r15);
            let Some(t16) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087e066); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            let Some(t18) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087e06c); };
            c.w32(2, t18);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e072); };
            c.w32(1, t19);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0xb0u64, 64);
            if c.cc_a() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x700087e083);
        }
        13 => {
            let Some(t20) = c.ld32(c.r64(3).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x700087e084); };
            c.cmp(t20, c.r32(1), 32);
            if c.cc_e() { blk = 221; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 223; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e09a); };
            c.w32(15, t21);
            let r22 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r22);
            let Some(t23) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087e0a1); };
            c.w32(15, t23);
            let r24 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r24);
            let Some(t25) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087e0a8); };
            c.w32(3, t25);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t26) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e0af); };
            c.w32(2, t26);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x44u64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x700087e0bd);
        }
        17 => {
            let Some(t27) = c.ld32(c.r64(15).wrapping_add(0x117)) else { return RegfileExit::fault(0x700087e0be); };
            c.cmp(t27, c.r32(2), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r64(3), 0xaeu64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x700087e0d0);
        }
        20 => {
            let Some(t28) = c.ld32(c.r64(15).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x700087e0d1); };
            c.cmp(t28, c.r32(2), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r64(3), 0xafu64, 64);
            if c.cc_a() { blk = 222; continue; }
            blk = 22; continue;
        }
        22 => {
            return RegfileExit::fault(0x700087e0e7);
        }
        23 => {
            let Some(t29) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087e0e8); };
            c.w32(15, t29);
            let r30 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r30);
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, c.r64(15));
            return c.call(0x70008a17f4, 0x700087e0fe, 0x700087e0f9);
        }
        26 => {
            c.w64(3, c.r64(0));
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087e101); };
            c.w64(6, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087e105); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087e109); };
            c.w64(12, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087e10d); };
            c.w64(11, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087e111); };
            c.w64(0, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087e115); };
            c.w64(9, t36);
            blk = 30; continue;
        }
        27 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_l() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(3, c.r64(15));
            blk = 30; continue;
        }
        29 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 30; continue;
        }
        30 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(0), 0x0u64, 64);
            c.w8(2, (c.cc_be() as u64));
            c.w32(2, c.r8(2));
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700087e138); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700087e13c); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 262; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t37) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087e14f); };
            c.w64(1, t37);
            c.w64(7, c.r64(1));
            blk = 34; continue;
        }
        33 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087e158); };
            c.w64(7, t38);
            blk = 34; continue;
        }
        34 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087e15f); };
            c.w64(1, t39);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700087e166); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t40) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e174); };
            c.w32(8, t40);
            let r41 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r41);
            let Some(t42) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087e17b); };
            c.logic(t42 & 0x2u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_be() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(8, c.r64(9));
            let Some(t43) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087e18f); };
            c.w64(8, t43);
            blk = 39; continue;
        }
        38 => {
            c.w64(8, c.r64(1));
            blk = 39; continue;
        }
        39 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700087e198); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0xfc0)) else { return RegfileExit::fault(0x700087e19c); };
            c.w64(9, t44);
            let Some(t45) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087e1a3); };
            c.cmp(t45, c.r32(15), 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(9, c.r64(12));
            let Some(t46) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e1b0); };
            c.w32(12, t46);
            let r47 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r47);
            let Some(t48) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087e1b7); };
            c.cmp(t48, 0x834u64, 16);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w32(2, 0x1u64);
            blk = 43; continue;
        }
        42 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t49) = c.ld8(c.r64(12).wrapping_add(0xa)) else { return RegfileExit::fault(0x700087e1d3); };
            c.w32(2, t49);
            c.w64(1, c.r64(2));
            let r50 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r50);
            let r51 = c.r32(1) & 0x3fu64;
            c.w32(1, r51);
            c.logic(r51, 32);
            c.cmp(c.r32(1), 0x5u64, 32);
            if c.cc_a() { blk = 209; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t52) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e1eb); };
            c.w32(1, t52);
            let r53 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r53);
            let Some(t54) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087e1f1); };
            c.w32(1, t54);
            let r55 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r55);
            let Some(t56) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087e1f7); };
            c.w32(11, t56);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4cu64, 64);
            if c.cc_a() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            return RegfileExit::fault(0x700087e207);
        }
        47 => {
            let Some(t57) = c.ld32(c.r64(1).wrapping_add(0x137)) else { return RegfileExit::fault(0x700087e208); };
            c.w32(8, t57);
            let Some(t58) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087e20f); };
            c.cmp(t58, c.r32(8), 32);
            if c.cc_ne() { blk = 209; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700087e21a); };
            c.w64(8, t59);
            let Some(t60) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087e221); };
            c.cmp(t60, c.r32(15), 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(12, c.r8(2));
            let r61 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r61);
            let r62 = c.r32(12) & 0x3fu64;
            c.w32(12, r62);
            c.logic(r62, 32);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let r63 = c.add(c.r64(12), 0x69u64, 64);
            c.w64(12, r63);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_b() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            return RegfileExit::fault(0x700087e243);
        }
        51 => {
            let Some(t64) = c.ld32(c.r64(1).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087e244); };
            c.w32(11, t64);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087e249); };
            c.w64(12, t65);
            let Some(t66) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700087e250); };
            c.w64(2, t66);
            c.w64(1, c.r64(2).wrapping_add(0x10));
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087e258); };
            c.w64(0, t67);
            let Some(t68) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700087e25f); };
            c.cmp(t68, c.r64(1), 64);
            if c.cc_be() { blk = 225; continue; }
            blk = 52; continue;
        }
        52 => {
            if !c.st64(c.r64(12), c.r64(1)) { return RegfileExit::fault(0x700087e268); }
            let r69 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r69);
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700087e270); }
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x700087e274); }
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700087e278); };
            c.w64(11, t70);
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700087e27f); }
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700087e283); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x700087e287); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 260; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t71) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e294); };
            c.w32(1, t71);
            let r72 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r72);
            if !c.st64(c.r64(5).wrapping_sub(0xb0), c.r64(1)) { return RegfileExit::fault(0x700087e29a); }
            let Some(t73) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087e2a1); };
            c.cmp(t73, 0x834u64, 16);
            if c.cc_ne() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), 0x1u64) { return RegfileExit::fault(0x700087e2a9); }
            blk = 57; continue;
        }
        56 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), 0x0u64) { return RegfileExit::fault(0x700087e2b3); }
            blk = 57; continue;
        }
        57 => {
            let Some(t74) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087e2bb); };
            c.cmp(t74, 0x0u64, 32);
            if c.cc_e() { blk = 259; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t75) = c.ld8(c.r64(1).wrapping_add(0xa)) else { return RegfileExit::fault(0x700087e2c5); };
            c.w32(7, t75);
            if !c.st64(c.r64(5).wrapping_sub(0xa0), c.r64(7)) { return RegfileExit::fault(0x700087e2c9); }
            let r76 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r76);
            let r77 = c.r32(7) & 0x3fu64;
            c.w32(7, r77);
            c.logic(r77, 32);
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_a() { blk = 259; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t78) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e2df); };
            c.w32(7, t78);
            let r79 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r79);
            let Some(t80) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087e2e5); };
            c.w32(7, t80);
            let r81 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r81);
            let Some(t82) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087e2eb); };
            c.w32(6, t82);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x4cu64, 64);
            if c.cc_a() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            return RegfileExit::fault(0x700087e2fa);
        }
        61 => {
            let Some(t83) = c.ld32(c.r64(7).wrapping_add(0x137)) else { return RegfileExit::fault(0x700087e2fb); };
            c.w32(6, t83);
            let Some(t84) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087e301); };
            c.cmp(t84, c.r32(6), 32);
            if c.cc_ne() { blk = 259; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t85) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087e30a); };
            c.cmp(t85, c.r32(15), 32);
            if c.cc_e() { blk = 259; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t86) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e314); };
            c.w32(7, t86);
            let r87 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r87);
            if !c.st64(c.r64(5).wrapping_sub(0xc0), c.r64(7)) { return RegfileExit::fault(0x700087e31b); }
            let Some(t88) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087e322); };
            c.cmp(t88, 0x834u64, 16);
            if c.cc_ne() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), 0x1u64) { return RegfileExit::fault(0x700087e32a); }
            blk = 66; continue;
        }
        65 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), 0x0u64) { return RegfileExit::fault(0x700087e334); }
            blk = 66; continue;
        }
        66 => {
            let Some(t89) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087e33c); };
            c.cmp(t89, 0x0u64, 32);
            if c.cc_e() { blk = 258; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t90) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x700087e346); };
            c.w32(8, t90);
            c.w64(9, c.r64(8));
            let r91 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r91);
            let r92 = c.r32(9) & 0x3fu64;
            c.w32(9, r92);
            c.logic(r92, 32);
            if !c.st64(c.r64(5).wrapping_sub(0xb8), c.r64(8)) { return RegfileExit::fault(0x700087e356); }
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_a() { blk = 258; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t93) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087e367); };
            c.cmp(t93, c.r32(6), 32);
            if c.cc_ne() { blk = 258; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t94) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087e370); };
            c.w32(9, t94);
            c.w64(6, c.r64(9));
            let r95 = c.r32(6) & 0x8100000u64;
            c.w32(6, r95);
            c.logic(r95, 32);
            c.cmp(c.r32(6), 0x8000000u64, 32);
            if c.cc_ne() { blk = 256; continue; }
            blk = 70; continue;
        }
        70 => {
            c.logic(c.r32(9) & 0x200000u64, 32);
            if c.cc_ne() { blk = 256; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t96) = c.ld32(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x700087e396); };
            c.w32(9, t96);
            let r97 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r97);
            let Some(t98) = c.ld16(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087e39d); };
            c.w32(6, t98);
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_a() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            return RegfileExit::fault(0x700087e3a8);
        }
        73 => {
            let Some(t99) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087e3a9); };
            c.w32(9, t99);
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 256; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t100) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087e3b7); };
            c.w32(9, t100);
            let r101 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r101);
            let Some(t102) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e3be); };
            c.w32(6, t102);
            let Some(t103) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700087e3c2); };
            c.cmp(t103, c.r32(6), 32);
            if c.cc_ne() { blk = 101; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t104) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087e3cf); };
            c.w32(6, t104);
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 100; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t105) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087e3dc); };
            c.w32(1, t105);
            c.cmp(c.r32(6), c.r32(1), 32);
            if c.cc_a() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(1, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(1));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_ae() { blk = 81; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700087e3f3); };
            c.w64(2, t106);
            let Some(t107) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x700087e3fa); };
            c.w32(2, t107);
            let r108 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r108);
            return c.call(0x7000824234, 0x700087e408, 0x700087e403);
        }
        79 => {
            return RegfileExit::fault(0x700087e408);
        }
        80 => {
            return RegfileExit::fault(0x700087e409);
        }
        81 => {
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_e() { blk = 97; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r64(6), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 229; continue; }
            blk = 83; continue;
        }
        83 => {
            c.w32(11, c.r64(6).wrapping_add(c.r64(6)));
            c.w64(6, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(6));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w32(7, c.r32(7));
            let r109 = c.imul2(c.r32(7), 0x4u64, 32);
            c.w32(7, r109);
            if c.cc_o() { blk = 96; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w32(8, 0x8u64);
            let r110 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r110);
            if c.cc_o() { blk = 95; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t111) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700087e44b); };
            c.w64(3, t111);
            c.w64(2, c.r64(3).wrapping_add(c.r64(7)));
            let Some(t112) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700087e453); };
            c.cmp(t112, c.r64(2), 64);
            if c.cc_be() { blk = 227; continue; }
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 227; continue; }
            blk = 87; continue;
        }
        87 => {
            if !c.st64(c.r64(12), c.r64(2)) { return RegfileExit::fault(0x700087e469); }
            let r113 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r113);
            blk = 88; continue;
        }
        88 => {
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700087e471); };
            c.w64(2, t114);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x700087e478); }
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700087e47b); }
            c.w64(11, c.r64(6).wrapping_mul(2).wrapping_add(0x8));
            c.w64(2, c.r64(8));
            blk = 90; continue;
        }
        89 => {
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x700087e48c); }
            let r115 = c.add(c.r64(8), 0x4u64, 64);
            c.w64(8, r115);
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_e() { blk = 94; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w64(6, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_e() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t116) = c.ld32(c.r64(9).wrapping_add(c.r64(2)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e4a7); };
            c.w32(6, t116);
            let r117 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r117);
            blk = 89; continue;
        }
        93 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087e4b2); };
            c.w64(6, t118);
            blk = 89; continue;
        }
        94 => {
            c.w64(11, c.r64(3));
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087e4be); };
            c.w64(3, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087e4c2); };
            c.w64(2, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x700087e4c6); };
            c.w64(8, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x700087e4cd); };
            c.w64(7, t122);
            blk = 97; continue;
        }
        95 => {
            return RegfileExit::fault(0x700087e4d6);
        }
        96 => {
            return RegfileExit::fault(0x700087e4d7);
        }
        97 => {
            c.w64(9, c.r64(2));
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700087e4db); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 99; continue; }
            blk = 98; continue;
        }
        98 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r123 = c.r64(2) & c.r64(9);
            c.w64(2, r123);
            c.logic(r123, 64);
            let Some(t124) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087e4ef); };
            c.logic(t124 & 0x4u64, 8);
            if c.cc_ne() { blk = 279; continue; }
            blk = 99; continue;
        }
        99 => {
            blk = 102; continue;
        }
        100 => {
            return RegfileExit::fault(0x700087e4fb);
        }
        101 => {
            c.w64(9, c.r64(2));
            blk = 102; continue;
        }
        102 => {
            c.w64(8, c.r64(15));
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x700087e502); }
            c.w64(2, c.r64(7));
            c.w64(7, c.r64(0));
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087e50c); };
            c.w64(0, t125);
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700087e510); };
            c.w64(1, t126);
            blk = 104; continue;
        }
        103 => {
            let r127 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r127);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x700087e520); }
            blk = 104; continue;
        }
        104 => {
            let Some(t128) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087e524); };
            c.cmp(c.r32(8), t128, 32);
            if c.cc_ge() { blk = 204; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t129) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e52e); };
            c.cmp(t129, c.r32(2), 32);
            if c.cc_ne() { blk = 255; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t130) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087e537); };
            c.cmp(t130, c.r32(15), 32);
            if c.cc_e() { blk = 255; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t131) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087e541); };
            c.w32(9, t131);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_le() { blk = 254; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t132) = c.ld8(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x700087e54e); };
            c.w32(9, t132);
            let r133 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r133);
            let r134 = c.r32(9) & 0x3fu64;
            c.w32(9, r134);
            c.logic(r134, 32);
            let r135 = c.shr(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r135);
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 113; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t136) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087e567); };
            c.w32(9, t136);
            let r137 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r137);
            let Some(t138) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e56e); };
            c.w32(11, t138);
            let r139 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r139);
            let Some(t140) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087e575); };
            c.w32(11, t140);
            let r141 = c.sub(c.r32(11), 0x77u64, 32);
            c.w32(11, r141);
            c.cmp(c.r32(11), 0xcu64, 32);
            if c.cc_a() { blk = 206; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t142) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087e588); };
            c.w32(11, t142);
            c.cmp(c.r32(8), c.r32(11), 32);
            if c.cc_ae() { blk = 205; continue; }
            blk = 111; continue;
        }
        111 => {
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t143) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087e598); };
            c.w32(9, t143);
            let r144 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r144);
            let Some(t145) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087e5a0); };
            c.cmp(t145, c.r32(9), 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 112; continue;
        }
        112 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x700087e5ad); }
            blk = 119; continue;
        }
        113 => {
            let Some(t146) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087e5b3); };
            c.w32(9, t146);
            let r147 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r147);
            let Some(t148) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e5ba); };
            c.w32(11, t148);
            let Some(t149) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700087e5be); };
            c.cmp(t149, c.r32(11), 32);
            if c.cc_ne() { blk = 208; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t150) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087e5cb); };
            c.w32(11, t150);
            c.cmp(c.r32(8), c.r32(11), 32);
            if c.cc_ae() { blk = 207; continue; }
            blk = 115; continue;
        }
        115 => {
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) { return RegfileExit::fault(0x700087e5db); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x700087e5e2); }
            c.w64(8, 0xfff7fffffff7ffffu64);
            let Some(t151) = c.ld64(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087e5f0); };
            c.cmp(t151, c.r64(8), 64);
            if c.cc_e() { blk = 202; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t152) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700087e5fb); };
            c.w64(8, t152);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t153) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700087e603); };
            c.cmp(t153, c.r64(9), 64);
            if c.cc_be() { blk = 231; continue; }
            blk = 117; continue;
        }
        117 => {
            if !c.st64(c.r64(12), c.r64(9)) { return RegfileExit::fault(0x700087e60c); }
            let r154 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r154);
            blk = 118; continue;
        }
        118 => {
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087e614); };
            c.w64(9, t155);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700087e61b); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087e61f); }
            c.w64(9, c.r64(8));
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087e628); };
            c.w64(8, t156);
            blk = 119; continue;
        }
        119 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087e62c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087e62d); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700087e62f); }
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087e631); };
            if !c.push64(t157) { return RegfileExit::fault(0x700087e631); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x700087e634); }
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087e638); };
            c.w64(6, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087e63c); };
            c.w64(7, t159);
            c.w32(0, 0x3u64);
            c.w64(11, c.r64(0));
            return c.call(0x70007f2274, 0x700087e64d, 0x700087e648);
        }
        120 => {
            let Some(t160) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700087e64d); };
            c.cmp(t160, c.r32(0), 32);
            if c.cc_e() { blk = 200; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 127; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t161) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700087e661); };
            c.cmp(t161, c.r32(0), 32);
            if c.cc_e() { blk = 200; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t162) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e66e); };
            c.w32(8, t162);
            let r163 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r163);
            let Some(t164) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087e675); };
            c.logic(t164 & 0x10u64, 8);
            if c.cc_ne() { blk = 200; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t165) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087e680); };
            c.cmp(t165, c.r32(8), 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t166) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087e68d); };
            c.cmp(t166, 0x41u64, 16);
            if c.cc_e() { blk = 233; continue; }
            blk = 126; continue;
        }
        126 => {
            blk = 128; continue;
        }
        127 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 200; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087e6a5); };
            c.w64(0, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x700087e6a9); };
            c.w64(8, t168);
            let Some(t169) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087e6b0); };
            c.cmp(t169, c.r32(8), 32);
            if c.cc_ne() { blk = 197; continue; }
            blk = 129; continue;
        }
        129 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700087e6bc); };
            c.w64(9, t170);
            let Some(t171) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087e6c3); };
            c.cmp(t171, c.r32(6), 32);
            if c.cc_e() { blk = 197; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t172) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087e6cd); };
            c.w32(11, t172);
            let Some(t173) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087e6d1); };
            c.cmp(c.r32(11), t173, 32);
            if c.cc_ne() { blk = 197; continue; }
            blk = 131; continue;
        }
        131 => {
            c.w64(12, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r174 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r174);
            let Some(t175) = c.ld8(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x700087e6e8); };
            c.w32(3, t175);
            let r176 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r176);
            let r177 = c.r32(3) & 0x3fu64;
            c.w32(3, r177);
            c.logic(r177, 32);
            c.w64(2, c.r64(3));
            let r178 = c.shr(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r178);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(12)) { return RegfileExit::fault(0x700087e6fa); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(15)) { return RegfileExit::fault(0x700087e6fe); }
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 132; continue;
        }
        132 => {
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_le() { blk = 151; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t179) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087e714); };
            c.w32(3, t179);
            let r180 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r180);
            let Some(t181) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087e71a); };
            c.w32(2, t181);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(2));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), c.r64(15), 64);
            if c.cc_l() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t182) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087e72b); };
            c.w64(15, t182);
            let Some(t183) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087e732); };
            c.w64(9, t183);
            blk = 147; continue;
        }
        135 => {
            c.w64(1, c.r64(15));
            if let Some(sr) = c.shr(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(1, c.r64(1).wrapping_add(c.r64(15)).wrapping_add(0x10));
            c.cmp(c.r64(1), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 197; continue; }
            blk = 136; continue;
        }
        136 => {
            c.cmp(c.r64(1), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 244; continue; }
            blk = 137; continue;
        }
        137 => {
            c.w32(7, c.r64(1).wrapping_add(c.r64(1)));
            c.w64(8, c.r64(1).wrapping_mul(4).wrapping_add(0x8));
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            let Some(t184) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087e776); };
            c.w64(9, t184);
            let Some(t185) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700087e77d); };
            c.w64(6, t185);
            c.w64(12, c.r64(8).wrapping_add(c.r64(6)));
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087e784); };
            c.w64(15, t186);
            let Some(t187) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700087e78b); };
            c.cmp(t187, c.r64(12), 64);
            if c.cc_be() { blk = 237; continue; }
            blk = 138; continue;
        }
        138 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700087e794); }
            c.w64(8, c.r64(6).wrapping_add(0x1));
            blk = 139; continue;
        }
        139 => {
            let Some(t188) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700087e79b); };
            c.w64(12, t188);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700087e7a2); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700087e7a6); }
            c.w64(12, c.r64(2).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), c.r64(12), 64);
            if c.cc_e() { blk = 142; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t189) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087e7b7); };
            c.w64(2, t189);
            blk = 141; continue;
        }
        141 => {
            let r190 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r190);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)), c.r32(2)) { return RegfileExit::fault(0x700087e7c2); }
            c.cmp(c.r64(1), c.r64(12), 64);
            if c.cc_ne() { blk = 141; continue; }
            blk = 142; continue;
        }
        142 => {
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_e() { blk = 144; continue; }
            blk = 143; continue;
        }
        143 => {
            let r191 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r191);
            let Some(t192) = c.ld32(c.r64(12).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x700087e7d5); };
            c.w32(2, t192);
            if !c.st32(c.r64(12).wrapping_add(c.r64(8)), c.r32(2)) { return RegfileExit::fault(0x700087e7d9); }
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_ne() { blk = 143; continue; }
            blk = 144; continue;
        }
        144 => {
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700087e7e3); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 146; continue; }
            blk = 145; continue;
        }
        145 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r193 = c.r64(12) & c.r64(0);
            c.w64(12, r193);
            c.logic(r193, 64);
            let Some(t194) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087e7f7); };
            c.logic(t194 & 0x4u64, 8);
            if c.cc_ne() { blk = 276; continue; }
            blk = 146; continue;
        }
        146 => {
            c.w64(3, c.r64(8));
            blk = 147; continue;
        }
        147 => {
            let Some(t195) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087e806); };
            c.w32(8, t195);
            c.cmp(c.r32(11), c.r32(8), 32);
            if c.cc_ae() { blk = 243; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700087e813); };
            c.w64(8, t196);
            let r197 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r197);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087e81a); };
            c.w64(12, t198);
            c.w64(11, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087e826); };
            c.w64(1, t199);
            if !c.st32(c.r64(3).wrapping_add(c.r64(11)), c.r32(1)) { return RegfileExit::fault(0x700087e82a); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 150; continue; }
            blk = 149; continue;
        }
        149 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r200 = c.r64(12) & c.r64(3);
            c.w64(12, r200);
            c.logic(r200, 64);
            let Some(t201) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087e83d); };
            c.logic(t201 & 0x4u64, 8);
            if c.cc_ne() { blk = 273; continue; }
            blk = 150; continue;
        }
        150 => {
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700087e849); }
            c.w64(8, c.r64(9));
            c.w64(12, c.r64(15));
            blk = 199; continue;
        }
        151 => {
            let Some(t202) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087e858); };
            c.w32(3, t202);
            let r203 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r203);
            let Some(t204) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087e85e); };
            c.w32(2, t204);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(2));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), c.r64(15), 64);
            if c.cc_l() { blk = 153; continue; }
            blk = 152; continue;
        }
        152 => {
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087e86f); };
            c.w64(12, t205);
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087e876); };
            c.w64(7, t206);
            blk = 165; continue;
        }
        153 => {
            c.w64(1, c.r64(15));
            if let Some(sr) = c.shr(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(1, c.r64(1).wrapping_add(c.r64(15)).wrapping_add(0x10));
            c.cmp(c.r64(1), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 168; continue; }
            blk = 154; continue;
        }
        154 => {
            c.cmp(c.r64(1), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 247; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087e8a7); };
            c.w64(7, t207);
            let Some(t208) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700087e8ae); };
            c.w64(8, t208);
            c.w64(9, c.r64(1).wrapping_mul(4).wrapping_add(0x8));
            c.w64(6, c.r64(9).wrapping_add(c.r64(8)));
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087e8bd); };
            c.w64(12, t209);
            let Some(t210) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700087e8c4); };
            c.cmp(t210, c.r64(6), 64);
            if c.cc_be() { blk = 239; continue; }
            blk = 156; continue;
        }
        156 => {
            if !c.st64(c.r64(7), c.r64(6)) { return RegfileExit::fault(0x700087e8ce); }
            let r211 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r211);
            blk = 157; continue;
        }
        157 => {
            c.w32(9, c.r64(1).wrapping_add(c.r64(1)));
            let Some(t212) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700087e8d9); };
            c.w64(6, t212);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x700087e8e0); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700087e8e4); }
            c.w64(9, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            c.w64(2, c.r64(2).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(2), 64);
            if c.cc_e() { blk = 160; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t213) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087e8fd); };
            c.w64(1, t213);
            blk = 159; continue;
        }
        159 => {
            let r214 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r214);
            if !c.st32(c.r64(9).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x700087e908); }
            c.cmp(c.r64(9), c.r64(2), 64);
            if c.cc_ne() { blk = 159; continue; }
            blk = 160; continue;
        }
        160 => {
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_e() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            let r215 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r215);
            let Some(t216) = c.ld32(c.r64(2).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x700087e91b); };
            c.w32(9, t216);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(9)) { return RegfileExit::fault(0x700087e91f); }
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_ne() { blk = 161; continue; }
            blk = 162; continue;
        }
        162 => {
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700087e929); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 164; continue; }
            blk = 163; continue;
        }
        163 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r217 = c.r64(9) & c.r64(0);
            c.w64(9, r217);
            c.logic(r217, 64);
            let Some(t218) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087e93d); };
            c.logic(t218 & 0x4u64, 8);
            if c.cc_ne() { blk = 270; continue; }
            blk = 164; continue;
        }
        164 => {
            c.w64(3, c.r64(8));
            blk = 165; continue;
        }
        165 => {
            let Some(t219) = c.ld8(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087e94b); };
            c.logic(t219 & 0x1u64, 8);
            if c.cc_ne() { blk = 197; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t220) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087e955); };
            c.w32(8, t220);
            c.cmp(c.r32(11), c.r32(8), 32);
            if c.cc_ae() { blk = 246; continue; }
            blk = 167; continue;
        }
        167 => {
            c.w32(8, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087e966); };
            c.w64(1, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087e96a); };
            c.w64(9, t222);
            if !c.st32(c.r64(3).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700087e96e); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700087e973); }
            c.w64(8, c.r64(7));
            blk = 199; continue;
        }
        168 => {
            let Some(t223) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087e97f); };
            c.w64(7, t223);
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087e986); };
            c.w64(12, t224);
            blk = 197; continue;
        }
        169 => {
            let Some(t225) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087e992); };
            c.w32(3, t225);
            let r226 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r226);
            let Some(t227) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087e998); };
            c.w32(2, t227);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(2));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), c.r64(15), 64);
            if c.cc_l() { blk = 171; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t228) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087e9a9); };
            c.w64(8, t228);
            blk = 187; continue;
        }
        171 => {
            c.w64(1, c.r64(15));
            if let Some(sr) = c.shr(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(1, c.r64(1).wrapping_add(c.r64(15)).wrapping_add(0x10));
            c.cmp(c.r64(1), 0x3fffu64, 64);
            if c.cc_ae() { blk = 196; continue; }
            blk = 172; continue;
        }
        172 => {
            c.cmp(c.r64(1), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 252; continue; }
            blk = 173; continue;
        }
        173 => {
            c.w64(7, c.r64(1).wrapping_mul(8).wrapping_add(0x8));
            c.cmp(c.r64(7), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 249; continue; }
            blk = 174; continue;
        }
        174 => {
            c.w32(9, c.r64(1).wrapping_add(c.r64(1)));
            let Some(t229) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087e9f3); };
            c.w64(8, t229);
            let Some(t230) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700087e9fa); };
            c.w64(12, t230);
            c.w64(15, c.r64(7).wrapping_add(c.r64(12)));
            let Some(t231) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087ea01); };
            c.cmp(t231, c.r64(15), 64);
            if c.cc_be() { blk = 241; continue; }
            blk = 175; continue;
        }
        175 => {
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x700087ea0b); }
            let r232 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r232);
            blk = 176; continue;
        }
        176 => {
            let Some(t233) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700087ea12); };
            c.w64(15, t233);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700087ea19); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x700087ea1e); }
            let Some(t234) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087ea23); };
            c.w64(9, t234);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700087ea2a); }
            c.w64(9, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            c.w64(15, c.r64(1).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_e() { blk = 178; continue; }
            blk = 177; continue;
        }
        177 => {
            let r235 = c.sub(c.r64(15), 0x8u64, 64);
            c.w64(15, r235);
            if !c.movsd_mr(c.r64(15).wrapping_add(c.r64(12)), 0) { return RegfileExit::fault(0x700087ea49); }
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_ne() { blk = 177; continue; }
            blk = 178; continue;
        }
        178 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 184; continue; }
            blk = 179; continue;
        }
        179 => {
            let Some(t236) = c.ld64(0x700087e5e8u64) else { return RegfileExit::fault(0x700087ea5a); };
            c.w64(15, t236);
            blk = 180; continue;
        }
        180 => {
            let r237 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r237);
            let Some(t238) = c.ld64(c.r64(9).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x700087ea65); };
            c.cmp(t238, c.r64(15), 64);
            if c.cc_e() { blk = 182; continue; }
            blk = 181; continue;
        }
        181 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(3))) { return RegfileExit::fault(0x700087ea6b); }
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(12)), 0) { return RegfileExit::fault(0x700087ea71); }
            blk = 183; continue;
        }
        182 => {
            if !c.st64(c.r64(9).wrapping_add(c.r64(12)), c.r64(15)) { return RegfileExit::fault(0x700087ea79); }
            blk = 183; continue;
        }
        183 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 180; continue; }
            blk = 184; continue;
        }
        184 => {
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700087ea83); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 186; continue; }
            blk = 185; continue;
        }
        185 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r239 = c.r64(9) & c.r64(0);
            c.w64(9, r239);
            c.logic(r239, 64);
            let Some(t240) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087ea97); };
            c.logic(t240 & 0x4u64, 8);
            if c.cc_ne() { blk = 267; continue; }
            blk = 186; continue;
        }
        186 => {
            c.w64(3, c.r64(12));
            blk = 187; continue;
        }
        187 => {
            let Some(t241) = c.ld8(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087eaa5); };
            c.logic(t241 & 0x1u64, 8);
            if c.cc_e() { blk = 190; continue; }
            blk = 188; continue;
        }
        188 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087eaab); };
            c.w64(1, t242);
            let Some(t243) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087eaaf); };
            c.w32(9, t243);
            let Some(t244) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087eab3); };
            c.cmp(t244, c.r32(9), 32);
            if c.cc_ne() { blk = 197; continue; }
            blk = 189; continue;
        }
        189 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 191; continue;
        }
        190 => {
            c.w32(9, 0x1u64);
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087eac7); };
            c.w64(1, t245);
            blk = 191; continue;
        }
        191 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 193; continue; }
            blk = 192; continue;
        }
        192 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700087ead1); }
            blk = 194; continue;
        }
        193 => {
            let r246 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r246);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 194; continue;
        }
        194 => {
            let Some(t247) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087eae2); };
            c.w32(9, t247);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_ae() { blk = 251; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700087eaef); };
            c.w64(9, t248);
            let r249 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r249);
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087eb00); };
            c.w64(11, t250);
            if !c.movsd_mr(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700087eb04); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700087eb0b); }
            let Some(t251) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087eb0f); };
            c.w64(12, t251);
            blk = 199; continue;
        }
        196 => {
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087eb18); };
            c.w64(1, t252);
            let Some(t253) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087eb1c); };
            c.w64(8, t253);
            blk = 197; continue;
        }
        197 => {
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087eb23); };
            c.w64(6, t254);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087eb27); };
            c.w64(3, t255);
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087eb2b); };
            c.w64(1, t256);
            return c.call(0x7000899bb4, 0x700087eb34, 0x700087eb2f);
        }
        198 => {
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087eb34); };
            c.w64(0, t257);
            let Some(t258) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087eb38); };
            c.w64(8, t258);
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087eb3f); };
            c.w64(12, t259);
            blk = 199; continue;
        }
        199 => {
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087eb46); };
            c.w64(11, t260);
            c.w32(9, c.r64(11).wrapping_add(0x2));
            c.w64(11, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 201; continue;
        }
        200 => {
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087eb53); };
            c.w64(11, t261);
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087eb57); };
            c.w64(0, t262);
            let Some(t263) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087eb5b); };
            c.w64(8, t263);
            let Some(t264) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087eb62); };
            c.w64(12, t264);
            blk = 201; continue;
        }
        201 => {
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x700087eb69); };
            c.w64(2, t265);
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087eb70); };
            c.w64(0, t266);
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(12));
            c.w64(12, c.r64(8));
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087eb7d); };
            c.w64(8, t267);
            let Some(t268) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700087eb81); };
            c.w64(1, t268);
            blk = 103; continue;
        }
        202 => {
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087eb8d); };
            c.w64(8, t269);
            blk = 203; continue;
        }
        203 => {
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087eb91); };
            c.w64(11, t270);
            blk = 103; continue;
        }
        204 => {
            let Some(t271) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087eb9a); };
            c.w64(9, t271);
            c.w64(8, c.r64(9).wrapping_add(0x1));
            let Some(t272) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087eba2); };
            c.w64(0, t272);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087eba9); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087ebaa); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087ebb0); }
            return c.ret(0x0, 0x700087ebb2);
        }
        205 => {
            return RegfileExit::fault(0x700087ebb3);
        }
        206 => {
            return RegfileExit::fault(0x700087ebb4);
        }
        207 => {
            return RegfileExit::fault(0x700087ebb5);
        }
        208 => {
            return RegfileExit::fault(0x700087ebb6);
        }
        209 => {
            c.w64(12, c.r64(9));
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087ebba); };
            c.w64(11, t273);
            blk = 210; continue;
        }
        210 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700087ebbe); }
            c.w32(0, 0x1u64);
            let Some(t274) = c.ld64(c.r64(13).wrapping_add(0x1c80)) else { return RegfileExit::fault(0x700087ebc5); };
            c.w64(3, t274);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x700087ebd4, 0x700087ebcf);
        }
        211 => {
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700087ebd7); }
            let Some(t275) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087ebd9); };
            c.w64(1, t275);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700087ebe0); }
            c.w64(7, c.r64(0));
            c.w64(2, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087ebec); };
            c.w64(6, t276);
            return c.call(0x70007f7214, 0x700087ebf5, 0x700087ebf0);
        }
        212 => {
            c.w32(8, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(8));
            blk = 213; continue;
        }
        213 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087ebfb); }
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087ebfd); };
            if !c.push64(t277) { return RegfileExit::fault(0x700087ebfd); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700087ec00); }
            c.w64(2, c.r64(0));
            let Some(t278) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087ec05); };
            c.w64(0, t278);
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087ec09); };
            c.w64(3, t279);
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087ec0d); };
            c.w64(1, t280);
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087ec11); };
            c.w64(6, t281);
            c.w64(7, c.r64(0));
            return c.call(0x700087d0b4, 0x700087ec1d, 0x700087ec18);
        }
        214 => {
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087ec1d); };
            c.w64(8, t282);
            let r283 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r283);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087ec28); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087ec29); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087ec2f); }
            return c.ret(0x0, 0x700087ec31);
        }
        215 => {
            c.w32(8, 0x36u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087ec38); }
            let Some(t284) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700087ec3a); };
            c.w64(8, t284);
            let Some(t285) = c.ld32(c.r64(8).wrapping_add(0x117)) else { return RegfileExit::fault(0x700087ec41); };
            c.w32(8, t285);
            let r286 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r286);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087ec4b); }
            let Some(t287) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700087ec4d); };
            c.w64(3, t287);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700087ec5e, 0x700087ec59);
        }
        216 => {
            return RegfileExit::fault(0x700087ec5e);
        }
        217 => {
            let Some(t288) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700087ec5f); };
            c.w64(3, t288);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700087ec70, 0x700087ec6b);
        }
        218 => {
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087ec70); };
            c.w64(6, t289);
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087ec74); };
            c.w64(0, t290);
            blk = 1; continue;
        }
        219 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a2054, 0x700087ec85, 0x700087ec80);
        }
        220 => {
            c.w64(12, c.r64(0));
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087ec88); };
            c.w64(6, t291);
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087ec8c); };
            c.w64(8, t292);
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087ec90); };
            c.w64(11, t293);
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087ec94); };
            c.w64(9, t294);
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087ec98); };
            c.w64(0, t295);
            blk = 5; continue;
        }
        221 => {
            let Some(t296) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087eca1); };
            c.w32(15, t296);
            let r297 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r297);
            blk = 24; continue;
        }
        222 => {
            let Some(t298) = c.ld32(c.r64(15).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x700087ecae); };
            c.cmp(t298, c.r32(2), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 223; continue;
        }
        223 => {
            let Some(t299) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700087ecbb); };
            c.w64(3, t299);
            c.w64(0, c.r64(12));
            return c.call(0x7000870914, 0x700087ecca, 0x700087ecc5);
        }
        224 => {
            c.w64(15, c.r64(0));
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087eccd); };
            c.w64(6, t300);
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087ecd1); };
            c.w64(8, t301);
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087ecd5); };
            c.w64(12, t302);
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087ecd9); };
            c.w64(11, t303);
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087ecdd); };
            c.w64(9, t304);
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087ece1); };
            c.w64(0, t305);
            blk = 24; continue;
        }
        225 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x700087ecef); }
            return c.call(0x70007febf4, 0x700087ecf8, 0x700087ecf3);
        }
        226 => {
            c.w64(2, c.r64(0));
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087ecfb); };
            c.w64(6, t306);
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087ecff); };
            c.w64(9, t307);
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087ed03); };
            c.w64(7, t308);
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087ed07); };
            c.w64(3, t309);
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t310) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087ed0e); };
            c.w64(12, t310);
            let Some(t311) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087ed15); };
            c.w64(0, t311);
            let Some(t312) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700087ed1c); };
            c.w64(8, t312);
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087ed23); };
            c.w64(11, t313);
            blk = 53; continue;
        }
        227 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x700087ed2c); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x700087ed30); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(6)) { return RegfileExit::fault(0x700087ed34); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(11)) { return RegfileExit::fault(0x700087ed38); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x700087ed44, 0x700087ed3f);
        }
        228 => {
            c.w64(3, c.r64(0));
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t314) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087ed4a); };
            c.w64(12, t314);
            let Some(t315) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087ed51); };
            c.w64(0, t315);
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087ed58); };
            c.w64(9, t316);
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087ed5c); };
            c.w64(1, t317);
            c.w32(8, 0x8u64);
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087ed66); };
            c.w64(6, t318);
            let Some(t319) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700087ed6a); };
            c.w64(11, t319);
            blk = 88; continue;
        }
        229 => {
            let Some(t320) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700087ed73); };
            c.w64(3, t320);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(15));
            return c.call(0x700086ed74, 0x700087ed84, 0x700087ed7f);
        }
        230 => {
            return RegfileExit::fault(0x700087ed84);
        }
        231 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x68), 0) { return RegfileExit::fault(0x700087ed8a); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x700087ed97, 0x700087ed92);
        }
        232 => {
            c.w64(8, c.r64(0));
            let Some(t321) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087ed9a); };
            c.w64(0, t321);
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t322) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087eda1); };
            c.w64(12, t322);
            let Some(t323) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087eda8); };
            c.w64(7, t323);
            let Some(t324) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700087edaf); };
            c.w64(1, t324);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x68)) { return RegfileExit::fault(0x700087edb6); }
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x700087edbb); };
            c.w64(2, t325);
            blk = 118; continue;
        }
        233 => {
            let Some(t326) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087edc7); };
            c.logic(t326 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 200; continue; }
            blk = 234; continue;
        }
        234 => {
            blk = 128; continue;
        }
        235 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087edd9); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 128; continue; }
            blk = 236; continue;
        }
        236 => {
            blk = 200; continue;
        }
        237 => {
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(3)) { return RegfileExit::fault(0x700087edf1); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(1)) { return RegfileExit::fault(0x700087edf5); }
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(2)) { return RegfileExit::fault(0x700087edfc); }
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(7)) { return RegfileExit::fault(0x700087ee03); }
            if !c.st64(c.r64(5).wrapping_sub(0xa8), c.r64(11)) { return RegfileExit::fault(0x700087ee0a); }
            c.w64(2, c.r64(8));
            return c.call(0x70007febf4, 0x700087ee19, 0x700087ee14);
        }
        238 => {
            c.w64(8, c.r64(0));
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087ee1c); };
            c.w64(0, t327);
            let Some(t328) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087ee20); };
            c.w64(9, t328);
            let Some(t329) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087ee27); };
            c.w64(15, t329);
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700087ee2e); };
            c.w64(3, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700087ee32); };
            c.w64(1, t331);
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x700087ee39); };
            c.w64(2, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x700087ee40); };
            c.w64(7, t333);
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x700087ee47); };
            c.w64(11, t334);
            blk = 139; continue;
        }
        239 => {
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(3)) { return RegfileExit::fault(0x700087ee53); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(2)) { return RegfileExit::fault(0x700087ee57); }
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(1)) { return RegfileExit::fault(0x700087ee5e); }
            if !c.st64(c.r64(5).wrapping_sub(0xa8), c.r64(11)) { return RegfileExit::fault(0x700087ee65); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febf4, 0x700087ee74, 0x700087ee6f);
        }
        240 => {
            c.w64(8, c.r64(0));
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087ee77); };
            c.w64(0, t335);
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700087ee7b); };
            c.w64(15, t336);
            let Some(t337) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087ee7f); };
            c.w64(7, t337);
            let Some(t338) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700087ee86); };
            c.w64(3, t338);
            let Some(t339) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700087ee8a); };
            c.w64(2, t339);
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x700087ee91); };
            c.w64(1, t340);
            let Some(t341) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087ee98); };
            c.w64(12, t341);
            let Some(t342) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x700087ee9f); };
            c.w64(11, t342);
            blk = 157; continue;
        }
        241 => {
            c.w32(12, c.r64(7).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700087eeaf); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700087eeb1); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(3)) { return RegfileExit::fault(0x700087eeb2); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(1)) { return RegfileExit::fault(0x700087eeb6); }
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(2)) { return RegfileExit::fault(0x700087eebd); }
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(9)) { return RegfileExit::fault(0x700087eec4); }
            if !c.st64(c.r64(5).wrapping_sub(0xa8), c.r64(11)) { return RegfileExit::fault(0x700087eecb); }
            c.w32(0, 0x2u64);
            let Some(t343) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700087eed7); };
            c.w64(3, t343);
            c.w64(12, c.r64(3));
            c.w64(15, c.r64(0));
            return c.call(0x700086ed74, 0x700087eee9, 0x700087eee4);
        }
        242 => {
            c.w64(12, c.r64(0));
            let Some(t344) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087eeec); };
            c.w64(0, t344);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t345) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700087eef2); };
            c.w64(3, t345);
            let Some(t346) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700087eef6); };
            c.w64(1, t346);
            let Some(t347) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x700087eefd); };
            c.w64(2, t347);
            let Some(t348) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x700087ef04); };
            c.w64(11, t348);
            let Some(t349) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087ef0b); };
            c.w64(8, t349);
            let Some(t350) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x700087ef12); };
            c.w64(9, t350);
            blk = 176; continue;
        }
        243 => {
            return RegfileExit::fault(0x700087ef1e);
        }
        244 => {
            c.w64(8, c.r64(6));
            let Some(t351) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700087ef22); };
            c.w64(3, t351);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087ef30, 0x700087ef2b);
        }
        245 => {
            return RegfileExit::fault(0x700087ef30);
        }
        246 => {
            return RegfileExit::fault(0x700087ef31);
        }
        247 => {
            c.w64(8, c.r64(6));
            let Some(t352) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700087ef35); };
            c.w64(3, t352);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087ef43, 0x700087ef3e);
        }
        248 => {
            return RegfileExit::fault(0x700087ef43);
        }
        249 => {
            c.w64(8, c.r64(6));
            let Some(t353) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700087ef47); };
            c.w64(3, t353);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087ef55, 0x700087ef50);
        }
        250 => {
            return RegfileExit::fault(0x700087ef55);
        }
        251 => {
            return RegfileExit::fault(0x700087ef56);
        }
        252 => {
            c.w64(8, c.r64(6));
            let Some(t354) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700087ef5a); };
            c.w64(3, t354);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087ef68, 0x700087ef63);
        }
        253 => {
            return RegfileExit::fault(0x700087ef68);
        }
        254 => {
            c.w64(15, c.r64(8));
            let Some(t355) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087ef6c); };
            c.w64(8, t355);
            blk = 257; continue;
        }
        255 => {
            c.w64(15, c.r64(8));
            let Some(t356) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087ef75); };
            c.w64(8, t356);
            blk = 257; continue;
        }
        256 => {
            let Some(t357) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087ef7b); };
            c.w64(0, t357);
            c.w64(8, c.r64(15));
            blk = 257; continue;
        }
        257 => {
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(15));
            c.w64(15, c.r64(10));
            c.w64(9, c.r64(0));
            let Some(t358) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087ef8e); };
            c.w64(3, t358);
            let Some(t359) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087ef92); };
            c.w64(2, t359);
            blk = 261; continue;
        }
        258 => {
            c.w64(8, c.r64(15));
            let Some(t360) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087ef9b); };
            c.w64(9, t360);
            blk = 261; continue;
        }
        259 => {
            c.w64(8, c.r64(15));
            blk = 261; continue;
        }
        260 => {
            c.w64(8, c.r64(15));
            blk = 261; continue;
        }
        261 => {
            c.w64(0, c.r64(2));
            blk = 213; continue;
        }
        262 => {
            let Some(t361) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087efb1); };
            c.w64(1, t361);
            blk = 263; continue;
        }
        263 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 265; continue; }
            blk = 264; continue;
        }
        264 => {
            let Some(t362) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087efbd); };
            c.w64(8, t362);
            let Some(t363) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087efc1); };
            c.w64(8, t363);
            c.w64(1, c.r64(8));
            blk = 265; continue;
        }
        265 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087efce); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700087efd0); }
            let Some(t364) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700087efd1); };
            c.w64(3, t364);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700087efe2, 0x700087efdd);
        }
        266 => {
            return RegfileExit::fault(0x700087efe2);
        }
        267 => {
            c.w32(12, c.r32(12));
            let r365 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r365);
            c.w64(9, 0xfffffffffffc0000u64);
            let r366 = c.r64(9) & c.r64(12);
            c.w64(9, r366);
            c.logic(r366, 64);
            let Some(t367) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087eff3); };
            c.logic(t367 & 0x2u64, 8);
            if c.cc_e() { blk = 186; continue; }
            blk = 268; continue;
        }
        268 => {
            c.w64(15, c.r64(0).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087f002); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087f003); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x700087f010, 0x700087f00b);
        }
        269 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700087f010); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700087f012); }
            blk = 186; continue;
        }
        270 => {
            c.w32(8, c.r32(8));
            let r368 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r368);
            c.w64(9, 0xfffffffffffc0000u64);
            let r369 = c.r64(9) & c.r64(8);
            c.w64(9, r369);
            c.logic(r369, 64);
            let Some(t370) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087f028); };
            c.logic(t370 & 0x2u64, 8);
            if c.cc_e() { blk = 164; continue; }
            blk = 271; continue;
        }
        271 => {
            c.w64(3, c.r64(0).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087f037); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087f038); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x700087f045, 0x700087f040);
        }
        272 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700087f045); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700087f047); }
            blk = 164; continue;
        }
        273 => {
            c.w32(1, c.r32(1));
            let r371 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r371);
            c.w64(12, 0xfffffffffffc0000u64);
            let r372 = c.r64(12) & c.r64(1);
            c.w64(12, r372);
            c.logic(r372, 64);
            let Some(t373) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087f05c); };
            c.logic(t373 & 0x2u64, 8);
            if c.cc_e() { blk = 150; continue; }
            blk = 274; continue;
        }
        274 => {
            c.w64(2, c.r64(3).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087f06c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087f06d); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x700087f07a, 0x700087f075);
        }
        275 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700087f07a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700087f07c); }
            blk = 150; continue;
        }
        276 => {
            c.w32(8, c.r32(8));
            let r374 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r374);
            c.w64(12, 0xfffffffffffc0000u64);
            let r375 = c.r64(12) & c.r64(8);
            c.w64(12, r375);
            c.logic(r375, 64);
            let Some(t376) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087f092); };
            c.logic(t376 & 0x2u64, 8);
            if c.cc_e() { blk = 146; continue; }
            blk = 277; continue;
        }
        277 => {
            c.w64(3, c.r64(0).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087f0a2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087f0a3); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x700087f0b0, 0x700087f0ab);
        }
        278 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700087f0b0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700087f0b2); }
            blk = 146; continue;
        }
        279 => {
            c.w32(11, c.r32(11));
            let r377 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r377);
            c.w64(2, 0xfffffffffffc0000u64);
            let r378 = c.r64(2) & c.r64(11);
            c.w64(2, r378);
            c.logic(r378, 64);
            let Some(t379) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087f0c8); };
            c.logic(t379 & 0x2u64, 8);
            if c.cc_e() { blk = 99; continue; }
            blk = 280; continue;
        }
        280 => {
            c.w64(1, c.r64(9).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087f0d6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087f0d7); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x700087f0e4, 0x700087f0df);
        }
        281 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700087f0e4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700087f0e6); }
            blk = 99; continue;
        }
        282 => {
            return RegfileExit::jump(0x700087f0f0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700087f0f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087f0f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700087f0f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087f0f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087f0fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700087f0ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087f109); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f116); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087f11d); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087f136); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f13f); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087f146); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700087f14d); };
            c.w64(7, t11);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f157); };
            c.w32(8, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087f15b); };
            c.cmp(t13, c.r32(8), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700087f164); };
            c.w64(8, t14);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f16e); };
            c.w32(9, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087f172); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087f17b); }
            c.w64(0, c.r64(2));
            let Some(t17) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700087f180); };
            c.w64(1, t17);
            return c.call(0x700087f374, 0x700087f189, 0x700087f184);
        }
        10 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087f189); };
            c.w64(1, t18);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087f190); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x28, 0x700087f197);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087f19a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087f1a1); }
            return c.ret(0x0, 0x700087f1a3);
        }
        13 => {
            return RegfileExit::fault(0x700087f1a4);
        }
        14 => {
            return RegfileExit::fault(0x700087f1a5);
        }
        15 => {
            return RegfileExit::fault(0x700087f1a6);
        }
        16 => {
            return RegfileExit::fault(0x700087f1a7);
        }
        17 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700087f1a8); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700087f1ac); };
            c.w64(3, t19);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087f1ba, 0x700087f1b5);
        }
        18 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087f1ba); };
            c.w64(6, t20);
            blk = 1; continue;
        }
        19 => {
            return RegfileExit::jump(0x700087f1c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700087f1d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087f1d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700087f1d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087f1d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087f1da); }
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700087f1db); };
            c.cmp(c.r64(4), t0, 64);
            if c.cc_be() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x700087f1e1);
        }
        2 => {
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700087f1e2); };
            c.w64(3, t1);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087f1f0, 0x700087f1eb);
        }
        3 => {
            blk = 1; continue;
        }
        4 => {
            return RegfileExit::jump(0x700087f1f4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700087f214(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087f214); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700087f218); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087f219); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087f21a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700087f21f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 27; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087f229); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f236); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087f23d); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 26; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087f256); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f263); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087f26a); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700087f275); };
            c.w64(7, t11);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f27f); };
            c.w32(8, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087f283); };
            c.cmp(t13, c.r32(8), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700087f290); };
            c.w64(8, t14);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f29a); };
            c.w32(9, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087f29e); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_add(0x40)) else { return RegfileExit::fault(0x700087f2ab); };
            c.w64(9, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700087f2af); };
            c.cmp(t18, c.r32(9), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(11, c.r32(9));
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700087f2c1); };
            c.cmp(t19, c.r32(9), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t20) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f2ca); };
            c.w32(11, t20);
            let r21 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r21);
            let Some(t22) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087f2d1); };
            c.logic(t22 & 0x10u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087f2d8); };
            c.cmp(t23, c.r32(11), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t24) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087f2e1); };
            c.cmp(t24, 0x41u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 15; continue;
        }
        15 => {
            blk = 17; continue;
        }
        16 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_add(0x38)) else { return RegfileExit::fault(0x700087f2f1); };
            c.w64(0, t25);
            blk = 18; continue;
        }
        18 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087f2f5); };
            c.w64(1, t26);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087f2fc); }
            c.cmp(c.r64(1), 0x6u64, 64);
            if c.cc_g() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return c.ret(0x38, 0x700087f303);
        }
        20 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087f306); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087f30d); }
            return c.ret(0x0, 0x700087f30f);
        }
        21 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087f310); }
            c.w64(0, c.r64(2));
            let Some(t27) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700087f315); };
            c.w64(1, t27);
            return c.call(0x700087f374, 0x700087f31e, 0x700087f319);
        }
        22 => {
            blk = 18; continue;
        }
        23 => {
            return RegfileExit::fault(0x700087f320);
        }
        24 => {
            return RegfileExit::fault(0x700087f321);
        }
        25 => {
            return RegfileExit::fault(0x700087f322);
        }
        26 => {
            return RegfileExit::fault(0x700087f323);
        }
        27 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700087f324); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700087f328); };
            c.w64(3, t28);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087f336, 0x700087f331);
        }
        28 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087f336); };
            c.w64(6, t29);
            blk = 1; continue;
        }
        29 => {
            let Some(t30) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087f33f); };
            c.logic(t30 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 17; continue;
        }
        31 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700087f34b); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 17; continue; }
            blk = 32; continue;
        }
        32 => {
            blk = 21; continue;
        }
        33 => {
            return RegfileExit::jump(0x700087f360);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700087f374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087f374); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700087f378); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087f37e); };
            c.w64(8, t1);
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700087f38e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700087f392); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700087f396); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700087f39a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700087f39e); }
            blk = 2; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087f3a4); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087f3a8); };
            c.w64(2, t3);
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087f3ac); };
            c.w64(6, t4);
            blk = 2; continue;
        }
        2 => {
            c.w32(11, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700087f3bf); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x700087f3c3); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087f3cd); }
            let Some(t5) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087f3d2); };
            c.cmp(t5, 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700087f3d8); }
            blk = 8; continue;
        }
        5 => {
            c.w64(15, c.r64(8));
            let r6 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r6);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(15) as u32 as i32 as i64));
            blk = 8; continue;
        }
        6 => {
            let Some(t7) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087f3f1); };
            c.cmp(t7, 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(15, c.r64(7));
            let r8 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r8);
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700087f3fd); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            blk = 8; continue;
        }
        8 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 24; continue;
        }
        10 => {
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_ge() { blk = 24; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700087f420); }
            c.w64(0, c.r64(2));
            c.w64(3, c.r64(7));
            return c.call(0x7000870914, 0x700087f42f, 0x700087f42a);
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087f42f); };
            if !c.push64(t9) { return RegfileExit::fault(0x700087f42f); }
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087f432); };
            if !c.push64(t10) { return RegfileExit::fault(0x700087f432); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087f435); }
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087f436); };
            if !c.push64(t11) { return RegfileExit::fault(0x700087f436); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700087f439); }
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087f43d); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087f441); };
            c.w64(7, t13);
            c.w32(0, 0x3u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x700087f452, 0x700087f44d);
        }
        13 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700087f452); };
            c.cmp(t14, c.r32(0), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700087f461); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f46a); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r17);
            let Some(t18) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087f470); };
            c.logic(t18 & 0x10u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087f476); };
            c.cmp(t19, c.r32(1), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t20) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087f47f); };
            c.cmp(t20, 0x41u64, 16);
            if c.cc_e() { blk = 27; continue; }
            blk = 19; continue;
        }
        19 => {
            blk = 26; continue;
        }
        20 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t21) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087f48d); };
            c.cmp(t21, 0x0u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087f493); };
            c.w64(1, t22);
            let r23 = c.add(c.r32(1), 0x2u64, 32);
            c.w32(1, r23);
            if c.cc_o() { blk = 31; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        24 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087f4a4); };
            c.w64(0, t24);
            blk = 25; continue;
        }
        25 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087f4ae); }
            return c.ret(0x8, 0x700087f4af);
        }
        26 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087f4b2); };
            c.w64(0, t25);
            blk = 25; continue;
        }
        27 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087f4b8); };
            c.logic(t26 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 26; continue;
        }
        29 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087f4c3); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 26; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 21; continue;
        }
        31 => {
            let Some(t27) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087f4d4); };
            c.cmp(t27, 0x0u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087f4da); };
            c.w64(7, t28);
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087f4de); }
            blk = 34; continue;
        }
        33 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087f4e5); };
            c.w64(7, t29);
            let r30 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r30);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 34; continue;
        }
        34 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(1, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 1) { return RegfileExit::fault(0x700087f511); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 40; continue; }
            blk = 35; continue;
        }
        35 => {
            if c.cc_ne() { blk = 40; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(8, c.r64(1));
            let r31 = c.add(c.r32(8), c.r32(1), 32);
            c.w32(8, r31);
            if c.cc_o() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            blk = 45; continue;
        }
        40 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087f542); };
            c.w64(1, t32);
            let Some(t33) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700087f549); };
            c.w64(8, t33);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087f550); };
            c.w64(11, t34);
            let Some(t35) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700087f557); };
            c.cmp(t35, c.r64(9), 64);
            if c.cc_a() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700087f566, 0x700087f561);
        }
        42 => {
            blk = 44; continue;
        }
        43 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700087f568); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 44; continue;
        }
        44 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087f572); };
            c.w64(1, t36);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700087f579); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x700087f57c); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087f581); }
            blk = 45; continue;
        }
        45 => {
            c.w64(7, c.r64(0));
            blk = 1; continue;
        }
        46 => {
            return RegfileExit::jump(0x700087f590);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700087f594(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087f594); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700087f598); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087f599); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087f59a); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700087f59f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700087f5a3); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700087f5a7); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 89; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087f5bf); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700087f5c4); };
            c.cmp(t3, c.r32(11), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087f5d3); };
            c.cmp(t4, c.r32(11), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700087f5e2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700087f5e6); }
            let r5 = c.r32(12) | c.r32(15);
            c.w32(12, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 87; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 91; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f5fd); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087f604); };
            c.cmp(t8, 0xbau64, 16);
            if c.cc_b() { blk = 91; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700087f61e); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f628); };
            c.w32(15, t9);
            let r10 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r10);
            let Some(t11) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087f62f); };
            c.cmp(t11, 0x834u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087f638); };
            c.w32(12, t12);
            let r13 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r13);
            c.w64(15, c.r64(12));
            blk = 29; continue;
        }
        7 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 9; continue;
        }
        8 => {
            c.w32(15, 0x1u64);
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f658); };
            c.w32(15, t14);
            let r15 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r15);
            let Some(t16) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087f65f); };
            c.w32(15, t16);
            let r17 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r17);
            let Some(t18) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087f666); };
            c.w32(3, t18);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f66d); };
            c.w32(2, t19);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0xb0u64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700087f67d);
        }
        12 => {
            let Some(t20) = c.ld32(c.r64(15).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x700087f67e); };
            c.cmp(t20, c.r32(2), 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 95; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f695); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087f69c); };
            c.w32(12, t23);
            let r24 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r24);
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087f6a4); };
            c.w32(15, t25);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t26) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f6ac); };
            c.w32(3, t26);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x44u64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x700087f6b9);
        }
        16 => {
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0x117)) else { return RegfileExit::fault(0x700087f6ba); };
            c.cmp(t27, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(15), 0xaeu64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x700087f6cd);
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(12).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x700087f6ce); };
            c.cmp(t28, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(15), 0xafu64, 64);
            if c.cc_a() { blk = 94; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x700087f6e5);
        }
        22 => {
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087f6e6); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(0, c.r64(12));
            return c.call(0x70008a17f4, 0x700087f6fb, 0x700087f6f6);
        }
        25 => {
            c.w64(15, c.r64(0));
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087f6fe); };
            c.w64(6, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087f702); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087f706); };
            c.w64(11, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087f70a); };
            c.w64(0, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087f70e); };
            c.w64(9, t35);
            blk = 29; continue;
        }
        26 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(15, c.r64(12));
            blk = 29; continue;
        }
        28 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 29; continue;
        }
        29 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(0), 0x0u64, 64);
            c.w8(3, (c.cc_be() as u64));
            c.w32(3, c.r8(3));
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x700087f732); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700087f736); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 103; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t36) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087f749); };
            c.w64(2, t36);
            c.w64(1, c.r64(2));
            blk = 33; continue;
        }
        32 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087f752); };
            c.w64(1, t37);
            blk = 33; continue;
        }
        33 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087f759); };
            c.w64(2, t38);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700087f760); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 104; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t39) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f76d); };
            c.w32(7, t39);
            let r40 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r40);
            let Some(t41) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087f773); };
            c.logic(t41 & 0x2u64, 8);
            if c.cc_e() { blk = 104; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_be() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(8, c.r64(9));
            let Some(t42) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700087f786); };
            c.w64(8, t42);
            blk = 38; continue;
        }
        37 => {
            c.w64(8, c.r64(2));
            blk = 38; continue;
        }
        38 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700087f78f); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 85; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t43) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f79d); };
            c.w32(9, t43);
            let r44 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r44);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x700087f7a4); }
            let Some(t45) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087f7a8); };
            c.cmp(t45, 0x834u64, 16);
            if c.cc_ne() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(3, 0x1u64);
            blk = 42; continue;
        }
        41 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t46) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x700087f7c3); };
            c.w32(3, t46);
            c.w64(7, c.r64(3));
            let r47 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r47);
            let r48 = c.r32(7) & 0x3fu64;
            c.w32(7, r48);
            c.logic(r48, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700087f7d1); }
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_a() { blk = 85; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t49) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f7de); };
            c.w32(7, t49);
            let r50 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r50);
            let Some(t51) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700087f7e4); };
            c.w32(7, t51);
            let r52 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r52);
            let Some(t53) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087f7ea); };
            c.w32(8, t53);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x4cu64, 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            return RegfileExit::fault(0x700087f7fa);
        }
        46 => {
            let Some(t54) = c.ld32(c.r64(7).wrapping_add(0x137)) else { return RegfileExit::fault(0x700087f7fb); };
            c.w32(8, t54);
            let Some(t55) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700087f802); };
            c.cmp(t55, c.r32(8), 32);
            if c.cc_ne() { blk = 85; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700087f80c); };
            c.w64(8, t56);
            let Some(t57) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087f813); };
            c.cmp(t57, c.r32(12), 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(7, c.r64(12));
            blk = 50; continue;
        }
        49 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087f822); };
            c.w64(7, t58);
            c.w32(8, c.r64(7).wrapping_add(0x2));
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700087f82d); };
            c.w64(9, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087f831); };
            c.w64(6, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087f835); };
            c.w64(0, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087f839); };
            c.w64(11, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087f83d); };
            c.w64(1, t63);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700087f844); };
            c.w64(8, t64);
            blk = 50; continue;
        }
        50 => {
            let Some(t65) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087f84b); };
            c.cmp(c.r32(7), t65, 32);
            if c.cc_ge() { blk = 77; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t66) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f854); };
            c.cmp(t66, c.r32(9), 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087f85e); };
            c.cmp(t67, c.r32(12), 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087f868); };
            c.w32(8, t68);
            c.cmp(c.r32(8), c.r32(7), 32);
            if c.cc_le() { blk = 83; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t69) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700087f875); };
            c.w32(8, t69);
            let r70 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r70);
            let r71 = c.r32(8) & 0x3fu64;
            c.w32(8, r71);
            c.logic(r71, 32);
            let r72 = c.shr(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r72);
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t73) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087f88b); };
            c.w32(8, t73);
            let r74 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r74);
            let Some(t75) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f892); };
            c.w32(9, t75);
            let r76 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r76);
            let Some(t77) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087f899); };
            c.w32(9, t77);
            let r78 = c.sub(c.r32(9), 0x77u64, 32);
            c.w32(9, r78);
            c.cmp(c.r32(9), 0xcu64, 32);
            if c.cc_a() { blk = 79; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t79) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087f8ac); };
            c.w32(9, t79);
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_ae() { blk = 78; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t80) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087f8bc); };
            c.w32(8, t80);
            let r81 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r81);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x700087f8c4); }
            let Some(t82) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700087f8c8); };
            c.cmp(t82, c.r32(8), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 67; continue;
        }
        59 => {
            let Some(t83) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087f8da); };
            c.w32(8, t83);
            let r84 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r84);
            let Some(t85) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f8e1); };
            c.w32(9, t85);
            let Some(t86) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700087f8e5); };
            c.cmp(t86, c.r32(9), 32);
            if c.cc_ne() { blk = 82; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t87) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087f8f2); };
            c.w32(9, t87);
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_ae() { blk = 80; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) { return RegfileExit::fault(0x700087f902); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x700087f909); }
            c.w64(11, 0xfff7fffffff7ffffu64);
            let Some(t88) = c.ld64(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087f917); };
            c.cmp(t88, c.r64(11), 64);
            if c.cc_e() { blk = 65; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087f91e); };
            c.w64(8, t89);
            let Some(t90) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700087f925); };
            c.w64(9, t90);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087f92c); };
            c.w64(12, t91);
            let Some(t92) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700087f933); };
            c.cmp(t92, c.r64(11), 64);
            if c.cc_be() { blk = 97; continue; }
            blk = 63; continue;
        }
        63 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700087f93d); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 64; continue;
        }
        64 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087f944); };
            c.w64(9, t93);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700087f94b); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700087f94f); }
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087f958); };
            c.w64(11, t94);
            blk = 67; continue;
        }
        65 => {
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087f95e); };
            c.w64(11, t95);
            blk = 66; continue;
        }
        66 => {
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087f962); };
            c.w64(8, t96);
            blk = 67; continue;
        }
        67 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700087f969); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087f96b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087f96c); }
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087f96e); };
            if !c.push64(t97) { return RegfileExit::fault(0x700087f96e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x700087f971); }
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(1));
            c.w64(9, c.r64(0));
            return c.call(0x70007f2274, 0x700087f985, 0x700087f980);
        }
        68 => {
            let Some(t98) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700087f985); };
            c.cmp(t98, c.r32(0), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700087f999); };
            c.cmp(t99, c.r32(0), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t100) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087f9a6); };
            c.w32(8, t100);
            let r101 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r101);
            let Some(t102) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087f9ad); };
            c.logic(t102 & 0x10u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t103) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087f9b8); };
            c.cmp(t103, c.r32(8), 32);
            if c.cc_e() { blk = 101; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t104) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087f9c5); };
            c.cmp(t104, 0x41u64, 16);
            if c.cc_e() { blk = 99; continue; }
            blk = 74; continue;
        }
        74 => {
            blk = 81; continue;
        }
        75 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 76; continue;
        }
        76 => {
            blk = 81; continue;
        }
        77 => {
            c.w64(8, c.r64(0));
            let r105 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r105);
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087f9e6); };
            c.w64(0, t106);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087f9f0); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087f9f1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087f9f7); }
            return c.ret(0x0, 0x700087f9f9);
        }
        78 => {
            return RegfileExit::fault(0x700087f9fa);
        }
        79 => {
            return RegfileExit::fault(0x700087f9fb);
        }
        80 => {
            return RegfileExit::fault(0x700087f9fc);
        }
        81 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087f9fd); };
            c.w64(8, t107);
            let r108 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700087fa05); };
            c.w64(0, t109);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087fa0c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087fa0d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087fa13); }
            return c.ret(0x0, 0x700087fa15);
        }
        82 => {
            return RegfileExit::fault(0x700087fa16);
        }
        83 => {
            c.w64(12, c.r64(7));
            blk = 85; continue;
        }
        84 => {
            c.w64(12, c.r64(7));
            blk = 85; continue;
        }
        85 => {
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087fa1f); };
            if !c.push64(t110) { return RegfileExit::fault(0x700087fa1f); }
            c.w64(0, c.r64(11));
            c.w64(3, c.r64(1));
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087fa28); };
            c.w64(1, t111);
            c.w64(2, c.r64(11));
            c.w64(7, c.r64(12));
            return c.call(0x700087f374, 0x700087fa37, 0x700087fa32);
        }
        86 => {
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087fa37); };
            c.w64(8, t112);
            let r113 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r113);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087fa42); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087fa43); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087fa49); }
            return c.ret(0x0, 0x700087fa4b);
        }
        87 => {
            c.w32(8, 0x36u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087fa52); }
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700087fa54); };
            c.w64(8, t114);
            let Some(t115) = c.ld32(c.r64(8).wrapping_add(0x11b)) else { return RegfileExit::fault(0x700087fa5b); };
            c.w32(8, t115);
            let r116 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r116);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087fa65); }
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700087fa67); };
            c.w64(3, t117);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700087fa78, 0x700087fa73);
        }
        88 => {
            return RegfileExit::fault(0x700087fa78);
        }
        89 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700087fa79); };
            c.w64(3, t118);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700087fa8a, 0x700087fa85);
        }
        90 => {
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087fa8a); };
            c.w64(6, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087fa8e); };
            c.w64(0, t120);
            blk = 1; continue;
        }
        91 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a2054, 0x700087fa9f, 0x700087fa9a);
        }
        92 => {
            c.w64(11, c.r64(0));
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087faa2); };
            c.w64(6, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087faa6); };
            c.w64(8, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087faaa); };
            c.w64(9, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087faae); };
            c.w64(0, t124);
            blk = 4; continue;
        }
        93 => {
            let Some(t125) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700087fab7); };
            c.w32(12, t125);
            let r126 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r126);
            blk = 23; continue;
        }
        94 => {
            let Some(t127) = c.ld32(c.r64(12).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x700087fac3); };
            c.cmp(t127, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700087fad1); };
            c.w64(3, t128);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x700087fae0, 0x700087fadb);
        }
        96 => {
            c.w64(12, c.r64(0));
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087fae3); };
            c.w64(6, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087fae7); };
            c.w64(8, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087faeb); };
            c.w64(11, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087faef); };
            c.w64(9, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087faf3); };
            c.w64(0, t133);
            blk = 23; continue;
        }
        97 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x700087fb01); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x700087fb0e, 0x700087fb09);
        }
        98 => {
            c.w64(8, c.r64(0));
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087fb11); };
            c.w64(6, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087fb15); };
            c.w64(0, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087fb19); };
            c.w64(1, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700087fb1d); };
            c.w64(7, t137);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x700087fb21); }
            blk = 64; continue;
        }
        99 => {
            let Some(t138) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087fb2b); };
            c.logic(t138 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 100; continue;
        }
        100 => {
            blk = 81; continue;
        }
        101 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087fb3d); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 81; continue; }
            blk = 102; continue;
        }
        102 => {
            blk = 49; continue;
        }
        103 => {
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700087fb55); };
            c.w64(2, t139);
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t140) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087fb61); };
            c.w64(8, t140);
            c.w64(2, c.r64(8));
            blk = 106; continue;
        }
        106 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087fb6e); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700087fb70); }
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700087fb71); };
            c.w64(3, t141);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700087fb82, 0x700087fb7d);
        }
        107 => {
            return RegfileExit::fault(0x700087fb82);
        }
        108 => {
            return RegfileExit::jump(0x700087fb84);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700087fb94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087fb94); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700087fb98); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087fb99); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087fb9a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700087fb9f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087fba9); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087fbb6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087fbbd); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087fbd6); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087fbdf); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087fbe6); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700087fbed); };
            c.w64(7, t11);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087fbf7); };
            c.w32(8, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087fbfb); };
            c.cmp(t13, c.r32(8), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700087fc04); };
            c.w64(8, t14);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087fc0e); };
            c.w32(9, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087fc12); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087fc1b); }
            c.w64(0, c.r64(2));
            let Some(t17) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700087fc20); };
            c.w64(1, t17);
            return c.call(0x700087fe14, 0x700087fc29, 0x700087fc24);
        }
        10 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087fc29); };
            c.w64(1, t18);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087fc30); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x28, 0x700087fc37);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087fc3a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087fc41); }
            return c.ret(0x0, 0x700087fc43);
        }
        13 => {
            return RegfileExit::fault(0x700087fc44);
        }
        14 => {
            return RegfileExit::fault(0x700087fc45);
        }
        15 => {
            return RegfileExit::fault(0x700087fc46);
        }
        16 => {
            return RegfileExit::fault(0x700087fc47);
        }
        17 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700087fc48); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700087fc4c); };
            c.w64(3, t19);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087fc5a, 0x700087fc55);
        }
        18 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087fc5a); };
            c.w64(6, t20);
            blk = 1; continue;
        }
        19 => {
            return RegfileExit::jump(0x700087fc64);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700087fc74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087fc74); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700087fc78); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087fc79); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087fc7a); }
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700087fc7b); };
            c.cmp(c.r64(4), t0, 64);
            if c.cc_be() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x700087fc81);
        }
        2 => {
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700087fc82); };
            c.w64(3, t1);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087fc90, 0x700087fc8b);
        }
        3 => {
            blk = 1; continue;
        }
        4 => {
            return RegfileExit::jump(0x700087fc94);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700087fcb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087fcb4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700087fcb8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700087fcb9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087fcba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700087fcbf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 27; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087fcc9); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087fcd6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087fcdd); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 26; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700087fcf6); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087fd03); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087fd0a); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x700087fd15); };
            c.w64(7, t11);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087fd1f); };
            c.w32(8, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087fd23); };
            c.cmp(t13, c.r32(8), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700087fd30); };
            c.w64(8, t14);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087fd3a); };
            c.w32(9, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087fd3e); };
            c.cmp(t16, c.r32(9), 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_add(0x40)) else { return RegfileExit::fault(0x700087fd4b); };
            c.w64(9, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700087fd4f); };
            c.cmp(t18, c.r32(9), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(11, c.r32(9));
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700087fd61); };
            c.cmp(t19, c.r32(9), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t20) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087fd6a); };
            c.w32(11, t20);
            let r21 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r21);
            let Some(t22) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087fd71); };
            c.logic(t22 & 0x10u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087fd78); };
            c.cmp(t23, c.r32(11), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t24) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087fd81); };
            c.cmp(t24, 0x41u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 15; continue;
        }
        15 => {
            blk = 17; continue;
        }
        16 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_add(0x38)) else { return RegfileExit::fault(0x700087fd91); };
            c.w64(0, t25);
            blk = 18; continue;
        }
        18 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087fd95); };
            c.w64(1, t26);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087fd9c); }
            c.cmp(c.r64(1), 0x6u64, 64);
            if c.cc_g() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return c.ret(0x38, 0x700087fda3);
        }
        20 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700087fda6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700087fdad); }
            return c.ret(0x0, 0x700087fdaf);
        }
        21 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700087fdb0); }
            c.w64(0, c.r64(2));
            let Some(t27) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700087fdb5); };
            c.w64(1, t27);
            return c.call(0x700087fe14, 0x700087fdbe, 0x700087fdb9);
        }
        22 => {
            blk = 18; continue;
        }
        23 => {
            return RegfileExit::fault(0x700087fdc0);
        }
        24 => {
            return RegfileExit::fault(0x700087fdc1);
        }
        25 => {
            return RegfileExit::fault(0x700087fdc2);
        }
        26 => {
            return RegfileExit::fault(0x700087fdc3);
        }
        27 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700087fdc4); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700087fdc8); };
            c.w64(3, t28);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700087fdd6, 0x700087fdd1);
        }
        28 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087fdd6); };
            c.w64(6, t29);
            blk = 1; continue;
        }
        29 => {
            let Some(t30) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087fddf); };
            c.logic(t30 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 17; continue;
        }
        31 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700087fdeb); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 17; continue; }
            blk = 32; continue;
        }
        32 => {
            blk = 21; continue;
        }
        33 => {
            return RegfileExit::jump(0x700087fe00);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700087fe14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700087fe14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700087fe18); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087fe1e); };
            c.w64(8, t1);
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700087fe2e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700087fe32); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700087fe36); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x700087fe3a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700087fe3e); }
            blk = 2; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700087fe44); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087fe48); };
            c.w64(2, t3);
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087fe4c); };
            c.w64(6, t4);
            blk = 2; continue;
        }
        2 => {
            c.w32(11, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700087fe5f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x700087fe63); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700087fe6d); }
            let Some(t5) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087fe72); };
            c.cmp(t5, 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700087fe78); }
            blk = 8; continue;
        }
        5 => {
            c.w64(15, c.r64(8));
            let r6 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r6);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(15) as u32 as i32 as i64));
            blk = 8; continue;
        }
        6 => {
            let Some(t7) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700087fe91); };
            c.cmp(t7, 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(15, c.r64(7));
            let r8 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r8);
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700087fe9d); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            blk = 8; continue;
        }
        8 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 24; continue;
        }
        10 => {
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_ge() { blk = 24; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x700087fec0); }
            c.w64(0, c.r64(2));
            c.w64(3, c.r64(7));
            return c.call(0x7000870914, 0x700087fecf, 0x700087feca);
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700087fecf); };
            if !c.push64(t9) { return RegfileExit::fault(0x700087fecf); }
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087fed2); };
            if !c.push64(t10) { return RegfileExit::fault(0x700087fed2); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700087fed5); }
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700087fed6); };
            if !c.push64(t11) { return RegfileExit::fault(0x700087fed6); }
            c.w32(0, 0x3u64);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700087fede); };
            c.w64(7, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700087fee2); };
            c.w64(6, t13);
            return c.call(0x70007f2274, 0x700087feeb, 0x700087fee6);
        }
        13 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700087feeb); };
            c.cmp(t14, c.r32(0), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700087fefa); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700087ff03); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r17);
            let Some(t18) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x700087ff09); };
            c.logic(t18 & 0x10u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700087ff0f); };
            c.cmp(t19, c.r32(1), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t20) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700087ff18); };
            c.cmp(t20, 0x41u64, 16);
            if c.cc_e() { blk = 27; continue; }
            blk = 19; continue;
        }
        19 => {
            blk = 26; continue;
        }
        20 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t21) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087ff26); };
            c.cmp(t21, 0x0u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700087ff2c); };
            c.w64(1, t22);
            let r23 = c.add(c.r32(1), 0x2u64, 32);
            c.w32(1, r23);
            if c.cc_o() { blk = 31; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        24 => {
            c.w64(0, 0xfffffffffffffffeu64);
            blk = 25; continue;
        }
        25 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700087ff47); }
            return c.ret(0x8, 0x700087ff48);
        }
        26 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087ff4b); };
            c.w64(0, t24);
            blk = 25; continue;
        }
        27 => {
            let Some(t25) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700087ff51); };
            c.logic(t25 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 26; continue;
        }
        29 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087ff5c); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 26; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 21; continue;
        }
        31 => {
            let Some(t26) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700087ff6d); };
            c.cmp(t26, 0x0u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087ff73); };
            c.w64(0, t27);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700087ff77); }
            blk = 34; continue;
        }
        33 => {
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700087ff7e); };
            c.w64(0, t28);
            let r29 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r29);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 34; continue;
        }
        34 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(1, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 1) { return RegfileExit::fault(0x700087ffaa); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 40; continue; }
            blk = 35; continue;
        }
        35 => {
            if c.cc_ne() { blk = 40; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(8, c.r64(1));
            let r30 = c.add(c.r32(8), c.r32(1), 32);
            c.w32(8, r30);
            if c.cc_o() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            blk = 45; continue;
        }
        40 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700087ffdb); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700087ffe2); };
            c.w64(8, t32);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700087ffe9); };
            c.w64(11, t33);
            let Some(t34) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700087fff0); };
            c.cmp(t34, c.r64(9), 64);
            if c.cc_a() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700087ffff, 0x700087fffa);
        }
        42 => {
            blk = 44; continue;
        }
        43 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x7000880001); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 44; continue;
        }
        44 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088000b); };
            c.w64(1, t35);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000880012); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x7000880015); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088001a); }
            blk = 45; continue;
        }
        45 => {
            c.w64(7, c.r64(0));
            blk = 1; continue;
        }
        46 => {
            return RegfileExit::jump(0x7000880028);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000880034(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000880034); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000880038); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000880039); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088003a); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700088003f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000880043); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000880047); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 89; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088005f); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000880064); };
            c.cmp(t3, c.r32(11), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000880073); };
            c.cmp(t4, c.r32(11), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000880082); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000880086); }
            let r5 = c.r32(12) | c.r32(15);
            c.w32(12, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 87; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 91; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088009d); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008800a4); };
            c.cmp(t8, 0xbau64, 16);
            if c.cc_b() { blk = 91; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008800be); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008800c8); };
            c.w32(15, t9);
            let r10 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r10);
            let Some(t11) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008800cf); };
            c.cmp(t11, 0x834u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008800d8); };
            c.w32(12, t12);
            let r13 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r13);
            c.w64(15, c.r64(12));
            blk = 29; continue;
        }
        7 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 9; continue;
        }
        8 => {
            c.w32(15, 0x1u64);
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008800f8); };
            c.w32(15, t14);
            let r15 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r15);
            let Some(t16) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008800ff); };
            c.w32(15, t16);
            let r17 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r17);
            let Some(t18) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000880106); };
            c.w32(3, t18);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088010d); };
            c.w32(2, t19);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0xb0u64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700088011d);
        }
        12 => {
            let Some(t20) = c.ld32(c.r64(15).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x700088011e); };
            c.cmp(t20, c.r32(2), 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 95; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000880135); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088013c); };
            c.w32(12, t23);
            let r24 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r24);
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000880144); };
            c.w32(15, t25);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t26) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088014c); };
            c.w32(3, t26);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x44u64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x7000880159);
        }
        16 => {
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0x117)) else { return RegfileExit::fault(0x700088015a); };
            c.cmp(t27, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(15), 0xaeu64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x700088016d);
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(12).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x700088016e); };
            c.cmp(t28, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(15), 0xafu64, 64);
            if c.cc_a() { blk = 94; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x7000880185);
        }
        22 => {
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000880186); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(0, c.r64(12));
            return c.call(0x70008a17f4, 0x700088019b, 0x7000880196);
        }
        25 => {
            c.w64(15, c.r64(0));
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088019e); };
            c.w64(6, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008801a2); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008801a6); };
            c.w64(11, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008801aa); };
            c.w64(0, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008801ae); };
            c.w64(9, t35);
            blk = 29; continue;
        }
        26 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(15, c.r64(12));
            blk = 29; continue;
        }
        28 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 29; continue;
        }
        29 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(0), 0x0u64, 64);
            c.w8(3, (c.cc_be() as u64));
            c.w32(3, c.r8(3));
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008801d2); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008801d6); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 103; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t36) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008801e9); };
            c.w64(2, t36);
            c.w64(1, c.r64(2));
            blk = 33; continue;
        }
        32 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008801f2); };
            c.w64(1, t37);
            blk = 33; continue;
        }
        33 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008801f9); };
            c.w64(2, t38);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000880200); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 104; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t39) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088020d); };
            c.w32(7, t39);
            let r40 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r40);
            let Some(t41) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000880213); };
            c.logic(t41 & 0x2u64, 8);
            if c.cc_e() { blk = 104; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_be() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(8, c.r64(9));
            let Some(t42) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000880226); };
            c.w64(8, t42);
            blk = 38; continue;
        }
        37 => {
            c.w64(8, c.r64(2));
            blk = 38; continue;
        }
        38 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700088022f); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 85; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t43) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088023d); };
            c.w32(9, t43);
            let r44 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r44);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x7000880244); }
            let Some(t45) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000880248); };
            c.cmp(t45, 0x834u64, 16);
            if c.cc_ne() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(3, 0x1u64);
            blk = 42; continue;
        }
        41 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t46) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000880263); };
            c.w32(3, t46);
            c.w64(7, c.r64(3));
            let r47 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r47);
            let r48 = c.r32(7) & 0x3fu64;
            c.w32(7, r48);
            c.logic(r48, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x7000880271); }
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_a() { blk = 85; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t49) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088027e); };
            c.w32(7, t49);
            let r50 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r50);
            let Some(t51) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000880284); };
            c.w32(7, t51);
            let r52 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r52);
            let Some(t53) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088028a); };
            c.w32(8, t53);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x4cu64, 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            return RegfileExit::fault(0x700088029a);
        }
        46 => {
            let Some(t54) = c.ld32(c.r64(7).wrapping_add(0x137)) else { return RegfileExit::fault(0x700088029b); };
            c.w32(8, t54);
            let Some(t55) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008802a2); };
            c.cmp(t55, c.r32(8), 32);
            if c.cc_ne() { blk = 85; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70008802ac); };
            c.w64(8, t56);
            let Some(t57) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008802b3); };
            c.cmp(t57, c.r32(12), 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(7, c.r64(12));
            blk = 50; continue;
        }
        49 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008802c2); };
            c.w64(0, t58);
            c.w32(8, c.r64(0).wrapping_add(0x2));
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008802cd); };
            c.w64(9, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008802d1); };
            c.w64(6, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008802d5); };
            c.w64(0, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008802d9); };
            c.w64(11, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008802dd); };
            c.w64(1, t63);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70008802e4); };
            c.w64(8, t64);
            blk = 50; continue;
        }
        50 => {
            let Some(t65) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008802eb); };
            c.cmp(c.r32(7), t65, 32);
            if c.cc_ge() { blk = 77; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t66) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008802f4); };
            c.cmp(t66, c.r32(9), 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008802fe); };
            c.cmp(t67, c.r32(12), 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000880308); };
            c.w32(8, t68);
            c.cmp(c.r32(8), c.r32(7), 32);
            if c.cc_le() { blk = 83; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t69) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000880315); };
            c.w32(8, t69);
            let r70 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r70);
            let r71 = c.r32(8) & 0x3fu64;
            c.w32(8, r71);
            c.logic(r71, 32);
            let r72 = c.shr(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r72);
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t73) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088032b); };
            c.w32(8, t73);
            let r74 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r74);
            let Some(t75) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000880332); };
            c.w32(9, t75);
            let r76 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r76);
            let Some(t77) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000880339); };
            c.w32(9, t77);
            let r78 = c.sub(c.r32(9), 0x77u64, 32);
            c.w32(9, r78);
            c.cmp(c.r32(9), 0xcu64, 32);
            if c.cc_a() { blk = 79; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t79) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088034c); };
            c.w32(9, t79);
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_ae() { blk = 78; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t80) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088035c); };
            c.w32(8, t80);
            let r81 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r81);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000880364); }
            let Some(t82) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000880368); };
            c.cmp(t82, c.r32(8), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 67; continue;
        }
        59 => {
            let Some(t83) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088037a); };
            c.w32(8, t83);
            let r84 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r84);
            let Some(t85) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000880381); };
            c.w32(9, t85);
            let Some(t86) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000880385); };
            c.cmp(t86, c.r32(9), 32);
            if c.cc_ne() { blk = 82; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t87) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000880392); };
            c.w32(9, t87);
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_ae() { blk = 80; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) { return RegfileExit::fault(0x70008803a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008803a9); }
            c.w64(11, 0xfff7fffffff7ffffu64);
            let Some(t88) = c.ld64(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008803b7); };
            c.cmp(t88, c.r64(11), 64);
            if c.cc_e() { blk = 65; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008803be); };
            c.w64(8, t89);
            let Some(t90) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008803c5); };
            c.w64(9, t90);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008803cc); };
            c.w64(12, t91);
            let Some(t92) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008803d3); };
            c.cmp(t92, c.r64(11), 64);
            if c.cc_be() { blk = 97; continue; }
            blk = 63; continue;
        }
        63 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008803dd); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 64; continue;
        }
        64 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008803e4); };
            c.w64(9, t93);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008803eb); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008803ef); }
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008803f8); };
            c.w64(11, t94);
            blk = 67; continue;
        }
        65 => {
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008803fe); };
            c.w64(11, t95);
            blk = 66; continue;
        }
        66 => {
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000880402); };
            c.w64(8, t96);
            blk = 67; continue;
        }
        67 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000880409); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088040b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088040c); }
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088040e); };
            if !c.push64(t97) { return RegfileExit::fault(0x700088040e); }
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x7000880421, 0x700088041c);
        }
        68 => {
            let Some(t98) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000880421); };
            c.cmp(t98, c.r32(0), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000880435); };
            c.cmp(t99, c.r32(0), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t100) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000880442); };
            c.w32(8, t100);
            let r101 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r101);
            let Some(t102) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000880449); };
            c.logic(t102 & 0x10u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t103) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000880454); };
            c.cmp(t103, c.r32(8), 32);
            if c.cc_e() { blk = 101; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t104) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000880461); };
            c.cmp(t104, 0x41u64, 16);
            if c.cc_e() { blk = 99; continue; }
            blk = 74; continue;
        }
        74 => {
            blk = 81; continue;
        }
        75 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 76; continue;
        }
        76 => {
            blk = 81; continue;
        }
        77 => {
            c.w64(8, c.r64(0));
            let r105 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r105);
            c.w64(0, 0xfffffffffffffffeu64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088048c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088048d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000880493); }
            return c.ret(0x0, 0x7000880495);
        }
        78 => {
            return RegfileExit::fault(0x7000880496);
        }
        79 => {
            return RegfileExit::fault(0x7000880497);
        }
        80 => {
            return RegfileExit::fault(0x7000880498);
        }
        81 => {
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000880499); };
            c.w64(8, t106);
            let r107 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008804a1); };
            c.w64(0, t108);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008804a8); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008804a9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008804af); }
            return c.ret(0x0, 0x70008804b1);
        }
        82 => {
            return RegfileExit::fault(0x70008804b2);
        }
        83 => {
            c.w64(12, c.r64(7));
            blk = 85; continue;
        }
        84 => {
            c.w64(12, c.r64(7));
            blk = 85; continue;
        }
        85 => {
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008804bb); };
            if !c.push64(t109) { return RegfileExit::fault(0x70008804bb); }
            c.w64(0, c.r64(11));
            c.w64(3, c.r64(1));
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008804c4); };
            c.w64(1, t110);
            c.w64(2, c.r64(11));
            c.w64(7, c.r64(12));
            return c.call(0x700087fe14, 0x70008804d3, 0x70008804ce);
        }
        86 => {
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008804d3); };
            c.w64(8, t111);
            let r112 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r112);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008804de); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008804df); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008804e5); }
            return c.ret(0x0, 0x70008804e7);
        }
        87 => {
            c.w32(8, 0x36u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008804ee); }
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008804f0); };
            c.w64(8, t113);
            let Some(t114) = c.ld32(c.r64(8).wrapping_add(0x11f)) else { return RegfileExit::fault(0x70008804f7); };
            c.w32(8, t114);
            let r115 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r115);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000880501); }
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000880503); };
            c.w64(3, t116);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000880514, 0x700088050f);
        }
        88 => {
            return RegfileExit::fault(0x7000880514);
        }
        89 => {
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000880515); };
            c.w64(3, t117);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000880526, 0x7000880521);
        }
        90 => {
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000880526); };
            c.w64(6, t118);
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088052a); };
            c.w64(0, t119);
            blk = 1; continue;
        }
        91 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a2054, 0x700088053b, 0x7000880536);
        }
        92 => {
            c.w64(11, c.r64(0));
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088053e); };
            c.w64(6, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000880542); };
            c.w64(8, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000880546); };
            c.w64(9, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088054a); };
            c.w64(0, t123);
            blk = 4; continue;
        }
        93 => {
            let Some(t124) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000880553); };
            c.w32(12, t124);
            let r125 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r125);
            blk = 23; continue;
        }
        94 => {
            let Some(t126) = c.ld32(c.r64(12).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x700088055f); };
            c.cmp(t126, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700088056d); };
            c.w64(3, t127);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x700088057c, 0x7000880577);
        }
        96 => {
            c.w64(12, c.r64(0));
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088057f); };
            c.w64(6, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000880583); };
            c.w64(8, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000880587); };
            c.w64(11, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088058b); };
            c.w64(9, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088058f); };
            c.w64(0, t132);
            blk = 23; continue;
        }
        97 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x700088059d); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x70008805aa, 0x70008805a5);
        }
        98 => {
            c.w64(8, c.r64(0));
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008805ad); };
            c.w64(6, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008805b1); };
            c.w64(0, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008805b5); };
            c.w64(1, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008805b9); };
            c.w64(7, t136);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x70008805bd); }
            blk = 64; continue;
        }
        99 => {
            let Some(t137) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008805c7); };
            c.logic(t137 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 100; continue;
        }
        100 => {
            blk = 81; continue;
        }
        101 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008805d9); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 81; continue; }
            blk = 102; continue;
        }
        102 => {
            blk = 49; continue;
        }
        103 => {
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008805f1); };
            c.w64(2, t138);
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t139) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008805fd); };
            c.w64(8, t139);
            c.w64(2, c.r64(8));
            blk = 106; continue;
        }
        106 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088060a); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700088060c); }
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700088060d); };
            c.w64(3, t140);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700088061e, 0x7000880619);
        }
        107 => {
            return RegfileExit::fault(0x700088061e);
        }
        108 => {
            return RegfileExit::jump(0x7000880620);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000880634(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000880634); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000880638); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700088063e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000880642); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000880646); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700088064a); }
            blk = 2; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000880650); };
            c.w64(6, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000880654); };
            c.w64(1, t2);
            blk = 2; continue;
        }
        2 => {
            c.w32(8, c.r32(2));
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.xorpd_rr(0, 0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700088066a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700088066e); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000880678); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            blk = 19; continue;
        }
        5 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_l() { blk = 19; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x7000880691); }
            c.w64(0, c.r64(1));
            c.w64(3, c.r64(2));
            return c.call(0x7000870914, 0x70008806a0, 0x700088069b);
        }
        7 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008806a0); };
            if !c.push64(t3) { return RegfileExit::fault(0x70008806a0); }
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008806a3); };
            if !c.push64(t4) { return RegfileExit::fault(0x70008806a3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008806a6); }
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008806a7); };
            if !c.push64(t5) { return RegfileExit::fault(0x70008806a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008806aa); }
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008806ae); };
            c.w64(6, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008806b2); };
            c.w64(7, t7);
            c.w32(0, 0x3u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x70008806c3, 0x70008806be);
        }
        8 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008806c3); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008806d2); };
            c.cmp(t9, c.r32(0), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008806db); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008806e1); };
            c.logic(t12 & 0x10u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008806e7); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t14) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008806f0); };
            c.cmp(t14, 0x41u64, 16);
            if c.cc_e() { blk = 22; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 21; continue;
        }
        15 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t15) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008806fe); };
            c.cmp(t15, 0x0u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000880704); };
            c.w64(1, t16);
            let r17 = c.sub(c.r32(1), 0x2u64, 32);
            c.w32(1, r17);
            if c.cc_o() { blk = 26; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        19 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000880715); };
            c.w64(0, t18);
            blk = 20; continue;
        }
        20 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088071f); }
            return c.ret(0x0, 0x7000880720);
        }
        21 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000880721); };
            c.w64(0, t19);
            blk = 20; continue;
        }
        22 => {
            let Some(t20) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000880727); };
            c.logic(t20 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 23; continue;
        }
        23 => {
            blk = 21; continue;
        }
        24 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000880732); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 21; continue; }
            blk = 25; continue;
        }
        25 => {
            blk = 16; continue;
        }
        26 => {
            let Some(t21) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000880743); };
            c.cmp(t21, 0x0u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000880749); };
            c.w64(2, t22);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700088074d); }
            blk = 29; continue;
        }
        28 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000880754); };
            c.w64(2, t23);
            let r24 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r24);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 29; continue;
        }
        29 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x7000880780); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            if c.cc_ne() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(8, c.r64(1));
            let r25 = c.add(c.r32(8), c.r32(1), 32);
            c.w32(8, r25);
            if c.cc_o() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(2, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(2));
            blk = 40; continue;
        }
        35 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008807b1); };
            c.w64(1, t26);
            let Some(t27) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008807b8); };
            c.w64(8, t27);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008807bf); };
            c.w64(11, t28);
            let Some(t29) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008807c6); };
            c.cmp(t29, c.r64(9), 64);
            if c.cc_a() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008807d5, 0x70008807d0);
        }
        37 => {
            blk = 39; continue;
        }
        38 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008807d7); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 39; continue;
        }
        39 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008807e1); };
            c.w64(1, t30);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008807e8); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008807eb); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008807f0); }
            blk = 40; continue;
        }
        40 => {
            c.w64(2, c.r64(0));
            blk = 1; continue;
        }
        41 => {
            return RegfileExit::jump(0x7000880800);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000880814(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000880814); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000880818); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000880819); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088081a); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700088081f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000880823); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000880827); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 98; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088083f); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000880844); };
            c.cmp(t3, c.r32(11), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000880853); };
            c.cmp(t4, c.r32(11), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000880862); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000880866); }
            let r5 = c.r32(12) | c.r32(15);
            c.w32(12, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 96; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 100; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088087d); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000880884); };
            c.cmp(t8, 0xbau64, 16);
            if c.cc_b() { blk = 100; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700088089e); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008808a8); };
            c.w32(15, t9);
            let r10 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r10);
            let Some(t11) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008808af); };
            c.cmp(t11, 0x834u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008808b8); };
            c.w32(12, t12);
            let r13 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r13);
            c.w64(15, c.r64(12));
            blk = 29; continue;
        }
        7 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 9; continue;
        }
        8 => {
            c.w32(15, 0x1u64);
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008808d8); };
            c.w32(15, t14);
            let r15 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r15);
            let Some(t16) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008808df); };
            c.w32(15, t16);
            let r17 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r17);
            let Some(t18) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008808e6); };
            c.w32(3, t18);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008808ed); };
            c.w32(2, t19);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0xb0u64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008808fd);
        }
        12 => {
            let Some(t20) = c.ld32(c.r64(15).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x70008808fe); };
            c.cmp(t20, c.r32(2), 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 104; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000880915); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088091c); };
            c.w32(12, t23);
            let r24 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r24);
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000880924); };
            c.w32(15, t25);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t26) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088092c); };
            c.w32(3, t26);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x44u64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x7000880939);
        }
        16 => {
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0x117)) else { return RegfileExit::fault(0x700088093a); };
            c.cmp(t27, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(15), 0xaeu64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x700088094d);
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(12).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x700088094e); };
            c.cmp(t28, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(15), 0xafu64, 64);
            if c.cc_a() { blk = 103; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x7000880965);
        }
        22 => {
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000880966); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(0, c.r64(12));
            return c.call(0x70008a17f4, 0x700088097b, 0x7000880976);
        }
        25 => {
            c.w64(15, c.r64(0));
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088097e); };
            c.w64(6, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000880982); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000880986); };
            c.w64(11, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088098a); };
            c.w64(0, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088098e); };
            c.w64(9, t35);
            blk = 29; continue;
        }
        26 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(15, c.r64(12));
            blk = 29; continue;
        }
        28 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 29; continue;
        }
        29 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(0), 0x0u64, 64);
            c.w8(3, (c.cc_be() as u64));
            c.w32(3, c.r8(3));
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008809b2); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 117; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t36) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008809c5); };
            c.w64(2, t36);
            c.w64(1, c.r64(2));
            blk = 33; continue;
        }
        32 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008809ce); };
            c.w64(1, t37);
            blk = 33; continue;
        }
        33 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008809d5); };
            c.w64(2, t38);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008809dc); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 118; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t39) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008809e9); };
            c.w32(7, t39);
            let r40 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r40);
            let Some(t41) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008809ef); };
            c.logic(t41 & 0x2u64, 8);
            if c.cc_e() { blk = 118; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_be() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(8, c.r64(9));
            let Some(t42) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000880a02); };
            c.w64(8, t42);
            blk = 38; continue;
        }
        37 => {
            c.w64(8, c.r64(2));
            blk = 38; continue;
        }
        38 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000880a17); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t43) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000880a25); };
            c.w32(9, t43);
            let r44 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r44);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x7000880a2c); }
            let Some(t45) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000880a30); };
            c.cmp(t45, 0x834u64, 16);
            if c.cc_ne() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(3, 0x1u64);
            blk = 42; continue;
        }
        41 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t46) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000880a4b); };
            c.w32(3, t46);
            c.w64(7, c.r64(3));
            let r47 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r47);
            let r48 = c.r32(7) & 0x3fu64;
            c.w32(7, r48);
            c.logic(r48, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x7000880a59); }
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_a() { blk = 84; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t49) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000880a66); };
            c.w32(7, t49);
            let r50 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r50);
            let Some(t51) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000880a6c); };
            c.w32(7, t51);
            let r52 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r52);
            let Some(t53) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000880a72); };
            c.w32(8, t53);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x4cu64, 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            return RegfileExit::fault(0x7000880a82);
        }
        46 => {
            let Some(t54) = c.ld32(c.r64(7).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000880a83); };
            c.w32(8, t54);
            c.w32(7, c.r64(15).wrapping_sub(0x2));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t55) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000880a91); };
            c.cmp(t55, c.r32(8), 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000880a9b); };
            c.w64(8, t56);
            let Some(t57) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000880aa2); };
            c.cmp(t57, c.r32(12), 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 50; continue;
        }
        49 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000880aae); };
            c.w64(7, t58);
            c.w32(8, c.r64(7).wrapping_sub(0x2));
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000880ab9); };
            c.w64(9, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000880abd); };
            c.w64(6, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000880ac1); };
            c.w64(0, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000880ac5); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000880ac9); };
            c.w64(11, t63);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000880acd); };
            c.w64(8, t64);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 77; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t65) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000880ae0); };
            c.cmp(t65, c.r32(9), 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t66) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000880aea); };
            c.cmp(t66, c.r32(12), 32);
            if c.cc_e() { blk = 94; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t67) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000880af4); };
            c.w32(15, t67);
            c.cmp(c.r32(15), c.r32(7), 32);
            if c.cc_le() { blk = 94; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t68) = c.ld8(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000880b01); };
            c.w32(15, t68);
            let r69 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r69);
            let r70 = c.r32(15) & 0x3fu64;
            c.w32(15, r70);
            c.logic(r70, 32);
            let r71 = c.shr(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r71);
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t72) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000880b17); };
            c.w32(15, t72);
            let r73 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r73);
            let Some(t74) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000880b1e); };
            c.w32(8, t74);
            let r75 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r75);
            let Some(t76) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000880b25); };
            c.w32(8, t76);
            let r77 = c.sub(c.r32(8), 0x77u64, 32);
            c.w32(8, r77);
            c.cmp(c.r32(8), 0xcu64, 32);
            if c.cc_a() { blk = 79; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t78) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000880b38); };
            c.w32(8, t78);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_ae() { blk = 78; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t79) = c.ld32(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000880b48); };
            c.w32(8, t79);
            let r80 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r80);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x7000880b50); }
            let Some(t81) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000880b54); };
            c.cmp(t81, c.r32(8), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 67; continue;
        }
        59 => {
            let Some(t82) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000880b66); };
            c.w32(15, t82);
            let r83 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r83);
            let Some(t84) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000880b6d); };
            c.w32(8, t84);
            let Some(t85) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000880b71); };
            c.cmp(t85, c.r32(8), 32);
            if c.cc_ne() { blk = 82; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t86) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000880b7e); };
            c.w32(8, t86);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_ae() { blk = 80; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(15).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) { return RegfileExit::fault(0x7000880b8e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x7000880b95); }
            c.w64(9, 0xfff7fffffff7ffffu64);
            let Some(t87) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000880ba3); };
            c.cmp(t87, c.r64(9), 64);
            if c.cc_e() { blk = 65; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000880baa); };
            c.w64(8, t88);
            let Some(t89) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000880bb1); };
            c.w64(9, t89);
            c.w64(15, c.r64(9).wrapping_add(0xc));
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000880bb8); };
            c.w64(11, t90);
            let Some(t91) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000880bbf); };
            c.cmp(t91, c.r64(15), 64);
            if c.cc_be() { blk = 106; continue; }
            blk = 63; continue;
        }
        63 => {
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x7000880bc8); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 64; continue;
        }
        64 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000880bcf); };
            c.w64(9, t92);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000880bd6); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000880bda); }
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000880be0); };
            c.w64(9, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000880be4); };
            c.w64(11, t94);
            blk = 67; continue;
        }
        65 => {
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000880bea); };
            c.w64(9, t95);
            blk = 66; continue;
        }
        66 => {
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000880bee); };
            c.w64(8, t96);
            blk = 67; continue;
        }
        67 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000880bf5); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000880bf7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000880bf8); }
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000880bfa); };
            if !c.push64(t97) { return RegfileExit::fault(0x7000880bfa); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x7000880bfd); }
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(1));
            c.w64(15, c.r64(0));
            return c.call(0x70007f2274, 0x7000880c11, 0x7000880c0c);
        }
        68 => {
            let Some(t98) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000880c11); };
            c.cmp(t98, c.r32(0), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000880c25); };
            c.cmp(t99, c.r32(0), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t100) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000880c32); };
            c.w32(8, t100);
            let r101 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r101);
            let Some(t102) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000880c39); };
            c.logic(t102 & 0x10u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t103) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000880c44); };
            c.cmp(t103, c.r32(8), 32);
            if c.cc_e() { blk = 110; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t104) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000880c51); };
            c.cmp(t104, 0x41u64, 16);
            if c.cc_e() { blk = 108; continue; }
            blk = 74; continue;
        }
        74 => {
            blk = 81; continue;
        }
        75 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 76; continue;
        }
        76 => {
            blk = 81; continue;
        }
        77 => {
            c.w64(8, c.r64(0));
            let r105 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r105);
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000880c72); };
            c.w64(0, t106);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000880c7c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000880c7d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000880c83); }
            return c.ret(0x0, 0x7000880c85);
        }
        78 => {
            return RegfileExit::fault(0x7000880c86);
        }
        79 => {
            return RegfileExit::fault(0x7000880c87);
        }
        80 => {
            return RegfileExit::fault(0x7000880c88);
        }
        81 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000880c89); };
            c.w64(8, t107);
            let r108 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000880c91); };
            c.w64(0, t109);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000880c98); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000880c99); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000880c9f); }
            return c.ret(0x0, 0x7000880ca1);
        }
        82 => {
            return RegfileExit::fault(0x7000880ca2);
        }
        83 => {
            blk = 94; continue;
        }
        84 => {
            c.w32(8, c.r64(15).wrapping_sub(0x2));
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 94; continue;
        }
        85 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000880cb7); }
            blk = 88; continue;
        }
        87 => {
            let r110 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r110);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            blk = 88; continue;
        }
        88 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 112; continue; }
            blk = 89; continue;
        }
        89 => {
            if c.cc_ne() { blk = 112; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            c.movq_rx(12, 0);
            if let Some(sr) = c.shr(c.r64(12), 0x20u64, 64) { c.w64(12, sr); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 112; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w64(12, c.r64(9));
            let r111 = c.add(c.r32(12), c.r32(9), 32);
            c.w32(12, r111);
            if c.cc_o() { blk = 112; continue; }
            blk = 93; continue;
        }
        93 => {
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 94; continue;
        }
        94 => {
            c.w64(0, c.r64(1));
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000880d26); };
            c.w64(3, t112);
            c.w64(1, c.r64(11));
            c.w64(2, c.r64(7));
            return c.call(0x7000880634, 0x7000880d35, 0x7000880d30);
        }
        95 => {
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000880d35); };
            c.w64(8, t113);
            let r114 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r114);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000880d40); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000880d41); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000880d47); }
            return c.ret(0x0, 0x7000880d49);
        }
        96 => {
            c.w32(8, 0x36u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000880d50); }
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000880d52); };
            c.w64(8, t115);
            let Some(t116) = c.ld32(c.r64(8).wrapping_add(0x123)) else { return RegfileExit::fault(0x7000880d59); };
            c.w32(8, t116);
            let r117 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r117);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000880d63); }
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000880d65); };
            c.w64(3, t118);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000880d76, 0x7000880d71);
        }
        97 => {
            return RegfileExit::fault(0x7000880d76);
        }
        98 => {
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000880d77); };
            c.w64(3, t119);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000880d88, 0x7000880d83);
        }
        99 => {
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000880d88); };
            c.w64(6, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000880d8c); };
            c.w64(0, t121);
            blk = 1; continue;
        }
        100 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a2054, 0x7000880d9d, 0x7000880d98);
        }
        101 => {
            c.w64(11, c.r64(0));
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000880da0); };
            c.w64(6, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000880da4); };
            c.w64(8, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000880da8); };
            c.w64(9, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000880dac); };
            c.w64(0, t125);
            blk = 4; continue;
        }
        102 => {
            let Some(t126) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000880db5); };
            c.w32(12, t126);
            let r127 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r127);
            blk = 23; continue;
        }
        103 => {
            let Some(t128) = c.ld32(c.r64(12).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x7000880dc1); };
            c.cmp(t128, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000880dcf); };
            c.w64(3, t129);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x7000880dde, 0x7000880dd9);
        }
        105 => {
            c.w64(12, c.r64(0));
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000880de1); };
            c.w64(6, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000880de5); };
            c.w64(8, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000880de9); };
            c.w64(11, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000880ded); };
            c.w64(9, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000880df1); };
            c.w64(0, t134);
            blk = 23; continue;
        }
        106 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 0) { return RegfileExit::fault(0x7000880dff); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x7000880e0c, 0x7000880e07);
        }
        107 => {
            c.w64(8, c.r64(0));
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000880e0f); };
            c.w64(6, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000880e13); };
            c.w64(0, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000880e17); };
            c.w64(1, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000880e1b); };
            c.w64(7, t138);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x7000880e1f); }
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 64; continue;
        }
        108 => {
            let Some(t139) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000880e2c); };
            c.logic(t139 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 109; continue;
        }
        109 => {
            blk = 81; continue;
        }
        110 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000880e3e); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 81; continue; }
            blk = 111; continue;
        }
        111 => {
            blk = 49; continue;
        }
        112 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000880e56); };
            c.w64(9, t140);
            let Some(t141) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000880e5d); };
            c.w64(12, t141);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x7000880e65); }
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000880e6a); };
            c.w64(3, t142);
            let Some(t143) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000880e71); };
            c.cmp(t143, c.r64(15), 64);
            if c.cc_a() { blk = 115; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000880e80, 0x7000880e7b);
        }
        114 => {
            c.w64(9, c.r64(0));
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000880e83); };
            c.w64(6, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000880e87); };
            c.w64(0, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000880e8b); };
            c.w64(1, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000880e8f); };
            c.w64(8, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000880e93); };
            c.w64(11, t148);
            blk = 116; continue;
        }
        115 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x7000880e99); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 116; continue;
        }
        116 => {
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000880ea1); };
            c.w64(12, t149);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000880ea8); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x7000880eac); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000880eb1); }
            c.w64(7, c.r64(9));
            blk = 94; continue;
        }
        117 => {
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000880ebf); };
            c.w64(2, t150);
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t151) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000880ecb); };
            c.w64(8, t151);
            c.w64(2, c.r64(8));
            blk = 120; continue;
        }
        120 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000880ed8); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000880eda); }
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000880edb); };
            c.w64(3, t152);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000880eec, 0x7000880ee7);
        }
        121 => {
            return RegfileExit::fault(0x7000880eec);
        }
        122 => {
            return RegfileExit::jump(0x7000880ef0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000880ef4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000880ef4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000880ef8); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000880efe); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000880f02); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000880f06); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x7000880f0a); }
            blk = 2; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000880f10); };
            c.w64(6, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000880f14); };
            c.w64(1, t2);
            blk = 2; continue;
        }
        2 => {
            c.w32(8, c.r32(2));
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.xorpd_rr(0, 0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000880f2a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000880f2e); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000880f38); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            blk = 19; continue;
        }
        5 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_l() { blk = 19; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000880f51); }
            c.w64(0, c.r64(1));
            c.w64(3, c.r64(2));
            return c.call(0x7000870914, 0x7000880f60, 0x7000880f5b);
        }
        7 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000880f60); };
            if !c.push64(t3) { return RegfileExit::fault(0x7000880f60); }
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000880f63); };
            if !c.push64(t4) { return RegfileExit::fault(0x7000880f63); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000880f66); }
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000880f67); };
            if !c.push64(t5) { return RegfileExit::fault(0x7000880f67); }
            c.w32(0, 0x3u64);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000880f6f); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000880f73); };
            c.w64(6, t7);
            return c.call(0x70007f2274, 0x7000880f7c, 0x7000880f77);
        }
        8 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000880f7c); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000880f8b); };
            c.cmp(t9, c.r32(0), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000880f94); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000880f9a); };
            c.logic(t12 & 0x10u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000880fa0); };
            c.cmp(t13, c.r32(1), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t14) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000880fa9); };
            c.cmp(t14, 0x41u64, 16);
            if c.cc_e() { blk = 22; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 21; continue;
        }
        15 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t15) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000880fb7); };
            c.cmp(t15, 0x0u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000880fbd); };
            c.w64(1, t16);
            let r17 = c.sub(c.r32(1), 0x2u64, 32);
            c.w32(1, r17);
            if c.cc_o() { blk = 26; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        19 => {
            c.w64(0, 0xfffffffffffffffeu64);
            blk = 20; continue;
        }
        20 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000880fd8); }
            return c.ret(0x0, 0x7000880fd9);
        }
        21 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000880fda); };
            c.w64(0, t18);
            blk = 20; continue;
        }
        22 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000880fe0); };
            c.logic(t19 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 23; continue;
        }
        23 => {
            blk = 21; continue;
        }
        24 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000880feb); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 21; continue; }
            blk = 25; continue;
        }
        25 => {
            blk = 16; continue;
        }
        26 => {
            let Some(t20) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000880ffc); };
            c.cmp(t20, 0x0u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000881002); };
            c.w64(0, t21);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000881006); }
            blk = 29; continue;
        }
        28 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088100d); };
            c.w64(0, t22);
            let r23 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r23);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 29; continue;
        }
        29 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x7000881039); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            if c.cc_ne() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(8, c.r64(1));
            let r24 = c.add(c.r32(8), c.r32(1), 32);
            c.w32(8, r24);
            if c.cc_o() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(2, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(2));
            blk = 40; continue;
        }
        35 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088106a); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000881071); };
            c.w64(8, t26);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000881078); };
            c.w64(11, t27);
            let Some(t28) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700088107f); };
            c.cmp(t28, c.r64(9), 64);
            if c.cc_a() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700088108e, 0x7000881089);
        }
        37 => {
            blk = 39; continue;
        }
        38 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x7000881090); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 39; continue;
        }
        39 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088109a); };
            c.w64(1, t29);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008810a1); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008810a4); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008810a9); }
            blk = 40; continue;
        }
        40 => {
            c.w64(2, c.r64(0));
            blk = 1; continue;
        }
        41 => {
            return RegfileExit::jump(0x70008810b8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008810d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008810d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008810d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008810d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008810da); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008810df); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008810e3); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008810e7); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 98; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008810ff); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000881104); };
            c.cmp(t3, c.r32(11), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000881113); };
            c.cmp(t4, c.r32(11), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000881122); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000881126); }
            let r5 = c.r32(12) | c.r32(15);
            c.w32(12, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 96; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 100; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088113d); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000881144); };
            c.cmp(t8, 0xbau64, 16);
            if c.cc_b() { blk = 100; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700088115e); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000881168); };
            c.w32(15, t9);
            let r10 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r10);
            let Some(t11) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088116f); };
            c.cmp(t11, 0x834u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000881178); };
            c.w32(12, t12);
            let r13 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r13);
            c.w64(15, c.r64(12));
            blk = 29; continue;
        }
        7 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 9; continue;
        }
        8 => {
            c.w32(15, 0x1u64);
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000881198); };
            c.w32(15, t14);
            let r15 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r15);
            let Some(t16) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088119f); };
            c.w32(15, t16);
            let r17 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r17);
            let Some(t18) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008811a6); };
            c.w32(3, t18);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008811ad); };
            c.w32(2, t19);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0xb0u64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008811bd);
        }
        12 => {
            let Some(t20) = c.ld32(c.r64(15).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x70008811be); };
            c.cmp(t20, c.r32(2), 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 104; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008811d5); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008811dc); };
            c.w32(12, t23);
            let r24 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r24);
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008811e4); };
            c.w32(15, t25);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t26) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008811ec); };
            c.w32(3, t26);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x44u64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x70008811f9);
        }
        16 => {
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0x117)) else { return RegfileExit::fault(0x70008811fa); };
            c.cmp(t27, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(15), 0xaeu64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x700088120d);
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(12).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x700088120e); };
            c.cmp(t28, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(15), 0xafu64, 64);
            if c.cc_a() { blk = 103; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x7000881225);
        }
        22 => {
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000881226); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(0, c.r64(12));
            return c.call(0x70008a17f4, 0x700088123b, 0x7000881236);
        }
        25 => {
            c.w64(15, c.r64(0));
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088123e); };
            c.w64(6, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000881242); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000881246); };
            c.w64(11, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088124a); };
            c.w64(0, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088124e); };
            c.w64(9, t35);
            blk = 29; continue;
        }
        26 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(15, c.r64(12));
            blk = 29; continue;
        }
        28 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 29; continue;
        }
        29 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(0), 0x0u64, 64);
            c.w8(3, (c.cc_be() as u64));
            c.w32(3, c.r8(3));
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000881272); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 117; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t36) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000881285); };
            c.w64(2, t36);
            c.w64(1, c.r64(2));
            blk = 33; continue;
        }
        32 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088128e); };
            c.w64(1, t37);
            blk = 33; continue;
        }
        33 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000881295); };
            c.w64(2, t38);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700088129c); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 118; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t39) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008812a9); };
            c.w32(7, t39);
            let r40 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r40);
            let Some(t41) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008812af); };
            c.logic(t41 & 0x2u64, 8);
            if c.cc_e() { blk = 118; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_be() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(8, c.r64(9));
            let Some(t42) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008812c2); };
            c.w64(8, t42);
            blk = 38; continue;
        }
        37 => {
            c.w64(8, c.r64(2));
            blk = 38; continue;
        }
        38 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008812d7); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t43) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008812e5); };
            c.w32(9, t43);
            let r44 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r44);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008812ec); }
            let Some(t45) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008812f0); };
            c.cmp(t45, 0x834u64, 16);
            if c.cc_ne() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(3, 0x1u64);
            blk = 42; continue;
        }
        41 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t46) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x700088130b); };
            c.w32(3, t46);
            c.w64(7, c.r64(3));
            let r47 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r47);
            let r48 = c.r32(7) & 0x3fu64;
            c.w32(7, r48);
            c.logic(r48, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x7000881319); }
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_a() { blk = 84; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t49) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000881326); };
            c.w32(7, t49);
            let r50 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r50);
            let Some(t51) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088132c); };
            c.w32(7, t51);
            let r52 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r52);
            let Some(t53) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000881332); };
            c.w32(8, t53);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x4cu64, 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            return RegfileExit::fault(0x7000881342);
        }
        46 => {
            let Some(t54) = c.ld32(c.r64(7).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000881343); };
            c.w32(8, t54);
            c.w32(7, c.r64(15).wrapping_sub(0x2));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t55) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000881351); };
            c.cmp(t55, c.r32(8), 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700088135b); };
            c.w64(8, t56);
            let Some(t57) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000881362); };
            c.cmp(t57, c.r32(12), 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 50; continue;
        }
        49 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088136e); };
            c.w64(0, t58);
            c.w32(8, c.r64(0).wrapping_sub(0x2));
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000881379); };
            c.w64(9, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088137d); };
            c.w64(6, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000881381); };
            c.w64(0, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000881385); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000881389); };
            c.w64(11, t63);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700088138d); };
            c.w64(8, t64);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 77; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t65) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008813a0); };
            c.cmp(t65, c.r32(9), 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t66) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008813aa); };
            c.cmp(t66, c.r32(12), 32);
            if c.cc_e() { blk = 94; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t67) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008813b4); };
            c.w32(15, t67);
            c.cmp(c.r32(15), c.r32(7), 32);
            if c.cc_le() { blk = 94; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t68) = c.ld8(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008813c1); };
            c.w32(15, t68);
            let r69 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r69);
            let r70 = c.r32(15) & 0x3fu64;
            c.w32(15, r70);
            c.logic(r70, 32);
            let r71 = c.shr(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r71);
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t72) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008813d7); };
            c.w32(15, t72);
            let r73 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r73);
            let Some(t74) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008813de); };
            c.w32(8, t74);
            let r75 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r75);
            let Some(t76) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008813e5); };
            c.w32(8, t76);
            let r77 = c.sub(c.r32(8), 0x77u64, 32);
            c.w32(8, r77);
            c.cmp(c.r32(8), 0xcu64, 32);
            if c.cc_a() { blk = 79; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t78) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008813f8); };
            c.w32(8, t78);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_ae() { blk = 78; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t79) = c.ld32(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000881408); };
            c.w32(8, t79);
            let r80 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r80);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x7000881410); }
            let Some(t81) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000881414); };
            c.cmp(t81, c.r32(8), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 67; continue;
        }
        59 => {
            let Some(t82) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000881426); };
            c.w32(15, t82);
            let r83 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r83);
            let Some(t84) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088142d); };
            c.w32(8, t84);
            let Some(t85) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000881431); };
            c.cmp(t85, c.r32(8), 32);
            if c.cc_ne() { blk = 82; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t86) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088143e); };
            c.w32(8, t86);
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_ae() { blk = 80; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(15).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) { return RegfileExit::fault(0x700088144e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x7000881455); }
            c.w64(9, 0xfff7fffffff7ffffu64);
            let Some(t87) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000881463); };
            c.cmp(t87, c.r64(9), 64);
            if c.cc_e() { blk = 65; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088146a); };
            c.w64(8, t88);
            let Some(t89) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000881471); };
            c.w64(9, t89);
            c.w64(15, c.r64(9).wrapping_add(0xc));
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000881478); };
            c.w64(11, t90);
            let Some(t91) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700088147f); };
            c.cmp(t91, c.r64(15), 64);
            if c.cc_be() { blk = 106; continue; }
            blk = 63; continue;
        }
        63 => {
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x7000881488); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 64; continue;
        }
        64 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088148f); };
            c.w64(9, t92);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000881496); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088149a); }
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008814a0); };
            c.w64(9, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008814a4); };
            c.w64(11, t94);
            blk = 67; continue;
        }
        65 => {
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008814aa); };
            c.w64(9, t95);
            blk = 66; continue;
        }
        66 => {
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008814ae); };
            c.w64(8, t96);
            blk = 67; continue;
        }
        67 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008814b5); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008814b7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008814b8); }
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008814ba); };
            if !c.push64(t97) { return RegfileExit::fault(0x70008814ba); }
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x70008814cd, 0x70008814c8);
        }
        68 => {
            let Some(t98) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008814cd); };
            c.cmp(t98, c.r32(0), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008814e1); };
            c.cmp(t99, c.r32(0), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t100) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008814ee); };
            c.w32(8, t100);
            let r101 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r101);
            let Some(t102) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008814f5); };
            c.logic(t102 & 0x10u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t103) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000881500); };
            c.cmp(t103, c.r32(8), 32);
            if c.cc_e() { blk = 110; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t104) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088150d); };
            c.cmp(t104, 0x41u64, 16);
            if c.cc_e() { blk = 108; continue; }
            blk = 74; continue;
        }
        74 => {
            blk = 81; continue;
        }
        75 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 76; continue;
        }
        76 => {
            blk = 81; continue;
        }
        77 => {
            c.w64(8, c.r64(0));
            let r105 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r105);
            c.w64(0, 0xfffffffffffffffeu64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000881538); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000881539); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088153f); }
            return c.ret(0x0, 0x7000881541);
        }
        78 => {
            return RegfileExit::fault(0x7000881542);
        }
        79 => {
            return RegfileExit::fault(0x7000881543);
        }
        80 => {
            return RegfileExit::fault(0x7000881544);
        }
        81 => {
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000881545); };
            c.w64(8, t106);
            let r107 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088154d); };
            c.w64(0, t108);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000881554); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000881555); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700088155b); }
            return c.ret(0x0, 0x700088155d);
        }
        82 => {
            return RegfileExit::fault(0x700088155e);
        }
        83 => {
            blk = 94; continue;
        }
        84 => {
            c.w32(8, c.r64(15).wrapping_sub(0x2));
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 94; continue;
        }
        85 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000881573); }
            blk = 88; continue;
        }
        87 => {
            let r109 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r109);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            blk = 88; continue;
        }
        88 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 112; continue; }
            blk = 89; continue;
        }
        89 => {
            if c.cc_ne() { blk = 112; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            c.movq_rx(12, 0);
            if let Some(sr) = c.shr(c.r64(12), 0x20u64, 64) { c.w64(12, sr); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 112; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w64(12, c.r64(9));
            let r110 = c.add(c.r32(12), c.r32(9), 32);
            c.w32(12, r110);
            if c.cc_o() { blk = 112; continue; }
            blk = 93; continue;
        }
        93 => {
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 94; continue;
        }
        94 => {
            c.w64(0, c.r64(1));
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008815e2); };
            c.w64(3, t111);
            c.w64(1, c.r64(11));
            c.w64(2, c.r64(7));
            return c.call(0x7000880ef4, 0x70008815f1, 0x70008815ec);
        }
        95 => {
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008815f1); };
            c.w64(8, t112);
            let r113 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r113);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008815fc); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008815fd); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000881603); }
            return c.ret(0x0, 0x7000881605);
        }
        96 => {
            c.w32(8, 0x36u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088160c); }
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700088160e); };
            c.w64(8, t114);
            let Some(t115) = c.ld32(c.r64(8).wrapping_add(0x127)) else { return RegfileExit::fault(0x7000881615); };
            c.w32(8, t115);
            let r116 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r116);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088161f); }
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000881621); };
            c.w64(3, t117);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000881632, 0x700088162d);
        }
        97 => {
            return RegfileExit::fault(0x7000881632);
        }
        98 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000881633); };
            c.w64(3, t118);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000881644, 0x700088163f);
        }
        99 => {
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000881644); };
            c.w64(6, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000881648); };
            c.w64(0, t120);
            blk = 1; continue;
        }
        100 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a2054, 0x7000881659, 0x7000881654);
        }
        101 => {
            c.w64(11, c.r64(0));
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088165c); };
            c.w64(6, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000881660); };
            c.w64(8, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000881664); };
            c.w64(9, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000881668); };
            c.w64(0, t124);
            blk = 4; continue;
        }
        102 => {
            let Some(t125) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000881671); };
            c.w32(12, t125);
            let r126 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r126);
            blk = 23; continue;
        }
        103 => {
            let Some(t127) = c.ld32(c.r64(12).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x700088167d); };
            c.cmp(t127, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700088168b); };
            c.w64(3, t128);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x700088169a, 0x7000881695);
        }
        105 => {
            c.w64(12, c.r64(0));
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088169d); };
            c.w64(6, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008816a1); };
            c.w64(8, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008816a5); };
            c.w64(11, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008816a9); };
            c.w64(9, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008816ad); };
            c.w64(0, t133);
            blk = 23; continue;
        }
        106 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 0) { return RegfileExit::fault(0x70008816bb); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x70008816c8, 0x70008816c3);
        }
        107 => {
            c.w64(8, c.r64(0));
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008816cb); };
            c.w64(6, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008816cf); };
            c.w64(0, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008816d3); };
            c.w64(1, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008816d7); };
            c.w64(7, t137);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x70008816db); }
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 64; continue;
        }
        108 => {
            let Some(t138) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008816e8); };
            c.logic(t138 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 109; continue;
        }
        109 => {
            blk = 81; continue;
        }
        110 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008816fa); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 81; continue; }
            blk = 111; continue;
        }
        111 => {
            blk = 49; continue;
        }
        112 => {
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000881712); };
            c.w64(9, t139);
            let Some(t140) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000881719); };
            c.w64(12, t140);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x7000881721); }
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000881726); };
            c.w64(3, t141);
            let Some(t142) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700088172d); };
            c.cmp(t142, c.r64(15), 64);
            if c.cc_a() { blk = 115; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700088173c, 0x7000881737);
        }
        114 => {
            c.w64(9, c.r64(0));
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088173f); };
            c.w64(6, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000881743); };
            c.w64(0, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000881747); };
            c.w64(1, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088174b); };
            c.w64(8, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088174f); };
            c.w64(11, t147);
            blk = 116; continue;
        }
        115 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x7000881755); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 116; continue;
        }
        116 => {
            let Some(t148) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088175d); };
            c.w64(12, t148);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000881764); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x7000881768); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088176d); }
            c.w64(7, c.r64(9));
            blk = 94; continue;
        }
        117 => {
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088177b); };
            c.w64(2, t149);
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t150) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000881787); };
            c.w64(8, t150);
            c.w64(2, c.r64(8));
            blk = 120; continue;
        }
        120 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000881794); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000881796); }
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000881797); };
            c.w64(3, t151);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008817a8, 0x70008817a3);
        }
        121 => {
            return RegfileExit::fault(0x70008817a8);
        }
        122 => {
            return RegfileExit::jump(0x70008817ac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008817b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008817b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008817b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008817b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008817ba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008817bf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008817c9); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008817d7); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008817de); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008817f7); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000881804); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088180b); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x7000881812); };
            c.w64(8, t11);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088181c); };
            c.w32(9, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000881820); };
            c.cmp(t13, c.r32(9), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x7000881829); };
            c.w64(9, t14);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000881833); };
            c.w32(11, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000881837); };
            c.cmp(t16, c.r32(11), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000881840); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000881842); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000881844); };
            c.w64(2, t17);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700088184b); }
            c.w64(0, c.r64(7));
            let Some(t18) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088184f); };
            c.w64(1, t18);
            return c.call(0x70008819b4, 0x7000881858, 0x7000881853);
        }
        10 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000881858); };
            c.w64(1, t19);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088185f); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x28, 0x7000881866);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000881869); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000881870); }
            return c.ret(0x0, 0x7000881872);
        }
        13 => {
            return RegfileExit::fault(0x7000881873);
        }
        14 => {
            return RegfileExit::fault(0x7000881874);
        }
        15 => {
            return RegfileExit::fault(0x7000881875);
        }
        16 => {
            return RegfileExit::fault(0x7000881876);
        }
        17 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000881877); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700088187b); };
            c.w64(3, t20);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000881889, 0x7000881884);
        }
        18 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000881889); };
            c.w64(6, t21);
            blk = 1; continue;
        }
        19 => {
            return RegfileExit::jump(0x7000881894);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008818b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008818b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008818b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008818b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008818ba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008818bf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008818c9); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008818d7); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008818de); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008818f7); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000881904); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088190b); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x7000881912); };
            c.w64(8, t11);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088191c); };
            c.w32(9, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000881920); };
            c.cmp(t13, c.r32(9), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x7000881929); };
            c.w64(9, t14);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000881933); };
            c.w32(11, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000881937); };
            c.cmp(t16, c.r32(11), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000881940); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000881942); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000881944); };
            c.w64(2, t17);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700088194b); }
            c.w64(0, c.r64(7));
            let Some(t18) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088194f); };
            c.w64(1, t18);
            return c.call(0x70008819b4, 0x7000881958, 0x7000881953);
        }
        10 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000881958); };
            c.w64(1, t19);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088195f); }
            c.cmp(c.r64(1), 0x5u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x30, 0x7000881966);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000881969); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000881970); }
            return c.ret(0x0, 0x7000881972);
        }
        13 => {
            return RegfileExit::fault(0x7000881973);
        }
        14 => {
            return RegfileExit::fault(0x7000881974);
        }
        15 => {
            return RegfileExit::fault(0x7000881975);
        }
        16 => {
            return RegfileExit::fault(0x7000881976);
        }
        17 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000881977); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700088197b); };
            c.w64(3, t20);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000881989, 0x7000881984);
        }
        18 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000881989); };
            c.w64(6, t21);
            blk = 1; continue;
        }
        19 => {
            return RegfileExit::jump(0x7000881994);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008819b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008819b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008819b8); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008819be); };
            c.w64(8, t1);
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008819ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008819d2); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008819d6); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70008819da); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x70008819de); }
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008819e2); };
            c.w64(0, t2);
            blk = 2; continue;
        }
        1 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008819e8); };
            c.w64(8, t3);
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008819ec); };
            c.w64(7, t4);
            blk = 2; continue;
        }
        2 => {
            c.w32(15, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x70008819fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70008819ff); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000881a08); }
            let Some(t5) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000881a0d); };
            c.cmp(t5, 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000881a13); }
            blk = 8; continue;
        }
        5 => {
            c.w64(15, c.r64(8));
            let r6 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r6);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(15) as u32 as i32 as i64));
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000881a2a); };
            c.w64(15, t7);
            blk = 8; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000881a30); };
            c.cmp(t8, 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(9, c.r64(0));
            let r9 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r9);
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000881a3c); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 8; continue;
        }
        8 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000881a4b); };
            c.w64(9, t10);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 12; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 250; continue;
        }
        10 => {
            c.cmp(c.r32(0), c.r32(8), 32);
            if c.cc_ge() { blk = 250; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000881a63); };
            c.w64(9, t11);
            blk = 12; continue;
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000881a67); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 274; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000881a75); };
            c.w32(8, t12);
            let r13 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r13);
            let Some(t14) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000881a7c); };
            c.w32(9, t14);
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_b() { blk = 274; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r16(9), 0xbcu64, 16);
            if c.cc_e() { blk = 267; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000881aa2); };
            c.w32(15, t15);
            let r16 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r16);
            let Some(t17) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000881aa9); };
            c.w32(15, t17);
            c.cmp(c.r16(15), 0x42u64, 16);
            if c.cc_ne() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000881ab5); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(3) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            if c.cc_ne() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(2, 0x1fffffffffffffu64);
            let r18 = c.add(c.r64(2), c.r64(3), 64);
            c.w64(2, r18);
            c.w64(1, 0x3ffffffffffffeu64);
            c.cmp(c.r64(2), c.r64(1), 64);
            if c.cc_be() { blk = 108; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000881af0); };
            c.w64(2, t19);
            blk = 21; continue;
        }
        21 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000881af4); };
            c.w64(1, t20);
            c.cmp(c.r16(15), 0x40u64, 16);
            if c.cc_e() { blk = 31; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r16(15), 0x40u64, 16);
            if c.cc_ae() { blk = 253; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000881b0a); };
            c.w32(3, t21);
            c.logic(c.r32(3) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 24; continue;
        }
        24 => {
            c.logic(c.r8(3) & 0x2u64, 8);
            if c.cc_e() { blk = 274; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(15), 0x25u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r16(15), 0x2du64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 274; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(15, c.r64(0));
            blk = 32; continue;
        }
        29 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000881b3b); };
            c.w32(15, t22);
            let r23 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r23);
            blk = 32; continue;
        }
        30 => {
            if let Some(sr) = c.shr(c.r64(3), 0x2u64, 64) { c.w64(3, sr); }
            let r24 = c.r64(3) & 0xffffffu64;
            c.w64(3, r24);
            c.logic(r24, 64);
            blk = 109; continue;
        }
        31 => {
            c.w64(15, c.r64(0));
            blk = 32; continue;
        }
        32 => {
            c.w64(3, c.r64(7));
            blk = 34; continue;
        }
        33 => {
            let Some(t25) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000881b5c); };
            c.w32(8, t25);
            let r26 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r26);
            let Some(t27) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000881b63); };
            c.w32(9, t27);
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(9), 0x410u64, 32);
            if c.cc_le() { blk = 61; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000881b75); };
            c.w32(2, t28);
            c.logic(c.r32(2) & 0x200000u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000881b85); };
            c.w32(3, t29);
            let r30 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r30);
            c.logic(c.r32(2) & 0xffc00u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 37; continue;
        }
        37 => {
            let r31 = c.shr(c.r32(2), 0xau64, 32).unwrap_or(c.r32(2));
            c.w32(2, r31);
            let r32 = c.r32(2) & 0x3ffu64;
            c.w32(2, r32);
            c.logic(r32, 32);
            c.cmp(c.r32(2), 0x20u64, 32);
            if c.cc_be() { blk = 50; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t33) = c.ld16(c.r64(3).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000881baa); };
            c.w32(1, (t33 as u16 as i16 as i64 as u64));
            let Some(t34) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000881bae); };
            c.w32(6, t34);
            let r35 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r35);
            let r36 = c.sub(c.r32(1), 0x1u64, 32);
            c.w32(1, r36);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x7000881bb8); }
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 40; continue;
        }
        39 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000881bc0); }
            blk = 40; continue;
        }
        40 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000881bc4); };
            c.w64(0, t37);
            let r38 = c.sub(c.r32(0), c.r32(7), 32);
            c.w32(0, r38);
            let r39 = c.shr(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r39);
            let r40 = c.add(c.r32(0), c.r32(7), 32);
            c.w32(0, r40);
            c.w32(12, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t41) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000881bd5); };
            c.w32(12, t41);
            let r42 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r42);
            let r43 = c.shr(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r43);
            let r44 = c.r32(12) & 0x3ffu64;
            c.w32(12, r44);
            c.logic(r44, 32);
            c.w32(12, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t45) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000881bef); };
            c.w32(12, t45);
            let r46 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r46);
            let Some(t47) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000881bf7); };
            c.w32(12, t47);
            let r48 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r48);
            c.cmp(c.r32(6), c.r32(12), 32);
            if c.cc_be() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w32(7, c.r64(0).wrapping_add(0x1));
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000881c08); };
            c.w64(0, t49);
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(7), c.r32(0), 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 43; continue;
        }
        43 => {
            blk = 45; continue;
        }
        44 => {
            let r50 = c.add(c.r32(7), 0x1u64, 32);
            c.w32(7, r50);
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_l() { blk = 72; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(12, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t51) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000881c24); };
            c.w32(12, t51);
            let r52 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r52);
            let r53 = c.shr(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r53);
            let r54 = c.r32(12) & 0x3ffu64;
            c.w32(12, r54);
            c.logic(r54, 32);
            c.w32(0, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t55) = c.ld32(c.r64(3).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000881c3e); };
            c.w32(0, t55);
            let r56 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r56);
            let Some(t57) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000881c45); };
            c.w32(11, t57);
            let r58 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r58);
            c.cmp(c.r32(6), c.r32(11), 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(0), c.r32(15), 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(2), c.r32(12), 32);
            if c.cc_be() { blk = 72; continue; }
            blk = 49; continue;
        }
        49 => {
            blk = 242; continue;
        }
        50 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            let r59 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(11, r59);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 72; continue; }
            blk = 51; continue;
        }
        51 => {
            let r60 = c.sub(c.r64(11), 0x3u64, 64);
            c.w64(11, r60);
            let Some(t61) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000881c7e); };
            c.cmp(t61, c.r32(15), 32);
            if c.cc_e() { blk = 242; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_ne() { blk = 51; continue; }
            blk = 53; continue;
        }
        53 => {
            blk = 72; continue;
        }
        54 => {
            let Some(t62) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000881c94); };
            c.w32(11, t62);
            let r63 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r63);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000881ca1); };
            c.w64(11, t64);
            blk = 56; continue;
        }
        56 => {
            let Some(t65) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000881ca8); };
            c.w32(12, t65);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r66 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r66);
            let Some(t67) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000881cb6); };
            c.w32(0, t67);
            let r68 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r68);
            let r69 = c.r64(0) & c.r64(12);
            c.w64(0, r69);
            c.logic(r69, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 58; continue;
        }
        57 => {
            let r70 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r70);
            let r71 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r71);
            c.w64(2, c.r64(12));
            let r72 = c.r64(2) & c.r64(0);
            c.w64(2, r72);
            c.logic(r72, 64);
            c.w64(0, c.r64(2));
            blk = 58; continue;
        }
        58 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t73) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000881cd9); };
            c.w32(2, t73);
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000881cde); };
            c.cmp(t74, c.r32(2), 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r32(2), c.r32(15), 32);
            if c.cc_e() { blk = 242; continue; }
            blk = 60; continue;
        }
        60 => {
            blk = 57; continue;
        }
        61 => {
            c.cmp(c.r32(9), 0xbdu64, 32);
            if c.cc_ne() { blk = 261; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t75) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000881d03); };
            c.logic(t75 & 0x24u64, 8);
            if c.cc_ne() { blk = 266; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t76) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000881d0e); };
            c.w32(11, t76);
            let r77 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r77);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000881d1b); };
            c.w64(11, t78);
            blk = 65; continue;
        }
        65 => {
            let Some(t79) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000881d22); };
            c.w32(12, t79);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r80 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r80);
            let Some(t81) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000881d30); };
            c.w32(0, t81);
            let r82 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r82);
            let r83 = c.r64(0) & c.r64(12);
            c.w64(0, r83);
            c.logic(r83, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 67; continue;
        }
        66 => {
            let r84 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r84);
            let r85 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r85);
            c.w64(2, c.r64(12));
            let r86 = c.r64(2) & c.r64(0);
            c.w64(2, r86);
            c.logic(r86, 64);
            c.w64(0, c.r64(2));
            blk = 67; continue;
        }
        67 => {
            c.w64(2, c.r64(0).wrapping_add(0x5));
            let Some(t87) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000881d52); };
            c.w32(1, t87);
            let r88 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r88);
            let Some(t89) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000881d5a); };
            c.cmp(t89, c.r32(1), 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t90) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000881d63); };
            c.cmp(t90, c.r32(1), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t91) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000881d6c); };
            c.cmp(t91, c.r32(15), 32);
            if c.cc_ne() { blk = 66; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t92) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000881d72); };
            c.w32(11, t92);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_ae() { blk = 290; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t93) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000881d85); };
            c.w32(11, t93);
            let Some(t94) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000881d89); };
            c.cmp(t94, c.r32(11), 32);
            if c.cc_ne() { blk = 242; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(9), 0x82eu64, 32);
            if c.cc_ne() { blk = 103; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t95) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000881da3); };
            c.w32(9, t95);
            let r96 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r96);
            let Some(t97) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000881daa); };
            c.w32(9, t97);
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_ae() { blk = 103; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t98) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000881dba); };
            c.w32(11, t98);
            let r99 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r99);
            c.w32(12, c.r32(11));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 75; continue;
        }
        75 => {
            c.cmp(c.r32(11), 0x30u64, 32);
            if c.cc_g() { blk = 103; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w32(11, c.r16(9));
            c.logic(c.r8(9) & 0x7u64, 8);
            if c.cc_e() { blk = 92; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(15));
            c.w64(9, c.r64(6));
            blk = 78; continue;
        }
        78 => {
            c.w64(0, c.r64(11));
            let r100 = c.r32(0) & 0x7u64;
            c.w32(0, r100);
            c.logic(r100, 32);
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_l() { blk = 83; continue; }
            blk = 79; continue;
        }
        79 => {
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(0), 0x5u64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 82; continue;
        }
        82 => {
            blk = 255; continue;
        }
        83 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 84; continue;
        }
        84 => {
            c.cmp(c.r32(0), 0x1u64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            blk = 255; continue;
        }
        86 => {
            let Some(t101) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000881e20); };
            c.w32(11, t101);
            let Some(t102) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000881e25); };
            c.cmp(t102, c.r32(11), 32);
            if c.cc_ne() { blk = 256; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t103) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000881e32); };
            c.w32(12, t103);
            let r104 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r104);
            let Some(t105) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000881e3a); };
            c.w32(11, t105);
            let r106 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r106);
            let Some(t107) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000881e42); };
            c.w32(11, t107);
            blk = 78; continue;
        }
        88 => {
            let Some(t108) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000881e49); };
            c.w32(11, t108);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r109 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r109);
            let Some(t110) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000881e57); };
            c.w32(12, t110);
            let r111 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r111);
            let Some(t112) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000881e5f); };
            c.w32(11, t112);
            let r113 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r113);
            let Some(t114) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000881e67); };
            c.w32(11, t114);
            blk = 78; continue;
        }
        89 => {
            let Some(t115) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000881e6e); };
            c.w32(12, t115);
            let r116 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r116);
            let Some(t117) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000881e76); };
            c.w32(11, t117);
            let r118 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r118);
            let Some(t119) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000881e7e); };
            c.w32(11, t119);
            blk = 78; continue;
        }
        90 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 93; continue;
        }
        91 => {
            c.w32(0, 0x1u64);
            blk = 93; continue;
        }
        92 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(15));
            c.w64(9, c.r64(6));
            blk = 93; continue;
        }
        93 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 257; continue; }
            blk = 94; continue;
        }
        94 => {
            let r120 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r120);
            blk = 95; continue;
        }
        95 => {
            let r121 = c.r32(11) & 0x8u64;
            c.w32(11, r121);
            c.logic(r121, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t122) = c.ld16(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(2))) else { return RegfileExit::fault(0x7000881eb4); };
            c.w32(9, t122);
            blk = 98; continue;
        }
        97 => {
            let Some(t123) = c.ld8(c.r64(12).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000881ebb); };
            c.w32(9, t123);
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r32(9), 0x2du64, 32);
            if c.cc_e() { blk = 265; continue; }
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(9), 0x49u64, 32);
            if c.cc_e() { blk = 264; continue; }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r32(9), 0x4eu64, 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(9), 0x30u64, 32);
            if c.cc_b() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            c.cmp(c.r32(9), 0x39u64, 32);
            if c.cc_be() { blk = 262; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000881eee); };
            c.w64(0, t124);
            let Some(t125) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000881ef2); };
            c.w32(9, t125);
            let Some(t126) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000881ef6); };
            c.cmp(t126, c.r32(9), 32);
            if c.cc_ne() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t127) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000881eff); };
            c.logic(t127 & 0x1u64, 8);
            if c.cc_ne() { blk = 243; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t128) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000881f09); };
            c.w32(3, t128);
            let r129 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r129);
            let Some(t130) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000881f10); };
            c.cmp(t130, c.r32(3), 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 106; continue;
        }
        106 => {
            blk = 33; continue;
        }
        107 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(3, c.r64(11));
            blk = 108; continue;
        }
        108 => {
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000881f2b); };
            c.w64(1, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000881f2f); };
            c.w64(2, t132);
            blk = 109; continue;
        }
        109 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000881f33); }
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(7));
            blk = 111; continue;
        }
        110 => {
            let Some(t133) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000881f42); };
            c.w32(9, t133);
            let r134 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r134);
            let Some(t135) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000881f49); };
            c.w32(11, t135);
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r32(11), 0x410u64, 32);
            if c.cc_le() { blk = 274; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t136) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000881f5b); };
            c.w32(11, t136);
            let r137 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r137);
            let r138 = c.r32(11) & 0x3fu64;
            c.w32(11, r138);
            c.logic(r138, 32);
            c.cmp(c.r32(11), 0x15u64, 32);
            if c.cc_l() { blk = 147; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(11), 0x1eu64, 32);
            if c.cc_l() { blk = 131; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r32(11), 0x23u64, 32);
            if c.cc_l() { blk = 123; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(11), 0x25u64, 32);
            if c.cc_l() { blk = 120; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(11), 0x25u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r32(11), 0x26u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r32(11), 0x28u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 119; continue;
        }
        119 => {
            blk = 274; continue;
        }
        120 => {
            c.cmp(c.r32(11), 0x23u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r32(11), 0x24u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 122; continue;
        }
        122 => {
            blk = 274; continue;
        }
        123 => {
            c.cmp(c.r32(11), 0x20u64, 32);
            if c.cc_l() { blk = 128; continue; }
            blk = 124; continue;
        }
        124 => {
            c.cmp(c.r32(11), 0x20u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 125; continue;
        }
        125 => {
            c.cmp(c.r32(11), 0x21u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 126; continue;
        }
        126 => {
            c.cmp(c.r32(11), 0x22u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 127; continue;
        }
        127 => {
            blk = 274; continue;
        }
        128 => {
            c.cmp(c.r32(11), 0x1eu64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 129; continue;
        }
        129 => {
            c.cmp(c.r32(11), 0x1fu64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 130; continue;
        }
        130 => {
            blk = 274; continue;
        }
        131 => {
            c.cmp(c.r32(11), 0x19u64, 32);
            if c.cc_l() { blk = 140; continue; }
            blk = 132; continue;
        }
        132 => {
            c.cmp(c.r32(11), 0x1bu64, 32);
            if c.cc_l() { blk = 137; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(11), 0x1bu64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 134; continue;
        }
        134 => {
            c.cmp(c.r32(11), 0x1cu64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 135; continue;
        }
        135 => {
            c.cmp(c.r32(11), 0x1du64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 136; continue;
        }
        136 => {
            blk = 274; continue;
        }
        137 => {
            c.cmp(c.r32(11), 0x19u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 138; continue;
        }
        138 => {
            c.cmp(c.r32(11), 0x1au64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 139; continue;
        }
        139 => {
            blk = 274; continue;
        }
        140 => {
            c.cmp(c.r32(11), 0x17u64, 32);
            if c.cc_l() { blk = 144; continue; }
            blk = 141; continue;
        }
        141 => {
            c.cmp(c.r32(11), 0x17u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 142; continue;
        }
        142 => {
            c.cmp(c.r32(11), 0x18u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 143; continue;
        }
        143 => {
            blk = 274; continue;
        }
        144 => {
            c.cmp(c.r32(11), 0x15u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 145; continue;
        }
        145 => {
            c.cmp(c.r32(11), 0x16u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 146; continue;
        }
        146 => {
            blk = 274; continue;
        }
        147 => {
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_l() { blk = 165; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r32(11), 0x10u64, 32);
            if c.cc_l() { blk = 157; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r32(11), 0x12u64, 32);
            if c.cc_l() { blk = 154; continue; }
            blk = 150; continue;
        }
        150 => {
            c.cmp(c.r32(11), 0x12u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 151; continue;
        }
        151 => {
            c.cmp(c.r32(11), 0x13u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 152; continue;
        }
        152 => {
            c.cmp(c.r32(11), 0x14u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 153; continue;
        }
        153 => {
            blk = 274; continue;
        }
        154 => {
            c.cmp(c.r32(11), 0x10u64, 32);
            if c.cc_e() { blk = 191; continue; }
            blk = 155; continue;
        }
        155 => {
            c.cmp(c.r32(11), 0x11u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 156; continue;
        }
        156 => {
            blk = 274; continue;
        }
        157 => {
            c.cmp(c.r32(11), 0xbu64, 32);
            if c.cc_l() { blk = 162; continue; }
            blk = 158; continue;
        }
        158 => {
            c.cmp(c.r32(11), 0xbu64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 159; continue;
        }
        159 => {
            c.cmp(c.r32(11), 0xcu64, 32);
            if c.cc_e() { blk = 192; continue; }
            blk = 160; continue;
        }
        160 => {
            c.cmp(c.r32(11), 0xfu64, 32);
            if c.cc_e() { blk = 185; continue; }
            blk = 161; continue;
        }
        161 => {
            blk = 274; continue;
        }
        162 => {
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 163; continue;
        }
        163 => {
            c.cmp(c.r32(11), 0xau64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 164; continue;
        }
        164 => {
            blk = 274; continue;
        }
        165 => {
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_l() { blk = 174; continue; }
            blk = 166; continue;
        }
        166 => {
            c.cmp(c.r32(11), 0x6u64, 32);
            if c.cc_l() { blk = 171; continue; }
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r32(11), 0x6u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 168; continue;
        }
        168 => {
            c.cmp(c.r32(11), 0x7u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 169; continue;
        }
        169 => {
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 170; continue;
        }
        170 => {
            blk = 274; continue;
        }
        171 => {
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 172; continue;
        }
        172 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 173; continue;
        }
        173 => {
            blk = 274; continue;
        }
        174 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_l() { blk = 178; continue; }
            blk = 175; continue;
        }
        175 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 176; continue;
        }
        176 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 177; continue;
        }
        177 => {
            blk = 274; continue;
        }
        178 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 179; continue;
        }
        179 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 180; continue;
        }
        180 => {
            blk = 274; continue;
        }
        181 => {
            blk = 186; continue;
        }
        182 => {
            let Some(t139) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000882184); };
            c.w32(8, t139);
            let r140 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r140);
            let Some(t141) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088218b); };
            c.w32(11, t141);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_ae() { blk = 187; continue; }
            blk = 183; continue;
        }
        183 => {
            c.w64(11, 0xfff7fffffff7ffffu64);
            let Some(t142) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008821a4); };
            c.cmp(t142, c.r64(11), 64);
            if c.cc_e() { blk = 205; continue; }
            blk = 184; continue;
        }
        184 => {
            blk = 242; continue;
        }
        185 => {
            let Some(t143) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008821b4); };
            c.w32(11, t143);
            let r144 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r144);
            let Some(t145) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008821bb); };
            c.w32(11, t145);
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_b() { blk = 242; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t146) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008821c8); };
            c.w32(8, t146);
            let r147 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r147);
            let Some(t148) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008821cf); };
            c.w32(11, t148);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_b() { blk = 189; continue; }
            blk = 187; continue;
        }
        187 => {
            c.w32(8, 0xfffffffeu64);
            c.cmp(c.r64(8), c.r64(3), 64);
            if c.cc_b() { blk = 274; continue; }
            blk = 188; continue;
        }
        188 => {
            blk = 205; continue;
        }
        189 => {
            let Some(t149) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008821f2); };
            c.w32(8, t149);
            let Some(t150) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008821f7); };
            c.cmp(t150, c.r32(8), 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 190; continue;
        }
        190 => {
            blk = 242; continue;
        }
        191 => {
            let Some(t151) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000882209); };
            c.w32(11, t151);
            let r152 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r152);
            let Some(t153) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000882210); };
            c.w32(11, t153);
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_b() { blk = 242; continue; }
            blk = 192; continue;
        }
        192 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x700088221d); }
            c.w32(11, 0xfffffffeu64);
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_b() { blk = 274; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t154) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000882230); };
            c.w32(8, t154);
            let r155 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r155);
            let Some(t156) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000882237); };
            c.w32(11, t156);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r157 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r157);
            c.w32(12, c.r32(3));
            c.w64(10, c.r64(4));
            let r158 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r158);
            let r159 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r159);
            c.logic(r159, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000882253); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x7000882258); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700088225c); }
            c.w64(2, c.r64(12));
            c.w64(1, c.r64(13));
            c.w64(15, c.r64(1));
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x7000882269); };
            c.w64(0, t160);
            c.w64(10, 0x7000882277u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000882277); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700088227b); }
            return c.call(c.r64(0), 0x7000882281, 0x700088227f);
        }
        194 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000882281); }
            let Some(t161) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000882289); };
            c.w64(4, t161);
            c.w32(8, c.r32(0));
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000882291); };
            let r163 = c.r64(8) & t162;
            c.w64(8, r163);
            c.logic(r163, 64);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000882298); };
            c.w64(11, t164);
            blk = 196; continue;
        }
        195 => {
            let r165 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r165);
            let r166 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r166);
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008822a5); };
            c.w64(12, t167);
            let r168 = c.r64(12) & c.r64(8);
            c.w64(12, r168);
            c.logic(r168, 64);
            c.w64(8, c.r64(12));
            blk = 196; continue;
        }
        196 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x4));
            let Some(t169) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008822b4); };
            c.w32(12, t169);
            let r170 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r170);
            let Some(t171) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008822bc); };
            c.cmp(t171, c.r32(12), 32);
            if c.cc_e() { blk = 204; continue; }
            blk = 197; continue;
        }
        197 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 202; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t172) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008822cb); };
            c.cmp(t172, c.r32(12), 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 199; continue;
        }
        199 => {
            c.xorpd_rr(0, 0);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008822d8); };
            c.cvtsi2sd(0, (t173 as i64));
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008822de); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 195; continue; }
            blk = 200; continue;
        }
        200 => {
            if c.cc_e() { blk = 242; continue; }
            blk = 201; continue;
        }
        201 => {
            blk = 195; continue;
        }
        202 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008822f9); };
            c.cmp(c.r64(12), t174, 64);
            if c.cc_e() { blk = 242; continue; }
            blk = 203; continue;
        }
        203 => {
            blk = 195; continue;
        }
        204 => {
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000882305); };
            c.w64(0, t175);
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000882309); };
            c.w64(9, t176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088230d); };
            c.w64(3, t177);
            blk = 205; continue;
        }
        205 => {
            c.w64(8, c.r64(9));
            let Some(t178) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000882314); };
            c.w32(8, t178);
            let r179 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r179);
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088231b); };
            c.w64(2, t180);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088231f); };
            c.w64(1, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000882323); };
            c.w64(7, t182);
            let Some(t183) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000882327); };
            c.cmp(t183, c.r32(8), 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 206; continue;
        }
        206 => {
            blk = 110; continue;
        }
        207 => {
            let Some(t184) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000882339); };
            c.w32(9, t184);
            let r185 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r185);
            let Some(t186) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000882340); };
            c.logic(t186 & 0x4u64, 8);
            if c.cc_ne() { blk = 243; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t187) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700088234b); };
            c.cmp(t187, c.r64(3), 64);
            if c.cc_a() { blk = 242; continue; }
            blk = 209; continue;
        }
        209 => {
            blk = 243; continue;
        }
        210 => {
            let Some(t188) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088235a); };
            c.w32(9, t188);
            let r189 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r189);
            let Some(t190) = c.ld32(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000882361); };
            c.w32(11, t190);
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 239; continue; }
            blk = 211; continue;
        }
        211 => {
            c.logic(c.r8(11) & 0x4u64, 8);
            if c.cc_ne() { blk = 243; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t191) = c.ld64(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000882379); };
            c.w64(9, t191);
            let Some(t192) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088237d); };
            c.w64(11, t192);
            let Some(t193) = c.ld8(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000882381); };
            c.logic(t193 & 0x1u64, 8);
            if c.cc_ne() { blk = 215; continue; }
            blk = 213; continue;
        }
        213 => {
            let Some(t194) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000882388); };
            c.w64(12, t194);
            let r195 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r195);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_a() { blk = 243; continue; }
            blk = 214; continue;
        }
        214 => {
            let Some(t196) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000882398); };
            c.w64(8, t196);
            blk = 241; continue;
        }
        215 => {
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_a() { blk = 243; continue; }
            blk = 216; continue;
        }
        216 => {
            let Some(t197) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008823aa); };
            c.w32(8, t197);
            let r198 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r198);
            let Some(t199) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008823b1); };
            c.w32(8, t199);
            let r200 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r200);
            let r201 = c.r32(8) & 0x3fu64;
            c.w32(8, r201);
            c.logic(r201, 32);
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_l() { blk = 226; continue; }
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_l() { blk = 222; continue; }
            blk = 218; continue;
        }
        218 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_e() { blk = 234; continue; }
            blk = 219; continue;
        }
        219 => {
            c.cmp(c.r32(8), 0x25u64, 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 220; continue;
        }
        220 => {
            c.cmp(c.r32(8), 0x26u64, 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 221; continue;
        }
        221 => {
            blk = 251; continue;
        }
        222 => {
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_e() { blk = 236; continue; }
            blk = 223; continue;
        }
        223 => {
            c.cmp(c.r32(8), 0x22u64, 32);
            if c.cc_e() { blk = 236; continue; }
            blk = 224; continue;
        }
        224 => {
            c.cmp(c.r32(8), 0x23u64, 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 225; continue;
        }
        225 => {
            blk = 251; continue;
        }
        226 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_l() { blk = 231; continue; }
            blk = 227; continue;
        }
        227 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 228; continue;
        }
        228 => {
            c.cmp(c.r32(8), 0x1fu64, 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 229; continue;
        }
        229 => {
            c.cmp(c.r32(8), 0x20u64, 32);
            if c.cc_e() { blk = 236; continue; }
            blk = 230; continue;
        }
        230 => {
            blk = 251; continue;
        }
        231 => {
            c.cmp(c.r32(8), 0x1cu64, 32);
            if c.cc_e() { blk = 234; continue; }
            blk = 232; continue;
        }
        232 => {
            c.cmp(c.r32(8), 0x1du64, 32);
            if c.cc_e() { blk = 234; continue; }
            blk = 233; continue;
        }
        233 => {
            blk = 251; continue;
        }
        234 => {
            c.w32(8, 0x1u64);
            blk = 238; continue;
        }
        235 => {
            c.w32(8, 0x2u64);
            blk = 238; continue;
        }
        236 => {
            c.w32(8, 0x4u64);
            blk = 238; continue;
        }
        237 => {
            c.w32(8, 0x8u64);
            blk = 238; continue;
        }
        238 => {
            let r202 = c.sub(c.r64(9), c.r64(11), 64);
            c.w64(9, r202);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(9));
            c.w64(9, c.r64(10));
            c.cqo();
            c.idiv(c.r64(8), 64);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000882461); };
            c.w64(11, t203);
            c.w64(8, c.r64(0));
            c.w64(0, c.r64(9));
            blk = 241; continue;
        }
        239 => {
            c.w64(10, c.r64(4));
            let r204 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r204);
            let r205 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r205);
            c.logic(r205, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000882478); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(8));
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x7000882483); };
            c.w64(0, t206);
            c.w64(10, 0x7000882491u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000882491); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000882495); }
            return c.call(c.r64(0), 0x700088249b, 0x7000882499);
        }
        240 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700088249b); }
            let Some(t207) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008824a3); };
            c.w64(4, t207);
            c.w64(8, c.r64(0));
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008824ab); };
            c.w64(0, t208);
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008824af); };
            c.w64(3, t209);
            blk = 241; continue;
        }
        241 => {
            c.cmp(c.r64(3), c.r64(8), 64);
            if c.cc_ae() { blk = 243; continue; }
            blk = 242; continue;
        }
        242 => {
            let Some(t210) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008824b8); };
            c.w64(8, t210);
            blk = 244; continue;
        }
        243 => {
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008824c1); };
            c.w64(8, t211);
            blk = 244; continue;
        }
        244 => {
            let Some(t212) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008824c8); };
            c.cmp(t212, c.r32(8), 32);
            if c.cc_ne() { blk = 247; continue; }
            blk = 245; continue;
        }
        245 => {
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008824d1); };
            c.w64(0, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008824d5); };
            c.w64(3, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008824d9); };
            c.w64(6, t215);
            return c.call(0x7000870914, 0x70008824e2, 0x70008824dd);
        }
        246 => {
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008824e2); };
            if !c.push64(t216) { return RegfileExit::fault(0x70008824e2); }
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008824e5); };
            if !c.push64(t217) { return RegfileExit::fault(0x70008824e5); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008824e8); }
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008824e9); };
            if !c.push64(t218) { return RegfileExit::fault(0x70008824e9); }
            c.w32(0, 0x3u64);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008824f1); };
            c.w64(7, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008824f5); };
            c.w64(6, t220);
            return c.call(0x70007f2274, 0x70008824fe, 0x70008824f9);
        }
        247 => {
            let Some(t221) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008824fe); };
            c.cmp(t221, 0x0u64, 32);
            if c.cc_e() { blk = 276; continue; }
            blk = 248; continue;
        }
        248 => {
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000882508); };
            c.w64(8, t222);
            let r223 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r223);
            if c.cc_o() { blk = 276; continue; }
            blk = 249; continue;
        }
        249 => {
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        250 => {
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088251e); };
            c.w64(0, t224);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000882528); }
            return c.ret(0x18, 0x7000882529);
        }
        251 => {
            return RegfileExit::fault(0x700088252c);
        }
        252 => {
            blk = 274; continue;
        }
        253 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_ne() { blk = 274; continue; }
            blk = 254; continue;
        }
        254 => {
            let Some(t225) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088253d); };
            c.w32(15, t225);
            let r226 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r226);
            blk = 32; continue;
        }
        255 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x7000882549); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x700088254d); }
            blk = 259; continue;
        }
        256 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x7000882553); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x7000882557); }
            blk = 259; continue;
        }
        257 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x700088255d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x7000882561); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 259; continue; }
            blk = 258; continue;
        }
        258 => {
            let Some(t227) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088256b); };
            c.w64(12, t227);
            blk = 95; continue;
        }
        259 => {
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x7000882575); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000882577); }
            let Some(t228) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x7000882578); };
            c.w64(3, t228);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000882589, 0x7000882584);
        }
        260 => {
            let r229 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r229);
            c.w64(9, c.r64(0));
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088258e); };
            c.w64(8, t230);
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000882592); };
            c.w64(15, t231);
            blk = 98; continue;
        }
        261 => {
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088259b); };
            c.w64(0, t232);
            blk = 274; continue;
        }
        262 => {
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008825a1); };
            c.w64(0, t233);
            blk = 274; continue;
        }
        263 => {
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008825a7); };
            c.w64(0, t234);
            blk = 274; continue;
        }
        264 => {
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008825ad); };
            c.w64(0, t235);
            blk = 274; continue;
        }
        265 => {
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008825b3); };
            c.w64(0, t236);
            blk = 274; continue;
        }
        266 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008825b9); };
            c.w64(0, t237);
            blk = 274; continue;
        }
        267 => {
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008825bf); };
            c.w64(6, t238);
            return c.call(0x70008a1af4, 0x70008825c8, 0x70008825c3);
        }
        268 => {
            let Some(t239) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008825c8); };
            c.w32(8, t239);
            let Some(t240) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008825cc); };
            c.cmp(t240, c.r32(8), 32);
            if c.cc_ne() { blk = 271; continue; }
            blk = 269; continue;
        }
        269 => {
            let Some(t241) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008825d5); };
            c.logic(t241 & 0x1u64, 8);
            if c.cc_e() { blk = 272; continue; }
            blk = 270; continue;
        }
        270 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008825db); };
            c.w64(0, t242);
            blk = 243; continue;
        }
        271 => {
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008825e4); };
            c.w64(3, t243);
            blk = 272; continue;
        }
        272 => {
            c.w64(3, c.r64(0));
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008825eb); };
            c.w64(0, t244);
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008825ef); };
            c.w64(6, t245);
            return c.call(0x70008bced4, 0x70008825f8, 0x70008825f3);
        }
        273 => {
            c.w64(8, c.r64(0));
            blk = 244; continue;
        }
        274 => {
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000882600); };
            if !c.push64(t246) { return RegfileExit::fault(0x7000882600); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000882603); }
            let Some(t247) = c.ld64(c.r64(13).wrapping_add(0x23b8)) else { return RegfileExit::fault(0x7000882604); };
            c.w64(3, t247);
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088260b); };
            c.w64(6, t248);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700088261c, 0x7000882617);
        }
        275 => {
            c.w64(8, c.r64(0));
            blk = 244; continue;
        }
        276 => {
            let Some(t249) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000882624); };
            c.cmp(t249, 0x0u64, 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 277; continue;
        }
        277 => {
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088262a); };
            c.w64(0, t250);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700088262e); }
            blk = 279; continue;
        }
        278 => {
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000882635); };
            c.w64(0, t251);
            let r252 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r252);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 279; continue;
        }
        279 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(8, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 1) { return RegfileExit::fault(0x7000882663); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 285; continue; }
            blk = 280; continue;
        }
        280 => {
            if c.cc_ne() { blk = 285; continue; }
            blk = 281; continue;
        }
        281 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 283; continue; }
            blk = 282; continue;
        }
        282 => {
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 285; continue; }
            blk = 283; continue;
        }
        283 => {
            c.w64(9, c.r64(8));
            let r253 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r253);
            if c.cc_o() { blk = 285; continue; }
            blk = 284; continue;
        }
        284 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        285 => {
            let Some(t254) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000882695); };
            c.w64(8, t254);
            let Some(t255) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700088269c); };
            c.w64(9, t255);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t256) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008826a3); };
            c.w64(12, t256);
            let Some(t257) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008826aa); };
            c.cmp(t257, c.r64(11), 64);
            if c.cc_a() { blk = 288; continue; }
            blk = 286; continue;
        }
        286 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008826ba, 0x70008826b5);
        }
        287 => {
            blk = 289; continue;
        }
        288 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008826bc); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 289; continue;
        }
        289 => {
            let Some(t258) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008826c6); };
            c.w64(8, t258);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008826cd); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008826d1); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008826d6); }
            blk = 1; continue;
        }
        290 => {
            return RegfileExit::fault(0x70008826e0);
        }
        291 => {
            return RegfileExit::jump(0x70008826e4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008826f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008826f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008826f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008826f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008826fa); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008826ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000882703); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000882707); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 79; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088271f); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000882724); };
            c.cmp(t3, c.r32(11), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000882733); };
            c.cmp(t4, c.r32(11), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000882742); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000882746); }
            let r5 = c.r32(12) | c.r32(15);
            c.w32(12, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 77; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 81; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088275d); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000882764); };
            c.cmp(t8, 0xbau64, 16);
            if c.cc_b() { blk = 81; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700088277e); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000882788); };
            c.w32(15, t9);
            let r10 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r10);
            let Some(t11) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088278f); };
            c.cmp(t11, 0x834u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000882798); };
            c.w32(12, t12);
            let r13 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r13);
            c.w64(15, c.r64(12));
            blk = 29; continue;
        }
        7 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 9; continue;
        }
        8 => {
            c.w32(15, 0x1u64);
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008827b8); };
            c.w32(15, t14);
            let r15 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r15);
            let Some(t16) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008827bf); };
            c.w32(15, t16);
            let r17 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r17);
            let Some(t18) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008827c6); };
            c.w32(3, t18);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008827cd); };
            c.w32(2, t19);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0xb0u64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008827dd);
        }
        12 => {
            let Some(t20) = c.ld32(c.r64(15).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x70008827de); };
            c.cmp(t20, c.r32(2), 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 85; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008827f5); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008827fc); };
            c.w32(12, t23);
            let r24 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r24);
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000882804); };
            c.w32(15, t25);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t26) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088280c); };
            c.w32(3, t26);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x44u64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x7000882819);
        }
        16 => {
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0x117)) else { return RegfileExit::fault(0x700088281a); };
            c.cmp(t27, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(15), 0xaeu64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x700088282d);
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(12).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x700088282e); };
            c.cmp(t28, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(15), 0xafu64, 64);
            if c.cc_a() { blk = 84; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x7000882845);
        }
        22 => {
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000882846); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(0, c.r64(12));
            return c.call(0x70008a17f4, 0x700088285b, 0x7000882856);
        }
        25 => {
            c.w64(15, c.r64(0));
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088285e); };
            c.w64(6, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000882862); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000882866); };
            c.w64(11, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088286a); };
            c.w64(0, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088286e); };
            c.w64(9, t35);
            blk = 29; continue;
        }
        26 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(15, c.r64(12));
            blk = 29; continue;
        }
        28 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 29; continue;
        }
        29 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(0), 0x0u64, 64);
            c.w8(3, (c.cc_be() as u64));
            c.w32(3, c.r8(3));
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x7000882892); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000882896); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 89; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t36) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008828a9); };
            c.w64(2, t36);
            c.w64(1, c.r64(2));
            blk = 33; continue;
        }
        32 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008828b2); };
            c.w64(1, t37);
            blk = 33; continue;
        }
        33 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008828b9); };
            c.w64(2, t38);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008828c0); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t39) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008828cd); };
            c.w32(7, t39);
            let r40 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r40);
            let Some(t41) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008828d3); };
            c.logic(t41 & 0x2u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_be() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(8, c.r64(9));
            let Some(t42) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008828e6); };
            c.w64(8, t42);
            blk = 38; continue;
        }
        37 => {
            c.w64(8, c.r64(2));
            blk = 38; continue;
        }
        38 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008828ef); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 75; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t43) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008828fd); };
            c.w32(9, t43);
            let r44 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r44);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x7000882904); }
            let Some(t45) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000882908); };
            c.cmp(t45, 0x834u64, 16);
            if c.cc_ne() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(3, 0x1u64);
            blk = 42; continue;
        }
        41 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t46) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000882923); };
            c.w32(3, t46);
            c.w64(7, c.r64(3));
            let r47 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r47);
            let r48 = c.r32(7) & 0x3fu64;
            c.w32(7, r48);
            c.logic(r48, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x7000882931); }
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_a() { blk = 75; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t49) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088293e); };
            c.w32(7, t49);
            let r50 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r50);
            let Some(t51) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000882944); };
            c.w32(7, t51);
            let r52 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r52);
            let Some(t53) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088294a); };
            c.w32(8, t53);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x4cu64, 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            return RegfileExit::fault(0x700088295a);
        }
        46 => {
            let Some(t54) = c.ld32(c.r64(7).wrapping_add(0x137)) else { return RegfileExit::fault(0x700088295b); };
            c.w32(8, t54);
            let Some(t55) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000882962); };
            c.cmp(t55, c.r32(8), 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700088296c); };
            c.w64(8, t56);
            let Some(t57) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000882973); };
            c.cmp(t57, c.r32(12), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(7, c.r64(12));
            blk = 50; continue;
        }
        49 => {
            c.w32(8, c.r64(7).wrapping_add(0x2));
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000882989); };
            c.w64(9, t58);
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700088298d); };
            c.w64(8, t59);
            blk = 50; continue;
        }
        50 => {
            let Some(t60) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000882994); };
            c.cmp(c.r32(7), t60, 32);
            if c.cc_ge() { blk = 68; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t61) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088299d); };
            c.cmp(t61, c.r32(9), 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t62) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008829a7); };
            c.cmp(t62, c.r32(12), 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t63) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008829b1); };
            c.w32(8, t63);
            c.cmp(c.r32(8), c.r32(7), 32);
            if c.cc_le() { blk = 73; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t64) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008829be); };
            c.w32(8, t64);
            let r65 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r65);
            let r66 = c.r32(8) & 0x3fu64;
            c.w32(8, r66);
            c.logic(r66, 32);
            let r67 = c.shr(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r67);
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008829d4); };
            c.w32(8, t68);
            let r69 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r69);
            let Some(t70) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008829db); };
            c.w32(9, t70);
            let r71 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r71);
            let Some(t72) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008829e2); };
            c.w32(9, t72);
            let r73 = c.sub(c.r32(9), 0x77u64, 32);
            c.w32(9, r73);
            c.cmp(c.r32(9), 0xcu64, 32);
            if c.cc_a() { blk = 70; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t74) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008829f5); };
            c.w32(9, t74);
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_ae() { blk = 69; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t75) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000882a05); };
            c.w32(8, t75);
            let r76 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r76);
            let Some(t77) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000882a0d); };
            c.cmp(t77, c.r32(8), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 58; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000882a1a); }
            blk = 65; continue;
        }
        59 => {
            let Some(t78) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000882a23); };
            c.w32(8, t78);
            let r79 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r79);
            let Some(t80) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000882a2a); };
            c.w32(9, t80);
            let Some(t81) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000882a2e); };
            c.cmp(t81, c.r32(9), 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t82) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000882a3b); };
            c.w32(9, t82);
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_ae() { blk = 71; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) { return RegfileExit::fault(0x7000882a4b); }
            c.w64(11, 0xfff7fffffff7ffffu64);
            let Some(t83) = c.ld64(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000882a5c); };
            c.cmp(t83, c.r64(11), 64);
            if c.cc_e() { blk = 67; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000882a63); };
            c.w64(8, t84);
            let Some(t85) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000882a6a); };
            c.w64(9, t85);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000882a71); }
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000882a75); };
            c.w64(12, t86);
            let Some(t87) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000882a7c); };
            c.cmp(t87, c.r64(11), 64);
            if c.cc_be() { blk = 87; continue; }
            blk = 63; continue;
        }
        63 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000882a86); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 64; continue;
        }
        64 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000882a8d); };
            c.w64(9, t88);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000882a94); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000882a98); }
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000882a9e); };
            c.w64(11, t89);
            blk = 65; continue;
        }
        65 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000882aa2); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000882aa4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000882aa5); }
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000882aa7); };
            if !c.push64(t90) { return RegfileExit::fault(0x7000882aa7); }
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x7000882aba, 0x7000882ab5);
        }
        66 => {
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000882aba); };
            c.w64(6, t91);
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000882abe); };
            c.w64(2, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000882ac5); };
            c.w64(0, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000882ac9); };
            c.w64(11, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000882acd); };
            c.w64(1, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000882ad1); };
            c.w64(7, t96);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 49; continue;
        }
        67 => {
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000882add); };
            c.w64(11, t97);
            blk = 49; continue;
        }
        68 => {
            c.w64(8, c.r64(0));
            let r98 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r98);
            c.w64(0, c.r64(2));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000882af3); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000882af4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000882afa); }
            return c.ret(0x0, 0x7000882afc);
        }
        69 => {
            return RegfileExit::fault(0x7000882afd);
        }
        70 => {
            return RegfileExit::fault(0x7000882afe);
        }
        71 => {
            return RegfileExit::fault(0x7000882aff);
        }
        72 => {
            return RegfileExit::fault(0x7000882b00);
        }
        73 => {
            c.w64(12, c.r64(7));
            blk = 75; continue;
        }
        74 => {
            c.w64(12, c.r64(7));
            blk = 75; continue;
        }
        75 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000882b09); }
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000882b0b); };
            if !c.push64(t99) { return RegfileExit::fault(0x7000882b0b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000882b0e); }
            c.w64(0, c.r64(11));
            c.w64(3, c.r64(1));
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000882b15); };
            c.w64(1, t100);
            c.w64(7, c.r64(11));
            return c.call(0x70008819b4, 0x7000882b21, 0x7000882b1c);
        }
        76 => {
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000882b21); };
            c.w64(8, t101);
            let r102 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r102);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000882b2c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000882b2d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000882b33); }
            return c.ret(0x0, 0x7000882b35);
        }
        77 => {
            c.w32(8, 0x36u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000882b3c); }
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000882b3e); };
            c.w64(8, t103);
            let Some(t104) = c.ld32(c.r64(8).wrapping_add(0x12b)) else { return RegfileExit::fault(0x7000882b45); };
            c.w32(8, t104);
            let r105 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r105);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000882b4f); }
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000882b51); };
            c.w64(3, t106);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000882b62, 0x7000882b5d);
        }
        78 => {
            return RegfileExit::fault(0x7000882b62);
        }
        79 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000882b63); };
            c.w64(3, t107);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000882b74, 0x7000882b6f);
        }
        80 => {
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000882b74); };
            c.w64(6, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000882b78); };
            c.w64(0, t109);
            blk = 1; continue;
        }
        81 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a2054, 0x7000882b89, 0x7000882b84);
        }
        82 => {
            c.w64(11, c.r64(0));
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000882b8c); };
            c.w64(6, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000882b90); };
            c.w64(8, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000882b94); };
            c.w64(9, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000882b98); };
            c.w64(0, t113);
            blk = 4; continue;
        }
        83 => {
            let Some(t114) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000882ba1); };
            c.w32(12, t114);
            let r115 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r115);
            blk = 23; continue;
        }
        84 => {
            let Some(t116) = c.ld32(c.r64(12).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x7000882bad); };
            c.cmp(t116, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000882bbb); };
            c.w64(3, t117);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x7000882bca, 0x7000882bc5);
        }
        86 => {
            c.w64(12, c.r64(0));
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000882bcd); };
            c.w64(6, t118);
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000882bd1); };
            c.w64(8, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000882bd5); };
            c.w64(11, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000882bd9); };
            c.w64(9, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000882bdd); };
            c.w64(0, t122);
            blk = 23; continue;
        }
        87 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x7000882beb); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x7000882bf8, 0x7000882bf3);
        }
        88 => {
            c.w64(8, c.r64(0));
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000882bfb); };
            c.w64(6, t123);
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000882bff); };
            c.w64(2, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000882c06); };
            c.w64(0, t125);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000882c0a); };
            c.w64(1, t126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000882c0e); };
            c.w64(7, t127);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x7000882c12); }
            blk = 64; continue;
        }
        89 => {
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000882c1c); };
            c.w64(2, t128);
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t129) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000882c28); };
            c.w64(8, t129);
            c.w64(2, c.r64(8));
            blk = 92; continue;
        }
        92 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000882c35); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000882c37); }
            let Some(t130) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000882c38); };
            c.w64(3, t130);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000882c49, 0x7000882c44);
        }
        93 => {
            return RegfileExit::fault(0x7000882c49);
        }
        94 => {
            return RegfileExit::jump(0x7000882c4c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000882c54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000882c54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000882c58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000882c59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000882c5a); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000882c5f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000882c63); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 257; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000882c7b); };
            c.w64(11, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x11c0)) else { return RegfileExit::fault(0x7000882c80); };
            c.w64(12, t3);
            let Some(t4) = c.ld8(c.r64(12)) else { return RegfileExit::fault(0x7000882c87); };
            c.logic(t4 & 0xffffffffffffffffu64, 8);
            if c.cc_ne() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000882c8e); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000882c95); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000882c9c); };
            c.w32(12, t9);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x2bu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x7000882cac);
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(11).wrapping_add(0xb3)) else { return RegfileExit::fault(0x7000882cad); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            blk = 5; continue;
        }
        5 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000882cba); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x7000882cbe); }
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_ne() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000882cc8); };
            c.w32(15, t12);
            let r13 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r13);
            let Some(t14) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000882ccf); };
            c.w32(15, t14);
            let r15 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r15);
            let Some(t16) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000882cd6); };
            c.w32(0, t16);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x2bu64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x7000882ce6);
        }
        8 => {
            let Some(t17) = c.ld32(c.r64(15).wrapping_add(0xb3)) else { return RegfileExit::fault(0x7000882ce7); };
            c.cmp(t17, c.r32(11), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 80; continue;
        }
        10 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t18) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000882cfd); };
            c.w64(3, t18);
            blk = 13; continue;
        }
        12 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000882d03); };
            c.w64(3, t19);
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x7000882d1e); };
            c.w64(7, t20);
            let Some(t21) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000882d25); };
            c.cmp(t21, c.r32(1), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t22) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000882d2e); };
            c.w32(7, t22);
            let r23 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r23);
            let Some(t24) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000882d34); };
            c.cmp(t24, 0x834u64, 16);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), 0x1u64) { return RegfileExit::fault(0x7000882d3c); }
            blk = 18; continue;
        }
        17 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), 0x0u64) { return RegfileExit::fault(0x7000882d46); }
            blk = 18; continue;
        }
        18 => {
            let Some(t25) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000882d4e); };
            c.cmp(t25, 0x0u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t26) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000882d54); };
            c.w32(11, t26);
            let r27 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r27);
            let r28 = c.r32(11) & 0x3fu64;
            c.w32(11, r28);
            c.logic(r28, 32);
            c.cmp(c.r32(11), 0xbu64, 32);
            if c.cc_g() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(0), 0x4cu64, 64);
            if c.cc_a() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x7000882d6d);
        }
        22 => {
            let Some(t29) = c.ld32(c.r64(15).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000882d6e); };
            c.w32(11, t29);
            let Some(t30) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000882d75); };
            c.cmp(t30, c.r32(11), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000882d7b); };
            c.w64(11, t31);
            let Some(t32) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000882d82); };
            c.cmp(t32, c.r32(1), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(11, 0x1u64);
            blk = 27; continue;
        }
        25 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 26; continue;
        }
        26 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x7000882d9e); };
            c.w64(7, t33);
            let Some(t34) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000882da5); };
            c.cmp(t34, c.r32(1), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t35) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000882daa); };
            c.w32(7, t35);
            let r36 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r36);
            let Some(t37) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000882db0); };
            c.cmp(t37, 0x834u64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), 0x1u64) { return RegfileExit::fault(0x7000882db8); }
            blk = 32; continue;
        }
        31 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), 0x0u64) { return RegfileExit::fault(0x7000882dc2); }
            blk = 32; continue;
        }
        32 => {
            let Some(t38) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000882dca); };
            c.cmp(t38, 0x0u64, 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t39) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000882dd0); };
            c.w32(6, t39);
            let r40 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r40);
            let r41 = c.r32(6) & 0x3fu64;
            c.w32(6, r41);
            c.logic(r41, 32);
            c.cmp(c.r32(6), 0x5u64, 32);
            if c.cc_a() { blk = 39; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r64(0), 0x4cu64, 64);
            if c.cc_a() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            return RegfileExit::fault(0x7000882de5);
        }
        36 => {
            let Some(t42) = c.ld32(c.r64(15).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000882de6); };
            c.w32(6, t42);
            let Some(t43) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000882ded); };
            c.cmp(t43, c.r32(6), 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000882df2); };
            c.w64(6, t44);
            let Some(t45) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000882df9); };
            c.cmp(t45, c.r32(1), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(6, 0x1u64);
            blk = 40; continue;
        }
        39 => {
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 40; continue;
        }
        40 => {
            let r46 = c.r32(11) | c.r32(6);
            c.w32(11, r46);
            c.logic(r46, 32);
            if c.cc_ne() { blk = 255; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t47) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000882e15); };
            c.w32(11, t47);
            let r48 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r48);
            let Some(t49) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000882e1c); };
            c.cmp(t49, 0x40u64, 16);
            if c.cc_b() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 50; continue;
        }
        44 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x1020)) else { return RegfileExit::fault(0x7000882e29); };
            c.w64(11, t50);
            c.w32(6, 0x2u64);
            let Some(t51) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000882e35); };
            c.cmp(t51, c.r32(6), 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 50; continue;
        }
        46 => {
            let Some(t52) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000882e40); };
            c.w32(11, t52);
            c.cmp(c.r64(11), 0x2000000u64, 64);
            if c.cc_g() { blk = 80; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(0, c.r64(3));
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000882e54); };
            c.w64(6, t53);
            return c.call(0x700089ceb4, 0x7000882e5d, 0x7000882e58);
        }
        48 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000882e5d); };
            c.w64(11, t54);
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000882e68); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000882e69); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000882e6f); }
            return c.ret(0x0, 0x7000882e71);
        }
        49 => {
            c.w32(11, 0x1u64);
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t55) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000882e82); };
            c.w32(11, t55);
            let r56 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r56);
            let Some(t57) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000882e89); };
            c.w32(6, t57);
            c.cmp(c.r16(6), 0x825u64, 16);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r16(6), 0x827u64, 16);
            if c.cc_ne() { blk = 63; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t58) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000882ea0); };
            c.cmp(t58, c.r32(1), 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x1008)) else { return RegfileExit::fault(0x7000882ea5); };
            c.w64(6, t59);
            c.w32(7, 0x2u64);
            let Some(t60) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000882eb1); };
            c.cmp(t60, c.r32(7), 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r64(0), 0x52u64, 64);
            if c.cc_a() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            return RegfileExit::fault(0x7000882ebc);
        }
        57 => {
            let Some(t61) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000882ebd); };
            c.w32(11, t61);
            let r62 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r62);
            let Some(t63) = c.ld32(c.r64(15).wrapping_add(0x14f)) else { return RegfileExit::fault(0x7000882ec4); };
            c.cmp(t63, c.r32(11), 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r64(0), 0x51u64, 64);
            if c.cc_a() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            return RegfileExit::fault(0x7000882ed3);
        }
        60 => {
            let Some(t64) = c.ld32(c.r64(15).wrapping_add(0x14b)) else { return RegfileExit::fault(0x7000882ed4); };
            c.w32(6, t64);
            let Some(t65) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000882edb); };
            c.w32(11, t65);
            let r66 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r66);
            let Some(t67) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000882ee2); };
            c.cmp(t67, c.r32(6), 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(0, c.r64(3));
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000882eeb); };
            c.w64(6, t68);
            return c.call(0x70008497f4, 0x7000882ef4, 0x7000882eef);
        }
        62 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000882ef4); };
            c.w64(11, t69);
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000882eff); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000882f00); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000882f06); }
            return c.ret(0x0, 0x7000882f08);
        }
        63 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 80; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t70) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000882f12); };
            c.w32(11, t70);
            let r71 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r71);
            let Some(t72) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000882f19); };
            c.w32(2, t72);
            c.cmp(c.r16(2), 0x830u64, 16);
            if c.cc_e() { blk = 74; continue; }
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r16(2), 0x829u64, 16);
            if c.cc_ne() { blk = 80; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t73) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000882f30); };
            c.cmp(t73, c.r32(1), 32);
            if c.cc_ne() { blk = 80; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r64(0), 0x55u64, 64);
            if c.cc_a() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            return RegfileExit::fault(0x7000882f3f);
        }
        69 => {
            let Some(t74) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000882f40); };
            c.w32(11, t74);
            let r75 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r75);
            let Some(t76) = c.ld32(c.r64(15).wrapping_add(0x15b)) else { return RegfileExit::fault(0x7000882f47); };
            c.cmp(t76, c.r32(11), 32);
            if c.cc_ne() { blk = 80; continue; }
            blk = 70; continue;
        }
        70 => {
            c.cmp(c.r64(0), 0x51u64, 64);
            if c.cc_a() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            return RegfileExit::fault(0x7000882f56);
        }
        72 => {
            let Some(t77) = c.ld32(c.r64(15).wrapping_add(0x14b)) else { return RegfileExit::fault(0x7000882f57); };
            c.w32(15, t77);
            let Some(t78) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000882f5e); };
            c.w32(11, t78);
            let r79 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r79);
            let Some(t80) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000882f65); };
            c.cmp(t80, c.r32(15), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 73; continue;
        }
        73 => {
            blk = 80; continue;
        }
        74 => {
            c.cmp(c.r64(0), 0x56u64, 64);
            if c.cc_a() { blk = 76; continue; }
            blk = 75; continue;
        }
        75 => {
            return RegfileExit::fault(0x7000882f73);
        }
        76 => {
            let Some(t81) = c.ld32(c.r64(15).wrapping_add(0x15f)) else { return RegfileExit::fault(0x7000882f74); };
            c.w32(15, t81);
            let Some(t82) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000882f7b); };
            c.cmp(t82, c.r32(15), 32);
            if c.cc_ne() { blk = 80; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x1018)) else { return RegfileExit::fault(0x7000882f81); };
            c.w64(11, t83);
            c.w32(15, 0x2u64);
            let Some(t84) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000882f8e); };
            c.cmp(t84, c.r32(15), 32);
            if c.cc_ne() { blk = 80; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(0, c.r64(3));
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000882f97); };
            c.w64(6, t85);
            return c.call(0x700085c694, 0x7000882fa0, 0x7000882f9b);
        }
        79 => {
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000882fa0); };
            c.w64(11, t86);
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000882fab); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000882fac); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000882fb2); }
            return c.ret(0x0, 0x7000882fb4);
        }
        80 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t87) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000882fbb); };
            c.w64(8, t87);
            c.w64(11, c.r64(8));
            blk = 83; continue;
        }
        82 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000882fc4); };
            c.w64(11, t88);
            blk = 83; continue;
        }
        83 => {
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000882fcb); };
            c.w64(8, t89);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(11)) { return RegfileExit::fault(0x7000882fd2); }
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_be() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t90) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000882fdc); };
            c.w64(15, t90);
            blk = 86; continue;
        }
        85 => {
            c.w64(15, c.r64(8));
            blk = 86; continue;
        }
        86 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x7000882fe5); }
            c.cmp(c.r64(12), 0x2u64, 64);
            if c.cc_be() { blk = 88; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t91) = c.ld64(c.r64(9).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000882fef); };
            c.w64(8, t91);
            blk = 88; continue;
        }
        88 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x7000882ff3); }
            let Some(t92) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000882ff7); };
            c.cmp(t92, c.r32(15), 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 89; continue;
        }
        89 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 92; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t93) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000883006); };
            c.w32(9, t93);
            let r94 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r94);
            let Some(t95) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088300d); };
            c.logic(t95 & 0x2u64, 8);
            if c.cc_e() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w32(9, 0x1u64);
            blk = 93; continue;
        }
        92 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 93; continue;
        }
        93 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 259; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w32(9, 0x1u64);
            blk = 96; continue;
        }
        95 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 96; continue;
        }
        96 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x7000883034); }
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000883038); };
            c.w64(3, t96);
            c.w64(0, c.r64(11));
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883042); };
            c.w64(6, t97);
            return c.call(0x7000870914, 0x700088304b, 0x7000883046);
        }
        97 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700088304b); }
            let Some(t98) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088304f); };
            c.cmp(t98, c.r32(0), 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700088305c); };
            c.cmp(t99, c.r32(0), 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 99; continue;
        }
        99 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 170; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t100) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000883071); };
            c.w32(8, t100);
            let r101 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r101);
            let Some(t102) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000883078); };
            c.logic(t102 & 0x2u64, 8);
            if c.cc_e() { blk = 170; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t103) = c.ld8(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000883083); };
            c.logic(t103 & 0x1u64, 8);
            if c.cc_e() { blk = 105; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000883089); };
            c.w64(2, t104);
            let Some(t105) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088308d); };
            c.w32(8, t105);
            let r106 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r106);
            let Some(t107) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000883094); };
            c.logic(t107 & 0x40u64, 8);
            if c.cc_e() { blk = 105; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088309b); };
            c.w64(8, t108);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008830a2); }
            c.w64(7, c.r64(2));
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008830a7); };
            c.w64(6, t109);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f7214, 0x70008830b2, 0x70008830ad);
        }
        104 => {
            c.w64(9, c.r64(0));
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008830b5); };
            c.w64(6, t110);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008830bc); };
            c.w64(0, t111);
            blk = 110; continue;
        }
        105 => {
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008830c2); };
            c.w64(6, t112);
            let Some(t113) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008830c6); };
            c.w32(8, t113);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x69u64, 64);
            if c.cc_a() { blk = 107; continue; }
            blk = 106; continue;
        }
        106 => {
            return RegfileExit::fault(0x70008830d6);
        }
        107 => {
            let Some(t114) = c.ld32(c.r64(6).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x70008830d7); };
            c.w32(8, t114);
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008830de); };
            c.w64(9, t115);
            let Some(t116) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008830e5); };
            c.w64(11, t116);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008830ec); };
            c.w64(15, t117);
            let Some(t118) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008830f3); };
            c.cmp(t118, c.r64(12), 64);
            if c.cc_be() { blk = 261; continue; }
            blk = 108; continue;
        }
        108 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008830fc); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 109; continue;
        }
        109 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000883103); }
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700088310a); }
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700088310e); };
            c.w64(11, t119);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000883115); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000883119); }
            blk = 110; continue;
        }
        110 => {
            c.w64(7, c.r64(0));
            let Some(t120) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000883120); };
            c.w32(11, t120);
            let r121 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r121);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000883127); }
            let Some(t122) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088312b); };
            c.logic(t122 & 0x2u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000883136); };
            if !c.push64(t123) { return RegfileExit::fault(0x7000883136); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000883140, 0x700088313b);
        }
        112 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x7000883140); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 288; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t124) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088314c); };
            c.w32(8, t124);
            let r125 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r125);
            let Some(t126) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000883153); };
            c.cmp(t126, 0xbau64, 16);
            if c.cc_b() { blk = 288; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000883160); };
            c.w64(3, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883167); };
            c.w64(6, t128);
            return c.call(0x7000870914, 0x7000883170, 0x700088316b);
        }
        115 => {
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883170); };
            c.w64(6, t129);
            let Some(t130) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000883174); };
            c.w32(8, t130);
            let r131 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r131);
            let Some(t132) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088317b); };
            c.w32(8, t132);
            let r133 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r133);
            let Some(t134) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000883182); };
            c.w32(9, t134);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(0)) { return RegfileExit::fault(0x700088318c); }
            c.cmp(c.r64(9), 0x68u64, 64);
            if c.cc_a() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            return RegfileExit::fault(0x7000883196);
        }
        117 => {
            let Some(t135) = c.ld32(c.r64(8).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x7000883197); };
            c.w32(8, t135);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x700088319e); }
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 119; continue;
        }
        118 => {
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008831a6); };
            c.w64(1, t136);
            c.w32(8, c.r64(1).wrapping_add(0x2));
            c.w64(3, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008831b1); };
            c.w64(0, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008831b5); };
            c.w64(6, t138);
            blk = 119; continue;
        }
        119 => {
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008831b9); };
            if !c.push64(t139) { return RegfileExit::fault(0x70008831b9); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008831bc); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f21d4, 0x70008831ca, 0x70008831c5);
        }
        120 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008831ca); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 286; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t140) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008831d6); };
            c.w32(8, t140);
            let r141 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r141);
            let Some(t142) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008831dd); };
            c.cmp(c.r32(8), t142, 32);
            if c.cc_e() { blk = 136; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t143) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008831e7); };
            c.cmp(t143, 0xbau64, 16);
            if c.cc_b() { blk = 286; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x70008831f4); };
            c.w64(3, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008831fb); };
            c.w64(6, t145);
            return c.call(0x7000870914, 0x7000883204, 0x70008831ff);
        }
        124 => {
            let Some(t146) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000883204); };
            c.cmp(t146, c.r32(0), 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 125; continue;
        }
        125 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 133; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t147) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000883214); };
            c.cmp(t147, c.r32(0), 32);
            if c.cc_e() { blk = 132; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t148) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088321d); };
            c.w32(8, t148);
            let r149 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r149);
            let Some(t150) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000883224); };
            c.logic(t150 & 0x10u64, 8);
            if c.cc_ne() { blk = 131; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t151) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088322b); };
            c.cmp(t151, c.r32(8), 32);
            if c.cc_e() { blk = 268; continue; }
            blk = 129; continue;
        }
        129 => {
            let Some(t152) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000883238); };
            c.cmp(t152, 0x41u64, 16);
            if c.cc_e() { blk = 266; continue; }
            blk = 130; continue;
        }
        130 => {
            blk = 152; continue;
        }
        131 => {
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000883249); };
            c.w64(0, t153);
            blk = 144; continue;
        }
        132 => {
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000883252); };
            c.w64(0, t154);
            blk = 144; continue;
        }
        133 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 152; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000883265); };
            c.w64(0, t155);
            blk = 144; continue;
        }
        135 => {
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088326b); };
            c.w64(0, t156);
            blk = 144; continue;
        }
        136 => {
            let Some(t157) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000883271); };
            c.w32(8, t157);
            let r158 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r158);
            let Some(t159) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000883278); };
            c.cmp(t159, c.r32(8), 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 137; continue;
        }
        137 => {
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 143; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t160) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700088328a); };
            c.cmp(t160, c.r32(8), 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t161) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000883293); };
            c.w32(9, t161);
            let r162 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r162);
            let Some(t163) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088329a); };
            c.logic(t163 & 0x10u64, 8);
            if c.cc_ne() { blk = 144; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t164) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008832a1); };
            c.cmp(t164, c.r32(9), 32);
            if c.cc_e() { blk = 272; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t165) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008832ae); };
            c.cmp(t165, 0x41u64, 16);
            if c.cc_e() { blk = 270; continue; }
            blk = 142; continue;
        }
        142 => {
            blk = 152; continue;
        }
        143 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 152; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008832c2); };
            c.w64(8, t166);
            let Some(t167) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008832c6); };
            c.cmp(t167, c.r32(8), 32);
            if c.cc_e() { blk = 147; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t168) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x70008832cc); };
            c.w64(3, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008832d3); };
            c.w64(6, t169);
            return c.call(0x7000870914, 0x70008832dc, 0x70008832d7);
        }
        146 => {
            blk = 148; continue;
        }
        147 => {
            let Some(t170) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008832de); };
            c.w32(0, t170);
            let r171 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r171);
            blk = 148; continue;
        }
        148 => {
            let Some(t172) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008832e4); };
            c.cmp(t172, 0x0u64, 32);
            if c.cc_e() { blk = 150; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008832ea); };
            if !c.push64(t173) { return RegfileExit::fault(0x70008832ea); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008832ed); }
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008832ee); };
            if !c.push64(t174) { return RegfileExit::fault(0x70008832ee); }
            c.w32(0, 0x2u64);
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008832f6); };
            c.w64(7, t175);
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008832fa); };
            c.w64(6, t176);
            return c.call(0x70007f2274, 0x7000883303, 0x70008832fe);
        }
        150 => {
            c.w64(1, c.r64(0));
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000883306); };
            c.w64(0, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088330a); };
            c.w64(3, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088330e); };
            c.w64(6, t179);
            return c.call(0x7000899bb4, 0x7000883317, 0x7000883312);
        }
        151 => {
            blk = 118; continue;
        }
        152 => {
            let Some(t180) = c.ld8(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088331c); };
            c.logic(t180 & 0x1u64, 8);
            if c.cc_e() { blk = 163; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000883326); };
            c.w64(0, t181);
            let Some(t182) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088332a); };
            c.w32(8, t182);
            let r183 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r183);
            let Some(t184) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000883331); };
            c.cmp(t184, 0x834u64, 16);
            if c.cc_ne() { blk = 155; continue; }
            blk = 154; continue;
        }
        154 => {
            c.w32(9, 0x1u64);
            blk = 156; continue;
        }
        155 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 156; continue;
        }
        156 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 164; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t185) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700088334b); };
            c.w32(9, t185);
            let r186 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r186);
            let r187 = c.r32(9) & 0x3fu64;
            c.w32(9, r187);
            c.logic(r187, 32);
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_a() { blk = 164; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088335e); };
            c.w64(6, t188);
            let Some(t189) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000883362); };
            c.w32(9, t189);
            let r190 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r190);
            let Some(t191) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000883369); };
            c.w32(9, t191);
            let r192 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r192);
            let Some(t193) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000883370); };
            c.w32(11, t193);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4cu64, 64);
            if c.cc_a() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            return RegfileExit::fault(0x7000883380);
        }
        160 => {
            let Some(t194) = c.ld32(c.r64(9).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000883381); };
            c.w32(9, t194);
            let Some(t195) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000883388); };
            c.cmp(t195, c.r32(9), 32);
            if c.cc_ne() { blk = 164; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t196) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700088338e); };
            c.w64(8, t196);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t197) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000883398); };
            c.cmp(t197, c.r32(9), 32);
            if c.cc_e() { blk = 164; continue; }
            blk = 162; continue;
        }
        162 => {
            c.w32(8, 0x1u64);
            blk = 165; continue;
        }
        163 => {
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008833a6); };
            c.w64(0, t198);
            blk = 164; continue;
        }
        164 => {
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008833ad); };
            c.w64(6, t199);
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 167; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t200) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008833b7); };
            c.w32(8, t200);
            let Some(t201) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008833bb); };
            c.cmp(c.r32(8), t201, 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008833c1); };
            c.w64(3, t202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008833c8); };
            c.w64(1, t203);
            return c.call(0x7000871cb4, 0x70008833d1, 0x70008833cc);
        }
        168 => {
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008833d1); };
            c.w64(0, t204);
            blk = 169; continue;
        }
        169 => {
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008833d5); };
            c.w64(8, t205);
            let r206 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r206);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008833e0); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008833e1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008833e7); }
            return c.ret(0x0, 0x70008833e9);
        }
        170 => {
            c.w32(8, 0x90u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008833f0); }
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008833f2); };
            c.w64(3, t207);
            c.w32(0, 0x1u64);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008833fe); };
            c.w64(6, t208);
            return c.call(0x700086ed74, 0x7000883407, 0x7000883402);
        }
        171 => {
            return RegfileExit::fault(0x7000883407);
        }
        172 => {
            let Some(t209) = c.ld8(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000883408); };
            c.logic(t209 & 0x1u64, 8);
            if c.cc_e() { blk = 291; continue; }
            blk = 173; continue;
        }
        173 => {
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000883412); };
            c.w64(0, t210);
            let Some(t211) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000883416); };
            c.w32(8, t211);
            let r212 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r212);
            let Some(t213) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088341d); };
            c.cmp(t213, 0xbau64, 16);
            if c.cc_b() { blk = 292; continue; }
            blk = 174; continue;
        }
        174 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(0)) { return RegfileExit::fault(0x7000883434); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 178; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t214) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088343e); };
            c.w32(9, t214);
            let r215 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r215);
            let Some(t216) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000883445); };
            c.cmp(t216, 0x834u64, 16);
            if c.cc_ne() { blk = 177; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t217) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088344e); };
            c.w32(8, t217);
            let r218 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883455); };
            c.w64(6, t219);
            blk = 200; continue;
        }
        177 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 179; continue;
        }
        178 => {
            c.w32(9, 0x1u64);
            blk = 179; continue;
        }
        179 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 184; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088346f); };
            c.w64(6, t220);
            let Some(t221) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000883473); };
            c.w32(9, t221);
            let r222 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r222);
            let Some(t223) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088347a); };
            c.w32(9, t223);
            let r224 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r224);
            let Some(t225) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000883481); };
            c.w32(11, t225);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t226) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000883488); };
            c.w32(12, t226);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xb0u64, 64);
            if c.cc_a() { blk = 182; continue; }
            blk = 181; continue;
        }
        181 => {
            return RegfileExit::fault(0x7000883498);
        }
        182 => {
            let Some(t227) = c.ld32(c.r64(9).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x7000883499); };
            c.cmp(t227, c.r32(12), 32);
            if c.cc_ne() { blk = 185; continue; }
            blk = 183; continue;
        }
        183 => {
            blk = 294; continue;
        }
        184 => {
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008834a7); };
            c.w64(6, t228);
            blk = 185; continue;
        }
        185 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 296; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t229) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008834b5); };
            c.w32(8, t229);
            let r230 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r230);
            let Some(t231) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008834bc); };
            c.w32(8, t231);
            let r232 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r232);
            let Some(t233) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008834c3); };
            c.w32(9, t233);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t234) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008834ca); };
            c.w32(11, t234);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x44u64, 64);
            if c.cc_a() { blk = 188; continue; }
            blk = 187; continue;
        }
        187 => {
            return RegfileExit::fault(0x70008834d7);
        }
        188 => {
            let Some(t235) = c.ld32(c.r64(8).wrapping_add(0x117)) else { return RegfileExit::fault(0x70008834d8); };
            c.cmp(t235, c.r32(11), 32);
            if c.cc_e() { blk = 194; continue; }
            blk = 189; continue;
        }
        189 => {
            c.cmp(c.r64(9), 0xaeu64, 64);
            if c.cc_a() { blk = 191; continue; }
            blk = 190; continue;
        }
        190 => {
            return RegfileExit::fault(0x70008834ea);
        }
        191 => {
            let Some(t236) = c.ld32(c.r64(8).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x70008834eb); };
            c.cmp(t236, c.r32(11), 32);
            if c.cc_e() { blk = 194; continue; }
            blk = 192; continue;
        }
        192 => {
            c.cmp(c.r64(9), 0xafu64, 64);
            if c.cc_a() { blk = 295; continue; }
            blk = 193; continue;
        }
        193 => {
            return RegfileExit::fault(0x7000883501);
        }
        194 => {
            let Some(t237) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000883502); };
            c.w32(8, t237);
            let r238 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r238);
            blk = 195; continue;
        }
        195 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 198; continue; }
            blk = 196; continue;
        }
        196 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a17f4, 0x7000883517, 0x7000883512);
        }
        197 => {
            c.w64(8, c.r64(0));
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088351a); };
            c.w64(6, t239);
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700088351e); };
            c.w64(0, t240);
            blk = 200; continue;
        }
        198 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ge() { blk = 200; continue; }
            blk = 199; continue;
        }
        199 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 200; continue;
        }
        200 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000883539); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x700088353d); }
            let Some(t241) = c.ld8(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000883541); };
            c.logic(t241 & 0x1u64, 8);
            if c.cc_e() { blk = 204; continue; }
            blk = 201; continue;
        }
        201 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000883547); };
            c.w64(2, t242);
            let Some(t243) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088354b); };
            c.w32(11, t243);
            let r244 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r244);
            let Some(t245) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000883552); };
            c.logic(t245 & 0x40u64, 8);
            if c.cc_e() { blk = 204; continue; }
            blk = 202; continue;
        }
        202 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000883559); }
            let Some(t246) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088355b); };
            c.w64(11, t246);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000883562); }
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(2));
            c.w64(11, c.r64(0));
            return c.call(0x70007f7214, 0x7000883574, 0x700088356f);
        }
        203 => {
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000883574); };
            c.w64(9, t247);
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883578); };
            c.w64(6, t248);
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088357c); };
            c.w64(8, t249);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 222; continue;
        }
        204 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 207; continue; }
            blk = 205; continue;
        }
        205 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700088358d); }
            c.w64(10, 0x40cffe0000000000u64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 302; continue; }
            blk = 206; continue;
        }
        206 => {
            blk = 208; continue;
        }
        207 => {
            c.cmp(c.r32(8), 0x7ff8u64, 32);
            if c.cc_ge() { blk = 302; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t250) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008835bb); };
            c.w32(11, t250);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x69u64, 64);
            if c.cc_a() { blk = 210; continue; }
            blk = 209; continue;
        }
        209 => {
            return RegfileExit::fault(0x70008835cb);
        }
        210 => {
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t251) = c.ld32(c.r64(6).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x70008835cf); };
            c.w32(12, t251);
            c.w64(15, c.r64(11));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 214; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t252) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008835e2); };
            c.w64(3, t252);
            let Some(t253) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008835e9); };
            c.w64(2, t253);
            c.w64(1, c.r64(11).wrapping_mul(2).wrapping_add(0x18));
            c.w64(7, c.r64(1).wrapping_add(c.r64(2)));
            let Some(t254) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008835f8); };
            c.w64(0, t254);
            let Some(t255) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008835ff); };
            c.cmp(t255, c.r64(7), 64);
            if c.cc_be() { blk = 298; continue; }
            blk = 212; continue;
        }
        212 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x7000883608); }
            c.w64(0, c.r64(2).wrapping_add(0x1));
            blk = 213; continue;
        }
        213 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700088360f); }
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x7000883615); }
            let Some(t256) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000883618); };
            c.w64(1, t256);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x700088361f); }
            c.w64(12, c.r64(0).wrapping_add(0x10));
            let r257 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r257);
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x7000883629); }
            let Some(t258) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700088362d); };
            c.w64(3, t258);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000883634); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x7000883639); }
            c.w64(15, c.r64(0));
            c.w64(1, c.r64(12));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 217; continue;
        }
        214 => {
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000883649); };
            c.w64(15, t259);
            let Some(t260) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000883650); };
            c.w64(3, t260);
            c.w64(1, c.r64(3).wrapping_add(0x10));
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000883657); };
            c.w64(2, t261);
            let Some(t262) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700088365e); };
            c.cmp(t262, c.r64(1), 64);
            if c.cc_be() { blk = 300; continue; }
            blk = 215; continue;
        }
        215 => {
            if !c.st64(c.r64(15), c.r64(1)) { return RegfileExit::fault(0x7000883667); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            blk = 216; continue;
        }
        216 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700088366e); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x7000883674); }
            let Some(t263) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000883678); };
            c.w64(1, t263);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x700088367f); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000883683); }
            c.w32(12, 0x1u64);
            c.w64(2, c.r64(3));
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 221; continue; }
            blk = 218; continue;
        }
        218 => {
            c.w64(11, c.r64(11).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 221; continue; }
            blk = 219; continue;
        }
        219 => {
            let Some(t264) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008836a4); };
            c.w64(12, t264);
            blk = 220; continue;
        }
        220 => {
            let r265 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r265);
            if !c.st32(c.r64(11).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x70008836af); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 220; continue; }
            blk = 221; continue;
        }
        221 => {
            c.w64(0, c.r64(15));
            blk = 222; continue;
        }
        222 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008836bc); }
            c.w64(11, c.r64(2));
            blk = 225; continue;
        }
        223 => {
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008836c5); };
            c.w64(3, t266);
            c.w32(8, c.r64(3).wrapping_add(0x2));
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008836d0); }
            c.w64(1, c.r64(0));
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008836d7); };
            c.w64(0, t267);
            let Some(t268) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008836db); };
            c.w64(6, t268);
            return c.call(0x7000899bb4, 0x70008836e4, 0x70008836df);
        }
        224 => {
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008836e4); };
            c.w64(11, t269);
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008836e8); };
            c.w64(6, t270);
            let Some(t271) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008836ec); };
            c.w64(0, t271);
            let Some(t272) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008836f0); };
            c.w64(8, t272);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 225; continue;
        }
        225 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 229; continue; }
            blk = 226; continue;
        }
        226 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008836fc); }
            let Some(t273) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000883702); };
            c.cmp(t273, 0x0u64, 32);
            if c.cc_ne() { blk = 228; continue; }
            blk = 227; continue;
        }
        227 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000883708); }
            blk = 231; continue;
        }
        228 => {
            c.w64(12, c.r64(8));
            let r274 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r274);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            blk = 231; continue;
        }
        229 => {
            let Some(t275) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000883721); };
            c.cmp(t275, 0x0u64, 32);
            if c.cc_ne() { blk = 233; continue; }
            blk = 230; continue;
        }
        230 => {
            c.w64(12, c.r64(11));
            let r276 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r276);
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700088372d); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 231; continue;
        }
        231 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 234; continue; }
            blk = 232; continue;
        }
        232 => {
            blk = 238; continue;
        }
        233 => {
            c.cmp(c.r32(11), c.r32(8), 32);
            if c.cc_ge() { blk = 238; continue; }
            blk = 234; continue;
        }
        234 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000883749); }
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700088374d); };
            c.w64(0, t277);
            c.w64(3, c.r64(11));
            return c.call(0x7000870914, 0x7000883759, 0x7000883754);
        }
        235 => {
            let Some(t278) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000883759); };
            c.cmp(t278, 0x0u64, 32);
            if c.cc_e() { blk = 223; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000883763); };
            if !c.push64(t279) { return RegfileExit::fault(0x7000883763); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000883766); }
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000883767); };
            if !c.push64(t280) { return RegfileExit::fault(0x7000883767); }
            c.w32(0, 0x2u64);
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088376f); };
            c.w64(7, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883773); };
            c.w64(6, t282);
            return c.call(0x70007f2274, 0x700088377c, 0x7000883777);
        }
        237 => {
            blk = 223; continue;
        }
        238 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 249; continue; }
            blk = 239; continue;
        }
        239 => {
            let Some(t283) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000883789); };
            c.w32(9, t283);
            let r284 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r284);
            let Some(t285) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000883790); };
            c.cmp(t285, 0x834u64, 16);
            if c.cc_ne() { blk = 241; continue; }
            blk = 240; continue;
        }
        240 => {
            c.w32(11, 0x1u64);
            blk = 242; continue;
        }
        241 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 242; continue;
        }
        242 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 249; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t286) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008837aa); };
            c.w32(11, t286);
            let r287 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r287);
            let r288 = c.r32(11) & 0x3fu64;
            c.w32(11, r288);
            c.logic(r288, 32);
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_a() { blk = 249; continue; }
            blk = 244; continue;
        }
        244 => {
            let Some(t289) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008837bd); };
            c.w32(11, t289);
            let r290 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r290);
            let Some(t291) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008837c4); };
            c.w32(11, t291);
            let r292 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r292);
            let Some(t293) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008837cb); };
            c.w32(12, t293);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x4cu64, 64);
            if c.cc_a() { blk = 246; continue; }
            blk = 245; continue;
        }
        245 => {
            return RegfileExit::fault(0x70008837db);
        }
        246 => {
            let Some(t294) = c.ld32(c.r64(11).wrapping_add(0x137)) else { return RegfileExit::fault(0x70008837dc); };
            c.w32(11, t294);
            let Some(t295) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008837e3); };
            c.cmp(t295, c.r32(11), 32);
            if c.cc_ne() { blk = 249; continue; }
            blk = 247; continue;
        }
        247 => {
            let Some(t296) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70008837e9); };
            c.w64(9, t296);
            c.w64(11, c.r64(2));
            let Some(t297) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008837f3); };
            c.cmp(t297, c.r32(11), 32);
            if c.cc_e() { blk = 249; continue; }
            blk = 248; continue;
        }
        248 => {
            c.w32(9, 0x1u64);
            blk = 250; continue;
        }
        249 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 250; continue;
        }
        250 => {
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000883804); };
            c.w64(11, t298);
            let r299 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r299);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700088380c); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 252; continue; }
            blk = 251; continue;
        }
        251 => {
            let Some(t300) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000883816); };
            c.w32(9, t300);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_e() { blk = 254; continue; }
            blk = 252; continue;
        }
        252 => {
            let Some(t301) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700088381f); };
            c.w64(3, t301);
            c.w64(1, c.r64(8));
            return c.call(0x7000871cb4, 0x700088382e, 0x7000883829);
        }
        253 => {
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088382e); };
            c.w64(11, t302);
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000883832); };
            c.w64(0, t303);
            blk = 254; continue;
        }
        254 => {
            c.w64(8, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088383c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088383d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000883843); }
            return c.ret(0x0, 0x7000883845);
        }
        255 => {
            c.w64(0, c.r64(3));
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883849); };
            c.w64(6, t304);
            return c.call(0x7000827a94, 0x7000883852, 0x700088384d);
        }
        256 => {
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000883852); };
            c.w64(8, t305);
            let r306 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r306);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088385d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700088385e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000883864); }
            return c.ret(0x0, 0x7000883866);
        }
        257 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000883867); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t307) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700088386d); };
            c.w64(3, t307);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700088387c, 0x7000883877);
        }
        258 => {
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088387c); };
            c.w64(6, t308);
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000883880); };
            c.w64(0, t309);
            blk = 1; continue;
        }
        259 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088388f); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x7000883891); }
            let Some(t310) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000883893); };
            c.w64(3, t310);
            c.w32(0, 0x2u64);
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088389f); };
            c.w64(6, t311);
            return c.call(0x700086ed74, 0x70008838a8, 0x70008838a3);
        }
        260 => {
            return RegfileExit::fault(0x70008838a8);
        }
        261 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008838ae); }
            return c.call(0x70007febf4, 0x70008838b7, 0x70008838b2);
        }
        262 => {
            c.w64(9, c.r64(0));
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008838ba); };
            c.w64(6, t312);
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008838be); };
            c.w64(0, t313);
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008838c2); };
            c.w64(8, t314);
            blk = 109; continue;
        }
        263 => {
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008838cb); };
            if !c.push64(t315) { return RegfileExit::fault(0x70008838cb); }
            c.w32(0, 0x1u64);
            let Some(t316) = c.ld64(c.r64(13).wrapping_add(0x21f0)) else { return RegfileExit::fault(0x70008838d3); };
            c.w64(3, t316);
            return c.call(0x700086ed74, 0x70008838df, 0x70008838da);
        }
        264 => {
            return RegfileExit::fault(0x70008838df);
        }
        265 => {
            blk = 288; continue;
        }
        266 => {
            let Some(t317) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008838e5); };
            c.logic(t317 & 0x7ffffffeu64, 32);
            if c.cc_ne() { blk = 152; continue; }
            blk = 267; continue;
        }
        267 => {
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008838f2); };
            c.w64(0, t318);
            blk = 144; continue;
        }
        268 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008838fb); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 152; continue; }
            blk = 269; continue;
        }
        269 => {
            let Some(t319) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088390e); };
            c.w64(0, t319);
            blk = 144; continue;
        }
        270 => {
            let Some(t320) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000883917); };
            c.logic(t320 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 271; continue;
        }
        271 => {
            blk = 152; continue;
        }
        272 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700088392a); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 152; continue; }
            blk = 273; continue;
        }
        273 => {
            blk = 144; continue;
        }
        274 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000883943); }
            let Some(t321) = c.ld64(c.r64(13).wrapping_add(0x9e8)) else { return RegfileExit::fault(0x7000883947); };
            c.w64(3, t321);
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088394e); };
            c.w64(0, t322);
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883952); };
            c.w64(6, t323);
            return c.call(0x7000870914, 0x700088395b, 0x7000883956);
        }
        275 => {
            let Some(t324) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088395b); };
            c.cmp(t324, c.r32(0), 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 276; continue;
        }
        276 => {
            let Some(t325) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000883964); };
            c.cmp(t325, c.r32(0), 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 277; continue;
        }
        277 => {
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088396d); };
            if !c.push64(t326) { return RegfileExit::fault(0x700088396d); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883975); };
            c.w64(6, t327);
            return c.call(0x70007f2274, 0x700088397e, 0x7000883979);
        }
        278 => {
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088397e); };
            if !c.push64(t328) { return RegfileExit::fault(0x700088397e); }
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883981); };
            c.w64(6, t329);
            let Some(t330) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x7000883985); };
            c.w64(3, t330);
            c.w32(0, 0x1u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000883999, 0x7000883994);
        }
        279 => {
            return RegfileExit::fault(0x7000883999);
        }
        280 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700088399a); }
            let Some(t331) = c.ld64(c.r64(13).wrapping_add(0x9e8)) else { return RegfileExit::fault(0x700088399e); };
            c.w64(3, t331);
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008839a5); };
            c.w64(0, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008839a9); };
            c.w64(6, t333);
            return c.call(0x7000870914, 0x70008839b2, 0x70008839ad);
        }
        281 => {
            let Some(t334) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008839b2); };
            c.cmp(t334, c.r32(0), 32);
            if c.cc_e() { blk = 284; continue; }
            blk = 282; continue;
        }
        282 => {
            let Some(t335) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008839bb); };
            c.cmp(t335, c.r32(0), 32);
            if c.cc_e() { blk = 284; continue; }
            blk = 283; continue;
        }
        283 => {
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008839c4); };
            if !c.push64(t336) { return RegfileExit::fault(0x70008839c4); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008839cc); };
            c.w64(6, t337);
            return c.call(0x70007f2274, 0x70008839d5, 0x70008839d0);
        }
        284 => {
            let Some(t338) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008839d5); };
            if !c.push64(t338) { return RegfileExit::fault(0x70008839d5); }
            let Some(t339) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x70008839d8); };
            c.w64(3, t339);
            c.w32(0, 0x1u64);
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008839e4); };
            c.w64(6, t340);
            return c.call(0x700086ed74, 0x70008839ed, 0x70008839e8);
        }
        285 => {
            return RegfileExit::fault(0x70008839ed);
        }
        286 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008839ee); }
            let Some(t341) = c.ld64(c.r64(13).wrapping_add(0x2200)) else { return RegfileExit::fault(0x70008839ef); };
            c.w64(3, t341);
            c.w32(0, 0x1u64);
            let Some(t342) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008839fb); };
            c.w64(6, t342);
            return c.call(0x700086ed74, 0x7000883a04, 0x70008839ff);
        }
        287 => {
            return RegfileExit::fault(0x7000883a04);
        }
        288 => {
            let Some(t343) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x7000883a05); };
            c.w64(3, t343);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t344) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883a0e); };
            c.w64(6, t344);
            return c.call(0x700086ed74, 0x7000883a17, 0x7000883a12);
        }
        289 => {
            return RegfileExit::fault(0x7000883a17);
        }
        290 => {
            blk = 292; continue;
        }
        291 => {
            let Some(t345) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000883a1a); };
            c.w64(0, t345);
            blk = 292; continue;
        }
        292 => {
            let Some(t346) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883a1e); };
            c.w64(6, t346);
            return c.call(0x70008a2054, 0x7000883a27, 0x7000883a22);
        }
        293 => {
            blk = 174; continue;
        }
        294 => {
            let Some(t347) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000883a2c); };
            c.w32(8, t347);
            let r348 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r348);
            blk = 195; continue;
        }
        295 => {
            let Some(t349) = c.ld32(c.r64(8).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x7000883a38); };
            c.cmp(t349, c.r32(11), 32);
            if c.cc_e() { blk = 194; continue; }
            blk = 296; continue;
        }
        296 => {
            let Some(t350) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000883a45); };
            c.w64(3, t350);
            return c.call(0x7000870914, 0x7000883a51, 0x7000883a4c);
        }
        297 => {
            c.w64(8, c.r64(0));
            let Some(t351) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883a54); };
            c.w64(6, t351);
            let Some(t352) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000883a58); };
            c.w64(0, t352);
            blk = 195; continue;
        }
        298 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000883a61); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000883a65); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x7000883a69); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x7000883a75, 0x7000883a70);
        }
        299 => {
            let Some(t353) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000883a75); };
            c.w64(15, t353);
            let Some(t354) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883a79); };
            c.w64(6, t354);
            let Some(t355) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000883a7d); };
            c.w64(8, t355);
            let Some(t356) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000883a81); };
            c.w64(9, t356);
            let Some(t357) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000883a85); };
            c.w64(11, t357);
            let Some(t358) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000883a89); };
            c.w64(12, t358);
            blk = 213; continue;
        }
        300 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000883a97); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000883a9b); }
            return c.call(0x70007febf4, 0x7000883aa4, 0x7000883a9f);
        }
        301 => {
            c.w64(15, c.r64(0));
            let Some(t359) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883aa7); };
            c.w64(6, t359);
            let Some(t360) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000883aab); };
            c.w64(8, t360);
            let Some(t361) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000883aaf); };
            c.w64(12, t361);
            let Some(t362) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000883ab3); };
            c.w64(0, t362);
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000883ab7); };
            c.w64(9, t363);
            let Some(t364) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000883abb); };
            c.w64(11, t364);
            blk = 216; continue;
        }
        302 => {
            let Some(t365) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000883ac4); };
            c.w32(11, t365);
            let r366 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r366);
            let Some(t367) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000883acb); };
            c.w32(11, t367);
            let r368 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r368);
            let Some(t369) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000883ad2); };
            c.w32(12, t369);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x2bu64, 64);
            if c.cc_a() { blk = 304; continue; }
            blk = 303; continue;
        }
        303 => {
            return RegfileExit::fault(0x7000883ae2);
        }
        304 => {
            let Some(t370) = c.ld32(c.r64(11).wrapping_add(0xb3)) else { return RegfileExit::fault(0x7000883ae3); };
            c.w32(11, t370);
            let r371 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r371);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000883aed); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000883aef); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000883af1); }
            let Some(t372) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000883af3); };
            c.w64(11, t372);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000883afa); }
            let Some(t373) = c.ld64(c.r64(13).wrapping_add(0x1c98)) else { return RegfileExit::fault(0x7000883afc); };
            c.w64(3, t373);
            c.w32(0, 0x4u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x7000883b10, 0x7000883b0b);
        }
        305 => {
            c.w64(15, c.r64(0));
            let Some(t374) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883b13); };
            c.w64(6, t374);
            let Some(t375) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000883b17); };
            c.w64(8, t375);
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t376) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000883b1d); };
            c.w64(9, t376);
            blk = 221; continue;
        }
        306 => {
            return RegfileExit::jump(0x7000883b28);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000883b34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000883b34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000883b38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000883b39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000883b3a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000883b3f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000883b49); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000883b52); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000883b58); };
            c.cmp(t5, 0x834u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000883b60); };
            c.w64(0, t6);
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000883b67); };
            c.w64(1, t7);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000883b6e); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x10, 0x7000883b75);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000883b78); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000883b7f); }
            return c.ret(0x0, 0x7000883b81);
        }
        7 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 9; continue;
        }
        8 => {
            c.w32(1, 0x1u64);
            blk = 9; continue;
        }
        9 => {
            c.w32(0, 0x1u64);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000883b95); };
            c.w32(1, t8);
            let r9 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r9);
            let Some(t10) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000883b9b); };
            c.cmp(t10, 0xbcu64, 16);
            if c.cc_ne() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000883ba3); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x1c78)) else { return RegfileExit::fault(0x7000883ba4); };
            c.w64(3, t11);
            return c.call(0x700086ed74, 0x7000883bb0, 0x7000883bab);
        }
        12 => {
            blk = 4; continue;
        }
        13 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000883bb2); };
            c.w64(0, t12);
            blk = 4; continue;
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000883bbb); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000883bc1); };
            c.w64(3, t13);
            return c.call(0x700086ed74, 0x7000883bcd, 0x7000883bc8);
        }
        15 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000883bcd); };
            c.w64(6, t14);
            blk = 1; continue;
        }
        16 => {
            return RegfileExit::jump(0x7000883bd8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000883bf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000883bf4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000883bf8); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000883bfe); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000883c04); };
            c.w32(8, t3);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(2, c.r32(3));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r4 = c.sub(c.r64(8), 0x1u64, 64);
            c.w64(8, r4);
            c.w64(10, c.r64(4));
            let r5 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r5);
            let r6 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r6);
            c.logic(r6, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000883c1f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000883c24); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x7000883c28); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000883c2c); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000883c30); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000883c34); }
            c.w64(1, c.r64(13));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x7000883c3b); };
            c.w64(0, t7);
            c.w64(10, 0x7000883c49u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000883c49); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000883c4d); }
            return c.call(c.r64(0), 0x7000883c53, 0x7000883c51);
        }
        1 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000883c53); }
            let Some(t8) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000883c5b); };
            c.w64(4, t8);
            c.w32(7, c.r32(0));
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000883c62); };
            let r10 = c.r64(7) & t9;
            c.w64(7, r10);
            c.logic(r10, 64);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000883c69); };
            c.w64(9, t11);
            blk = 3; continue;
        }
        2 => {
            let r12 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r12);
            let r13 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000883c76); };
            c.w64(11, t14);
            let r15 = c.r64(11) & c.r64(7);
            c.w64(11, r15);
            c.logic(r15, 64);
            c.w64(7, c.r64(11));
            blk = 3; continue;
        }
        3 => {
            c.w64(11, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x4));
            c.w64(12, c.r64(11).wrapping_mul(4));
            let Some(t16) = c.ld32(c.r64(9).wrapping_add(c.r64(12)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000883c8d); };
            c.w32(15, t16);
            let r17 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000883c95); };
            c.cmp(t18, c.r32(15), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000883ca8); };
            c.cmp(t19, c.r32(15), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 6; continue;
        }
        6 => {
            c.xorpd_rr(0, 0);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883cb5); };
            c.cvtsi2sd(0, (t20 as i64));
            if !c.movsd_rm(1, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000883cbb); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 2; continue; }
            blk = 7; continue;
        }
        7 => {
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            blk = 2; continue;
        }
        9 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883cd1); };
            c.cmp(c.r64(15), t21, 64);
            if c.cc_ne() { blk = 2; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t22) = c.ld32(c.r64(9).wrapping_add(c.r64(12)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000883cd7); };
            c.w32(7, t22);
            c.logic(c.r8(7) & 0x2u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883ce2); };
            c.cmp(t23, 0x3fffffffu64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883cec); };
            c.w64(7, t24);
            let r25 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r25);
            blk = 13; continue;
        }
        13 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000883cf2); };
            c.w64(0, t26);
            c.w64(3, c.r64(7));
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000883cf9); };
            c.w64(6, t27);
            return c.call(0x7000870914, 0x7000883d02, 0x7000883cfd);
        }
        14 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000883d05); }
            return c.ret(0x0, 0x7000883d06);
        }
        15 => {
            let Some(t28) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000883d07); };
            c.w32(7, t28);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ae() { blk = 27; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t29) = c.ld32(c.r64(9).wrapping_add(c.r64(12)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000883d1e); };
            c.w32(0, t29);
            let r30 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r30);
            blk = 14; continue;
        }
        17 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000883d28); };
            c.w64(0, t31);
            blk = 14; continue;
        }
        18 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000883d31); };
            c.w64(7, t32);
            let Some(t33) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000883d38); };
            c.w64(8, t33);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000883d3f); };
            c.w64(11, t34);
            let Some(t35) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000883d46); };
            c.cmp(t35, c.r64(9), 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000883d55, 0x7000883d50);
        }
        20 => {
            blk = 22; continue;
        }
        21 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x7000883d57); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 22; continue;
        }
        22 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883d61); };
            c.w64(10, t36);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.logic(c.r64(10) & c.r64(10), 64);
            if c.cc_ns() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            let r37 = c.r64(10) | 0x1u64;
            c.w64(10, r37);
            c.logic(r37, 64);
            blk = 25; continue;
        }
        25 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 26; continue;
        }
        26 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000883d89); };
            c.w64(7, t38);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000883d90); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000883d93); }
            c.w64(7, c.r64(0));
            blk = 13; continue;
        }
        27 => {
            return RegfileExit::fault(0x7000883da0);
        }
        28 => {
            return RegfileExit::jump(0x7000883da4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000883db4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000883db4); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000883dba); };
            c.w32(1, t2);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(3), c.r64(1), 64);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x7000883dc8);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000883dc9); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000883dd0); };
            c.cmp(t5, c.r32(0), 32);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return c.ret(0x0, 0x7000883dd9);
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000883dda); };
            c.w64(0, t6);
            return c.ret(0x0, 0x7000883de1);
        }
        5 => {
            return RegfileExit::jump(0x7000883de4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000883df4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000883df4); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000883dfa); };
            c.w32(6, t2);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(3), c.r64(6), 64);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x7000883e08);
        }
        2 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(c.r64(3).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x7000883e09); }
            c.w64(6, 0xfff7fffffff7ffffu64);
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000883e19); };
            c.cmp(t3, c.r64(6), 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000883e20); };
            c.w64(3, t4);
            let Some(t5) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000883e27); };
            c.w64(1, t5);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000883e2e); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000883e35); };
            c.cmp(t7, c.r64(6), 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x7000883e3a); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000883e41); };
            c.w64(1, t8);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000883e48); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000883e4b); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x7000883e53);
        }
        6 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000883e54); };
            c.w64(0, t9);
            return c.ret(0x0, 0x7000883e5b);
        }
        7 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000883e5c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000883e60); }
            let r10 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r10);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x7000883e6b); }
            return c.call(0x70007febf4, 0x7000883e75, 0x7000883e70);
        }
        8 => {
            c.w64(3, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x7000883e78); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000883e80); }
            blk = 5; continue;
        }
        9 => {
            return RegfileExit::jump(0x7000883e84);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000883e94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000883e94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000883e98); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000883e9e); };
            c.cmp(t1, c.r32(0), 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000883eab); };
            c.cmp(t2, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x7000883eba); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x7000883ebe); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x7000883ec2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000883ec6); }
            let r3 = c.r32(2) | c.r32(8);
            c.w32(2, r3);
            c.logic(r3, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000883ed3); };
            c.w64(2, t4);
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x12f)) else { return RegfileExit::fault(0x7000883eda); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            c.w64(3, c.r64(2));
            return c.call(0x7000870914, 0x7000883eeb, 0x7000883ee6);
        }
        2 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000883ef3); };
            c.w32(2, t7);
            let r8 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r8);
            let Some(t9) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000883ef9); };
            c.logic(t9 & 0x2u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000883f03); };
            c.w64(2, t10);
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000883f07); };
            c.cmp(t11, c.r32(2), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000883f14); };
            c.cmp(t12, c.r32(2), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let r13 = c.r32(1) | c.r32(8);
            c.w32(1, r13);
            c.logic(r13, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000883f28); };
            c.w64(1, t14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000883f2c); };
            c.cmp(t15, c.r32(1), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000883f3b); };
            c.cmp(t16, c.r32(1), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let r17 = c.r32(8) | c.r32(9);
            c.w32(8, r17);
            c.logic(r17, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000883f4f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000883f50); }
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000883f51); };
            if !c.push64(t18) { return RegfileExit::fault(0x7000883f51); }
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000883f54); };
            c.w64(6, t19);
            c.w64(7, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(2, c.r64(0));
            return c.call(0x70007f2274, 0x7000883f68, 0x7000883f63);
        }
        7 => {
            blk = 11; continue;
        }
        8 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000883f6a); }
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000883f6b); };
            if !c.push64(t20) { return RegfileExit::fault(0x7000883f6b); }
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000883f6e); };
            c.w64(6, t21);
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(2, c.r64(0));
            return c.call(0x70007f2274, 0x7000883f82, 0x7000883f7d);
        }
        9 => {
            blk = 11; continue;
        }
        10 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000883f84); };
            if !c.push64(t22) { return RegfileExit::fault(0x7000883f84); }
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000883f87); };
            c.w64(6, t23);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x70007f2274, 0x7000883f98, 0x7000883f93);
        }
        11 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t24) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000883f9c); };
            c.w32(2, t24);
            let r25 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r25);
            let Some(t26) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000883fa2); };
            c.cmp(t26, 0x40u64, 16);
            if c.cc_ae() { blk = 17; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000883fac); }
            return c.ret(0x0, 0x7000883fad);
        }
        14 => {
            c.w32(2, 0x32u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000883fb3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000883fb4); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000883fb5); };
            c.w64(3, t27);
            c.w32(0, 0x2u64);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000883fc1); };
            c.w64(6, t28);
            return c.call(0x700086ed74, 0x7000883fca, 0x7000883fc5);
        }
        15 => {
            return RegfileExit::fault(0x7000883fca);
        }
        16 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000883fcb); };
            c.w64(0, t29);
            blk = 13; continue;
        }
        17 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000883fd4); };
            c.w64(6, t30);
            return c.call(0x700089c794, 0x7000883fdd, 0x7000883fd8);
        }
        18 => {
            blk = 13; continue;
        }
        19 => {
            return RegfileExit::jump(0x7000883fe0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000883ff4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000883ff4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000883ff8); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000883ffe); };
            c.w32(1, t1);
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(7));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700088400a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700088400e); }
            c.w32(9, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(9));
            blk = 2; continue;
        }
        1 => {
            let r2 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r2);
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ge() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 34; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000884034); };
            c.w32(15, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000884039); };
            c.cmp(t4, c.r32(15), 32);
            if c.cc_e() { blk = 36; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(15), c.r32(3), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 1; continue;
        }
        7 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_b() { blk = 39; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(11, c.r64(8));
            if let Some(sr) = c.shr(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r5 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r5);
            let r6 = c.add(c.r64(8), 0x10u64, 64);
            c.w64(8, r6);
            if c.cc_e() { blk = 23; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 44; continue; }
            blk = 10; continue;
        }
        10 => {
            let r7 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r7);
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w32(11, c.r32(11));
            let r8 = c.imul2(c.r32(11), 0x4u64, 32);
            c.w32(11, r8);
            if c.cc_o() { blk = 22; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w32(12, 0x8u64);
            let r9 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r9);
            if c.cc_o() { blk = 21; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008840a6); };
            c.w64(15, t10);
            let Some(t11) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008840ad); };
            c.w64(2, t11);
            c.w64(6, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008840b4); };
            c.w64(3, t12);
            let Some(t13) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008840bb); };
            c.cmp(t13, c.r64(6), 64);
            if c.cc_be() { blk = 42; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 42; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(15), c.r64(6)) { return RegfileExit::fault(0x70008840d1); }
            c.w64(11, c.r64(2).wrapping_add(0x1));
            blk = 15; continue;
        }
        15 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008840d8); };
            c.w64(15, t14);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008840df); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008840e3); }
            c.w64(8, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008840ef); };
            c.w64(15, t15);
            c.w64(9, c.r64(12));
            blk = 17; continue;
        }
        16 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(12)).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008840fb); }
            let r16 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r16);
            c.w64(9, c.r64(3));
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_e() { blk = 24; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(3, c.r64(7).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t17) = c.ld32(c.r64(0).wrapping_add(c.r64(9)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000884119); };
            c.w32(3, t17);
            let r18 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r18);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(3));
            c.w64(3, c.r64(10));
            blk = 16; continue;
        }
        20 => {
            c.w64(3, c.r64(9));
            c.w64(9, c.r64(15));
            blk = 16; continue;
        }
        21 => {
            return RegfileExit::fault(0x7000884135);
        }
        22 => {
            return RegfileExit::fault(0x7000884136);
        }
        23 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000884137); };
            c.w64(0, t19);
            c.w64(11, c.r64(0));
            blk = 24; continue;
        }
        24 => {
            let Some(t20) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000884141); };
            c.w32(8, t20);
            c.cmp(c.r32(1), c.r32(8), 32);
            if c.cc_b() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            return RegfileExit::fault(0x700088414a);
        }
        26 => {
            c.w64(1, 0xffffffffffffffffu64);
            c.w64(1, c.r64(1).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x8));
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000884157); };
            c.w64(7, t21);
            if !c.st32(c.r64(11).wrapping_add(c.r64(1)), c.r32(7)) { return RegfileExit::fault(0x700088415b); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r22 = c.r64(8) & c.r64(11);
            c.w64(8, r22);
            c.logic(r22, 64);
            let Some(t23) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088416f); };
            c.logic(t23 & 0x4u64, 8);
            if c.cc_ne() { blk = 52; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088417a); };
            c.w64(1, t24);
            let Some(t25) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088417e); };
            c.w32(1, t25);
            let r26 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r26);
            let Some(t27) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000884184); };
            c.w32(1, t27);
            let r28 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r28);
            let Some(t29) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088418a); };
            c.w32(7, t29);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x2cu64, 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x7000884199);
        }
        30 => {
            if !c.st32(c.r64(1).wrapping_add(0xb7), c.r32(11)) { return RegfileExit::fault(0x700088419a); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r30 = c.r64(7) & c.r64(1);
            c.w64(7, r30);
            c.logic(r30, 64);
            let Some(t31) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008841b1); };
            c.logic(t31 & 0x4u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008841bb); };
            c.w64(0, t32);
            blk = 33; continue;
        }
        33 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008841c5); }
            return c.ret(0x0, 0x70008841c6);
        }
        34 => {
            return RegfileExit::fault(0x70008841c7);
        }
        35 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008841c8); };
            c.w64(0, t33);
            blk = 33; continue;
        }
        36 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(12)), c.r32(3)) { return RegfileExit::fault(0x70008841d1); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r34 = c.r64(1) & c.r64(0);
            c.w64(1, r34);
            c.logic(r34, 64);
            let Some(t35) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008841e4); };
            c.logic(t35 & 0x4u64, 8);
            if c.cc_ne() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008841ea); };
            c.w64(0, t36);
            blk = 33; continue;
        }
        39 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008841f3); };
            c.w64(2, t37);
            let Some(t38) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008841fa); };
            c.w32(2, t38);
            let r39 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r39);
            return c.call(0x7000824234, 0x7000884208, 0x7000884203);
        }
        40 => {
            return RegfileExit::fault(0x7000884208);
        }
        41 => {
            return RegfileExit::fault(0x7000884209);
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700088420a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700088420e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000884212); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000884216); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700088421a); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x7000884226, 0x7000884221);
        }
        43 => {
            c.w64(11, c.r64(0));
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000884229); };
            c.w64(0, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088422d); };
            c.w64(7, t41);
            c.w32(12, 0x8u64);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000884237); };
            c.w64(9, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088423b); };
            c.w64(8, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088423f); };
            c.w64(1, t44);
            blk = 15; continue;
        }
        44 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000884248); };
            c.w64(3, t45);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(9));
            return c.call(0x700086ed74, 0x7000884259, 0x7000884254);
        }
        45 => {
            return RegfileExit::fault(0x7000884259);
        }
        46 => {
            c.w32(3, c.r32(3));
            let r46 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r46);
            c.w64(1, 0xfffffffffffc0000u64);
            let r47 = c.r64(1) & c.r64(3);
            c.w64(1, r47);
            c.logic(r47, 64);
            let Some(t48) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000884269); };
            c.logic(t48 & 0x2u64, 8);
            if c.cc_e() { blk = 38; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(7, c.r64(0).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000884277); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000884278); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(0));
            return c.call(0x70007f0e54, 0x7000884285, 0x7000884280);
        }
        48 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000884285); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000884287); }
            blk = 38; continue;
        }
        49 => {
            c.w32(11, c.r32(11));
            let r49 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r49);
            c.w64(7, 0xfffffffffffc0000u64);
            let r50 = c.r64(7) & c.r64(11);
            c.w64(7, r50);
            c.logic(r50, 64);
            let Some(t51) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088429d); };
            c.logic(t51 & 0x2u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(8, c.r64(1).wrapping_add(0xb7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008842ae); }
            c.w64(7, c.r64(1));
            return c.call(0x70007f0e54, 0x70008842b7, 0x70008842b2);
        }
        51 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008842b7); }
            blk = 32; continue;
        }
        52 => {
            c.w32(7, c.r32(7));
            let r52 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r52);
            c.w64(8, 0xfffffffffffc0000u64);
            let r53 = c.r64(8) & c.r64(7);
            c.w64(8, r53);
            c.logic(r53, 64);
            let Some(t54) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008842cc); };
            c.logic(t54 & 0x2u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(9, c.r64(11).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008842db); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008842dc); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008842e9, 0x70008842e4);
        }
        54 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008842e9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008842eb); }
            blk = 28; continue;
        }
        55 => {
            return RegfileExit::jump(0x70008842f4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000884314(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000884314); };
            c.w32(1, t0);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 2; continue;
        }
        1 => {
            let r1 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r1);
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_ge() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_ae() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088432f); };
            c.cmp(t2, c.r32(3), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            blk = 1; continue;
        }
        6 => {
            return RegfileExit::fault(0x7000884337);
        }
        7 => {
            return RegfileExit::fault(0x7000884338);
        }
        8 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000884345); };
            c.w64(3, t3);
            if !c.st32(c.r64(0).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x700088434c); }
            blk = 11; continue;
        }
        11 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000884350); };
            c.w64(0, t4);
            return c.ret(0x0, 0x7000884357);
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000884358); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700088435c); }
            let r5 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000884362); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000884369); };
            c.w64(1, t7);
            c.w64(7, c.r64(1).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x7000884370); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000884374); };
            c.w64(8, t8);
            let Some(t9) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700088437b); };
            c.cmp(t9, c.r64(7), 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700088438a, 0x7000884385);
        }
        14 => {
            blk = 16; continue;
        }
        15 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x700088438c); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            c.w64(0, c.r64(3));
            blk = 16; continue;
        }
        16 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000884396); };
            c.w64(3, t10);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700088439d); }
            c.w32(3, 0x4u64);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008843a5); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008843a8); };
            c.w64(3, t11);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008843af); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008843b2); }
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008843b5); };
            c.w64(6, t12);
            let Some(t13) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008843b9); };
            c.w32(3, t13);
            let r14 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r14);
            let Some(t15) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008843bf); };
            c.w32(3, t15);
            let r16 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r16);
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008843c5); };
            c.w32(1, t17);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x2cu64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x70008843d4);
        }
        18 => {
            if !c.st32(c.r64(3).wrapping_add(0xb7), c.r32(0)) { return RegfileExit::fault(0x70008843d5); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r18 = c.r64(1) & c.r64(3);
            c.w64(1, r18);
            c.logic(r18, 64);
            let Some(t19) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008843e9); };
            c.logic(t19 & 0x4u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008843f2); }
            blk = 11; continue;
        }
        21 => {
            c.w32(0, c.r32(0));
            let r20 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r20);
            c.w64(1, 0xfffffffffffc0000u64);
            let r21 = c.r64(1) & c.r64(0);
            c.w64(1, r21);
            c.logic(r21, 64);
            let Some(t22) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000884407); };
            c.logic(t22 & 0x2u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(6, c.r64(3).wrapping_add(0xb7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000884414); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000884415); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x7000884422, 0x700088441d);
        }
        23 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000884422); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000884424); }
            blk = 20; continue;
        }
        24 => {
            return RegfileExit::jump(0x7000884428);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000884434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000884434); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000884438); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000884439); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088443a); }
            let r0 = c.sub(c.r64(4), 0xd0u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000884442); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000884446); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 266; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(5));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(7)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088445a); };
            c.w64(11, t2);
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000884462); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088446c); };
            c.w64(7, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000884472); };
            c.w64(7, t4);
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(5).wrapping_sub(0xe8), c.r64(7)) { return RegfileExit::fault(0x7000884479); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 268; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088448a); };
            c.w32(15, t5);
            let r6 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r6);
            let Some(t7) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000884491); };
            c.cmp(t7, 0xbau64, 16);
            if c.cc_b() { blk = 268; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008844aa); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t8) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008844b4); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            let Some(t10) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008844bb); };
            c.cmp(t10, 0x834u64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008844c3); };
            c.w32(15, t11);
            let r12 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r12);
            c.w64(0, c.r64(15));
            blk = 32; continue;
        }
        9 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 11; continue;
        }
        10 => {
            c.w32(0, 0x1u64);
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t13) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008844e0); };
            c.w32(0, t13);
            let r14 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r14);
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008844e6); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            let Some(t17) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008844ec); };
            c.w32(3, t17);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t18) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008844f2); };
            c.w32(2, t18);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0xb0u64, 64);
            if c.cc_a() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return RegfileExit::fault(0x7000884502);
        }
        14 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x7000884503); };
            c.cmp(t19, c.r32(2), 32);
            if c.cc_e() { blk = 270; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 272; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t20) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000884519); };
            c.w32(15, t20);
            let r21 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r21);
            let Some(t22) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000884520); };
            c.w32(15, t22);
            let r23 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r23);
            let Some(t24) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000884527); };
            c.w32(0, t24);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t25) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088452e); };
            c.w32(3, t25);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x44u64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x700088453b);
        }
        18 => {
            let Some(t26) = c.ld32(c.r64(15).wrapping_add(0x117)) else { return RegfileExit::fault(0x700088453c); };
            c.cmp(t26, c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(0), 0xaeu64, 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            return RegfileExit::fault(0x700088454d);
        }
        21 => {
            let Some(t27) = c.ld32(c.r64(15).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x700088454e); };
            c.cmp(t27, c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r64(0), 0xafu64, 64);
            if c.cc_a() { blk = 271; continue; }
            blk = 23; continue;
        }
        23 => {
            return RegfileExit::fault(0x7000884563);
        }
        24 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000884564); };
            c.w32(15, t28);
            let r29 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r29);
            c.w64(0, c.r64(15));
            blk = 25; continue;
        }
        25 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            return c.call(0x70008a17f4, 0x7000884577, 0x7000884572);
        }
        27 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0xe8)) else { return RegfileExit::fault(0x7000884577); };
            c.w64(7, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088457e); };
            c.w64(6, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000884582); };
            c.w64(11, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000884586); };
            c.w64(12, t33);
            blk = 32; continue;
        }
        28 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(2, c.r64(0));
            blk = 31; continue;
        }
        30 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 31; continue;
        }
        31 => {
            c.w64(0, c.r64(2));
            blk = 32; continue;
        }
        32 => {
            c.w32(15, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            if !c.st64(c.r64(5).wrapping_sub(0xb0), c.r64(0)) { return RegfileExit::fault(0x70008845a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008845ad); }
            if !c.st64(c.r64(5).wrapping_sub(0xb8), c.r64(3)) { return RegfileExit::fault(0x70008845b1); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008845b8); };
            c.w64(2, t34);
            let Some(t35) = c.ld32(c.r64(2).wrapping_add(0x133)) else { return RegfileExit::fault(0x70008845bf); };
            c.w32(2, t35);
            let r36 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r36);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008845cd); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008845d7); }
            blk = 38; continue;
        }
        35 => {
            c.w64(2, c.r64(0));
            let r37 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r37);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 38; continue;
        }
        36 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 37; continue;
        }
        37 => {
            let r38 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r38);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008845f4); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 38; continue;
        }
        38 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 264; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 41; continue;
        }
        40 => {
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_l() { blk = 264; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000884615); };
            c.cmp(t39, c.r32(7), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 274; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t40) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000884628); };
            c.w32(2, t40);
            let r41 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r41);
            let Some(t42) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088462e); };
            c.cmp(t42, 0x40u64, 16);
            if c.cc_b() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            blk = 274; continue;
        }
        45 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700088463a); };
            c.w64(7, t43);
            let Some(t44) = c.ld32(c.r64(7).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000884641); };
            c.w32(7, t44);
            let r45 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r45);
            blk = 46; continue;
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0xe8), c.r64(7)) { return RegfileExit::fault(0x700088464a); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000884656); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 50; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 72; continue;
        }
        49 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_le() { blk = 72; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t46) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000884673); };
            c.w32(2, t46);
            let r47 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r47);
            let Some(t48) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000884679); };
            c.w32(2, t48);
            let r49 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r49);
            let Some(t50) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088467f); };
            c.w32(1, t50);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x2cu64, 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            return RegfileExit::fault(0x700088468e);
        }
        52 => {
            let Some(t51) = c.ld32(c.r64(2).wrapping_add(0xb7)) else { return RegfileExit::fault(0x700088468f); };
            c.w32(2, t51);
            let r52 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r52);
            let Some(t53) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000884698); };
            c.cmp(t53, c.r32(2), 32);
            if c.cc_ne() { blk = 62; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008846a5); };
            c.w64(2, t54);
            let Some(t55) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008846ac); };
            c.w64(1, t55);
            c.w64(8, c.r64(1).wrapping_add(0x10));
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008846b3); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008846ba); };
            c.cmp(t57, c.r64(8), 64);
            if c.cc_be() { blk = 276; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st64(c.r64(2), c.r64(8)) { return RegfileExit::fault(0x70008846c3); }
            c.w64(2, c.r64(1).wrapping_add(0x1));
            blk = 55; continue;
        }
        55 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008846ca); };
            c.w64(1, t58);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008846d1); }
            c.w32(1, 0x4u64);
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008846d9); }
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008846dc); };
            c.w64(1, t59);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008846e3); }
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008846e6); }
            let Some(t60) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008846e9); };
            c.w32(1, t60);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_a() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            return RegfileExit::fault(0x70008846f8);
        }
        57 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008846f9); }
            let Some(t61) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008846fd); };
            c.w32(1, t61);
            let r62 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r62);
            let Some(t63) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000884703); };
            c.w32(1, t63);
            let r64 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r64);
            let Some(t65) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000884709); };
            c.w32(8, t65);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x2cu64, 64);
            if c.cc_a() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            return RegfileExit::fault(0x7000884719);
        }
        59 => {
            if !c.st32(c.r64(1).wrapping_add(0xb7), c.r32(2)) { return RegfileExit::fault(0x700088471a); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r66 = c.r64(8) & c.r64(1);
            c.w64(8, r66);
            c.logic(r66, 64);
            let Some(t67) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088472f); };
            c.logic(t67 & 0x4u64, 8);
            if c.cc_ne() { blk = 470; continue; }
            blk = 61; continue;
        }
        61 => {
            blk = 70; continue;
        }
        62 => {
            let Some(t68) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088473f); };
            c.w32(1, t68);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_a() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            return RegfileExit::fault(0x700088474e);
        }
        64 => {
            let Some(t69) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088474f); };
            c.w32(1, t69);
            let Some(t70) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000884752); };
            c.cmp(t70, c.r32(1), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(0, c.r64(2));
            c.w64(3, c.r64(11));
            return c.call(0x7000883ff4, 0x7000884766, 0x7000884761);
        }
        66 => {
            let Some(t71) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000884766); };
            c.cmp(t71, c.r32(0), 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000884773); };
            c.w64(6, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000884777); };
            c.w64(11, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088477b); };
            c.w64(12, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0xe8)) else { return RegfileExit::fault(0x700088477f); };
            c.w64(7, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x7000884786); };
            c.w64(0, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x700088478d); };
            c.w64(3, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000884794); };
            c.w64(15, t78);
            blk = 70; continue;
        }
        68 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700088479a); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r79 = c.r64(1) & c.r64(2);
            c.w64(1, r79);
            c.logic(r79, 64);
            let Some(t80) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008847ae); };
            c.logic(t80 & 0x4u64, 8);
            if c.cc_ne() { blk = 467; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w32(2, 0x1u64);
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(7, c.r64(12).wrapping_add(0x1));
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008847c7); };
            c.w64(0, t81);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008847d1); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008847d2); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008847d8); }
            return c.ret(0x0, 0x70008847da);
        }
        73 => {
            let Some(t82) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008847db); };
            c.w32(2, t82);
            let r83 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r83);
            let Some(t84) = c.ld8(c.r64(2).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008847e2); };
            c.w32(1, t84);
            if !c.st64(c.r64(5).wrapping_sub(0xc0), c.r64(2)) { return RegfileExit::fault(0x70008847e6); }
            let Some(t85) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008847ed); };
            c.cmp(t85, 0x834u64, 16);
            if c.cc_ne() { blk = 97; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t86) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008847f9); };
            c.w32(8, t86);
            let r87 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r87);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 78; continue; }
            blk = 75; continue;
        }
        75 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000884806); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000884811); }
            blk = 80; continue;
        }
        77 => {
            c.w64(8, c.r64(0));
            let r88 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r88);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            blk = 80; continue;
        }
        78 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 79; continue;
        }
        79 => {
            let r89 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r89);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000884831); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 80; continue;
        }
        80 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 84; continue; }
            blk = 81; continue;
        }
        81 => {
            if c.cc_e() { blk = 85; continue; }
            blk = 82; continue;
        }
        82 => {
            blk = 84; continue;
        }
        83 => {
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 86; continue;
        }
        85 => {
            c.w32(8, 0x1u64);
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t90) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000884863); };
            c.w32(8, t90);
            let r91 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r91);
            let Some(t92) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088486a); };
            c.w32(8, t92);
            let r93 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r93);
            let Some(t94) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000884871); };
            c.w32(9, t94);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4cu64, 64);
            if c.cc_a() { blk = 89; continue; }
            blk = 88; continue;
        }
        88 => {
            return RegfileExit::fault(0x7000884881);
        }
        89 => {
            let Some(t95) = c.ld32(c.r64(8).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000884882); };
            c.w32(8, t95);
            let Some(t96) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000884889); };
            c.cmp(t96, c.r32(8), 32);
            if c.cc_ne() { blk = 97; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700088488f); };
            c.w64(8, t97);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t98) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000884899); };
            c.cmp(t98, c.r32(9), 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w64(8, c.r64(1));
            let r99 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r99);
            let r100 = c.r32(8) & 0x3fu64;
            c.w32(8, r100);
            c.logic(r100, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_le() { blk = 96; continue; }
            blk = 92; continue;
        }
        92 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_le() { blk = 95; continue; }
            blk = 93; continue;
        }
        93 => {
            let r101 = c.r32(1) & 0xfcu64;
            c.w32(1, r101);
            c.logic(r101, 32);
            c.cmp(c.r32(1), 0x30u64, 32);
            if c.cc_e() { blk = 279; continue; }
            blk = 94; continue;
        }
        94 => {
            blk = 97; continue;
        }
        95 => {
            c.w32(15, 0x54cu64);
            blk = 98; continue;
        }
        96 => {
            c.w32(15, 0x54au64);
            blk = 98; continue;
        }
        97 => {
            c.w32(15, 0x8a4u64);
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 98; continue;
        }
        98 => {
            if !c.st64(c.r64(5).wrapping_sub(0xd0), c.r64(15)) { return RegfileExit::fault(0x70008848e0); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 100; continue; }
            blk = 99; continue;
        }
        99 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            blk = 103; continue;
        }
        100 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008848f4); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 103; continue; }
            blk = 101; continue;
        }
        101 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(1, cv);
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_s() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            c.w64(10, 0x8000000000000000u64);
            let r102 = c.r64(1) | c.r64(10);
            c.w64(1, r102);
            c.logic(r102, 64);
            blk = 103; continue;
        }
        103 => {
            if !c.st64(c.r64(5).wrapping_sub(0xc8), c.r64(1)) { return RegfileExit::fault(0x700088492e); }
            c.cmp(c.r64(1), 0x7ffeu64, 64);
            if c.cc_a() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            c.w64(8, c.r64(1));
            blk = 106; continue;
        }
        105 => {
            c.w32(8, 0x7ffeu64);
            blk = 106; continue;
        }
        106 => {
            let Some(t103) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000884949); };
            c.w32(15, t103);
            if !c.st64(c.r64(5).wrapping_sub(0xe0), c.r64(15)) { return RegfileExit::fault(0x700088494d); }
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 439; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000884961); };
            c.w64(7, t104);
            let Some(t105) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000884968); };
            c.w64(7, t105);
            c.w64(0, c.r64(8).wrapping_mul(4));
            c.w64(3, c.r64(0).wrapping_add(0x8));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000884977); }
            c.w64(0, c.r64(3).wrapping_add(c.r64(7)));
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088497f); };
            c.w64(2, t106);
            let Some(t107) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000884986); };
            c.cmp(t107, c.r64(0), 64);
            if c.cc_be() { blk = 307; continue; }
            blk = 108; continue;
        }
        108 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_ae() { blk = 307; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088499c); };
            c.w64(3, t108);
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x70008849a3); }
            let r109 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r109);
            blk = 110; continue;
        }
        110 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008849ae); };
            c.w64(3, t110);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008849b5); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008849b8); }
            c.w64(0, c.r64(7).wrapping_add(0x7));
            c.cmp(c.r64(15), 0x0u64, 64);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.w64(10, c.r64(4));
            let r111 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r111);
            let r112 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r112);
            c.logic(r112, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008849d4); }
            if !c.st64(c.r64(5).wrapping_sub(0xd8), c.r64(7)) { return RegfileExit::fault(0x70008849d9); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x70008849e0); }
            c.w64(1, c.r64(0));
            c.w64(2, c.r64(9));
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008849ea); };
            c.w64(8, t113);
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008849ee); };
            c.w64(0, t114);
            c.w64(10, 0x70008849fcu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008849fc); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000884a00); }
            return c.call(c.r64(0), 0x7000884a06, 0x7000884a04);
        }
        111 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000884a06); }
            let Some(t115) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000884a0e); };
            c.w64(4, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0xe8)) else { return RegfileExit::fault(0x7000884a13); };
            c.w64(7, t116);
            let Some(t117) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000884a1a); };
            c.w32(11, t117);
            let r118 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r118);
            let Some(t119) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000884a21); };
            c.w32(11, t119);
            let r120 = c.r32(11) & 0x8u64;
            c.w32(11, r120);
            c.logic(r120, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000884a36); }
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(2));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x7000884a3f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000884a43); }
            blk = 113; continue;
        }
        112 => {
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000884a49); };
            c.w64(2, t121);
            let r122 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000884a51); };
            c.w64(3, t123);
            blk = 113; continue;
        }
        113 => {
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0xd0)) else { return RegfileExit::fault(0x7000884a55); };
            c.w64(1, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x7000884a5c); };
            c.w64(0, t125);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000884a63); };
            c.w64(6, t126);
            c.w32(7, 0);
            c.logic(0, 32);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x7000884a69); };
            c.w64(9, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0xc0)) else { return RegfileExit::fault(0x7000884a70); };
            c.w64(8, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0xc8)) else { return RegfileExit::fault(0x7000884a77); };
            c.cmp(c.r64(2), t129, 64);
            if c.cc_ae() { blk = 203; continue; }
            blk = 114; continue;
        }
        114 => {
            if !c.st64(c.r64(5).wrapping_sub(0xd8), c.r64(0)) { return RegfileExit::fault(0x7000884a84); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x7000884a8b); }
            c.cmp(c.r32(1), 0x8a4u64, 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 115; continue;
        }
        115 => {
            let Some(t130) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000884a9b); };
            c.cmp(t130, c.r32(8), 32);
            if c.cc_ne() { blk = 130; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t131) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000884aa5); };
            c.w32(11, t131);
            let r132 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r132);
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t133) = c.ld32(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000884ab8); };
            c.cmp(t133, 0x0u64, 32);
            if c.cc_ne() { blk = 120; continue; }
            blk = 117; continue;
        }
        117 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000884ac1); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000884acd); }
            blk = 122; continue;
        }
        119 => {
            let r134 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r134);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 122; continue;
        }
        120 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 125; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w64(12, c.r64(9));
            let r135 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r135);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000884aef); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 122; continue;
        }
        122 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 126; continue; }
            blk = 123; continue;
        }
        123 => {
            if c.cc_e() { blk = 127; continue; }
            blk = 124; continue;
        }
        124 => {
            blk = 126; continue;
        }
        125 => {
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 127; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 128; continue;
        }
        127 => {
            c.w32(11, 0x1u64);
            blk = 128; continue;
        }
        128 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 130; continue; }
            blk = 129; continue;
        }
        129 => {
            let Some(t136) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000884b1e); };
            c.w64(11, t136);
            let Some(t137) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000884b25); };
            c.cmp(t137, c.r32(7), 32);
            if c.cc_ne() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w32(1, 0x8a4u64);
            c.w64(11, c.r64(1));
            blk = 131; continue;
        }
        131 => {
            if !c.st64(c.r64(5).wrapping_sub(0xd0), c.r64(1)) { return RegfileExit::fault(0x7000884b33); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 133; continue; }
            blk = 132; continue;
        }
        132 => {
            let r138 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r138);
            blk = 133; continue;
        }
        133 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000884b44); }
            c.w64(0, c.r64(6));
            c.w64(3, c.r64(2));
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000884b4e); };
            c.w64(6, t139);
            c.w64(15, c.r64(1));
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x7000884b55); };
            return c.call(t140, 0x7000884b5d, 0x7000884b55);
        }
        134 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 138; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t141) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000884b6c); };
            c.w32(11, t141);
            let r142 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r142);
            let Some(t143) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000884b73); };
            c.cmp(t143, 0x3fu64, 16);
            if c.cc_a() { blk = 138; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t144) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000884b7b); };
            c.cmp(t144, c.r32(0), 32);
            if c.cc_e() { blk = 112; continue; }
            blk = 137; continue;
        }
        137 => {
            blk = 187; continue;
        }
        138 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 140; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t145) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000884b92); };
            c.w32(11, t145);
            let Some(t146) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000884b96); };
            c.cmp(t146, c.r32(11), 32);
            if c.cc_ne() { blk = 185; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x10f0)) else { return RegfileExit::fault(0x7000884ba3); };
            c.w64(11, t147);
            let Some(t148) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000884baa); };
            c.w32(12, t148);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(15, c.r64(12));
            let r149 = c.shr(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r149);
            let r150 = c.sub(c.r32(15), 0x1u64, 32);
            c.w32(15, r150);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 147; continue; }
            blk = 141; continue;
        }
        141 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000884bc3); }
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 309; continue; }
            blk = 142; continue;
        }
        142 => {
            if c.cc_ne() { blk = 309; continue; }
            blk = 143; continue;
        }
        143 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            c.movq_rx(3, 0);
            if let Some(sr) = c.shr(c.r64(3), 0x20u64, 64) { c.w64(3, sr); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 309; continue; }
            blk = 145; continue;
        }
        145 => {
            c.w64(3, c.r64(7));
            let r151 = c.add(c.r32(3), c.r32(7), 32);
            c.w32(3, r151);
            if c.cc_o() { blk = 309; continue; }
            blk = 146; continue;
        }
        146 => {
            c.w64(7, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 148; continue;
        }
        147 => {
            c.w64(7, c.r64(0));
            blk = 148; continue;
        }
        148 => {
            c.w64(3, c.r64(7));
            let r152 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r152);
            c.w32(1, c.r32(7));
            let r153 = c.r32(15) & c.r32(3);
            c.w32(15, r153);
            c.logic(r153, 32);
            let r154 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r154);
            c.w64(6, c.r64(15).wrapping_mul(4));
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(7)) { return RegfileExit::fault(0x7000884c23); }
            if !c.st64(c.r64(5).wrapping_sub(0xa8), c.r64(15)) { return RegfileExit::fault(0x7000884c27); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(6)) { return RegfileExit::fault(0x7000884c2e); }
            let Some(t155) = c.ld32(c.r64(11).wrapping_add(c.r64(6)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000884c32); };
            c.cmp(t155, c.r32(7), 32);
            if c.cc_ne() { blk = 150; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t156) = c.ld32(c.r64(11).wrapping_add(c.r64(6)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000884c39); };
            c.w32(0, t156);
            let r157 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r157);
            blk = 187; continue;
        }
        150 => {
            c.cmp(c.r64(12), 0x8000u64, 64);
            if c.cc_l() { blk = 329; continue; }
            blk = 151; continue;
        }
        151 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            c.w8(12, (c.cc_l() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 154; continue; }
            blk = 152; continue;
        }
        152 => {
            c.w32(2, 0x80000000u64);
            c.cmp(c.r32(2), c.r32(3), 32);
            if c.cc_e() { blk = 329; continue; }
            blk = 153; continue;
        }
        153 => {
            let r158 = c.neg(c.r32(3), 32);
            c.w32(3, r158);
            blk = 155; continue;
        }
        154 => {
            c.cmp(c.r32(3), 0xau64, 32);
            if c.cc_l() { blk = 177; continue; }
            blk = 155; continue;
        }
        155 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 157; continue; }
            blk = 156; continue;
        }
        156 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 158; continue;
        }
        157 => {
            c.w32(0, 0x1u64);
            blk = 158; continue;
        }
        158 => {
            c.w64(9, c.r64(3));
            c.w64(8, c.r64(0));
            c.w32(0, 0x66666667u64);
            blk = 160; continue;
        }
        159 => {
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(9));
            c.imul1(c.r32(7), 32);
            let r159 = c.shr(c.r32(9), 0x1fu64, 32).unwrap_or(c.r32(9));
            c.w32(9, r159);
            let r160 = c.sar(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r160);
            let r161 = c.add(c.r32(9), c.r32(2), 32);
            c.w32(9, r161);
            let r162 = c.add(c.r32(8), 0x1u64, 32);
            c.w32(8, r162);
            c.w64(0, c.r64(7));
            blk = 160; continue;
        }
        160 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_g() { blk = 159; continue; }
            blk = 161; continue;
        }
        161 => {
            c.w64(7, c.r64(0));
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(2, 0xcu64);
            let r163 = c.add(c.r64(2), c.r64(0), 64);
            c.w64(2, r163);
            if c.cc_o() { blk = 262; continue; }
            blk = 162; continue;
        }
        162 => {
            let r164 = c.add(c.r64(2), 0x3u64, 64);
            c.w64(2, r164);
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000884cd1); };
            c.w64(9, t165);
            let Some(t166) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000884cd8); };
            c.w64(9, t166);
            let r167 = c.r64(2) & 0xfffffffffffffffcu64;
            c.w64(2, r167);
            c.logic(r167, 64);
            c.w64(6, c.r64(9).wrapping_add(c.r64(2)));
            let Some(t168) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000884ce3); };
            c.w64(15, t168);
            let Some(t169) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000884cea); };
            c.cmp(t169, c.r64(6), 64);
            if c.cc_be() { blk = 314; continue; }
            blk = 163; continue;
        }
        163 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 314; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000884d00); };
            c.w64(15, t170);
            if !c.st64(c.r64(15), c.r64(6)) { return RegfileExit::fault(0x7000884d07); }
            c.w64(15, c.r64(9).wrapping_add(0x1));
            blk = 165; continue;
        }
        165 => {
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x7000884d0e); };
            c.w64(6, t171);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000884d15); }
            if !c.st32(c.r64(15).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000884d19); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000884d21); }
            let r172 = c.sub(c.r64(0), 0x1u64, 64);
            c.w64(0, r172);
            c.w64(9, c.r64(3));
            c.w64(3, c.r64(0));
            blk = 167; continue;
        }
        166 => {
            if !c.st8(c.r64(15).wrapping_add(c.r64(3)).wrapping_add(0xb), c.r8(2)) { return RegfileExit::fault(0x7000884d31); }
            let r173 = c.sub(c.r64(3), 0x1u64, 64);
            c.w64(3, r173);
            c.w64(9, c.r64(0));
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_le() { blk = 172; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t174) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000884d43); };
            c.w64(0, (t174 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(3), c.r64(0), 64);
            if c.cc_ae() { blk = 261; continue; }
            blk = 169; continue;
        }
        169 => {
            c.w64(0, c.r64(7));
            c.imul1(c.r32(9), 32);
            c.w64(0, c.r64(9));
            let r175 = c.shr(c.r32(0), 0x1fu64, 32).unwrap_or(c.r32(0));
            c.w32(0, r175);
            let r176 = c.sar(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r176);
            let r177 = c.add(c.r32(0), c.r32(2), 32);
            c.w32(0, r177);
            let r178 = c.imul2(c.r32(0), 0xau64, 32);
            c.w32(2, r178);
            let r179 = c.sub(c.r32(9), c.r32(2), 32);
            c.w32(9, r179);
            c.cmp(c.r32(9), 0xau64, 32);
            if c.cc_l() { blk = 171; continue; }
            blk = 170; continue;
        }
        170 => {
            c.w32(2, c.r64(9).wrapping_add(0x57));
            blk = 166; continue;
        }
        171 => {
            c.w32(2, c.r64(9).wrapping_add(0x30));
            blk = 166; continue;
        }
        172 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 174; continue; }
            blk = 173; continue;
        }
        173 => {
            let r180 = c.shl(c.r32(8), 0x1au64, 32).unwrap_or(c.r32(8));
            c.w32(8, r180);
            c.w32(7, c.r64(1).wrapping_add(c.r64(1)));
            let r181 = c.r32(8) | c.r32(7);
            c.w32(8, r181);
            c.logic(r181, 32);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000884d89); }
            blk = 176; continue;
        }
        174 => {
            let Some(t182) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000884d8f); };
            c.w64(7, (t182 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_be() { blk = 260; continue; }
            blk = 175; continue;
        }
        175 => {
            if !c.st8(c.r64(15).wrapping_add(0xb), 0x2du64) { return RegfileExit::fault(0x7000884d9d); }
            blk = 176; continue;
        }
        176 => {
            c.w64(2, c.r64(15));
            blk = 180; continue;
        }
        177 => {
            c.w32(12, c.r64(3).wrapping_add(0x30));
            c.cmp(c.r32(12), 0xffu64, 32);
            if c.cc_g() { blk = 316; continue; }
            blk = 178; continue;
        }
        178 => {
            c.w64(0, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            let Some(t183) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x7000884dc0); };
            c.w64(3, t183);
            let Some(t184) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000884dc7); };
            c.w32(2, t184);
            let r185 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r185);
            let Some(t186) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000884dcf); };
            c.cmp(t186, c.r32(2), 32);
            if c.cc_e() { blk = 321; continue; }
            blk = 179; continue;
        }
        179 => {
            let Some(t187) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x7000884ddc); };
            c.w64(6, t187);
            blk = 180; continue;
        }
        180 => {
            let Some(t188) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000884de3); };
            c.w32(7, t188);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000884ded); };
            c.w64(12, t189);
            c.cmp(c.r64(12), c.r64(7), 64);
            if c.cc_ae() { blk = 413; continue; }
            blk = 181; continue;
        }
        181 => {
            c.w64(7, c.r64(12).wrapping_add(0x1));
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000884e02); };
            c.w64(12, t190);
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000884e06); };
            c.w64(15, t191);
            if !c.st32(c.r64(11).wrapping_add(c.r64(12)).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000884e0a); }
            let Some(t192) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000884e0f); };
            c.w32(12, t192);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(7), c.r64(12), 64);
            if c.cc_ae() { blk = 414; continue; }
            blk = 182; continue;
        }
        182 => {
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(11).wrapping_add(c.r64(7)), c.r32(2)) { return RegfileExit::fault(0x7000884e2a); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 184; continue; }
            blk = 183; continue;
        }
        183 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r193 = c.r64(12) & c.r64(11);
            c.w64(12, r193);
            c.logic(r193, 64);
            let Some(t194) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000884e3d); };
            c.logic(t194 & 0x4u64, 8);
            if c.cc_ne() { blk = 464; continue; }
            blk = 184; continue;
        }
        184 => {
            c.w64(0, c.r64(2));
            blk = 187; continue;
        }
        185 => {
            let Some(t195) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000884e4e); };
            c.cmp(t195, c.r32(0), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let Some(t196) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000884e5d); };
            c.cmp(t196, c.r32(0), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let r197 = c.r32(7) | c.r32(11);
            c.w32(7, r197);
            c.logic(r197, 32);
            if c.cc_ne() { blk = 112; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000884e75); };
            c.w64(6, t198);
            return c.call(0x700089c794, 0x7000884e7e, 0x7000884e79);
        }
        187 => {
            c.w64(7, c.r64(0));
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000884e81); };
            c.cmp(t199, 0x0u64, 64);
            if c.cc_e() { blk = 195; continue; }
            blk = 188; continue;
        }
        188 => {
            let Some(t200) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000884e8c); };
            c.cmp(t200, 0x0u64, 32);
            if c.cc_ne() { blk = 195; continue; }
            blk = 189; continue;
        }
        189 => {
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0xe0)) else { return RegfileExit::fault(0x7000884e92); };
            c.w64(11, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000884e99); };
            let r203 = c.imul2(c.r64(11), t202, 64);
            c.w64(11, r203);
            c.w64(0, c.r64(11));
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0xe0)) else { return RegfileExit::fault(0x7000884ea1); };
            c.w64(12, t204);
            c.cqo();
            c.idiv(c.r64(12), 64);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000884ead); };
            c.cmp(c.r64(0), t205, 64);
            if c.cc_ne() { blk = 427; continue; }
            blk = 190; continue;
        }
        190 => {
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000884eb7); };
            let r207 = c.add(c.r64(11), t206, 64);
            c.w64(11, r207);
            if c.cc_o() { blk = 425; continue; }
            blk = 191; continue;
        }
        191 => {
            c.cmp(c.r64(11), 0x1fffffe8u64, 64);
            if c.cc_g() { blk = 425; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000884ece); };
            c.cmp(t208, 0x1u64, 64);
            c.w8(15, (c.cc_g() as u64));
            c.w32(15, c.r8(15));
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000884edb); };
            c.cmp(t209, 0x0u64, 64);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            let r210 = c.r32(15) | c.r32(0);
            c.w32(15, r210);
            c.logic(r210, 32);
            if c.cc_ne() { blk = 331; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x7000884eef); };
            c.w64(0, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000884ef6); };
            c.w64(15, t212);
            blk = 194; continue;
        }
        194 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(15));
            c.w64(15, c.r64(10));
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 196; continue;
        }
        195 => {
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000884f0e); };
            c.w64(11, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000884f12); };
            c.w64(0, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x7000884f16); };
            c.w64(15, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0xe0)) else { return RegfileExit::fault(0x7000884f1d); };
            c.w64(12, t216);
            blk = 196; continue;
        }
        196 => {
            let Some(t217) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000884f24); };
            c.w32(3, t217);
            let r218 = c.add(c.r64(3), c.r64(0), 64);
            c.w64(3, r218);
            if c.cc_o() { blk = 437; continue; }
            blk = 197; continue;
        }
        197 => {
            c.cmp(c.r64(3), 0x1fffffe8u64, 64);
            if c.cc_g() { blk = 437; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t219) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000884f3d); };
            c.w32(0, t219);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(0));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(11), c.r64(2), 64);
            if c.cc_ge() { blk = 355; continue; }
            blk = 199; continue;
        }
        199 => {
            c.cmp(c.r64(11), c.r64(2), 64);
            if c.cc_ae() { blk = 263; continue; }
            blk = 200; continue;
        }
        200 => {
            c.w64(0, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(15).wrapping_add(c.r64(0)), c.r32(7)) { return RegfileExit::fault(0x7000884f64); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 202; continue; }
            blk = 201; continue;
        }
        201 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r220 = c.r64(2) & c.r64(15);
            c.w64(2, r220);
            c.logic(r220, 64);
            let Some(t221) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000884f78); };
            c.logic(t221 & 0x4u64, 8);
            if c.cc_ne() { blk = 461; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t222) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000884f82); };
            c.w32(7, t222);
            let r223 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r223);
            let Some(t224) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000884f88); };
            c.w32(7, t224);
            let r225 = c.r32(7) & 0x8u64;
            c.w32(7, r225);
            c.logic(r225, 32);
            c.cmp(c.r32(7), 0x8u64, 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let r226 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r226);
            let Some(t227) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000884f9e); };
            let r228 = c.r32(7) & t227;
            c.w32(7, r228);
            c.logic(r228, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000884fa1); }
            if !c.st64(c.r64(5).wrapping_sub(0xd8), c.r64(15)) { return RegfileExit::fault(0x7000884fa5); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000884fac); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000884fb0); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), 0x0u64) { return RegfileExit::fault(0x7000884fb4); }
            blk = 112; continue;
        }
        203 => {
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_e() { blk = 213; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t229) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000884fcb); };
            c.cmp(t229, 0x0u64, 32);
            if c.cc_ne() { blk = 213; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0xe0)) else { return RegfileExit::fault(0x7000884fd1); };
            c.w64(12, t230);
            let r231 = c.imul2(c.r64(12), c.r64(3), 64);
            c.w64(12, r231);
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(12));
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0xe0)) else { return RegfileExit::fault(0x7000884fe2); };
            c.w64(11, t232);
            c.cqo();
            c.idiv(c.r64(11), 64);
            c.cmp(c.r64(0), c.r64(3), 64);
            if c.cc_ne() { blk = 406; continue; }
            blk = 206; continue;
        }
        206 => {
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000884ff7); };
            let r234 = c.add(c.r64(12), t233, 64);
            c.w64(12, r234);
            if c.cc_o() { blk = 404; continue; }
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r64(12), 0x1fffffe8u64, 64);
            if c.cc_g() { blk = 404; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t235) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088500e); };
            c.w32(11, t235);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(11));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088501b); };
            c.w64(0, t236);
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_ge() { blk = 377; continue; }
            blk = 209; continue;
        }
        209 => {
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_b() { blk = 211; continue; }
            blk = 210; continue;
        }
        210 => {
            return RegfileExit::fault(0x700088502d);
        }
        211 => {
            c.w32(7, c.r64(3).wrapping_add(c.r64(3)));
            if !c.st32(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x7000885031); }
            blk = 212; continue;
        }
        212 => {
            c.w64(7, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(15));
            c.w64(15, c.r64(12));
            blk = 214; continue;
        }
        213 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000885042); };
            c.w64(15, t237);
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000885046); };
            c.w64(7, t238);
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 246; continue; }
            blk = 215; continue;
        }
        215 => {
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_e() { blk = 237; continue; }
            blk = 216; continue;
        }
        216 => {
            c.w32(11, c.r32(15));
            c.cmp(c.r32(11), 0x0u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t239) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088506d); };
            c.cmp(t239, 0x0u64, 32);
            if c.cc_ne() { blk = 227; continue; }
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 226; continue; }
            blk = 218; continue;
        }
        218 => {
            c.w64(12, c.r64(11));
            let r240 = c.add(c.r32(12), c.r32(11), 32);
            c.w32(12, r240);
            if c.cc_o() { blk = 225; continue; }
            blk = 219; continue;
        }
        219 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(12, 0xcu64);
            let r241 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r241);
            if c.cc_o() { blk = 224; continue; }
            blk = 220; continue;
        }
        220 => {
            let Some(t242) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000885093); };
            c.w64(12, t242);
            let Some(t243) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700088509a); };
            c.w64(12, t243);
            let r244 = c.add(c.r64(11), 0x3u64, 64);
            c.w64(11, r244);
            let r245 = c.r64(11) & 0xfffffffffffffffcu64;
            c.w64(11, r245);
            c.logic(r245, 64);
            c.w64(3, c.r64(12).wrapping_add(c.r64(11)));
            let Some(t246) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008850aa); };
            c.w64(2, t246);
            let Some(t247) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008850b1); };
            c.cmp(t247, c.r64(3), 64);
            if c.cc_be() { blk = 408; continue; }
            blk = 221; continue;
        }
        221 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 408; continue; }
            blk = 222; continue;
        }
        222 => {
            let Some(t248) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008850c7); };
            c.w64(11, t248);
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008850ce); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 223; continue;
        }
        223 => {
            let Some(t249) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008850d6); };
            c.w64(12, t249);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008850dd); }
            if !c.st32(c.r64(11).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008850e1); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008850e9); }
            blk = 235; continue;
        }
        224 => {
            return RegfileExit::fault(0x70008850f2);
        }
        225 => {
            return RegfileExit::fault(0x70008850f3);
        }
        226 => {
            let Some(t250) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008850f4); };
            c.w64(11, t250);
            blk = 235; continue;
        }
        227 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 234; continue; }
            blk = 228; continue;
        }
        228 => {
            c.w64(11, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w32(12, 0xcu64);
            let r251 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r251);
            if c.cc_o() { blk = 233; continue; }
            blk = 229; continue;
        }
        229 => {
            let Some(t252) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000885114); };
            c.w64(12, t252);
            let Some(t253) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700088511b); };
            c.w64(12, t253);
            let r254 = c.add(c.r64(11), 0x3u64, 64);
            c.w64(11, r254);
            let r255 = c.r64(11) & 0xfffffffffffffffcu64;
            c.w64(11, r255);
            c.logic(r255, 64);
            c.w64(3, c.r64(12).wrapping_add(c.r64(11)));
            let Some(t256) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088512b); };
            c.w64(2, t256);
            let Some(t257) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000885132); };
            c.cmp(t257, c.r64(3), 64);
            if c.cc_be() { blk = 410; continue; }
            blk = 230; continue;
        }
        230 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 410; continue; }
            blk = 231; continue;
        }
        231 => {
            let Some(t258) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000885148); };
            c.w64(11, t258);
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700088514f); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 232; continue;
        }
        232 => {
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x7000885157); };
            c.w64(12, t259);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700088515e); }
            if !c.st32(c.r64(11).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000885162); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700088516a); }
            blk = 235; continue;
        }
        233 => {
            return RegfileExit::fault(0x7000885170);
        }
        234 => {
            let Some(t260) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000885171); };
            c.w64(11, t260);
            blk = 235; continue;
        }
        235 => {
            c.w64(10, c.r64(4));
            let r261 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r261);
            let r262 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r262);
            c.logic(r262, 64);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000885183); }
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(11)) { return RegfileExit::fault(0x7000885188); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(0));
            c.w64(8, c.r64(7));
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0xe8)) else { return RegfileExit::fault(0x7000885196); };
            c.w64(9, t263);
            let Some(t264) = c.ld64(c.r64(13).wrapping_add(0x13a8)) else { return RegfileExit::fault(0x700088519d); };
            c.w64(0, t264);
            c.w64(10, 0x70008851abu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008851ab); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008851af); }
            return c.call(c.r64(0), 0x70008851b5, 0x70008851b3);
        }
        236 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008851b5); }
            let Some(t265) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x70008851bd); };
            c.w64(4, t265);
            c.w64(3, c.r64(0));
            blk = 247; continue;
        }
        237 => {
            let Some(t266) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008851c7); };
            c.w32(7, t266);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_a() { blk = 239; continue; }
            blk = 238; continue;
        }
        238 => {
            return RegfileExit::fault(0x70008851d6);
        }
        239 => {
            let Some(t267) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008851d7); };
            c.w32(3, t267);
            let r268 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r268);
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 240; continue;
        }
        240 => {
            let Some(t269) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008851ed); };
            c.w32(11, t269);
            let r270 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r270);
            let Some(t271) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008851f4); };
            c.cmp(t271, 0x3fu64, 16);
            if c.cc_be() { blk = 247; continue; }
            blk = 241; continue;
        }
        241 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 243; continue; }
            blk = 242; continue;
        }
        242 => {
            let Some(t272) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000885201); };
            c.w32(7, t272);
            let Some(t273) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000885204); };
            c.cmp(t273, c.r32(7), 32);
            if c.cc_ne() { blk = 245; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0xe8)) else { return RegfileExit::fault(0x700088520d); };
            c.w64(0, t274);
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000885214); };
            c.w64(6, t275);
            return c.call(0x70008d2374, 0x700088521d, 0x7000885218);
        }
        244 => {
            c.w64(3, c.r64(0));
            blk = 247; continue;
        }
        245 => {
            return RegfileExit::fault(0x7000885222);
        }
        246 => {
            let Some(t276) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000885223); };
            c.w64(0, t276);
            c.w64(3, c.r64(0));
            blk = 247; continue;
        }
        247 => {
            c.w64(0, c.r64(3));
            blk = 248; continue;
        }
        248 => {
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000885230); };
            c.w64(6, t277);
            let Some(t278) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000885234); };
            c.w32(7, t278);
            let r279 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r279);
            let Some(t280) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088523a); };
            c.w32(7, t280);
            let r281 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r281);
            let Some(t282) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000885240); };
            c.w32(11, t282);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700088524a); }
            c.cmp(c.r64(11), 0x2cu64, 64);
            if c.cc_a() { blk = 250; continue; }
            blk = 249; continue;
        }
        249 => {
            return RegfileExit::fault(0x7000885254);
        }
        250 => {
            let Some(t283) = c.ld32(c.r64(7).wrapping_add(0xb7)) else { return RegfileExit::fault(0x7000885255); };
            c.w32(7, t283);
            let r284 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r284);
            let Some(t285) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088525e); };
            c.cmp(t285, c.r32(7), 32);
            if c.cc_ne() { blk = 252; continue; }
            blk = 251; continue;
        }
        251 => {
            return RegfileExit::fault(0x7000885267);
        }
        252 => {
            let Some(t286) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000885268); };
            c.w32(11, t286);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_a() { blk = 254; continue; }
            blk = 253; continue;
        }
        253 => {
            return RegfileExit::fault(0x7000885278);
        }
        254 => {
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000885279); };
            c.w64(3, t287);
            let Some(t288) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088527d); };
            c.cmp(t288, c.r32(3), 32);
            if c.cc_ne() { blk = 256; continue; }
            blk = 255; continue;
        }
        255 => {
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_e() { blk = 258; continue; }
            blk = 256; continue;
        }
        256 => {
            c.w64(0, c.r64(7));
            return c.call(0x7000884314, 0x7000885290, 0x700088528b);
        }
        257 => {
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000885290); };
            c.w64(0, t289);
            blk = 259; continue;
        }
        258 => {
            let Some(t290) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000885296); };
            c.w64(11, t290);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700088529d); }
            blk = 259; continue;
        }
        259 => {
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008852a1); };
            c.w64(7, t291);
            let r292 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r292);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008852ac); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008852ad); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008852b3); }
            return c.ret(0x0, 0x70008852b5);
        }
        260 => {
            return RegfileExit::fault(0x70008852b6);
        }
        261 => {
            return RegfileExit::fault(0x70008852b7);
        }
        262 => {
            return RegfileExit::fault(0x70008852b8);
        }
        263 => {
            return RegfileExit::fault(0x70008852b9);
        }
        264 => {
            c.w32(7, 0x192u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008852bf); }
            let Some(t293) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008852c0); };
            c.w64(3, t293);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008852d1, 0x70008852cc);
        }
        265 => {
            return RegfileExit::fault(0x70008852d1);
        }
        266 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008852d2); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t294) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008852d8); };
            c.w64(3, t294);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008852e7, 0x70008852e2);
        }
        267 => {
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008852e7); };
            c.w64(6, t295);
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008852eb); };
            c.w64(0, t296);
            blk = 1; continue;
        }
        268 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a2054, 0x70008852fc, 0x70008852f7);
        }
        269 => {
            c.w64(11, c.r64(0));
            let Some(t297) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008852ff); };
            c.w64(6, t297);
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000885303); };
            c.w64(12, t298);
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0xe8)) else { return RegfileExit::fault(0x7000885307); };
            c.w64(7, t299);
            blk = 6; continue;
        }
        270 => {
            let Some(t300) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000885313); };
            c.w32(15, t300);
            let r301 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r301);
            c.w64(0, c.r64(15));
            blk = 25; continue;
        }
        271 => {
            let Some(t302) = c.ld32(c.r64(15).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x7000885322); };
            c.cmp(t302, c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t303) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700088532f); };
            c.w64(3, t303);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x700088533e, 0x7000885339);
        }
        273 => {
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0xe8)) else { return RegfileExit::fault(0x700088533e); };
            c.w64(7, t304);
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000885345); };
            c.w64(6, t305);
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000885349); };
            c.w64(11, t306);
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088534d); };
            c.w64(12, t307);
            blk = 25; continue;
        }
        274 => {
            c.w64(0, c.r64(7));
            return c.call(0x700089c794, 0x700088535e, 0x7000885359);
        }
        275 => {
            c.w64(7, c.r64(0));
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000885361); };
            c.w64(6, t308);
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000885365); };
            c.w64(11, t309);
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000885369); };
            c.w64(12, t310);
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x700088536d); };
            c.w64(0, t311);
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x7000885374); };
            c.w64(3, t312);
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088537b); };
            c.w64(15, t313);
            blk = 46; continue;
        }
        276 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700088538e, 0x7000885389);
        }
        277 => {
            c.w64(2, c.r64(0));
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000885391); };
            c.w64(6, t314);
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000885395); };
            c.w64(11, t315);
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000885399); };
            c.w64(12, t316);
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0xe8)) else { return RegfileExit::fault(0x700088539d); };
            c.w64(7, t317);
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x70008853a4); };
            c.w64(0, t318);
            let Some(t319) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x70008853ab); };
            c.w64(3, t319);
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008853b2); };
            c.w64(15, t320);
            blk = 55; continue;
        }
        278 => {
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t321) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008853bd); };
            c.w64(6, t321);
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008853c1); };
            c.w64(11, t322);
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008853c5); };
            c.w64(12, t323);
            let Some(t324) = c.ld64(c.r64(5).wrapping_sub(0xe8)) else { return RegfileExit::fault(0x70008853c9); };
            c.w64(7, t324);
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x70008853d0); };
            c.w64(0, t325);
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x70008853d7); };
            c.w64(3, t326);
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008853de); };
            c.w64(15, t327);
            blk = 71; continue;
        }
        279 => {
            let Some(t328) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008853e7); };
            c.w32(1, t328);
            let r329 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r329);
            let Some(t330) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008853ee); };
            c.w32(8, t330);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_a() { blk = 281; continue; }
            blk = 280; continue;
        }
        280 => {
            return RegfileExit::fault(0x70008853fe);
        }
        281 => {
            let Some(t331) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008853ff); };
            c.w32(1, t331);
            c.w32(1, c.r32(1));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 283; continue; }
            blk = 282; continue;
        }
        282 => {
            c.w32(15, 0x548u64);
            blk = 98; continue;
        }
        283 => {
            let Some(t332) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000885414); };
            c.cmp(t332, c.r32(7), 32);
            if c.cc_e() { blk = 306; continue; }
            blk = 284; continue;
        }
        284 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 287; continue; }
            blk = 285; continue;
        }
        285 => {
            let r333 = c.sub(c.r32(15), 0x2u64, 32);
            c.w32(15, r333);
            if c.cc_o() { blk = 287; continue; }
            blk = 286; continue;
        }
        286 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(15));
            blk = 301; continue;
        }
        287 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 289; continue; }
            blk = 288; continue;
        }
        288 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700088543c); }
            blk = 290; continue;
        }
        289 => {
            let r334 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r334);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 290; continue;
        }
        290 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(15, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(15) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700088546d); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 296; continue; }
            blk = 291; continue;
        }
        291 => {
            if c.cc_ne() { blk = 296; continue; }
            blk = 292; continue;
        }
        292 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 294; continue; }
            blk = 293; continue;
        }
        293 => {
            c.movq_rx(0, 0);
            if let Some(sr) = c.shr(c.r64(0), 0x20u64, 64) { c.w64(0, sr); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 296; continue; }
            blk = 294; continue;
        }
        294 => {
            c.w64(0, c.r64(15));
            let r335 = c.add(c.r32(0), c.r32(15), 32);
            c.w32(0, r335);
            if c.cc_o() { blk = 296; continue; }
            blk = 295; continue;
        }
        295 => {
            c.w64(15, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(15));
            blk = 301; continue;
        }
        296 => {
            let Some(t336) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088549e); };
            c.w64(15, t336);
            let Some(t337) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008854a5); };
            c.w64(0, t337);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t338) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008854ac); };
            c.w64(2, t338);
            let Some(t339) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008854b3); };
            c.cmp(t339, c.r64(3), 64);
            if c.cc_a() { blk = 299; continue; }
            blk = 297; continue;
        }
        297 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008854c2, 0x70008854bd);
        }
        298 => {
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0xe8)) else { return RegfileExit::fault(0x70008854c2); };
            c.w64(7, t340);
            let Some(t341) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008854c9); };
            c.w64(6, t341);
            let Some(t342) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008854cd); };
            c.w64(11, t342);
            let Some(t343) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008854d1); };
            c.w64(12, t343);
            blk = 300; continue;
        }
        299 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x70008854d7); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(15));
            blk = 300; continue;
        }
        300 => {
            let Some(t344) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008854e1); };
            c.w64(15, t344);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008854e8); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008854ec); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008854f1); }
            blk = 301; continue;
        }
        301 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 304; continue; }
            blk = 302; continue;
        }
        302 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(7));
            return c.call(0x70008d2374, 0x7000885505, 0x7000885500);
        }
        303 => {
            blk = 248; continue;
        }
        304 => {
            let Some(t345) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x700088550a); };
            c.w64(3, t345);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000885518, 0x7000885513);
        }
        305 => {
            return RegfileExit::fault(0x7000885518);
        }
        306 => {
            let Some(t346) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000885519); };
            c.w64(0, t346);
            blk = 248; continue;
        }
        307 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000885525); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x7000885531, 0x700088552c);
        }
        308 => {
            c.w64(7, c.r64(0));
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t347) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000885537); };
            c.w64(8, t347);
            let Some(t348) = c.ld64(c.r64(5).wrapping_sub(0xe0)) else { return RegfileExit::fault(0x700088553b); };
            c.w64(15, t348);
            blk = 110; continue;
        }
        309 => {
            let Some(t349) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000885547); };
            c.w32(7, t349);
            let Some(t350) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088554a); };
            c.w32(12, t350);
            c.w64(3, c.r64(7));
            let r351 = c.r32(3) ^ c.r32(12);
            c.w32(3, r351);
            c.logic(r351, 32);
            let r352 = c.r32(3) & c.r32(15);
            c.w32(3, r352);
            c.logic(r352, 32);
            c.w32(15, c.r64(3).wrapping_add(c.r64(3)));
            if let Some(sr) = c.shl(c.r64(15), 0x2u64, 64) { c.w64(15, sr); }
            let Some(t353) = c.ld32(c.r64(11).wrapping_add(c.r64(15)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088555f); };
            c.w32(3, t353);
            let r354 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r354);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 329; continue; }
            blk = 310; continue;
        }
        310 => {
            let Some(t355) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000885570); };
            c.w32(2, t355);
            let Some(t356) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000885573); };
            c.cmp(t356, c.r32(2), 32);
            if c.cc_ne() { blk = 329; continue; }
            blk = 311; continue;
        }
        311 => {
            let Some(t357) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000885580); };
            c.w32(2, t357);
            let Some(t358) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000885583); };
            c.cmp(t358, c.r32(7), 32);
            if c.cc_ne() { blk = 329; continue; }
            blk = 312; continue;
        }
        312 => {
            c.cmp(c.r32(2), c.r32(12), 32);
            if c.cc_ne() { blk = 329; continue; }
            blk = 313; continue;
        }
        313 => {
            let Some(t359) = c.ld32(c.r64(11).wrapping_add(c.r64(15)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000885595); };
            c.w32(0, t359);
            let r360 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r360);
            blk = 187; continue;
        }
        314 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x70008855a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(1)) { return RegfileExit::fault(0x70008855a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(8)) { return RegfileExit::fault(0x70008855aa); }
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(12)) { return RegfileExit::fault(0x70008855b1); }
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(3)) { return RegfileExit::fault(0x70008855b8); }
            if !c.st64(c.r64(5).wrapping_sub(0xa0), c.r64(0)) { return RegfileExit::fault(0x70008855bf); }
            return c.call(0x70007febb4, 0x70008855cb, 0x70008855c6);
        }
        315 => {
            c.w64(15, c.r64(0));
            let Some(t361) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008855ce); };
            c.w64(11, t361);
            let Some(t362) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008855d2); };
            c.w64(1, t362);
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008855d6); };
            c.w64(8, t363);
            let Some(t364) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008855dd); };
            c.w64(12, t364);
            c.w32(7, 0x66666667u64);
            let Some(t365) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008855e9); };
            c.w64(3, t365);
            let Some(t366) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008855f0); };
            c.w64(0, t366);
            blk = 165; continue;
        }
        316 => {
            let Some(t367) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008855fc); };
            c.w64(0, t367);
            let Some(t368) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000885603); };
            c.w64(3, t368);
            c.w64(2, c.r64(3).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x700088560a); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(12)) { return RegfileExit::fault(0x700088560e); }
            let Some(t369) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000885615); };
            c.w64(1, t369);
            let Some(t370) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700088561c); };
            c.cmp(t370, c.r64(2), 64);
            if c.cc_a() { blk = 319; continue; }
            blk = 317; continue;
        }
        317 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700088562b, 0x7000885626);
        }
        318 => {
            blk = 320; continue;
        }
        319 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x700088562d); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 320; continue;
        }
        320 => {
            let Some(t371) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x7000885634); };
            c.w64(7, t371);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700088563b); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x700088563e); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000885645); }
            let Some(t372) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700088564c); };
            c.w64(12, t372);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(12)) { return RegfileExit::fault(0x7000885653); }
            c.w64(2, c.r64(0));
            let Some(t373) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088565b); };
            c.w64(11, t373);
            let Some(t374) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x700088565f); };
            c.w64(6, t374);
            let Some(t375) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000885663); };
            c.w64(7, t375);
            let Some(t376) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000885667); };
            c.w64(15, t376);
            blk = 179; continue;
        }
        321 => {
            let Some(t377) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000885673); };
            c.w64(2, t377);
            let Some(t378) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700088567a); };
            c.w64(1, t378);
            c.w64(8, c.r64(1).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x7000885681); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(0)) { return RegfileExit::fault(0x7000885685); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(12)) { return RegfileExit::fault(0x7000885689); }
            let Some(t379) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000885690); };
            c.w64(9, t379);
            let Some(t380) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000885697); };
            c.cmp(t380, c.r64(8), 64);
            if c.cc_a() { blk = 324; continue; }
            blk = 322; continue;
        }
        322 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008856a6, 0x70008856a1);
        }
        323 => {
            blk = 325; continue;
        }
        324 => {
            if !c.st64(c.r64(2), c.r64(8)) { return RegfileExit::fault(0x70008856a8); }
            c.w64(2, c.r64(1).wrapping_add(0x1));
            c.w64(0, c.r64(2));
            blk = 325; continue;
        }
        325 => {
            let Some(t381) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008856b2); };
            c.w64(7, t381);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008856b9); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008856bc); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008856c3); }
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008856ca); };
            c.w64(11, t382);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(11)) { return RegfileExit::fault(0x70008856d1); }
            let Some(t383) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008856d5); };
            c.w64(3, t383);
            let Some(t384) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008856dc); };
            c.w32(12, t384);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ae() { blk = 412; continue; }
            blk = 326; continue;
        }
        326 => {
            let Some(t385) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008856ef); };
            c.w64(11, t385);
            if !c.st32(c.r64(3).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008856f3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 328; continue; }
            blk = 327; continue;
        }
        327 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r386 = c.r64(12) & c.r64(3);
            c.w64(12, r386);
            c.logic(r386, 64);
            let Some(t387) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000885705); };
            c.logic(t387 & 0x4u64, 8);
            if c.cc_ne() { blk = 458; continue; }
            blk = 328; continue;
        }
        328 => {
            c.w64(2, c.r64(0));
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000885714); };
            c.w64(11, t388);
            let Some(t389) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000885718); };
            c.w64(6, t389);
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700088571c); };
            c.w64(7, t390);
            let Some(t391) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000885720); };
            c.w64(15, t391);
            blk = 179; continue;
        }
        329 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088572c); }
            let Some(t392) = c.ld64(c.r64(13).wrapping_add(0x22d8)) else { return RegfileExit::fault(0x700088572d); };
            c.w64(3, t392);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000885740, 0x700088573b);
        }
        330 => {
            blk = 187; continue;
        }
        331 => {
            let Some(t393) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x7000885745); };
            c.w64(0, t393);
            let Some(t394) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088574c); };
            c.w32(15, t394);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x7000885759); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700088575d); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(15)) { return RegfileExit::fault(0x7000885761); }
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000885765); };
            c.w64(2, t395);
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_l() { blk = 352; continue; }
            blk = 332; continue;
        }
        332 => {
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_b() { blk = 416; continue; }
            blk = 333; continue;
        }
        333 => {
            c.w64(1, c.r64(3));
            if let Some(sr) = c.shr(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r396 = c.add(c.r64(3), c.r64(1), 64);
            c.w64(3, r396);
            let r397 = c.add(c.r64(3), 0x10u64, 64);
            c.w64(3, r397);
            if c.cc_e() { blk = 349; continue; }
            blk = 334; continue;
        }
        334 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 421; continue; }
            blk = 335; continue;
        }
        335 => {
            let r398 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r398);
            c.w64(1, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(1));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w32(6, c.r32(6));
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(3)) { return RegfileExit::fault(0x70008857a9); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(1)) { return RegfileExit::fault(0x70008857b0); }
            let r399 = c.imul2(c.r32(6), 0x4u64, 32);
            c.w32(6, r399);
            if c.cc_o() { blk = 420; continue; }
            blk = 336; continue;
        }
        336 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w32(8, 0x8u64);
            let r400 = c.add(c.r64(6), c.r64(8), 64);
            c.w64(6, r400);
            if c.cc_o() { blk = 419; continue; }
            blk = 337; continue;
        }
        337 => {
            let Some(t401) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008857cf); };
            c.w64(9, t401);
            let Some(t402) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008857d6); };
            c.w64(9, t402);
            c.w64(7, c.r64(9).wrapping_add(c.r64(6)));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(9)) { return RegfileExit::fault(0x70008857dd); }
            let Some(t403) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008857e1); };
            c.w64(9, t403);
            let Some(t404) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008857e8); };
            c.cmp(t404, c.r64(7), 64);
            if c.cc_be() { blk = 339; continue; }
            blk = 338; continue;
        }
        338 => {
            c.cmp(c.r64(6), 0x20000u64, 64);
            if c.cc_b() { blk = 341; continue; }
            blk = 339; continue;
        }
        339 => {
            c.w64(2, c.r64(6));
            return c.call(0x70007febb4, 0x70008857fe, 0x70008857f9);
        }
        340 => {
            c.w64(7, c.r64(0));
            let Some(t405) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x7000885801); };
            c.w64(0, t405);
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000885808); };
            c.w64(2, t406);
            c.w32(8, 0x8u64);
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000885812); };
            c.w64(11, t407);
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000885816); };
            c.w64(15, t408);
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x700088581a); };
            c.w64(1, t409);
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700088581e); };
            c.w64(3, t410);
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0xe0)) else { return RegfileExit::fault(0x7000885825); };
            c.w64(12, t411);
            blk = 342; continue;
        }
        341 => {
            let Some(t412) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088582e); };
            c.w64(6, t412);
            if !c.st64(c.r64(6), c.r64(7)) { return RegfileExit::fault(0x7000885835); }
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000885838); };
            c.w64(7, t413);
            let r414 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r414);
            blk = 342; continue;
        }
        342 => {
            let Some(t415) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000885840); };
            c.w64(6, t415);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000885847); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x700088584a); }
            c.w64(3, c.r64(1));
            c.w64(3, c.r64(3).wrapping_mul(2).wrapping_add(0x8));
            c.w64(1, c.r64(15));
            c.w64(15, c.r64(8));
            c.w64(6, c.r64(0));
            c.w64(0, c.r64(8));
            blk = 343; continue;
        }
        343 => {
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 348; continue; }
            blk = 344; continue;
        }
        344 => {
            c.w64(9, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(9), c.r64(0), 64);
            if c.cc_e() { blk = 346; continue; }
            blk = 345; continue;
        }
        345 => {
            let Some(t416) = c.ld32(c.r64(6).wrapping_add(c.r64(0)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000885876); };
            c.w32(9, t416);
            let r417 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r417);
            blk = 347; continue;
        }
        346 => {
            let Some(t418) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000885881); };
            c.w64(9, t418);
            blk = 347; continue;
        }
        347 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(15)).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000885888); }
            let r419 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r419);
            blk = 343; continue;
        }
        348 => {
            c.w64(0, c.r64(7));
            blk = 350; continue;
        }
        349 => {
            let Some(t420) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000885898); };
            c.w64(0, t420);
            c.w32(8, 0x8u64);
            blk = 350; continue;
        }
        350 => {
            let Some(t421) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008858a5); };
            c.w32(7, t421);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(15, c.r64(2));
            c.cmp(c.r64(15), c.r64(7), 64);
            if c.cc_ae() { blk = 423; continue; }
            blk = 351; continue;
        }
        351 => {
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008858ba); };
            c.w64(7, t422);
            let r423 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r423);
            if !c.st32(c.r64(0).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008858c0); }
            c.w64(7, c.r64(15));
            c.w64(15, c.r64(0));
            blk = 354; continue;
        }
        352 => {
            c.w64(15, c.r64(0));
            c.w64(1, c.r64(2));
            c.cmp(c.r64(1), c.r64(3), 64);
            if c.cc_ae() { blk = 424; continue; }
            blk = 353; continue;
        }
        353 => {
            let Some(t424) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008858dc); };
            c.w64(3, t424);
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            if !c.st32(c.r64(15).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008858e3); }
            c.w32(8, 0x8u64);
            c.w64(7, c.r64(1));
            blk = 354; continue;
        }
        354 => {
            let r425 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r425);
            c.w64(0, c.r64(15));
            c.w64(15, c.r64(7));
            let Some(t426) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008858fb); };
            c.w64(7, t426);
            blk = 194; continue;
        }
        355 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000885904); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000885908); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700088590c); }
            if !c.st64(c.r64(5).wrapping_sub(0xd8), c.r64(15)) { return RegfileExit::fault(0x7000885910); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000885917); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_b() { blk = 429; continue; }
            blk = 356; continue;
        }
        356 => {
            c.w64(1, c.r64(2));
            if let Some(sr) = c.shr(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r427 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r427);
            let r428 = c.add(c.r64(2), 0x10u64, 64);
            c.w64(2, r428);
            if c.cc_e() { blk = 372; continue; }
            blk = 357; continue;
        }
        357 => {
            c.cmp(c.r64(2), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 434; continue; }
            blk = 358; continue;
        }
        358 => {
            let r429 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r429);
            c.w64(1, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(1));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w32(6, c.r32(6));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(2)) { return RegfileExit::fault(0x7000885952); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(1)) { return RegfileExit::fault(0x7000885956); }
            let r430 = c.imul2(c.r32(6), 0x4u64, 32);
            c.w32(6, r430);
            if c.cc_o() { blk = 433; continue; }
            blk = 359; continue;
        }
        359 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w32(8, 0x8u64);
            let r431 = c.add(c.r64(6), c.r64(8), 64);
            c.w64(6, r431);
            if c.cc_o() { blk = 432; continue; }
            blk = 360; continue;
        }
        360 => {
            let Some(t432) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000885975); };
            c.w64(9, t432);
            let Some(t433) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700088597c); };
            c.w64(9, t433);
            c.w64(7, c.r64(9).wrapping_add(c.r64(6)));
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(9)) { return RegfileExit::fault(0x7000885983); }
            let Some(t434) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000885987); };
            c.w64(9, t434);
            let Some(t435) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700088598e); };
            c.cmp(t435, c.r64(7), 64);
            if c.cc_be() { blk = 362; continue; }
            blk = 361; continue;
        }
        361 => {
            c.cmp(c.r64(6), 0x20000u64, 64);
            if c.cc_b() { blk = 364; continue; }
            blk = 362; continue;
        }
        362 => {
            c.w64(2, c.r64(6));
            return c.call(0x70007febb4, 0x70008859a4, 0x700088599f);
        }
        363 => {
            c.w64(7, c.r64(0));
            c.w32(8, 0x8u64);
            let Some(t436) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008859ad); };
            c.w64(3, t436);
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008859b1); };
            c.w64(11, t437);
            let Some(t438) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x70008859b5); };
            c.w64(15, t438);
            let Some(t439) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008859bc); };
            c.w64(0, t439);
            let Some(t440) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008859c0); };
            c.w64(1, t440);
            let Some(t441) = c.ld64(c.r64(5).wrapping_sub(0xe0)) else { return RegfileExit::fault(0x70008859c4); };
            c.w64(12, t441);
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008859cb); };
            c.w64(2, t442);
            blk = 365; continue;
        }
        364 => {
            let Some(t443) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008859d1); };
            c.w64(6, t443);
            if !c.st64(c.r64(6), c.r64(7)) { return RegfileExit::fault(0x70008859d8); }
            let Some(t444) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008859db); };
            c.w64(7, t444);
            let r445 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r445);
            blk = 365; continue;
        }
        365 => {
            let Some(t446) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008859e3); };
            c.w64(6, t446);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008859ea); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008859ed); }
            c.w64(2, c.r64(1));
            c.w64(2, c.r64(2).wrapping_mul(2).wrapping_add(0x8));
            c.w64(1, c.r64(8));
            c.w64(6, c.r64(8));
            blk = 366; continue;
        }
        366 => {
            c.cmp(c.r64(2), c.r64(1), 64);
            if c.cc_e() { blk = 371; continue; }
            blk = 367; continue;
        }
        367 => {
            c.w64(9, c.r64(0).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(9), c.r64(6), 64);
            if c.cc_e() { blk = 369; continue; }
            blk = 368; continue;
        }
        368 => {
            let Some(t447) = c.ld32(c.r64(15).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000885a13); };
            c.w32(9, t447);
            let r448 = c.add(c.r64(6), 0x4u64, 64);
            c.w64(6, r448);
            blk = 370; continue;
        }
        369 => {
            let Some(t449) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000885a1e); };
            c.w64(9, t449);
            blk = 370; continue;
        }
        370 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000885a25); }
            let r450 = c.add(c.r64(1), 0x4u64, 64);
            c.w64(1, r450);
            blk = 366; continue;
        }
        371 => {
            c.w64(0, c.r64(7));
            blk = 373; continue;
        }
        372 => {
            let Some(t451) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000885a35); };
            c.w64(0, t451);
            c.w32(8, 0x8u64);
            blk = 373; continue;
        }
        373 => {
            let Some(t452) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000885a42); };
            c.w32(7, t452);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 436; continue; }
            blk = 374; continue;
        }
        374 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t453) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000885a5c); };
            c.w64(1, t453);
            if !c.st32(c.r64(0).wrapping_add(c.r64(7)), c.r32(1)) { return RegfileExit::fault(0x7000885a60); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 376; continue; }
            blk = 375; continue;
        }
        375 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r454 = c.r64(15) & c.r64(0);
            c.w64(15, r454);
            c.logic(r454, 64);
            let Some(t455) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000885a72); };
            c.logic(t455 & 0x4u64, 8);
            if c.cc_ne() { blk = 455; continue; }
            blk = 376; continue;
        }
        376 => {
            c.w64(15, c.r64(0));
            c.w64(7, c.r64(1));
            blk = 202; continue;
        }
        377 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000885a88); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000885a8c); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x7000885a90); }
            if !c.st64(c.r64(5).wrapping_sub(0xd8), c.r64(15)) { return RegfileExit::fault(0x7000885a94); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_ae() { blk = 381; continue; }
            blk = 378; continue;
        }
        378 => {
            let Some(t456) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000885aa1); };
            c.w64(2, t456);
            let Some(t457) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x7000885aa8); };
            c.w32(2, t457);
            let r458 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r458);
            return c.call(0x7000824234, 0x7000885ab6, 0x7000885ab1);
        }
        379 => {
            return RegfileExit::fault(0x7000885ab6);
        }
        380 => {
            return RegfileExit::fault(0x7000885ab7);
        }
        381 => {
            c.w64(1, c.r64(2));
            if let Some(sr) = c.shr(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r459 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r459);
            let r460 = c.add(c.r64(2), 0x10u64, 64);
            c.w64(2, r460);
            if c.cc_e() { blk = 400; continue; }
            blk = 382; continue;
        }
        382 => {
            c.cmp(c.r64(2), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 398; continue; }
            blk = 383; continue;
        }
        383 => {
            c.w32(7, c.r64(2).wrapping_add(c.r64(2)));
            c.w64(2, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(2));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w32(1, c.r32(1));
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x7000885ae6); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x7000885aea); }
            let r461 = c.imul2(c.r32(1), 0x4u64, 32);
            c.w32(1, r461);
            if c.cc_o() { blk = 397; continue; }
            blk = 384; continue;
        }
        384 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w32(8, 0x8u64);
            let r462 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r462);
            if c.cc_o() { blk = 396; continue; }
            blk = 385; continue;
        }
        385 => {
            let Some(t463) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000885b09); };
            c.w64(9, t463);
            let Some(t464) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000885b10); };
            c.w64(9, t464);
            c.w64(7, c.r64(9).wrapping_add(c.r64(1)));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x7000885b17); }
            let Some(t465) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000885b1b); };
            c.w64(9, t465);
            let Some(t466) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000885b22); };
            c.cmp(t466, c.r64(7), 64);
            if c.cc_be() { blk = 387; continue; }
            blk = 386; continue;
        }
        386 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_b() { blk = 389; continue; }
            blk = 387; continue;
        }
        387 => {
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x7000885b38, 0x7000885b33);
        }
        388 => {
            c.w64(7, c.r64(0));
            let Some(t467) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000885b3b); };
            c.w64(6, t467);
            let Some(t468) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x7000885b3f); };
            c.w64(15, t468);
            let Some(t469) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000885b46); };
            c.w64(0, t469);
            let Some(t470) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000885b4a); };
            c.w64(12, t470);
            let Some(t471) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000885b4e); };
            c.w64(3, t471);
            let Some(t472) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000885b52); };
            c.w64(11, t472);
            c.w32(8, 0x8u64);
            let Some(t473) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000885b5c); };
            c.w64(2, t473);
            blk = 390; continue;
        }
        389 => {
            let Some(t474) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000885b62); };
            c.w64(1, t474);
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x7000885b69); }
            let Some(t475) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000885b6c); };
            c.w64(7, t475);
            let r476 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r476);
            blk = 390; continue;
        }
        390 => {
            let Some(t477) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000885b74); };
            c.w64(1, t477);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000885b7b); }
            let Some(t478) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000885b7e); };
            c.w64(1, t478);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000885b82); }
            c.w64(2, c.r64(2).wrapping_mul(2).wrapping_add(0x8));
            c.w64(1, c.r64(15));
            c.w64(15, c.r64(8));
            blk = 391; continue;
        }
        391 => {
            c.cmp(c.r64(2), c.r64(8), 64);
            if c.cc_e() { blk = 401; continue; }
            blk = 392; continue;
        }
        392 => {
            c.w64(9, c.r64(11).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_e() { blk = 394; continue; }
            blk = 393; continue;
        }
        393 => {
            let Some(t479) = c.ld32(c.r64(1).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000885ba5); };
            c.w32(9, t479);
            let r480 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r480);
            blk = 395; continue;
        }
        394 => {
            let Some(t481) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000885bb0); };
            c.w64(9, t481);
            blk = 395; continue;
        }
        395 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000885bb7); }
            let r482 = c.add(c.r64(8), 0x4u64, 64);
            c.w64(8, r482);
            blk = 391; continue;
        }
        396 => {
            return RegfileExit::fault(0x7000885bc2);
        }
        397 => {
            return RegfileExit::fault(0x7000885bc3);
        }
        398 => {
            let Some(t483) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000885bc4); };
            c.w64(3, t483);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000885bd5, 0x7000885bd0);
        }
        399 => {
            return RegfileExit::fault(0x7000885bd5);
        }
        400 => {
            let Some(t484) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000885bd6); };
            c.w64(7, t484);
            blk = 401; continue;
        }
        401 => {
            let Some(t485) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000885bdd); };
            c.w32(11, t485);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(15, c.r64(0));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_b() { blk = 403; continue; }
            blk = 402; continue;
        }
        402 => {
            return RegfileExit::fault(0x7000885bef);
        }
        403 => {
            c.w32(11, c.r64(3).wrapping_add(c.r64(3)));
            if !c.st32(c.r64(7).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000885bf4); }
            c.w64(0, c.r64(15));
            c.w64(15, c.r64(7));
            blk = 212; continue;
        }
        404 => {
            let Some(t486) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000885c04); };
            c.w64(3, t486);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t487) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000885c0d); };
            c.w64(6, t487);
            return c.call(0x700086ed74, 0x7000885c16, 0x7000885c11);
        }
        405 => {
            return RegfileExit::fault(0x7000885c16);
        }
        406 => {
            let Some(t488) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000885c17); };
            c.w64(3, t488);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t489) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000885c20); };
            c.w64(6, t489);
            return c.call(0x700086ed74, 0x7000885c29, 0x7000885c24);
        }
        407 => {
            return RegfileExit::fault(0x7000885c29);
        }
        408 => {
            if !c.st64(c.r64(5).wrapping_sub(0xd8), c.r64(0)) { return RegfileExit::fault(0x7000885c2a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000885c31); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x7000885c35); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x7000885c41, 0x7000885c3c);
        }
        409 => {
            c.w64(11, c.r64(0));
            let Some(t490) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000885c44); };
            c.w64(6, t490);
            let Some(t491) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x7000885c48); };
            c.w64(0, t491);
            let Some(t492) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000885c4f); };
            c.w64(7, t492);
            let Some(t493) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000885c53); };
            c.w64(15, t493);
            blk = 223; continue;
        }
        410 => {
            if !c.st64(c.r64(5).wrapping_sub(0xd8), c.r64(0)) { return RegfileExit::fault(0x7000885c5c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000885c63); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x7000885c67); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x7000885c73, 0x7000885c6e);
        }
        411 => {
            c.w64(11, c.r64(0));
            let Some(t494) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000885c76); };
            c.w64(6, t494);
            let Some(t495) = c.ld64(c.r64(5).wrapping_sub(0xd8)) else { return RegfileExit::fault(0x7000885c7a); };
            c.w64(0, t495);
            let Some(t496) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000885c81); };
            c.w64(7, t496);
            let Some(t497) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000885c85); };
            c.w64(15, t497);
            blk = 232; continue;
        }
        412 => {
            return RegfileExit::fault(0x7000885c8e);
        }
        413 => {
            return RegfileExit::fault(0x7000885c8f);
        }
        414 => {
            return RegfileExit::fault(0x7000885c90);
        }
        415 => {
            blk = 441; continue;
        }
        416 => {
            let Some(t498) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000885c96); };
            c.w64(2, t498);
            let Some(t499) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x7000885c9d); };
            c.w32(2, t499);
            let r500 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r500);
            return c.call(0x7000824234, 0x7000885cab, 0x7000885ca6);
        }
        417 => {
            return RegfileExit::fault(0x7000885cab);
        }
        418 => {
            return RegfileExit::fault(0x7000885cac);
        }
        419 => {
            return RegfileExit::fault(0x7000885cad);
        }
        420 => {
            return RegfileExit::fault(0x7000885cae);
        }
        421 => {
            let Some(t501) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000885caf); };
            c.w64(3, t501);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000885cbf, 0x7000885cba);
        }
        422 => {
            return RegfileExit::fault(0x7000885cbf);
        }
        423 => {
            return RegfileExit::fault(0x7000885cc0);
        }
        424 => {
            return RegfileExit::fault(0x7000885cc1);
        }
        425 => {
            let Some(t502) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000885cc2); };
            c.w64(3, t502);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t503) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000885ccb); };
            c.w64(6, t503);
            return c.call(0x700086ed74, 0x7000885cd4, 0x7000885ccf);
        }
        426 => {
            return RegfileExit::fault(0x7000885cd4);
        }
        427 => {
            let Some(t504) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000885cd5); };
            c.w64(3, t504);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t505) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000885cde); };
            c.w64(6, t505);
            return c.call(0x700086ed74, 0x7000885ce7, 0x7000885ce2);
        }
        428 => {
            return RegfileExit::fault(0x7000885ce7);
        }
        429 => {
            let Some(t506) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000885ce8); };
            c.w64(2, t506);
            let Some(t507) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x7000885cef); };
            c.w32(2, t507);
            let r508 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r508);
            return c.call(0x7000824234, 0x7000885cfd, 0x7000885cf8);
        }
        430 => {
            return RegfileExit::fault(0x7000885cfd);
        }
        431 => {
            return RegfileExit::fault(0x7000885cfe);
        }
        432 => {
            return RegfileExit::fault(0x7000885cff);
        }
        433 => {
            return RegfileExit::fault(0x7000885d00);
        }
        434 => {
            let Some(t509) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000885d01); };
            c.w64(3, t509);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000885d11, 0x7000885d0c);
        }
        435 => {
            return RegfileExit::fault(0x7000885d11);
        }
        436 => {
            return RegfileExit::fault(0x7000885d12);
        }
        437 => {
            let Some(t510) = c.ld64(c.r64(13).wrapping_add(0x21e0)) else { return RegfileExit::fault(0x7000885d13); };
            c.w64(3, t510);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t511) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000885d1c); };
            c.w64(6, t511);
            return c.call(0x700086ed74, 0x7000885d25, 0x7000885d20);
        }
        438 => {
            return RegfileExit::fault(0x7000885d25);
        }
        439 => {
            let Some(t512) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000885d26); };
            c.w64(3, t512);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(9));
            return c.call(0x700086ed74, 0x7000885d37, 0x7000885d32);
        }
        440 => {
            blk = 454; continue;
        }
        441 => {
            let Some(t513) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000885d3c); };
            c.w64(6, t513);
            let Some(t514) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000885d40); };
            c.w32(7, t514);
            let r515 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r515);
            let Some(t516) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000885d46); };
            c.w32(7, t516);
            let r517 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r517);
            let Some(t518) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000885d4c); };
            c.w32(11, t518);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000885d56); }
            c.cmp(c.r64(11), 0x2cu64, 64);
            if c.cc_a() { blk = 443; continue; }
            blk = 442; continue;
        }
        442 => {
            return RegfileExit::fault(0x7000885d60);
        }
        443 => {
            let Some(t519) = c.ld32(c.r64(7).wrapping_add(0xb7)) else { return RegfileExit::fault(0x7000885d61); };
            c.w32(7, t519);
            let r520 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r520);
            let Some(t521) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000885d6a); };
            c.cmp(t521, c.r32(7), 32);
            if c.cc_ne() { blk = 445; continue; }
            blk = 444; continue;
        }
        444 => {
            return RegfileExit::fault(0x7000885d73);
        }
        445 => {
            let Some(t522) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000885d74); };
            c.w32(11, t522);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_a() { blk = 447; continue; }
            blk = 446; continue;
        }
        446 => {
            return RegfileExit::fault(0x7000885d84);
        }
        447 => {
            let Some(t523) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000885d85); };
            c.w64(3, t523);
            let Some(t524) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000885d89); };
            c.cmp(t524, c.r32(3), 32);
            if c.cc_ne() { blk = 449; continue; }
            blk = 448; continue;
        }
        448 => {
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_e() { blk = 451; continue; }
            blk = 449; continue;
        }
        449 => {
            c.w64(0, c.r64(7));
            return c.call(0x7000884314, 0x7000885d9c, 0x7000885d97);
        }
        450 => {
            let Some(t525) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000885d9c); };
            c.w64(6, t525);
            let Some(t526) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000885da0); };
            c.w64(0, t526);
            blk = 452; continue;
        }
        451 => {
            let Some(t527) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000885da6); };
            c.w64(11, t527);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000885dad); }
            blk = 452; continue;
        }
        452 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000885db1); }
            let Some(t528) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x7000885db2); };
            c.w64(3, t528);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x7000885dc3, 0x7000885dbe);
        }
        453 => {
            return RegfileExit::fault(0x7000885dc3);
        }
        454 => {
            return RegfileExit::fault(0x7000885dc4);
        }
        455 => {
            c.w32(1, c.r32(1));
            let r529 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r529);
            c.w64(15, 0xfffffffffffc0000u64);
            let r530 = c.r64(15) & c.r64(1);
            c.w64(15, r530);
            c.logic(r530, 64);
            let Some(t531) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000885dd4); };
            c.logic(t531 & 0x2u64, 8);
            if c.cc_e() { blk = 376; continue; }
            blk = 456; continue;
        }
        456 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000885de3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000885de4); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x7000885df1, 0x7000885dec);
        }
        457 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000885df1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000885df3); }
            blk = 376; continue;
        }
        458 => {
            c.w32(0, c.r32(0));
            let r532 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r532);
            c.w64(12, 0xfffffffffffc0000u64);
            let r533 = c.r64(12) & c.r64(0);
            c.w64(12, r533);
            c.logic(r533, 64);
            let Some(t534) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000885e08); };
            c.logic(t534 & 0x2u64, 8);
            if c.cc_e() { blk = 328; continue; }
            blk = 459; continue;
        }
        459 => {
            c.w64(15, c.r64(3).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000885e18); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000885e19); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000885e26, 0x7000885e21);
        }
        460 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000885e26); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000885e28); }
            blk = 328; continue;
        }
        461 => {
            c.w32(7, c.r32(7));
            let r535 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r535);
            c.w64(2, 0xfffffffffffc0000u64);
            let r536 = c.r64(2) & c.r64(7);
            c.w64(2, r536);
            c.logic(r536, 64);
            let Some(t537) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000885e3d); };
            c.logic(t537 & 0x2u64, 8);
            if c.cc_e() { blk = 202; continue; }
            blk = 462; continue;
        }
        462 => {
            c.w64(1, c.r64(15).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000885e4b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000885e4c); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x7000885e59, 0x7000885e54);
        }
        463 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000885e59); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000885e5b); }
            blk = 202; continue;
        }
        464 => {
            c.w32(2, c.r32(2));
            let r538 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r538);
            c.w64(12, 0xfffffffffffc0000u64);
            let r539 = c.r64(12) & c.r64(2);
            c.w64(12, r539);
            c.logic(r539, 64);
            let Some(t540) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000885e70); };
            c.logic(t540 & 0x2u64, 8);
            if c.cc_e() { blk = 184; continue; }
            blk = 465; continue;
        }
        465 => {
            c.w64(15, c.r64(11).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000885e80); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000885e81); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000885e8e, 0x7000885e89);
        }
        466 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000885e8e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000885e90); }
            blk = 184; continue;
        }
        467 => {
            c.w32(11, c.r32(11));
            let r541 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r541);
            c.w64(1, 0xfffffffffffc0000u64);
            let r542 = c.r64(1) & c.r64(11);
            c.w64(1, r542);
            c.logic(r542, 64);
            let Some(t543) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000885ea6); };
            c.logic(t543 & 0x2u64, 8);
            if c.cc_e() { blk = 70; continue; }
            blk = 468; continue;
        }
        468 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000885eb4); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000885ebd, 0x7000885eb8);
        }
        469 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000885ebd); }
            blk = 70; continue;
        }
        470 => {
            c.w32(2, c.r32(2));
            let r544 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r544);
            c.w64(8, 0xfffffffffffc0000u64);
            let r545 = c.r64(8) & c.r64(2);
            c.w64(8, r545);
            c.logic(r545, 64);
            let Some(t546) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000885ed2); };
            c.logic(t546 & 0x2u64, 8);
            if c.cc_e() { blk = 61; continue; }
            blk = 471; continue;
        }
        471 => {
            c.w64(9, c.r64(1).wrapping_add(0xb7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000885ee4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000885ee5); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000885ef2, 0x7000885eed);
        }
        472 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000885ef2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000885ef4); }
            blk = 61; continue;
        }
        473 => {
            return RegfileExit::jump(0x7000885efc);
        }
            _ => unreachable!(),
        }
    }
}
